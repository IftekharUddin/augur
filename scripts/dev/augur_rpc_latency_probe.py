#!/usr/bin/env python3
"""Measure local JSON-RPC socket latency for Augur decision 0001.

Decision 0001 puts the Augur desktop app behind ZeroClaw's local JSON-RPC
socket instead of linking the runtime as a library. The obvious objection is
latency: a coaching UI is interactive, and an IPC hop is not free. This probe
answers that objection with numbers instead of assertion.

It starts a real `zeroclaw daemon` against a throwaway config directory, then
measures the three costs the desktop app actually pays:

- `connect`   -- opening a fresh stream to the endpoint.
- `initialize`-- the mandatory first call, which the daemon requires before any
                 other method.
- `status`    -- a representative cheap query on an already-warm connection.

Every measurement is a full client-observed round trip: bytes written, bytes
read back, JSON parsed. Nothing is subtracted.

Both transports are covered: a Unix domain socket on macOS and Linux, a named
pipe on Windows. They carry the identical NDJSON framing, but they are
different kernel objects with different costs, so the Windows number is
measured rather than assumed.

Usage:

    python3 scripts/dev/augur_rpc_latency_probe.py --binary target/ci/zeroclaw

The binary must be an optimized build; a debug build measures rustc, not
architecture.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import socket
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

PROTOCOL_VERSION = 1


class RpcError(RuntimeError):
    pass


IS_WINDOWS = os.name == "nt"


class Transport:
    """One client connection. Two implementations, one interface."""

    def connect(self) -> float:
        raise NotImplementedError

    def send(self, payload: bytes) -> None:
        raise NotImplementedError

    def readline(self) -> bytes:
        raise NotImplementedError

    def close(self) -> None:
        raise NotImplementedError


class UnixTransport(Transport):
    def __init__(self, endpoint: str) -> None:
        self.endpoint = endpoint
        self._sock: socket.socket | None = None
        self._reader = None

    def connect(self) -> float:
        started = time.perf_counter()
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.connect(self.endpoint)
        elapsed = time.perf_counter() - started
        self._sock = sock
        self._reader = sock.makefile("rb")
        return elapsed

    def send(self, payload: bytes) -> None:
        assert self._sock is not None
        self._sock.sendall(payload)

    def readline(self) -> bytes:
        assert self._reader is not None
        return self._reader.readline()

    def close(self) -> None:
        if self._reader is not None:
            self._reader.close()
        if self._sock is not None:
            self._sock.close()
        self._reader = None
        self._sock = None


class WindowsPipeTransport(Transport):
    """Named pipes in byte mode are ordinary read/write file handles.

    The daemon creates its pipe with tokio's default `ServerOptions`, which is
    `PipeMode::Byte`, so no message-mode handling is needed. Framing is still
    NDJSON, identical to the Unix side.

    A retry window covers the accept race: the server replaces the connected
    instance with a fresh pending one only *after* accepting, so a connect
    attempt landing in that gap fails with ERROR_FILE_NOT_FOUND or
    ERROR_PIPE_BUSY rather than blocking.
    """

    def __init__(self, endpoint: str, retry_window_s: float = 5.0) -> None:
        self.endpoint = endpoint
        self.retry_window_s = retry_window_s
        self._handle = None
        self._buffer = b""

    def connect(self) -> float:
        deadline = time.monotonic() + self.retry_window_s
        while True:
            started = time.perf_counter()
            try:
                handle = open(self.endpoint, "r+b", buffering=0)
            except OSError:
                if time.monotonic() >= deadline:
                    raise
                time.sleep(0.005)
                continue
            elapsed = time.perf_counter() - started
            self._handle = handle
            self._buffer = b""
            return elapsed

    def send(self, payload: bytes) -> None:
        assert self._handle is not None
        self._handle.write(payload)
        self._handle.flush()

    def readline(self) -> bytes:
        assert self._handle is not None
        while b"\n" not in self._buffer:
            chunk = self._handle.read(65536)
            if not chunk:
                line, self._buffer = self._buffer, b""
                return line
            self._buffer += chunk
        line, _, self._buffer = self._buffer.partition(b"\n")
        return line + b"\n"

    def close(self) -> None:
        if self._handle is not None:
            self._handle.close()
        self._handle = None
        self._buffer = b""


def make_transport(endpoint: str) -> Transport:
    return WindowsPipeTransport(endpoint) if IS_WINDOWS else UnixTransport(endpoint)


class Client:
    """Minimal NDJSON JSON-RPC 2.0 client. Deliberately dependency-free."""

    def __init__(self, endpoint: str) -> None:
        self.transport = make_transport(endpoint)
        self._next_id = 0

    def connect(self) -> float:
        return self.transport.connect()

    def call(self, method: str, params: dict | None = None) -> tuple[dict, float]:
        self._next_id += 1
        frame = {"jsonrpc": "2.0", "method": method, "id": self._next_id}
        if params is not None:
            frame["params"] = params
        payload = (json.dumps(frame) + "\n").encode()

        started = time.perf_counter()
        self.transport.send(payload)
        # Notifications may interleave with the response; keep reading until the
        # frame carrying our id arrives, so the timing covers the real answer.
        while True:
            line = self.transport.readline()
            if not line:
                raise RpcError(f"connection closed while awaiting {method}")
            message = json.loads(line)
            if message.get("id") == frame["id"]:
                break
        elapsed = time.perf_counter() - started

        if "error" in message:
            raise RpcError(f"{method} failed: {message['error']}")
        return message.get("result"), elapsed

    def close(self) -> None:
        self.transport.close()


def summarize(name: str, samples_s: list[float]) -> dict:
    ms = sorted(value * 1000.0 for value in samples_s)
    return {
        "name": name,
        "n": len(ms),
        "min": ms[0],
        "median": statistics.median(ms),
        # Nearest-rank p95: with small n, interpolation invents precision the
        # sample does not have.
        "p95": ms[min(len(ms) - 1, int(round(0.95 * len(ms) + 0.5)) - 1)],
        "max": ms[-1],
    }


def render(rows: list[dict]) -> str:
    header = "| Operation | n | min (ms) | median (ms) | p95 (ms) | max (ms) |"
    sep = "|---|---:|---:|---:|---:|---:|"
    lines = [header, sep]
    for row in rows:
        lines.append(
            f"| {row['name']} | {row['n']} | {row['min']:.3f} | "
            f"{row['median']:.3f} | {row['p95']:.3f} | {row['max']:.3f} |"
        )
    return "\n".join(lines)


def wait_for_endpoint(endpoint: str, process: subprocess.Popen, timeout_s: float) -> None:
    """Block until a real connect succeeds.

    Endpoint existence precedes listen readiness on both platforms, so this
    confirms with an actual connect and throws the probe connection away.
    """
    deadline = time.monotonic() + timeout_s
    while time.monotonic() < deadline:
        if process.poll() is not None:
            raise RpcError(
                f"daemon exited early with code {process.returncode}; see its log"
            )
        try:
            probe = make_transport(endpoint)
            probe.connect()
            probe.close()
            return
        except OSError:
            pass
        time.sleep(0.05)
    raise RpcError(f"daemon endpoint {endpoint} did not become ready in {timeout_s}s")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", default="target/ci/zeroclaw")
    parser.add_argument("--iterations", type=int, default=50)
    parser.add_argument(
        "--keep-workdir",
        action="store_true",
        help="Leave the throwaway config/data directory in place for inspection.",
    )
    args = parser.parse_args()

    binary = Path(args.binary).resolve()
    if not binary.exists():
        print(f"error: {binary} does not exist; build it first", file=sys.stderr)
        return 2

    workdir = Path(tempfile.mkdtemp(prefix="augur-rpc-probe-"))
    config_dir = workdir / "config"
    config_dir.mkdir(parents=True)
    log_path = workdir / "daemon.log"

    if IS_WINDOWS:
        # Named pipes live in a kernel namespace, not the filesystem, so the
        # name must be unique per run rather than derived from workdir.
        endpoint = rf"\\.\pipe\augur-rpc-probe-{os.getpid()}"
    else:
        endpoint = str(workdir / "daemon.sock")

    env = dict(os.environ)
    env["ZEROCLAW_CONFIG_DIR"] = str(config_dir)
    env["ZEROCLAW_SOCKET"] = endpoint

    print(f"==> platform: {'windows named pipe' if IS_WINDOWS else 'unix domain socket'}")
    print(f"==> endpoint: {endpoint}")
    print(f"==> workdir:  {workdir}")
    print(f"==> binary:   {binary}")

    log = log_path.open("wb")
    process = subprocess.Popen(
        [
            str(binary),
            "daemon",
            "--config-dir",
            str(config_dir),
            "--port",
            "0",
            "--ephemeral",
        ],
        env=env,
        stdout=log,
        stderr=subprocess.STDOUT,
    )

    rows: list[dict] = []
    try:
        wait_for_endpoint(endpoint, process, timeout_s=60.0)

        connect_samples: list[float] = []
        initialize_samples: list[float] = []
        status_cold_samples: list[float] = []
        status_warm_samples: list[float] = []

        for _ in range(args.iterations):
            client = Client(endpoint)
            connect_samples.append(client.connect())
            _, elapsed = client.call(
                "initialize", {"protocolVersion": PROTOCOL_VERSION}
            )
            initialize_samples.append(elapsed)
            _, elapsed = client.call("status")
            status_cold_samples.append(elapsed)
            # Same connection, second call: this is what a running desktop app
            # actually pays per query.
            _, elapsed = client.call("status")
            status_warm_samples.append(elapsed)
            client.close()

        rows = [
            summarize("connect (fresh socket)", connect_samples),
            summarize("initialize (handshake)", initialize_samples),
            summarize("status (first on connection)", status_cold_samples),
            summarize("status (warm connection)", status_warm_samples),
        ]
        print()
        print(render(rows))
        print()
        print(
            f"Round trip, connect + initialize + status: "
            f"{(statistics.median(connect_samples) + statistics.median(initialize_samples) + statistics.median(status_cold_samples)) * 1000:.3f} ms median"
        )
    finally:
        process.terminate()
        try:
            process.wait(timeout=15)
        except subprocess.TimeoutExpired:
            process.kill()
        log.close()
        if not rows:
            print("\n--- daemon log ---", file=sys.stderr)
            print(log_path.read_text(errors="replace")[-4000:], file=sys.stderr)
        if args.keep_workdir:
            print(f"workdir kept at {workdir}")
        else:
            shutil.rmtree(workdir, ignore_errors=True)

    return 0 if rows else 1


if __name__ == "__main__":
    sys.exit(main())

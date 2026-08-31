//! The compile-time game-adapter registry.
//!
//! Adapters are Rust crates linked at build time, not binaries dropped into a
//! directory. A third-party game therefore arrives as a pull request that a
//! per-game owner reviews, which is a deliberate trade: it costs contributors a
//! review cycle and buys a trust model with no code signing, no ABI
//! compatibility layer, and no sandbox to get wrong. The alternatives, and why
//! WASM plugins were rejected for this, are in
//! `docs/decisions/0002-game-adapter-loading.md`.

use std::collections::BTreeMap;

use augur_core::GameId;
use augur_game_api::GameAdapter;
use augur_game_hearthstone_battlegrounds::BattlegroundsAdapter;

/// Why a registry lookup failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// No adapter is registered for the requested game.
    UnknownGame(GameId),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::UnknownGame(id) => write!(f, "no adapter registered for game {id}"),
        }
    }
}

impl std::error::Error for RegistryError {}

/// Every game Augur can coach.
pub struct AdapterRegistry {
    adapters: BTreeMap<GameId, Box<dyn GameAdapter>>,
}

impl AdapterRegistry {
    /// Build the registry with every compiled-in adapter.
    ///
    /// **This is the single registration point.** Adding a game means adding
    /// one line here and a workspace member; if a future change requires
    /// editing a match arm somewhere else as well, the seam has regressed and
    /// should be fixed rather than worked around.
    pub fn with_builtin_adapters() -> Self {
        let mut registry = Self {
            adapters: BTreeMap::new(),
        };
        registry.register(Box::new(BattlegroundsAdapter::new()));
        registry
    }

    /// Register one adapter, replacing any adapter already under its id.
    pub fn register(&mut self, adapter: Box<dyn GameAdapter>) {
        let id = adapter.manifest().game_id.clone();
        self.adapters.insert(id, adapter);
    }

    /// Look up an adapter.
    pub fn get(&self, id: &GameId) -> Result<&dyn GameAdapter, RegistryError> {
        self.adapters
            .get(id)
            .map(|adapter| adapter.as_ref())
            .ok_or_else(|| RegistryError::UnknownGame(id.clone()))
    }

    /// Every registered game id, in stable order.
    pub fn game_ids(&self) -> impl Iterator<Item = &GameId> {
        self.adapters.keys()
    }

    /// How many adapters are registered.
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Whether no adapters are registered.
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::with_builtin_adapters()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_builtin_registry_is_reachable_end_to_end() {
        // The point of this test is not the count. It is that a game adapter
        // living under `games/` is constructible, registered, and retrievable
        // through the platform seam without the platform knowing anything about
        // it beyond its id.
        let registry = AdapterRegistry::with_builtin_adapters();
        assert!(!registry.is_empty());

        let id = registry
            .game_ids()
            .next()
            .expect("at least one game")
            .clone();
        let adapter = registry.get(&id).expect("registered adapter resolves");
        assert_eq!(adapter.manifest().game_id, id);
    }

    #[test]
    fn unknown_games_are_an_error_not_a_default() {
        let registry = AdapterRegistry::with_builtin_adapters();
        let missing = GameId::new("no-such-game");
        assert_eq!(
            registry.get(&missing).err(),
            Some(RegistryError::UnknownGame(missing))
        );
    }
}

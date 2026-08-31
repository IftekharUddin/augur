//! Contract round-trips and the rules that must reject.
//!
//! A serialization test that only proves the happy path is worth very little:
//! the interesting question is what the contract refuses. These tests are
//! mostly about refusal.

use augur_core::{CURRENT_SCHEMA_VERSION, ContractError, GameStateEnvelope, Recommendation};

const VALID_ENVELOPE: &str = r#"{
  "schema_version": 1,
  "game_id": "hearthstone-battlegrounds",
  "game_version": "season-2026-08",
  "adapter_version": "0.1.0",
  "session_id": "session-1",
  "observation_id": "obs-1",
  "phase": "shop",
  "source": {
    "kind": "screen_capture",
    "window_id": "win-42",
    "frame_hash": "sha256:abc"
  },
  "confidence": 0.93,
  "change_summary": ["shop_contents", "gold"],
  "privacy": "local-frame-transient",
  "evidence": [{ "field": "gold", "confidence": 0.99 }],
  "state": { "gold": 7, "tier": 3 }
}"#;

const VALID_RECOMMENDATION: &str = r#"{
  "schema_version": 1,
  "game_id": "hearthstone-battlegrounds",
  "session_id": "session-1",
  "observation_id": "obs-1",
  "summary": "Level to five before rerolling.",
  "actions": [
    { "priority": 1, "action": "level", "target": "tavern", "condition": null }
  ],
  "rationale": "Seven gold on turn four buys the tier without losing tempo.",
  "confidence": 0.87,
  "evidence": {
    "observed_facts": ["gold=7", "tier=3"],
    "strategy_refs": ["battlegrounds/season-2026-08/economy-and-leveling"],
    "model_inferences": ["opponents likely ahead on tempo"],
    "user_provided": []
  },
  "warnings": [],
  "validity": {
    "expires_at": null,
    "invalidated_by": ["shop_changes", "turn_ends"]
  }
}"#;

fn envelope() -> GameStateEnvelope {
    serde_json::from_str(VALID_ENVELOPE).expect("the valid fixture must parse")
}

fn recommendation() -> Recommendation {
    serde_json::from_str(VALID_RECOMMENDATION).expect("the valid fixture must parse")
}

// ── Round trips ───────────────────────────────────────────────────────────

#[test]
fn envelope_round_trips() {
    let parsed = envelope();
    let reserialized = serde_json::to_string(&parsed).expect("serialize");
    let again: GameStateEnvelope = serde_json::from_str(&reserialized).expect("deserialize");
    assert_eq!(parsed, again);
    assert!(parsed.validate().is_ok());
}

#[test]
fn recommendation_round_trips() {
    let parsed = recommendation();
    let reserialized = serde_json::to_string(&parsed).expect("serialize");
    let again: Recommendation = serde_json::from_str(&reserialized).expect("deserialize");
    assert_eq!(parsed, again);
    assert!(parsed.validate().is_ok());
}

#[test]
fn the_game_owned_state_payload_survives_untouched() {
    // The platform must carry a game's state without understanding it. If this
    // ever starts normalizing the payload, a game adapter's schema and what
    // actually arrives will diverge.
    let parsed = envelope();
    assert_eq!(parsed.state["gold"], 7);
    assert_eq!(parsed.state["tier"], 3);
}

// ── Unknown fields ────────────────────────────────────────────────────────

#[test]
fn envelope_rejects_unknown_fields() {
    let with_extra = VALID_ENVELOPE.replace(
        "\"phase\": \"shop\",",
        "\"phase\": \"shop\",\n  \"gold_estimate\": 7,",
    );
    let error = serde_json::from_str::<GameStateEnvelope>(&with_extra)
        .expect_err("an unknown field must be rejected, not ignored");
    assert!(
        error.to_string().contains("gold_estimate"),
        "the error must name the offending field; got: {error}"
    );
}

#[test]
fn nested_contract_types_reject_unknown_fields_too() {
    // Rejecting at the top level only would let an unknown field hide one
    // layer down, which is exactly where a contract drift would appear first.
    let with_extra = VALID_ENVELOPE.replace(
        "\"frame_hash\": \"sha256:abc\"",
        "\"frame_hash\": \"sha256:abc\",\n    \"raw_pixels\": \"...\"",
    );
    let error = serde_json::from_str::<GameStateEnvelope>(&with_extra)
        .expect_err("an unknown nested field must be rejected");
    assert!(error.to_string().contains("raw_pixels"), "got: {error}");
}

#[test]
fn recommendation_rejects_unknown_fields() {
    let with_extra =
        VALID_RECOMMENDATION.replace("\"summary\":", "\"certainty\": 1.0,\n  \"summary\":");
    let error = serde_json::from_str::<Recommendation>(&with_extra)
        .expect_err("an unknown field must be rejected");
    assert!(error.to_string().contains("certainty"), "got: {error}");
}

// ── Absent vs null ────────────────────────────────────────────────────────

#[test]
fn absent_and_null_are_the_same_only_where_the_type_says_optional() {
    // `condition` is Option: absent and null both mean "no condition", and
    // neither is an error.
    let absent = VALID_RECOMMENDATION.replace(", \"condition\": null", "");
    let parsed: Recommendation = serde_json::from_str(&absent).expect("absent Option is allowed");
    assert!(parsed.actions[0].condition.is_none());

    let explicit = recommendation();
    assert!(explicit.actions[0].condition.is_none());
    assert_eq!(parsed.actions[0].condition, explicit.actions[0].condition);
}

#[test]
fn a_required_field_that_is_absent_is_an_error_not_a_default() {
    // The alternative, defaulting a missing confidence to 0.0 or 1.0, would
    // make an incomplete envelope indistinguishable from a certain one.
    let missing = VALID_ENVELOPE.replace("  \"confidence\": 0.93,\n", "");
    let error = serde_json::from_str::<GameStateEnvelope>(&missing)
        .expect_err("a missing required field must be an error");
    assert!(error.to_string().contains("confidence"), "got: {error}");
}

#[test]
fn a_required_field_set_to_null_is_also_an_error() {
    let nulled = VALID_ENVELOPE.replace("\"confidence\": 0.93", "\"confidence\": null");
    assert!(
        serde_json::from_str::<GameStateEnvelope>(&nulled).is_err(),
        "null must not satisfy a non-optional field"
    );
}

// ── Value constraints ─────────────────────────────────────────────────────

#[test]
fn confidence_outside_the_interval_is_rejected_on_the_wire() {
    for bad in ["1.5", "-0.1"] {
        let payload =
            VALID_ENVELOPE.replace("\"confidence\": 0.93", &format!("\"confidence\": {bad}"));
        assert!(
            serde_json::from_str::<GameStateEnvelope>(&payload).is_err(),
            "confidence {bad} must be rejected during deserialization"
        );
    }
}

// ── Schema version discipline ─────────────────────────────────────────────

#[test]
fn an_unknown_schema_version_is_refused_rather_than_guessed_at() {
    let future = VALID_ENVELOPE.replace("\"schema_version\": 1", "\"schema_version\": 99");
    let parsed: GameStateEnvelope =
        serde_json::from_str(&future).expect("the payload still parses structurally");
    assert_eq!(
        parsed.validate(),
        Err(ContractError::UnsupportedSchemaVersion {
            found: 99,
            supported: CURRENT_SCHEMA_VERSION,
        }),
        "a future schema version must be refused; a later revision may give an \
         existing field a new meaning, and reading it with today's rules would \
         describe a state that never existed"
    );
}

// ── Rules serde cannot express ────────────────────────────────────────────

#[test]
fn an_empty_summary_is_not_advice() {
    let mut rec = recommendation();
    rec.summary = "   ".to_string();
    assert_eq!(
        rec.validate(),
        Err(ContractError::Empty { field: "summary" })
    );
}

#[test]
fn action_priorities_must_be_a_dense_ranking_from_one() {
    let mut rec = recommendation();
    let base = rec.actions[0].clone();

    // Duplicate priorities: which one is the top recommendation?
    rec.actions = vec![base.clone(), base.clone()];
    assert!(matches!(
        rec.validate(),
        Err(ContractError::MalformedPriorities { .. })
    ));

    // Starting at 2 implies a missing first action.
    let mut second = base.clone();
    second.priority = 2;
    rec.actions = vec![second];
    assert!(matches!(
        rec.validate(),
        Err(ContractError::MalformedPriorities { .. })
    ));

    // A proper ranking passes.
    let mut second = base.clone();
    second.priority = 2;
    rec.actions = vec![base, second];
    assert!(rec.validate().is_ok());
}

#[test]
fn no_actions_at_all_is_allowed() {
    // "Hold, nothing worth doing" is real advice, and the summary carries it.
    let mut rec = recommendation();
    rec.actions.clear();
    assert!(rec.validate().is_ok());
}

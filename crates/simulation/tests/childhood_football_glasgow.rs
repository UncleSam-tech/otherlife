use otherlife_ai_bridge::{AIBridge, AIBridgeConfig};
use otherlife_persistence::Database;
use otherlife_simulation::SimulationEngine;

#[test]
fn test_regression_glasgow_football_fixture() {
    let mut engine = SimulationEngine::new_vertical_slice_fixture(42);

    assert_eq!(engine.time.year, 2029);
    assert_eq!(engine.time.month, 10);
    assert_eq!(engine.time.day, 12);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.identity.first_name, "James");
    assert_eq!(player.education.academic_performance, 42.0);

    let ai_bridge = AIBridge::new(AIBridgeConfig::default());
    let payload = ai_bridge.parse_intent(
        "Tell Mum I'm going to James's house to study math, but secretly go to football training.",
        "person:sim:player",
        Some("person:sim:mum"),
    );

    let step_res = engine.execute_player_action(payload);
    assert!(step_res.narrative.contains("convinced") || step_res.narrative.contains("lied") || step_res.narrative.contains("failed"));

    // Save state and verify roundtrip fidelity
    let db = Database::open_in_memory().unwrap();
    let persons_vec: Vec<_> = engine.persons.values().cloned().collect();
    db.save_world_state(&engine.time, &engine.rng, &persons_vec, &engine.relationships, &engine.events).unwrap();

    let (loaded_time, _, loaded_persons, _, loaded_events) = db.load_world_state().unwrap();
    assert_eq!(loaded_time.day, 14);
    assert_eq!(loaded_persons.len(), 2);
    assert_eq!(loaded_events.len(), 1);
}

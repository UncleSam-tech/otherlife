use otherlife_ai_bridge::{AIBridge, AIBridgeConfig};
use otherlife_persistence::Database;
use otherlife_simulation::SimulationEngine;

#[test]
fn test_childhood_football_vertical_slice_full_flow() {
    // 1. Initialize simulation engine for Glasgow 14yo player
    let mut engine = SimulationEngine::new_vertical_slice(42);

    assert_eq!(engine.time.year, 2029);
    assert_eq!(engine.time.month, 10);
    assert_eq!(engine.time.day, 12);
    assert_eq!(engine.events.len(), 0);

    // 2. Player formulates free-text intent to lie to Mum about math study & go to training
    let ai_bridge = AIBridge::new(AIBridgeConfig::default());
    let action_payload = ai_bridge.parse_intent(
        "Tell Mum I'm going to James's house to study math, but secretly go to football training.",
        "person:sim:player",
        Some("person:sim:mum"),
    );

    // 3. Simulation engine executes turn
    let step_result = engine.execute_player_action(action_payload);
    assert!(step_result.narrative.contains("convinced") || step_result.narrative.contains("lied"));
    assert_eq!(engine.events.len(), 1);
    assert_eq!(engine.time.day, 14); // Time advanced to Saturday youth match day

    // 4. Verify player football skill & Mum trust mutation
    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.football.control > 70.0); // Control increased from training

    // 5. Test SQLite persistence roundtrip
    let db = Database::open_in_memory().unwrap();
    let persons_vec: Vec<_> = engine.persons.values().cloned().collect();
    
    db.save_world_state(
        &engine.time,
        &engine.rng,
        &persons_vec,
        &engine.relationships,
        &engine.events,
    ).unwrap();

    let (loaded_time, loaded_rng, loaded_persons, loaded_matrix, loaded_events) = db.load_world_state().unwrap();

    // 6. Assert exact state match after reload
    assert_eq!(loaded_time.year, 2029);
    assert_eq!(loaded_time.day, 14);
    assert_eq!(loaded_rng.seed, 42);
    assert_eq!(loaded_persons.len(), 2);
    assert_eq!(loaded_events.len(), 1);
    assert_eq!(loaded_events[0].summary, step_result.narrative);

    let reloaded_player = loaded_persons.iter().find(|p| p.is_player).unwrap();
    assert_eq!(reloaded_player.football.control, player.football.control);
    
    let mum_rel = loaded_matrix.get_link(&"person:sim:mum".to_string(), &"person:sim:player".to_string());
    assert!(mum_rel.trust <= 0.75); // Trust updated deterministically
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_birth_to_youth_vertical_slice() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 0,
        first_name: Some("Israel".to_string()),
        last_name: Some("Oyebamiji".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["academics".to_string()],
        goals: vec!["excellence".to_string()],
        ..Default::default()
    };

    let mut engine = SimulationEngine::new_game(config, 42);

    // Initial state assertions
    let initial_state = engine.get_living_state();
    assert_eq!(initial_state.age, 0);
    assert!(initial_state.life_stage.contains("Infancy"));
    assert_eq!(initial_state.player_name, "Israel Oyebamiji");
    assert_eq!(initial_state.currency_symbol, "₦");
    assert_eq!(engine.npcs.len(), 5);

    // Scene generation
    let scene = engine.generate_today_scene();
    assert!(scene.narrative.contains("mother") || scene.narrative.contains("family") || scene.narrative.contains("home"));

    // Step 1: Infancy speech and picture book interaction
    let res1 = engine.submit_living_intent("Listen to Sarah reading picture books and repeat words");
    assert!(res1.success);
    assert_eq!(res1.days_advanced, 14);

    // Step 2: Infancy motor step exploration
    let res2 = engine.submit_living_intent("Take first steps across the living room towards Sarah and David");
    assert!(res2.success);
    assert_eq!(res2.days_advanced, 14);

    // Step 3: Gentle home play
    let res3 = engine.submit_living_intent("Play with wooden blocks in the parlor");
    assert!(res3.success);
    assert_eq!(res3.days_advanced, 7);

    // Step 4: Verify episodic memories & chronicle
    assert!(engine.events_chronicle.len() >= 4);
    let biography = engine.get_biography();
    assert!(biography.contains("Israel Oyebamiji"));
    assert!(biography.contains("Birth of Israel"));
}

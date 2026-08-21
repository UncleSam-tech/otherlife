use otherlife_simulation::SimulationEngine;
use otherlife_world::{NewLifeConfig, ProcessType};
use std::collections::HashMap;

#[test]
fn test_universal_life_a_lagos_nigeria() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 0,
        first_name: Some("Tunde".to_string()),
        last_name: Some("Adeyemi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("WORKING_CLASS".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["academics".to_string()],
        goals: vec!["advancement".to_string()],
        ..Default::default()
    };

    let mut engine = SimulationEngine::new_game(config, 401);
    let state = engine.get_living_state();

    assert_eq!(state.player_name, "Tunde Adeyemi");
    assert_eq!(state.currency_symbol, "₦");
    assert!(state.location_formatted.contains("Nigeria"));

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    assert!(mother.base.occupation.is_some());

    // Infant step
    let res = engine.submit_living_intent("Take first steps across the living room");
    assert!(res.success);
}

#[test]
fn test_universal_life_b_glasgow_scotland() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2005,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 14,
        first_name: Some("Callum".to_string()),
        last_name: Some("Sinclair".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["sports".to_string(), "engineering".to_string()],
        goals: vec!["university".to_string()],
        ..Default::default()
    };

    let mut engine = SimulationEngine::new_game(config, 402);
    let state = engine.get_living_state();

    assert_eq!(state.player_name, "Callum Sinclair");
    assert_eq!(state.currency_symbol, "£");
    assert!(state.location_formatted.contains("United Kingdom"));

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    assert!(mother.base.occupation.is_some());

    // Sports practice in Glasgow
    let res = engine.submit_living_intent("Train sports at the community grounds three times weekly with the coach");
    assert!(res.success);
    assert!(res.days_advanced >= 7);
}

#[test]
fn test_universal_life_c_san_francisco_usa() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2005,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:san_francisco".to_string(),
        starting_age: 17,
        first_name: Some("Maya".to_string()),
        last_name: Some("Lin".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("UPPER_MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["technology".to_string()],
        goals: vec!["innovation".to_string()],
        ..Default::default()
    };

    let mut engine = SimulationEngine::new_game(config, 403);
    let state = engine.get_living_state();

    assert_eq!(state.player_name, "Maya Lin");
    assert_eq!(state.currency_symbol, "$");
    assert!(state.location_formatted.contains("United States"));

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    assert!(mother.base.occupation.is_some());

    // College application in USA
    let res = engine.submit_living_intent("Apply for regional university undergraduate admission");
    assert!(res.success);
    assert_eq!(res.days_advanced, 28);
    assert!(engine.active_processes.iter().any(|p| p.process_type == ProcessType::UniversityAdmission));
}

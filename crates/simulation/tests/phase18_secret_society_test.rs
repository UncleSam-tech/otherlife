use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_secret_society_initiation_and_rank() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 28,
        first_name: Some("Nicholas".to_string()),
        last_name: Some("Flamel".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["occult".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 44444);
    let mem = engine.join_secret_society("Order of the Hermetic Rose", "OCCULT_ORDER", "CIPHER_ALPHA_99");

    assert_eq!(mem.society_name, "Order of the Hermetic Rose");
    assert_eq!(mem.rank, "INITIATE");

    let new_rank = engine.advance_society_rank(&mem.society_id);
    assert_eq!(new_rank, "ADEPT");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.secret_memberships.len(), 1);
    assert_eq!(player.secret_memberships[0].rank, "ADEPT");
}

#[test]
fn test_covert_ritual_performance() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 32,
        first_name: Some("Aleister".to_string()),
        last_name: Some("Crowley".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["occult".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 55555);
    let mem = engine.join_secret_society("Subterranean Cyber Syndicate", "CYBER_UNDERGROUND", "ROOT_SUDO_ACCESS");

    let new_rep = engine.perform_covert_ritual(&mem.society_id, "Rite of the Eclipse Protocol");
    assert!(new_rep > 15.0);

    assert!(engine.events.iter().any(|e| e.event_type == "COVERT_RITUAL"));
}

#[test]
fn test_covert_operation_execution() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 35,
        first_name: Some("Adam".to_string()),
        last_name: Some("Weishaupt".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["politics".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 66666);
    let mem = engine.join_secret_society("Illuminati Global Council", "ILLUMINATI", "NOVUS_ORDO_SECLORUM");

    let op = engine.launch_covert_operation(&mem.society_id, "Operation Blackout Subversion", "org:sim:central_bank");

    assert_eq!(op.operation_name, "Operation Blackout Subversion");
    assert_eq!(op.status, "SUCCESSFUL");
    assert_eq!(engine.covert_operations.len(), 1);
}

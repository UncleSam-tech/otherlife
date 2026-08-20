use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_passport_and_visa_issuance() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 22,
        first_name: Some("Amelia".to_string()),
        last_name: Some("Earhart".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["travel".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 70707);
    let passport = engine.issue_passport("country:real:united_kingdom");
    assert!(passport.is_valid);
    assert_eq!(passport.country_id, "country:real:united_kingdom");

    let visa = engine.apply_visa("country:real:united_states", "WORK");
    assert_eq!(visa.target_country_id, "country:real:united_states");
    assert_eq!(visa.visa_type, "WORK");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.passports.len(), 1);
    assert_eq!(player.visas.len(), 1);
}

#[test]
fn test_international_flight_booking() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 24,
        first_name: Some("Phileas".to_string()),
        last_name: Some("Fogg".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["travel".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 80808);
    let initial_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;

    let res = engine.book_and_take_flight("city:real:tokyo", "country:real:japan", 850.0);
    assert!(res.is_ok());

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.location_id, "city:real:tokyo");
    assert_eq!(player.finances.cash, initial_cash - 850.0);
    assert_eq!(player.travel_history.len(), 1);
    assert!(engine.world_news.iter().any(|n| n.category == "TRAVEL"));
}

#[test]
fn test_cross_border_relocation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 28,
        first_name: Some("Marco".to_string()),
        last_name: Some("Polo".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["travel".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 90909);
    engine.relocate_residence("city:real:paris", "country:real:france");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.location_id, "city:real:paris");
    assert_eq!(player.identity.country_id, "country:real:france");
    assert!(engine.events.iter().any(|e| e.event_type == "CROSS_BORDER_RELOCATION"));
}

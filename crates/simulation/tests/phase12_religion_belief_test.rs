use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_faith_conversion_and_worship() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 25,
        first_name: Some("Marcus".to_string()),
        last_name: Some("Aurelius".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["philosophy".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 40404);
    engine.convert_faith("STOICISM", "Stoic Philosophy");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.belief.faith_id, "STOICISM");
    assert_eq!(player.belief.faith_name, "Stoic Philosophy");

    let initial_stress = player.health.stress;
    engine.attend_worship_service();

    let updated_player = engine.persons.get("person:sim:player").unwrap();
    assert!(updated_player.belief.devotion_level > 35.0);
    assert!(updated_player.health.stress < initial_stress);
}

#[test]
fn test_tithe_donation_and_financial_offering() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:chicago".to_string(),
        starting_age: 30,
        first_name: Some("Francis".to_string()),
        last_name: Some("Assisi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["religion".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 50505);
    let initial_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;

    let res = engine.donate_tithe(250.0);
    assert!(res.is_ok());

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.finances.cash, initial_cash - 250.0);
    assert_eq!(player.belief.tithes_donated, 250.0);
}

#[test]
fn test_found_faith_movement_and_leadership() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:france".to_string(),
        location_id: "city:real:paris".to_string(),
        starting_age: 32,
        first_name: Some("Voltaire".to_string()),
        last_name: Some("Arouet".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["philosophy".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 60606);
    let movement = engine.found_faith_movement("Order of Rational Ethics", "Reason and compassion for all human life.");

    assert_eq!(movement.name, "Order of Rational Ethics");
    assert_eq!(movement.congregation_size, 45);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.belief.spiritual_rank, "LEADER");
    assert_eq!(player.founded_movements.len(), 1);
    assert!(engine.world_news.iter().any(|n| n.category == "RELIGION"));
}

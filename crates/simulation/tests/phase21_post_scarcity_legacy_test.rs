use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_universal_basic_dividend_distribution() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 30,
        first_name: Some("Kardashev".to_string()),
        last_name: Some("Prime".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 404040);
    let init_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;

    let dividend = engine.distribute_universal_basic_dividend(5000.0);
    assert_eq!(dividend, 5000.0);

    let final_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;
    assert_eq!(final_cash, init_cash + 5000.0);
    assert!(engine.events.iter().any(|e| e.event_type == "UNIVERSAL_BASIC_DIVIDEND"));
}

#[test]
fn test_dyson_swarm_megastructure_construction() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 40,
        first_name: Some("Freeman".to_string()),
        last_name: Some("Dyson".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 505050);
    let mega = engine.construct_cosmic_megastructure("Sol Dyson Array Alpha", "DYSON_SWARM", 850000.0);

    assert_eq!(mega.name, "Sol Dyson Array Alpha");
    assert_eq!(mega.structure_type, "DYSON_SWARM");
    assert_eq!(mega.completion_pct, 100.0);

    let legacy = engine.evaluate_cosmic_legacy();
    assert_eq!(legacy.civilization_kardashev_tier, "TYPE_II");
    assert_eq!(engine.cosmic_megastructures.len(), 1);
}

#[test]
fn test_interstellar_colony_and_kardashev_legacy() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 45,
        first_name: Some("Carl".to_string()),
        last_name: Some("Sagan".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 606060);
    let colony_count = engine.establish_interstellar_colony("Proxima Centauri b", 50000);

    assert_eq!(colony_count, 1);

    let legacy = engine.evaluate_cosmic_legacy();
    assert_eq!(legacy.interstellar_colonies_count, 1);
    assert!(engine.events.iter().any(|e| e.event_type == "INTERSTELLAR_COLONY_ESTABLISHED"));
}

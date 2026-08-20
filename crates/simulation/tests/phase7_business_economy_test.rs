use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_business_foundation_and_operations() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 25,
        first_name: Some("Arthur".to_string()),
        last_name: Some("Dent".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["business".to_string()],
        goals: vec!["found_company".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 9090);
    assert_eq!(engine.persons.get("person:sim:player").unwrap().finances.cash, 2500.0);

    // 1. Found Business
    let biz = engine.found_business("Apex Tech Solutions", "Technology", 1000.0).unwrap();
    assert_eq!(biz.name, "Apex Tech Solutions");
    assert_eq!(biz.industry, "Technology");
    assert_eq!(biz.cash_reserve, 1000.0);

    // Verify capital deduction
    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.finances.cash, 1500.0);
    assert_eq!(player.owned_business_ids.len(), 1);

    // 2. Operate Business Turn
    let dividend = engine.operate_business_turn(&biz.id);
    assert!(dividend > 0.0);

    let updated_player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(updated_player.finances.cash, 1500.0 + dividend);
}

#[test]
fn test_macro_economy_cycles() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:new_york".to_string(),
        starting_age: 30,
        first_name: Some("Gordon".to_string()),
        last_name: Some("Gekko".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["finance".to_string()],
        goals: vec!["build_conglomerate".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 10101);

    // Test Boom
    engine.set_economic_cycle("BOOM");
    assert_eq!(engine.macro_economy.economic_cycle, "BOOM");
    assert_eq!(engine.macro_economy.interest_rate, 0.060);

    // Test Recession
    engine.set_economic_cycle("RECESSION");
    assert_eq!(engine.macro_economy.economic_cycle, "RECESSION");
    assert_eq!(engine.macro_economy.interest_rate, 0.020);
}

#[test]
fn test_business_bankruptcy_handling() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 22,
        first_name: Some("Elon".to_string()),
        last_name: Some("Venture".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["startups".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 20202);
    let biz = engine.found_business("Failing Retail Ltd", "Retail", 500.0).unwrap();

    // Mutate biz into insolvent state
    if let Some(b) = engine.businesses.get_mut(&biz.id) {
        b.cash_reserve = -500.0;
        b.debt = 10000.0;
        b.valuation = 100.0;
    }

    let is_bankrupt = engine.handle_business_bankruptcy(&biz.id);
    assert!(is_bankrupt);
    assert!(!engine.businesses.contains_key(&biz.id));
    assert!(engine.persons.get("person:sim:player").unwrap().owned_business_ids.is_empty());
}

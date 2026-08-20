use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_seasonal_weather_simulation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 20,
        first_name: Some("Greta".to_string()),
        last_name: Some("Thunberg".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["environment".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 11111);
    let weather = engine.simulate_weather_turn();

    assert_eq!(weather.year, 2026);
    assert!(!weather.season.is_empty());
    assert!(!weather.condition.is_empty());
    assert_eq!(engine.weather_events.len(), 1);
}

#[test]
fn test_natural_disaster_trigger_and_impact() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 24,
        first_name: Some("David".to_string()),
        last_name: Some("Attenborough".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["nature".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 22222);
    let disaster = engine.trigger_natural_disaster("FLOOD", 1.5);

    assert_eq!(disaster.disaster_type, "FLOOD");
    assert!(disaster.is_active);
    assert_eq!(disaster.damage_cost, 75000.0);
    assert_eq!(engine.active_disasters.len(), 1);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.health.stress > 20.0);
}

#[test]
fn test_infrastructure_rebuilding_and_relief() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 30,
        first_name: Some("Charles".to_string()),
        last_name: Some("Darwin".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["environment".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 33333);
    let disaster = engine.trigger_natural_disaster("HURRICANE", 1.0);

    let relief_res = engine.rebuild_infrastructure(&disaster.id, 1000.0);
    assert!(relief_res.is_ok());

    let dis_ref = engine.active_disasters.iter().find(|d| d.id == disaster.id).unwrap();
    assert!(!dis_ref.is_active);
}

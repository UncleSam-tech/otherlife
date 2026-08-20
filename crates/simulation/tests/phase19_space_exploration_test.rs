use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_space_agency_founding_and_mission_launch() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 38,
        first_name: Some("Elon".to_string()),
        last_name: Some("Vance".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 77777);
    let agency = engine.fund_space_agency("Aetheria Orbital Systems", "PRIVATE_AEROSPACE", 1000.0).unwrap();

    assert_eq!(agency.name, "Aetheria Orbital Systems");
    assert_eq!(engine.space_agencies.len(), 1);

    let mission = engine.launch_space_mission("Red Planet Explorer I", "MARS_ROVER", "Mars", 850.0);
    assert_eq!(mission.destination, "Mars");
    assert_eq!(mission.status, "ORBIT_SUCCESS");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.space_missions.len(), 1);
}

#[test]
fn test_satellite_orbital_deployment() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 35,
        first_name: Some("Astrid".to_string()),
        last_name: Some("Lind".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 88888);
    let sat = engine.deploy_satellite("AstraSat-1", "Low Earth Orbit");

    assert_eq!(sat.name, "AstraSat-1");
    assert_eq!(sat.mission_type, "ORBITAL_SATELLITE");
    assert_eq!(sat.destination, "Low Earth Orbit");
    assert!(engine.events.iter().any(|e| e.event_type == "SPACE_MISSION_LAUNCH"));
}

#[test]
fn test_space_technology_patent() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 40,
        first_name: Some("Robert".to_string()),
        last_name: Some("Goddard".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 99999);
    let patent = engine.register_space_patent("Closed-Loop Methane Rocket Thruster", 750000.0);

    assert_eq!(patent.title, "Closed-Loop Methane Rocket Thruster");
    assert_eq!(patent.field, "Aerospace Tech");
    assert_eq!(patent.estimated_valuation, 750000.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.patents.len(), 1);
}

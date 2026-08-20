use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_military_enlistment_and_promotion() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 20,
        first_name: Some("Richard".to_string()),
        last_name: Some("Sharpe".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["military".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 10101);
    let rec = engine.enlist_military("ARMY");
    assert_eq!(rec.branch, "ARMY");
    assert_eq!(rec.rank, "PRIVATE");
    assert!(rec.is_active_duty);

    let rank1 = engine.promote_military_rank();
    assert_eq!(rank1, "SERGEANT");

    let rank2 = engine.promote_military_rank();
    assert_eq!(rank2, "LIEUTENANT");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.military_record.as_ref().unwrap().rank, "LIEUTENANT");
}

#[test]
fn test_combat_deployment_and_medals() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 23,
        first_name: Some("John".to_string()),
        last_name: Some("Miller".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["military".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 20202);
    engine.enlist_military("MARINES");

    let deployments = engine.deploy_to_combat("Alpha Shield");
    assert_eq!(deployments, 1);

    let player = engine.persons.get("person:sim:player").unwrap();
    let mil = player.military_record.as_ref().unwrap();
    assert_eq!(mil.combat_deployments_count, 1);
    assert_eq!(mil.medals.len(), 1);
    assert!(mil.medals[0].contains("Alpha Shield"));
}

#[test]
fn test_veteran_discharge_and_pension() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 26,
        first_name: Some("Audie".to_string()),
        last_name: Some("Murphy".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["military".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 30303);
    engine.enlist_military("AIR_FORCE");
    engine.deploy_to_combat("Peacekeeper");

    let pension = engine.discharge_military_veteran();
    assert_eq!(pension, 1200.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    let mil = player.military_record.as_ref().unwrap();
    assert!(!mil.is_active_duty);
    assert!(mil.is_veteran);
    assert_eq!(player.employment.monthly_salary, 1200.0);
}

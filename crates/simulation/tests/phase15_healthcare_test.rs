use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_medical_diagnosis_and_surgery() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 45,
        first_name: Some("Arthur".to_string()),
        last_name: Some("Conan".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["health".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 40404);
    let diag = engine.diagnose_condition("Arrhythmia", "MODERATE");
    assert_eq!(diag.condition_name, "Arrhythmia");
    assert_eq!(diag.severity, "MODERATE");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.medical_history.len(), 1);
    assert_eq!(player.health.conditions.len(), 1);

    let surg_res = engine.undergo_surgery("Cardiac Pacemaker Implantation", 1500.0);
    assert!(surg_res.is_ok());

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.surgical_history.len(), 1);
    assert!(player.health.conditions.is_empty());
    assert!(player.medical_history[0].is_cured);
}

#[test]
fn test_will_and_testament_drafting() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 60,
        first_name: Some("Winston".to_string()),
        last_name: Some("Churchill".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["law".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 50505);
    let beneficiaries = vec!["person:sim:mum".to_string(), "person:sim:child_1".to_string()];
    let will = engine.draft_will_and_testament(beneficiaries, "Divide liquid estate 50/50 between spouse and children.");

    assert_eq!(will.beneficiary_ids.len(), 2);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.will_and_testament.is_some());
    assert_eq!(player.will_and_testament.as_ref().unwrap().executor_person_id, "person:sim:mum");
}

#[test]
fn test_epidemic_exposure_and_quarantine() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 30,
        first_name: Some("Florence".to_string()),
        last_name: Some("Nightingale".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["health".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 60606);
    let initial_stress = engine.persons.get("person:sim:player").unwrap().health.stress;

    let exposed = engine.evaluate_epidemic_exposure("Influenza Variant X");
    assert!(exposed);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.health.stress, initial_stress + 15.0);
    assert!(engine.world_news.iter().any(|n| n.category == "HEALTHCARE"));
}

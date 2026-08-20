use otherlife_simulation::SimulationEngine;
use otherlife_world::{LegalStatus, NewLifeConfig};
use std::collections::HashMap;

#[test]
fn test_commit_crime_success_and_loot() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 22,
        first_name: Some("Tommy".to_string()),
        last_name: Some("Shelby".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("LOW".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["crime".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 99901);
    let initial_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;

    let success = engine.commit_crime("BURGLARY", 1500.0);
    assert!(success);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.finances.cash, initial_cash + 1500.0);
    assert_eq!(player.criminal_records.len(), 1);
    assert_eq!(player.criminal_records[0].crime_type, "BURGLARY");
}

#[test]
fn test_crime_failure_and_police_investigation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 24,
        first_name: Some("Arthur".to_string()),
        last_name: Some("Shelby".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("LOW".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["crime".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 11111); // Seed causes roll <= 0.40

    let success = engine.commit_crime("FRAUD", 5000.0);
    assert!(!success);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.legal_status, LegalStatus::UnderInvestigation);
    assert!(engine.events.iter().any(|e| e.event_type == "CRIME_FAILED"));
}

#[test]
fn test_court_trial_conviction_and_prison_sentence() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 26,
        first_name: Some("John".to_string()),
        last_name: Some("Shelby".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("LOW".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec![],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 22222);

    // Conduct court trial with poor public defender (lawyer_skill = 5.0) -> Guilty verdict
    let acquitted = engine.conduct_court_trial(5.0);
    assert!(!acquitted);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.legal_status, LegalStatus::Imprisoned);
    assert!(player.prison_sentence.is_some());
    assert_eq!(player.prison_sentence.as_ref().unwrap().months_total, 12);

    // Serve prison turns until release
    for _ in 0..12 {
        engine.serve_prison_turn();
    }

    let released_player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(released_player.legal_status, LegalStatus::Parole);
    assert!(released_player.prison_sentence.is_none());
}

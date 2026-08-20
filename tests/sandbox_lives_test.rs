use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_scenario_a_nigeria_music_16yo() {
    let mut skills = HashMap::new();
    skills.insert("singing".to_string(), 70.0);
    skills.insert("songwriting".to_string(), 65.0);

    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 16,
        first_name: Some("Tunde".to_string()),
        last_name: Some("Adeyemi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills,
        interests: vec!["music".to_string(), "writing".to_string()],
        goals: vec!["become_musician".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 101);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.first_name, "Tunde");
    assert_eq!(player.identity.country_id, "country:real:nigeria");
    assert_eq!(player.location_id, "city:real:lagos");
    assert!(player.interests.contains("music"));
    assert!(!player.interests.contains("football")); // Zero football bias

    let suggestions = engine.get_suggested_actions();
    assert!(suggestions.iter().any(|s| s.contains("singing") || s.contains("band") || s.contains("music")));
}

#[test]
fn test_scenario_b_usa_accountant_30yo() {
    let mut skills = HashMap::new();
    skills.insert("finance_accounting".to_string(), 85.0);
    skills.insert("mathematics".to_string(), 80.0);

    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:new_york".to_string(),
        starting_age: 30,
        first_name: Some("David".to_string()),
        last_name: Some("Miller".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills,
        interests: vec!["finance".to_string(), "business".to_string()],
        goals: vec!["become_wealthy".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 202);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.birth_year, 1996); // 2026 - 30
    assert_eq!(player.finances.cash, 2500.0); // High income tier
    assert_eq!(player.location_id, "city:real:new_york");
}

#[test]
fn test_scenario_c_spain_football_12yo() {
    let mut skills = HashMap::new();
    skills.insert("football_control".to_string(), 75.0);

    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:spain".to_string(),
        location_id: "city:real:madrid".to_string(),
        starting_age: 12,
        first_name: Some("Carlos".to_string()),
        last_name: Some("Silva".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills,
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 303);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.first_name, "Carlos");
    assert_eq!(player.location_id, "city:real:madrid");
    assert!(player.interests.contains("football"));
}

#[test]
fn test_scenario_d_uk_birth_random() {
    let config = NewLifeConfig {
        creation_mode: "RANDOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 0, // Age 0 Birth
        first_name: None,
        last_name: None,
        sex: None,
        household_income_tier: None,
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: Vec::new(),
        goals: Vec::new(),
    };

    let engine = SimulationEngine::new_game(config, 404);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.birth_year, 2026);
    assert_eq!(player.education.grade_level, 0);
    assert!(player.is_alive);
}

#[test]
fn test_scenario_e_france_politics_22yo() {
    let mut skills = HashMap::new();
    skills.insert("public_speaking".to_string(), 75.0);
    skills.insert("persuasion".to_string(), 72.0);

    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:france".to_string(),
        location_id: "city:real:paris".to_string(),
        starting_age: 22,
        first_name: Some("Camille".to_string()),
        last_name: Some("Dubois".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills,
        interests: vec!["politics".to_string(), "social_causes".to_string()],
        goals: vec!["become_prime_minister".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 505);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.first_name, "Camille");
    assert_eq!(player.location_id, "city:real:paris");
    assert!(player.interests.contains("politics"));

    let suggestions = engine.get_suggested_actions();
    assert!(suggestions.iter().any(|s| s.contains("debate") || s.contains("policy") || s.contains("essay")));
}

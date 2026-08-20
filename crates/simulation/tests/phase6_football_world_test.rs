use otherlife_simulation::SimulationEngine;
use otherlife_world::{FootballRole, NewLifeConfig};
use std::collections::HashMap;

#[test]
fn test_football_match_simulation_and_ratings() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 17,
        first_name: Some("Kieran".to_string()),
        last_name: Some("Tierney".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 6060);

    let match_res = engine.simulate_football_match("Celtic FC", "Rangers FC");
    assert_eq!(match_res.home_club_name, "Celtic FC");
    assert_eq!(match_res.away_club_name, "Rangers FC");
    assert!(match_res.player_rating >= 6.0 && match_res.player_rating <= 10.0);

    // Verify event & news ticker recording
    assert!(engine.events.iter().any(|e| e.event_type == "FOOTBALL_MATCH"));
    assert!(engine.world_news.iter().any(|n| n.category == "FOOTBALL"));
}

#[test]
fn test_causal_scouting_and_contract_negotiation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:spain".to_string(),
        location_id: "city:real:madrid".to_string(),
        starting_age: 18,
        first_name: Some("Gavi".to_string()),
        last_name: Some("Paez".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 7070);

    // 1. Generate Scout Report
    let report = engine.generate_scout_report("person:sim:player");
    assert!(report.current_ability >= 40.0);
    assert!(report.potential_rating > report.current_ability);
    assert!(report.recommended_transfer_fee > 0.0);

    // 2. Negotiate Contract
    let contract = engine.negotiate_football_contract("Real Madrid CF", 25000.0, 4);
    assert_eq!(contract.club_name, "Real Madrid CF");
    assert_eq!(contract.weekly_wage, 25000.0);
    assert_eq!(contract.years_remaining, 4);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.football_role, FootballRole::Player);
    assert_eq!(player.employment.monthly_salary, 100000.0); // 25k * 4
}

#[test]
fn test_career_transitions() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 34,
        first_name: Some("Steven".to_string()),
        last_name: Some("Gerrard".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 8080);

    // Transition from Player to Coach
    engine.transition_football_role(FootballRole::Coach);
    let player1 = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player1.football_role, FootballRole::Coach);
    assert!(player1.employment.job_title.as_ref().unwrap().contains("Coach"));

    // Transition to Manager
    engine.transition_football_role(FootballRole::Manager);
    let player2 = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player2.football_role, FootballRole::Manager);
    assert!(player2.employment.job_title.as_ref().unwrap().contains("Manager"));
}

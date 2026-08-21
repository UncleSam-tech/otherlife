use std::collections::HashMap;
use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;

#[test]
fn test_creator_life_cycle_growth_burnout_and_pivot() {
    let mut config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2024,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 15,
        first_name: Some("Tunde".to_string()),
        last_name: Some("Adeyemi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["technology".to_string(), "creativity".to_string()],
        goals: vec!["create_media".to_string()],
    };
    config.skills.insert("creativity".to_string(), 45.0);

    let mut engine = SimulationEngine::new_game(config, 42);

    // 1. Launch Creator Channel
    let res = engine.resolve_situation_choice("situation_test", "launch_creator_channel");
    assert!(res.success);
    assert!(engine.creator_channel.is_some());
    let ch = engine.creator_channel.as_ref().unwrap();
    assert_eq!(ch.channel_handle, "@tunde");
    assert!(ch.subscriber_count > 500);

    // 2. Produce videos until burnout rises
    for _ in 0..5 {
        let res_prod = engine.resolve_situation_choice("situation_test", "produce_creator_video");
        assert!(res_prod.success);
    }

    let ch_after = engine.creator_channel.as_ref().unwrap();
    assert!(ch_after.subscriber_count > 5000);
    assert!(ch_after.burnout_level >= 75.0);
    assert!(!engine.active_crises.is_empty());
    assert_eq!(engine.active_crises[0].crisis_type, "CREATIVE_BURNOUT");

    // 3. Resolve Burnout through Pivot to Media Agency
    let res_pivot = engine.resolve_situation_choice("situation_test", "handle_burnout_pivot_production");
    assert!(res_pivot.success);
    assert!(engine.active_crises.is_empty());
    assert_eq!(engine.life_pivots.len(), 1);
    assert_eq!(engine.life_pivots[0].new_path, "Digital Media Studio Executive");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.employment.job_title.as_deref(), Some("Managing Director (Media Agency)"));
    assert_eq!(player.employment.monthly_salary, 4200.0);
}

#[test]
fn test_football_life_cycle_trial_match_and_pivot_to_coaching() {
    let mut config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 16,
        first_name: Some("Callum".to_string()),
        last_name: Some("Boyd".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("WORKING".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };
    config.skills.insert("football_control".to_string(), 75.0);
    config.skills.insert("athleticism".to_string(), 70.0);

    let mut engine = SimulationEngine::new_game(config, 101);

    // 1. Regional Youth Trial
    let res_trial = engine.resolve_situation_choice("situation_test", "action_football_trial_youth");
    assert!(res_trial.success);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.football_contract.is_some());
    assert_eq!(player.football_contract.as_ref().unwrap().club_name, "Regional Youth Academy FC");

    // 2. Play competitive match
    let res_match = engine.resolve_situation_choice("situation_test", "start_saturday_match");
    assert!(res_match.success);

    // 3. Pivot to Coaching License & Scouting Staff
    let res_coach = engine.resolve_situation_choice("situation_test", "football_pivot_coaching");
    assert!(res_coach.success);
    assert_eq!(engine.life_pivots.len(), 1);
    assert_eq!(engine.life_pivots[0].new_path, "Academy Coach & Talent Scout");

    let player_after = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player_after.employment.job_title.as_deref(), Some("Youth Academy Coach"));
    assert!(player_after.football_contract.is_none());
}

#[test]
fn test_civic_townhall_and_public_reputation_growth() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2025,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:new_york".to_string(),
        starting_age: 22,
        first_name: Some("Maya".to_string()),
        last_name: Some("Lin".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["politics".to_string(), "economics".to_string()],
        goals: vec!["public_service".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 77);
    let initial_rep = engine.reputation.public_standing;

    let res = engine.resolve_situation_choice("situation_test", "organize_community_townhall");
    assert!(res.success);
    assert!(engine.reputation.public_standing > initial_rep);
}

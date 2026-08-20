use otherlife_actions::{ActionPayload, ActionPrimitive};
use otherlife_simulation::SimulationEngine;
use otherlife_world::{ActivityType, KnowledgeRecord, NewLifeConfig, NpcTier, Person};
use std::collections::HashMap;

#[test]
fn test_npc_tiers_and_schedules() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 18,
        first_name: Some("Leo".to_string()),
        last_name: Some("Vance".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["technology".to_string()],
        goals: vec!["build_company".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 1111);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.tier, NpcTier::TierA);

    let mum = engine.persons.get("person:sim:mum").unwrap();
    assert_eq!(mum.tier, NpcTier::TierA);
    assert_eq!(mum.employment.job_title, Some("Senior Administrator".to_string()));
}

#[test]
fn test_independent_npc_events_and_promotions() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:new_york".to_string(),
        starting_age: 20,
        first_name: Some("Sarah".to_string()),
        last_name: Some("Connor".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["business".to_string()],
        goals: vec!["become_wealthy".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 2222);

    // Boost Mum's performance so she gets promoted
    if let Some(mum) = engine.persons.get_mut("person:sim:mum") {
        mum.employment.job_performance = 82.0;
    }

    // Run actions to trigger NPC simulation ticks
    let action = ActionPayload {
        action: ActionPrimitive::Rest,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.5,
        parameters: serde_json::json!({}),
    };

    for _ in 0..5 {
        engine.execute_player_action(action.clone());
    }

    let mum_after = engine.persons.get("person:sim:mum").unwrap();
    assert!(mum_after.employment.job_performance >= 85.0);
    assert!(mum_after.employment.job_title.as_ref().unwrap().contains("Lead"));

    // Verify World News contains NPC promotion event
    assert!(engine.world_news.iter().any(|n| n.category == "CAREER"));
}

#[test]
fn test_secret_propagation_to_player() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:france".to_string(),
        location_id: "city:real:paris".to_string(),
        starting_age: 16,
        first_name: Some("Luc".to_string()),
        last_name: Some("Moreau".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["writing".to_string()],
        goals: vec!["write_novel".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 3333);

    // Ensure Mum is socializing
    if let Some(mum) = engine.persons.get_mut("person:sim:mum") {
        mum.schedule.current_activity = ActivityType::Socializing;
    }

    // Execute multiple ticks to propagate secret
    let action = ActionPayload {
        action: ActionPrimitive::Rest,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.5,
        parameters: serde_json::json!({}),
    };

    for _ in 0..10 {
        if let Some(mum) = engine.persons.get_mut("person:sim:mum") {
            mum.schedule.current_activity = ActivityType::Socializing;
        }
        engine.execute_player_action(action.clone());
    }

    // Verify secret has been propagated
    let mum_secret = &engine.persons.get("person:sim:mum").unwrap().secrets[0];
    assert!(mum_secret.known_by_ids.contains("person:sim:player"));
}

#[test]
fn test_world_news_generation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:germany".to_string(),
        location_id: "city:real:berlin".to_string(),
        starting_age: 22,
        first_name: Some("Hans".to_string()),
        last_name: Some("Weber".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["engineering".to_string()],
        goals: vec!["build_company".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 4444);
    assert!(!engine.world_news.is_empty());
    assert_eq!(engine.world_news[0].category, "LOCAL");
}

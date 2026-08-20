use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_social_media_account_creation_and_posting() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 19,
        first_name: Some("Kai".to_string()),
        last_name: Some("Cenat".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["internet".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 70707);
    let acc = engine.create_social_media_account("YOUTUBE", "kai_vlogs");
    assert_eq!(acc.platform, "YOUTUBE");
    assert_eq!(acc.handle, "kai_vlogs");

    let post = engine.post_digital_content("YOUTUBE", "24 Hour Challenge Vlog in London!");
    assert_eq!(post.platform, "YOUTUBE");
    assert!(post.likes > 0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.social_media_accounts.len(), 1);
    assert_eq!(player.digital_posts.len(), 1);
}

#[test]
fn test_viral_content_and_brand_sponsorship() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 21,
        first_name: Some("Mr".to_string()),
        last_name: Some("Beast".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["internet".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 80808);
    let initial_cash = engine.persons.get("person:sim:player").unwrap().finances.cash;

    engine.create_social_media_account("INSTAGRAM", "mrbeast");
    let spons_res = engine.accept_brand_sponsorship("INSTAGRAM", 3500.0);
    assert!(spons_res.is_ok());

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.finances.cash, initial_cash + 3500.0);
}

#[test]
fn test_cyber_attack_and_digital_trolling() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 22,
        first_name: Some("Pewdie".to_string()),
        last_name: Some("Pie".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["internet".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 90909);
    let initial_rep = engine.persons.get("person:sim:player").unwrap().fame.public_reputation;

    engine.handle_cyber_attack("ACCOUNT_HIJACK_ATTEMPT");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.fame.public_reputation, initial_rep - 10.0);
    assert!(engine.events.iter().any(|e| e.event_type == "CYBER_ATTACK"));
}

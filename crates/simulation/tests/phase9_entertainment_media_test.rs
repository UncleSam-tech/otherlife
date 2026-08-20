use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_creative_release_production() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 20,
        first_name: Some("Burna".to_string()),
        last_name: Some("Boy".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["music".to_string()],
        goals: vec!["release_hit_album".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 60606);

    let release = engine.produce_creative_release("African Giant", "ALBUM");
    assert_eq!(release.title, "African Giant");
    assert_eq!(release.medium, "ALBUM");
    assert!(release.quality_rating >= 60.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.creative_releases.len(), 1);
    assert_eq!(player.creative_releases[0].title, "African Giant");
}

#[test]
fn test_media_promotion_and_fame_growth() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:los_angeles".to_string(),
        starting_age: 22,
        first_name: Some("Taylor".to_string()),
        last_name: Some("Swift".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["music".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 70707);
    let release = engine.produce_creative_release("1989 (Taylor's Version)", "ALBUM");

    let royalties = engine.promote_release(&release.id);
    assert!(royalties > 0.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.fame.fame_level > 0.0);
    assert_eq!(player.fame.fanbase_count, 8500);
}

#[test]
fn test_media_scandal_and_reputation_impact() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 28,
        first_name: Some("Kanye".to_string()),
        last_name: Some("West".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["music".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 80808);
    assert_eq!(engine.persons.get("person:sim:player").unwrap().fame.public_reputation, 50.0);

    engine.handle_media_scandal("Leaked controversial studio recording audio.");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.fame.public_reputation, 25.0);
    assert!(engine.events.iter().any(|e| e.event_type == "MEDIA_SCANDAL"));
}

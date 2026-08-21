use otherlife_simulation::SimulationEngine;
use otherlife_world::{CommunicationStyle, NewLifeConfig, ProcessType};
use std::collections::HashMap;

fn create_phase3_abuja_life(starting_age: u32) -> SimulationEngine {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2005 + starting_age as i32,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age,
        first_name: Some("Israel".to_string()),
        last_name: Some("Oyebamiji".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["technology".to_string(), "academics".to_string()],
        goals: vec!["excellence".to_string()],
        ..Default::default()
    };
    SimulationEngine::new_game(config, 300)
}

#[test]
fn test_relationship_memory_and_future_reaction() {
    let mut engine = create_phase3_abuja_life(9);

    // 1. Helping parents
    let res = engine.submit_living_intent("Talk to father David about career goals and advice");
    assert!(res.success);
    assert!(res.narrative.contains("father") || res.narrative.contains("family") || res.headline.contains("Deliberation"));

    let player = engine.persons.get("person:sim:player").unwrap();
    let father_rel = player.relationships.get("person:sim:father").unwrap();
    assert!(father_rel.trust >= 0.90);
}

#[test]
fn test_npc_personality_differentiated_reactions() {
    let engine = create_phase3_abuja_life(10);

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    let father = engine.npcs.get("person:sim:father").unwrap();
    let teacher = engine.npcs.get("person:sim:teacher").unwrap();
    let coach = engine.npcs.get("person:sim:coach").unwrap();

    // Differentiated communication styles and strictness
    assert_eq!(mother.personality.communication_style, CommunicationStyle::Nurturing);
    assert_eq!(father.personality.communication_style, CommunicationStyle::Disciplinarian);
    assert_eq!(teacher.personality.communication_style, CommunicationStyle::Inspirational);
    assert_eq!(coach.personality.communication_style, CommunicationStyle::Direct);

    assert!(father.personality.strictness > mother.personality.strictness);
    assert!(coach.personality.strictness > teacher.personality.strictness);
}

#[test]
fn test_social_reputation_and_biography_memoir() {
    let mut engine = create_phase3_abuja_life(15);

    // Academic practice
    engine.submit_living_intent("Study mathematics and science every evening for four weeks for WAEC");
    // Athletic practice
    engine.submit_living_intent("Train football at the sports pitch three times weekly with the coach");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.reputation.academic_reputation > 0.0);
    assert!(player.reputation.athletic_reputation > 0.0);

    let biography = engine.get_biography();
    assert!(biography.contains("Israel"));
    assert!(biography.contains("Birth") || biography.contains("Life"));
}

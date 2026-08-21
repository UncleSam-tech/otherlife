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

    // 1. Helping father repair family computer
    let res = engine.submit_living_intent("Help father David repair and clean the family desktop computer");
    assert!(res.success);
    assert!(res.narrative.contains("clearing dust"));

    let player = engine.persons.get("person:sim:player").unwrap();
    let father_rel = player.relationships.get("person:sim:father").unwrap();

    // Verify shared memory recorded in relationship history
    assert!(father_rel.history.support_moments >= 1);
    assert!(father_rel.history.shared_memories.iter().any(|m| m.event_summary.contains("troubleshoot")));
    assert!(father_rel.trust > 0.90);
}

#[test]
fn test_npc_personality_differentiated_reactions() {
    let engine = create_phase3_abuja_life(10);

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    let father = engine.npcs.get("person:sim:father").unwrap();
    let teacher = engine.npcs.get("person:sim:adewale_teacher").unwrap();
    let coach = engine.npcs.get("person:sim:coach_ibrahim").unwrap();

    // Differentiated communication styles and strictness
    assert_eq!(mother.personality.communication_style, CommunicationStyle::Nurturing);
    assert_eq!(father.personality.communication_style, CommunicationStyle::Disciplinarian);
    assert_eq!(teacher.personality.communication_style, CommunicationStyle::Inspirational);
    assert_eq!(coach.personality.communication_style, CommunicationStyle::Direct);

    assert!(father.personality.strictness > mother.personality.strictness);
    assert!(coach.personality.strictness > teacher.personality.strictness);
}

#[test]
fn test_friendship_drift_over_time() {
    let mut engine = create_phase3_abuja_life(11);

    // Long multi-week isolated technical practice
    let res = engine.submit_living_intent("Learn computer programming every weekend for six months");
    assert!(res.success);
    assert_eq!(res.days_advanced, 56);

    // Friend relationship tracks days elapsed
    let player = engine.persons.get("person:sim:player").unwrap();
    let chidi_rel = player.relationships.get("person:sim:chidi_nwosu").unwrap();
    assert!(chidi_rel.history.days_since_last_interaction >= 56);
}

#[test]
fn test_failure_recovery_and_resilience_path() {
    let mut engine = create_phase3_abuja_life(10);
    let initial_resilience = engine.persons.get("person:sim:player").unwrap().psychology.resilience;

    // Academic setback & recovery intent
    let res = engine.submit_living_intent("Review mistakes and struggle with Mr. Adewale after failed algebra quiz");
    assert!(res.success);
    assert!(res.narrative.contains("encouraging your determination"));

    // Resilience psychological growth & recovery process
    let final_resilience = engine.persons.get("person:sim:player").unwrap().psychology.resilience;
    assert!(final_resilience > initial_resilience);
    assert!(engine.active_processes.iter().any(|p| p.process_type == ProcessType::AcademicRecoveryPlan));
}

#[test]
fn test_social_reputation_and_biography_memoir() {
    let mut engine = create_phase3_abuja_life(15);

    // Academic practice
    engine.submit_living_intent("Study mathematics and science every evening for four weeks for WAEC");
    // Athletic practice
    engine.submit_living_intent("Train football at Area 10 sports ground three times weekly with Coach Ibrahim");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.reputation.academic_reputation > 20.0);
    assert!(player.reputation.athletic_reputation > 15.0);

    let biography = engine.get_biography();
    assert!(biography.contains("Israel Oyebamiji"));
    assert!(biography.contains("Living Intention") || biography.contains("examination") || biography.contains("Birth"));
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::{LifeStage, NewLifeConfig, ProcessType};
use std::collections::HashMap;

fn create_default_abuja_life(starting_age: u32) -> SimulationEngine {
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
        interests: vec!["academics".to_string()],
        goals: vec!["excellence".to_string()],
        ..Default::default()
    };
    SimulationEngine::new_game(config, 100)
}

#[test]
fn test_birth_and_infancy_simulation() {
    let mut engine = create_default_abuja_life(0);
    let state = engine.get_living_state();

    // 1. Birth verification
    assert_eq!(state.age, 0);
    assert_eq!(state.player_name, "Israel Oyebamiji");
    assert!(state.life_stage.contains("Infancy"));
    assert_eq!(state.currency_symbol, "₦");
    assert_eq!(engine.npcs.len(), 5);

    // Age restrictions
    let stage = LifeStage::from_age(0);
    assert!(!stage.can_work_full_time());
    assert!(!stage.can_transact_independent_credit());

    // 2. Infancy motor exploration
    let motor_res = engine.submit_living_intent("Take first steps across the living room towards Sarah");
    assert!(motor_res.success);
    assert_eq!(motor_res.days_advanced, 14);
    assert!(motor_res.narrative.contains("caught you"));

    // 3. Speech & picture book bonding
    let speech_res = engine.submit_living_intent("Listen to Sarah reading picture books and repeat words");
    assert!(speech_res.success);
    assert_eq!(speech_res.days_advanced, 14);
    assert!(speech_res.narrative.contains("picture books"));
}

#[test]
fn test_childhood_simulation_and_opportunity() {
    let mut engine = create_default_abuja_life(8);
    let state = engine.get_living_state();
    assert_eq!(state.age, 8);
    assert!(state.life_stage.contains("Childhood"));

    // 1. Primary school mathematics
    let math_res = engine.submit_living_intent("Study arithmetic and solve math problems in class with Mr. Adewale");
    assert!(math_res.success);
    assert!(math_res.narrative.contains("arithmetic"));

    // Opportunity emergence
    assert!(engine.active_opportunities.iter().any(|o| o.id.contains("math_challenge")));

    // 2. Playground football with peer
    let peer_res = engine.submit_living_intent("Play football with friends in the courtyard");
    assert!(peer_res.success);
    assert!(peer_res.narrative.contains("passes and ball control") || peer_res.narrative.contains("sports"));

    // 3. Household chores and allowance
    let chore_res = engine.submit_living_intent("Help mother Sarah with dinner preparations and sweeping");
    assert!(chore_res.success);
    assert!(chore_res.narrative.contains("allowance"));
}

#[test]
fn test_adolescence_simulation_waec_and_football_trials() {
    let mut engine = create_default_abuja_life(16);
    let state = engine.get_living_state();
    assert_eq!(state.age, 16);
    assert!(state.life_stage.contains("Adolescence"));

    // 1. Four weeks deliberate exam revision
    let waec_res = engine.submit_living_intent("I study mathematics and science every evening for four weeks for WAEC");
    assert!(waec_res.success);
    assert_eq!(waec_res.days_advanced, 28);
    assert!(waec_res.narrative.contains("four rigorous weeks"));
    assert!(engine.active_processes.iter().any(|p| p.process_type == ProcessType::SecondaryExamPreparation));

    // 2. Football training under coach
    let football_res = engine.submit_living_intent("Train football at the grounds three times weekly with Coach Ibrahim");
    assert!(football_res.success);
    assert_eq!(football_res.days_advanced, 21);
    assert!(football_res.narrative.contains("coach") || football_res.narrative.contains("Coach"));

    // Football opportunity emergence
    assert!(engine.active_opportunities.iter().any(|o| o.id.contains("trials")));

    // 3. University discovery
    let uni_res = engine.submit_living_intent("Visit university campus to inspect admission prerequisites");
    assert!(uni_res.success);
    assert!(engine.player_knowledge.iter().any(|k| k.topic_id.contains("university")));
}

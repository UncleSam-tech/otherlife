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
    assert_eq!(state.cash, 0.0, "Newborns must start with 0 cash");

    // Age restrictions
    let stage = LifeStage::from_age(0);
    assert!(!stage.can_work_full_time());
    assert!(!stage.can_transact_independent_credit());

    // 2. Infancy motor exploration
    let motor_res = engine.submit_living_intent("Take first steps across the living room");
    assert!(motor_res.success);
    assert!(motor_res.narrative.contains("steps"));

    // 3. Cuddle bonding
    let cuddle_res = engine.submit_living_intent("Cuddle close to mother on the sofa");
    assert!(cuddle_res.success);
    assert!(cuddle_res.narrative.contains("mother"));
}

#[test]
fn test_childhood_simulation_and_opportunity() {
    let mut engine = create_default_abuja_life(8);
    let state = engine.get_living_state();
    assert_eq!(state.age, 8);
    assert!(state.life_stage.contains("Childhood"));

    // 1. Primary school mathematics
    let math_res = engine.submit_living_intent("Study arithmetic and solve math problems in class");
    assert!(math_res.success);
    assert!(math_res.narrative.contains("academic") || math_res.narrative.contains("curriculum") || math_res.headline.contains("Study"));

    // 2. Playground football
    let peer_res = engine.submit_living_intent("Play football with friends on the sports pitch");
    assert!(peer_res.success);

    // 3. Allowance request
    let chore_res = engine.submit_living_intent("Ask parents for pocket money allowance");
    assert!(chore_res.success);
    assert!(chore_res.narrative.contains("allowance") || chore_res.narrative.contains("pocket money"));
}

#[test]
fn test_adolescence_simulation_waec_and_football_trials() {
    let mut engine = create_default_abuja_life(16);
    let state = engine.get_living_state();
    assert_eq!(state.age, 16);
    assert!(state.life_stage.contains("Adolescence"));

    // 1. Deliberate exam revision & WAEC completion
    let waec_res = engine.submit_living_intent("I study mathematics and science for WAEC national examinations");
    assert!(waec_res.success);
    assert!(waec_res.narrative.contains("examination") || waec_res.headline.contains("Examination"));

    // 2. Football training under coach
    let football_res = engine.submit_living_intent("Train football at the sports academy with the coach");
    assert!(football_res.success);
    assert!(football_res.narrative.contains("sports") || football_res.narrative.contains("training") || football_res.narrative.contains("pitch"));
}

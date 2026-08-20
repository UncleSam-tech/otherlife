use otherlife_actions::{ActionPayload, ActionPrimitive};
use otherlife_simulation::SimulationEngine;
use otherlife_world::{LifeStage, NewLifeConfig};
use std::collections::HashMap;

#[test]
fn test_life_stages_birth_to_old_age() {
    assert_eq!(LifeStage::from_age(0, true), LifeStage::Infancy);
    assert_eq!(LifeStage::from_age(8, true), LifeStage::Childhood);
    assert_eq!(LifeStage::from_age(15, true), LifeStage::Adolescence);
    assert_eq!(LifeStage::from_age(22, true), LifeStage::YoungAdulthood);
    assert_eq!(LifeStage::from_age(35, true), LifeStage::Adulthood);
    assert_eq!(LifeStage::from_age(55, true), LifeStage::MiddleAge);
    assert_eq!(LifeStage::from_age(70, true), LifeStage::OldAge);
    assert_eq!(LifeStage::from_age(70, false), LifeStage::Deceased);
}

#[test]
fn test_employment_and_earnings() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 20,
        first_name: Some("Arthur".to_string()),
        last_name: Some("Pemberton".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["business".to_string()],
        goals: vec!["become_wealthy".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 707);

    // Apply for Job
    let apply_action = ActionPayload {
        action: ActionPrimitive::ApplyJob,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.8,
        parameters: serde_json::json!({}),
    };

    let res1 = engine.execute_player_action(apply_action);
    assert!(res1.success);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.employment.job_title, Some("Staff Associate".to_string()));
    assert_eq!(player.employment.monthly_salary, 1800.0);

    // Work Shift to earn money
    let initial_cash = player.finances.cash;
    let work_action = ActionPayload {
        action: ActionPrimitive::WorkShift,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.9,
        parameters: serde_json::json!({}),
    };

    let res2 = engine.execute_player_action(work_action);
    assert!(res2.success);

    let player_after = engine.persons.get("person:sim:player").unwrap();
    assert!(player_after.finances.cash > initial_cash);
    assert!(player_after.employment.job_performance > 60.0);
}

#[test]
fn test_housing_rent_and_ownership() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:new_york".to_string(),
        starting_age: 25,
        first_name: Some("Rachel".to_string()),
        last_name: Some("Green".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["fashion".to_string()],
        goals: vec!["become_wealthy".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 808);

    // Rent apartment
    let rent_action = ActionPayload {
        action: ActionPrimitive::RentApartment,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.5,
        parameters: serde_json::json!({}),
    };

    let res1 = engine.execute_player_action(rent_action);
    assert!(res1.success);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.housing.housing_type, "Renting");
    assert_eq!(player.housing.monthly_cost, 550.0);

    // Give player enough cash to buy property
    if let Some(p) = engine.persons.get_mut("person:sim:player") {
        p.finances.cash = 25000.0;
    }

    let buy_action = ActionPayload {
        action: ActionPrimitive::BuyProperty,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 1.0,
        parameters: serde_json::json!({}),
    };

    let res2 = engine.execute_player_action(buy_action);
    assert!(res2.success);

    let player_owner = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player_owner.housing.housing_type, "Ownership");
    assert_eq!(player_owner.housing.monthly_cost, 0.0);
}

#[test]
fn test_romance_family_and_children() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:france".to_string(),
        location_id: "city:real:paris".to_string(),
        starting_age: 26,
        first_name: Some("Pierre".to_string()),
        last_name: Some("Laurent".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["art".to_string()],
        goals: vec!["build_family".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 909);

    // 1. Date
    let date_action = ActionPayload {
        action: ActionPrimitive::Date,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.8,
        parameters: serde_json::json!({}),
    };
    engine.execute_player_action(date_action);
    let p1 = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(p1.romance.marital_status, "Dating");

    // 2. Marry
    let marry_action = ActionPayload {
        action: ActionPrimitive::Marry,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 1.0,
        parameters: serde_json::json!({}),
    };
    engine.execute_player_action(marry_action);
    let p2 = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(p2.romance.marital_status, "Married");

    // 3. Have Child
    let child_action = ActionPayload {
        action: ActionPrimitive::HaveChild,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 1.0,
        parameters: serde_json::json!({}),
    };
    engine.execute_player_action(child_action);
    let p3 = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(p3.child_ids.len(), 1);
}

#[test]
fn test_health_care_and_mortality() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:japan".to_string(),
        location_id: "city:real:tokyo".to_string(),
        starting_age: 84,
        first_name: Some("Kenji".to_string()),
        last_name: Some("Takahashi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec!["live_quietly".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 1010);

    // Seek Medical Treatment
    let med_action = ActionPayload {
        action: ActionPrimitive::SeekMedicalTreatment,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 1.0,
        parameters: serde_json::json!({}),
    };
    let res = engine.execute_player_action(med_action);
    assert!(res.success);

    // Advance age to 85 to trigger old age natural death
    if let Some(p) = engine.persons.get_mut("person:sim:player") {
        p.identity.birth_year = 1941; // 2026 - 85
    }

    let dummy_action = ActionPayload {
        action: ActionPrimitive::Rest,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: None,
        intensity: 0.5,
        parameters: serde_json::json!({}),
    };
    engine.execute_player_action(dummy_action);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(!player.is_alive); // Old age mortality triggered
    assert!(engine.events.iter().any(|e| e.event_type == "DEATH"));
}

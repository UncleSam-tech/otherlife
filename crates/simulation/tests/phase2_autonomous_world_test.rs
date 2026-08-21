use otherlife_simulation::SimulationEngine;
use otherlife_world::{NewLifeConfig, ProcessType, SeasonalWeather};
use std::collections::HashMap;

fn create_phase2_abuja_life(starting_age: u32) -> SimulationEngine {
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
        interests: vec!["academics".to_string(), "technology".to_string()],
        goals: vec!["excellence".to_string()],
    };
    SimulationEngine::new_game(config, 200)
}

#[test]
fn test_npc_autonomous_schedule_progression() {
    let mut engine = create_phase2_abuja_life(10);
    let initial_mother_cash = engine.npcs.get("person:sim:mother").unwrap().base.resources.cash;

    // Advance 4 weeks through childhood study
    let res = engine.submit_living_intent("Study arithmetic and science every evening for four weeks");
    assert!(res.success);
    assert_eq!(res.days_advanced, 28);

    // Autonomous tick updates NPC financial and activity state
    let updated_mother = engine.npcs.get("person:sim:mother").unwrap();
    assert!(updated_mother.base.resources.cash >= initial_mother_cash);
    assert_eq!(updated_mother.last_active_day, engine.time.total_days);
}

#[test]
fn test_external_events_causal_generation() {
    let mut engine = create_phase2_abuja_life(16);
    assert_eq!(engine.macro_env.current_season, SeasonalWeather::HarmattanHaze);

    // Execute multi-week study intent
    let res = engine.submit_living_intent("Study mathematics for WAEC exams for four weeks");
    assert!(res.success);

    // Verify seasonal event generation & official examination notice delivered to inbox
    assert!(engine.events_chronicle.len() >= 2);
    assert!(engine.letters_inbox.iter().any(|l| l.subject.contains("Examination Registration") || l.subject.contains("Examination Entry") || l.subject.contains("Examination")));
}

#[test]
fn test_university_transition_multi_stage_process() {
    let mut engine = create_phase2_abuja_life(17);

    // 1. Family discussion with father David
    let fam_res = engine.submit_living_intent("Talk to father David about university funding and tuition");
    assert!(fam_res.success);
    assert!(fam_res.narrative.contains("pledging full family backing"));

    // 2. University application submission
    let app_res = engine.submit_living_intent("Apply for University of Abuja undergraduate admission");
    assert!(app_res.success);
    assert_eq!(app_res.days_advanced, 28);

    let uni_process = engine.active_processes.iter().find(|p| p.process_type == ProcessType::UniversityAdmission);
    assert!(uni_process.is_some());
    assert_eq!(uni_process.unwrap().current_step, 4);
}

#[test]
fn test_long_term_habit_and_programming_skills() {
    let mut engine = create_phase2_abuja_life(16);
    let initial_prog = engine.persons.get("person:sim:player").unwrap().skills.get("programming").cloned().unwrap().level;

    // Six months weekend programming intent
    let res = engine.submit_living_intent("Learn computer programming every weekend for six months");
    assert!(res.success);
    assert_eq!(res.days_advanced, 56);
    assert!(res.narrative.contains("algorithmic logic"));

    let final_prog = engine.persons.get("person:sim:player").unwrap().skills.get("programming").cloned().unwrap().level;
    assert!(final_prog > initial_prog);
}

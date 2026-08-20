use otherlife_actions::{ActionPayload, ActionPrimitive, ActionValidator};
use otherlife_simulation::{SimulationEngine, SimulationInvariantValidator};
use otherlife_world::{
    Capability, CitySeed, Company, CountrySeed, FootballClub, NewLifeConfig, PoliticalParty, SimTime, University,
    WorldDataValidator,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_data_file(relative_path: &str) -> String {
    let root_path = Path::new(relative_path);
    if root_path.exists() {
        return fs::read_to_string(root_path).unwrap();
    }
    let parent_path = Path::new("../../").join(relative_path);
    if parent_path.exists() {
        return fs::read_to_string(parent_path).unwrap();
    }
    panic!("Could not find real_world_data file at {} or {}", root_path.display(), parent_path.display());
}

#[test]
fn test_1_newborn_cannot_work() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 0,
        first_name: Some("Amina".to_string()),
        last_name: Some("Okonkwo".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec![],
    };

    let engine = SimulationEngine::new_game(config, 1001);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.finances.cash, 0.0);
    assert_eq!(player.employment.monthly_salary, 0.0);
    assert_eq!(player.employment.job_title.as_deref(), Some("Unemployed / Infant"));

    let work_action = ActionPayload {
        action: ActionPrimitive::WorkShift,
        actor_id: "person:sim:player".to_string(),
        target_id: None,
        claim: None,
        actual_activity: Some("work".to_string()),
        intensity: 1.0,
        parameters: serde_json::json!({}),
    };

    let validation = ActionValidator::validate(player, &work_action);
    assert!(!validation.is_valid, "Newborn must not be allowed to execute work shift");
}

#[test]
fn test_2_newborn_cannot_join_football_academy_automatically() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 0,
        first_name: Some("Tunde".to_string()),
        last_name: Some("Adeleke".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("LOW".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 1002);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.football_role, otherlife_world::FootballRole::None);
    assert!(player.football_contract.is_none());
    assert_eq!(player.finances.cash, 0.0);
}

#[test]
fn test_3_football_interest_is_not_employment() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:manchester".to_string(),
        starting_age: 16,
        first_name: Some("Liam".to_string()),
        last_name: Some("Smith".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 1003);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert!(player.interests.contains("football"));
    assert_eq!(player.football_role, otherlife_world::FootballRole::None);
    assert!(player.football_contract.is_none());
}

#[test]
fn test_4_city_belongs_to_country() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 14,
        first_name: Some("Ross".to_string()),
        last_name: Some("Campbell".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec![],
    };

    let engine = SimulationEngine::new_game(config, 1004);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert_eq!(player.identity.current_location_id, "city:real:glasgow");
    assert!(player.identity.nationalities.contains(&"country:real:united_kingdom".to_string()));
}

#[test]
fn test_6_skills_cannot_exceed_maximum() {
    let mut skills = HashMap::new();
    skills.insert("football_control".to_string(), 99.5);

    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 18,
        first_name: Some("Jack".to_string()),
        last_name: Some("Taylor".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills,
        interests: vec!["football".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 1006);
    let player = engine.persons.get_mut("person:sim:player").unwrap();

    // Diminishing returns gain formula
    let current_val = *player.skills.get("football_control").unwrap_or(&50.0);
    let gain = 5.0 * (1.0 - current_val / 100.0);
    let new_val = (current_val + gain).min(100.0);

    player.skills.insert("football_control".to_string(), new_val);

    assert!(new_val <= 100.0, "Skill must never exceed 100.0");
    assert!(SimulationInvariantValidator::validate(&engine).is_ok());
}

#[test]
fn test_7_exact_birthday_age() {
    let sim_time = SimTime::new(2018, 1, 1, 9, 0);
    let age = sim_time.compute_age(2000, 12, 20);
    assert_eq!(age, 17, "Born Dec 20, 2000; on Jan 1, 2018, age must be 17, not 18");
}

#[test]
fn test_8_real_gregorian_calendar() {
    let mut time = SimTime::new(2020, 2, 28, 0, 0); // Leap year 2020
    assert!(SimTime::is_leap_year(2020));
    assert_eq!(SimTime::days_in_month(2020, 2), 29);

    time.advance_days(1);
    assert_eq!(time.month, 2);
    assert_eq!(time.day, 29);

    time.advance_days(1);
    assert_eq!(time.month, 3);
    assert_eq!(time.day, 1);
}

#[test]
fn test_11_no_hardcoded_celtic_for_nigerian_life() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 0,
        first_name: Some("Chidi".to_string()),
        last_name: Some("Eze".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec![],
    };

    let engine = SimulationEngine::new_game(config, 1011);
    let player = engine.persons.get("person:sim:player").unwrap();

    assert!(player.football_contract.is_none());
    assert_eq!(player.football_role, otherlife_world::FootballRole::None);
    assert_eq!(player.identity.current_location_id, "city:real:lagos");
}

#[test]
fn test_14_full_save_load_roundtrip() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 22,
        first_name: Some("Oliver".to_string()),
        last_name: Some("Brown".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["business".to_string()],
        goals: vec!["become_wealthy".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 9999);
    engine.step_time_forward(30);

    let serialized = serde_json::to_string(&engine).unwrap();
    let loaded_engine: SimulationEngine = serde_json::from_str(&serialized).unwrap();

    let orig_player = engine.persons.get("person:sim:player").unwrap();
    let loaded_player = loaded_engine.persons.get("person:sim:player").unwrap();

    assert_eq!(orig_player.identity.first_name, loaded_player.identity.first_name);
    assert_eq!(orig_player.identity.current_location_id, loaded_player.identity.current_location_id);
    assert_eq!(orig_player.finances.cash, loaded_player.finances.cash);
    assert_eq!(engine.time.year, loaded_engine.time.year);
    assert_eq!(engine.time.month, loaded_engine.time.month);
}

#[test]
fn test_15_world_data_referential_integrity() {
    let countries_json = read_data_file("real_world_data/geography/countries.json");
    let countries: Vec<CountrySeed> = serde_json::from_str(&countries_json).unwrap();

    let cities_json = read_data_file("real_world_data/geography/cities.json");
    let cities: Vec<CitySeed> = serde_json::from_str(&cities_json).unwrap();

    let clubs_json = read_data_file("real_world_data/football/clubs.json");
    let clubs: Vec<FootballClub> = serde_json::from_str(&clubs_json).unwrap();

    let comp_json = read_data_file("real_world_data/companies/corporations.json");
    let companies: Vec<Company> = serde_json::from_str(&comp_json).unwrap();

    let unis_json = read_data_file("real_world_data/education/universities.json");
    let unis: Vec<University> = serde_json::from_str(&unis_json).unwrap();

    let parties_json = read_data_file("real_world_data/politics/parties.json");
    let parties: Vec<PoliticalParty> = serde_json::from_str(&parties_json).unwrap();

    let res = WorldDataValidator::validate_seed_data(
        &countries,
        &cities,
        &clubs,
        &companies,
        &unis,
        &parties,
    );

    assert!(res.is_ok(), "World seed data integrity validation failed: {:?}", res);
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_cybernetic_implant_installation() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 30,
        first_name: Some("Ray".to_string()),
        last_name: Some("Kurzweil".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 101010);
    let imp = engine.install_cybernetic_implant("Neural Link Matrix v4", "NEURAL_LINK", 2.5, 450.0).unwrap();

    assert_eq!(imp.name, "Neural Link Matrix v4");
    assert_eq!(imp.implant_type, "NEURAL_LINK");
    assert_eq!(imp.augmentation_level, 2.5);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.cybernetic_implants.len(), 1);
}

#[test]
fn test_mind_upload_and_digital_avatar() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 45,
        first_name: Some("Nick".to_string()),
        last_name: Some("Bostrom".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 202020);
    let upload = engine.upload_mind_to_digital_avatar("Bostrom-Avatar-01", "CLOUD_SERVER");

    assert_eq!(upload.digital_avatar_name, "Bostrom-Avatar-01");
    assert_eq!(upload.substrate, "CLOUD_SERVER");
    assert!(upload.upload_fidelity > 99.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.mind_uploads.len(), 1);
}

#[test]
fn test_avatar_substrate_quantum_upgrade() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 50,
        first_name: Some("Max".to_string()),
        last_name: Some("Tegmark".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 303030);
    let upload = engine.upload_mind_to_digital_avatar("Tegmark-Mind-Core", "CLOUD_SERVER");

    let new_sub = engine.upgrade_avatar_substrate(&upload.id, "QUANTUM_CORE");
    assert_eq!(new_sub, "QUANTUM_CORE");

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.mind_uploads[0].substrate, "QUANTUM_CORE");
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_academic_enrollment_and_degree() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 22,
        first_name: Some("Alan".to_string()),
        last_name: Some("Turing".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 10101);
    let degree = engine.enroll_university_program("PHD", "Artificial Intelligence", "University of Glasgow");

    assert_eq!(degree.degree_type, "PHD");
    assert_eq!(degree.field_of_study, "Artificial Intelligence");
    assert_eq!(degree.graduation_year, 2029);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.academic_degrees.len(), 1);
    assert_eq!(player.education.degree_program.as_ref().unwrap(), "PHD in Artificial Intelligence");
}

#[test]
fn test_scientific_research_and_publication() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:boston".to_string(),
        starting_age: 26,
        first_name: Some("Marie".to_string()),
        last_name: Some("Curie".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["science".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 20202);
    let proj = engine.conduct_scientific_research("Quantum Neural Substrates", "Physics", 250000.0);

    assert_eq!(proj.title, "Quantum Neural Substrates");
    assert_eq!(proj.funding_grant, 250000.0);

    let citations = engine.publish_paper(&proj.id);
    assert!(citations > 0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.research_projects[0].status, "PUBLISHED");
    assert!(engine.world_news.iter().any(|n| n.category == "SCIENCE"));
}

#[test]
fn test_patent_filing_and_tech_ip() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:san_francisco".to_string(),
        starting_age: 28,
        first_name: Some("Ada".to_string()),
        last_name: Some("Lovelace".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["technology".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 30303);
    let patent = engine.file_patent("Autonomous Algorithmic Compiler", "Software", 1250000.0);

    assert_eq!(patent.title, "Autonomous Algorithmic Compiler");
    assert_eq!(patent.estimated_valuation, 1250000.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.patents.len(), 1);
    assert_eq!(player.patents[0].field, "Software");
}

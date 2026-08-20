use otherlife_simulation::SimulationEngine;
use otherlife_world::{Company, FootballClub, NewLifeConfig, PoliticalParty, RealWorldSnapshot, University};
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
fn test_real_entity_namespaces_and_snapshot() {
    let snapshot = RealWorldSnapshot {
        snapshot_date: "2026-01-01".to_string(),
        source_version: "1.0.0".to_string(),
        canonical_id: "snapshot:real:2026_q1".to_string(),
    };

    assert_eq!(snapshot.snapshot_date, "2026-01-01");
    assert!(snapshot.canonical_id.starts_with("snapshot:real:"));
}

#[test]
fn test_real_world_registries_parsing() {
    let clubs_json = read_data_file("real_world_data/football/clubs.json");
    let clubs: Vec<FootballClub> = serde_json::from_str(&clubs_json).unwrap();
    assert!(clubs.iter().any(|c| c.id == "club:real:celtic"));
    assert!(clubs.iter().any(|c| c.id == "club:real:real_madrid"));

    let parties_json = read_data_file("real_world_data/politics/parties.json");
    let parties: Vec<PoliticalParty> = serde_json::from_str(&parties_json).unwrap();
    assert!(parties.iter().any(|p| p.id == "party:real:uk_labour"));

    let unis_json = read_data_file("real_world_data/education/universities.json");
    let unis: Vec<University> = serde_json::from_str(&unis_json).unwrap();
    assert!(unis.iter().any(|u| u.id == "uni:real:oxford"));

    let comp_json = read_data_file("real_world_data/companies/corporations.json");
    let companies: Vec<Company> = serde_json::from_str(&comp_json).unwrap();
    assert!(companies.iter().any(|c| c.id == "company:real:global_finance_inc"));
}

#[test]
fn test_alternate_timeline_divergence() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 18,
        first_name: Some("Callum".to_string()),
        last_name: Some("McGregor".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string(), "politics".to_string()],
        goals: vec!["play_pro_football".to_string()],
    };

    let engine = SimulationEngine::new_game(config, 5555);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.identity.nationalities.contains(&"country:real:united_kingdom".to_string()));
    assert_eq!(player.location_id, "city:real:glasgow");

    // Divergent timeline start
    assert_eq!(engine.time.year, 2026);
    assert!(!engine.world_news.is_empty());
}

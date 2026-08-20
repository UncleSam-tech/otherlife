use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;
use std::collections::HashMap;

#[test]
fn test_party_enrollment_and_campaign_launch() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 35,
        first_name: Some("Keir".to_string()),
        last_name: Some("Starmer".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["politics".to_string()],
        goals: vec!["become_prime_minister".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 30303);
    assert_eq!(engine.persons.get("person:sim:player").unwrap().finances.cash, 2500.0);

    // 1. Join Party
    engine.join_political_party("party:real:uk_labour");
    assert_eq!(
        engine.persons.get("person:sim:player").unwrap().political_party_id.as_deref(),
        Some("party:real:uk_labour")
    );

    // 2. Launch Campaign
    let campaign = engine
        .launch_political_campaign("office:real:mp", "Member of Parliament (MP)", 500.0)
        .unwrap();

    assert_eq!(campaign.office_title, "Member of Parliament (MP)");
    assert_eq!(campaign.polling_pct, 20.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.finances.cash, 2000.0);
    assert!(player.active_campaign.is_some());
}

#[test]
fn test_campaign_rallies_and_polling_boost() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_states".to_string(),
        location_id: "city:real:washington_dc".to_string(),
        starting_age: 40,
        first_name: Some("Kamala".to_string()),
        last_name: Some("Harris".to_string()),
        sex: Some("Female".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["politics".to_string()],
        goals: vec!["win_election".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 40404);
    engine.launch_political_campaign("office:real:us_president", "President of the United States", 1000.0).unwrap();

    // Hold Rallies
    let new_poll = engine.hold_campaign_rally();
    assert!(new_poll > 20.0);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert!(player.active_campaign.as_ref().unwrap().polling_pct > 20.0);
}

#[test]
fn test_election_simulation_and_office_appointment() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:france".to_string(),
        location_id: "city:real:paris".to_string(),
        starting_age: 38,
        first_name: Some("Emmanuel".to_string()),
        last_name: Some("Macron".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("HIGH".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["politics".to_string()],
        goals: vec![],
    };

    let mut engine = SimulationEngine::new_game(config, 50505);
    engine.launch_political_campaign("office:real:french_president", "President of France", 500.0).unwrap();

    // Mutate campaign polling to top probability
    if let Some(ref mut c) = engine.persons.get_mut("person:sim:player").unwrap().active_campaign {
        c.polling_pct = 99.0;
    }

    let won = engine.simulate_election();
    assert!(won);

    let player = engine.persons.get("person:sim:player").unwrap();
    assert_eq!(player.political_office_title.as_deref(), Some("President of France"));
    assert_eq!(player.employment.monthly_salary, 5500.0);
    assert!(player.active_campaign.is_none());
}

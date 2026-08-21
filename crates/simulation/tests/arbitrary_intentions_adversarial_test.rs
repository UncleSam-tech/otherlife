use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;

#[test]
fn test_arbitrary_unscripted_intentions_resolution() {
    let config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:lagos".to_string(),
        starting_age: 15,
        birth_year: Some(1990),
        birth_month: Some(6),
        birth_day: Some(14),
        first_name: Some("Israel".to_string()),
        last_name: Some("Adeyemi".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        ..Default::default()
    };

    let mut engine = SimulationEngine::new_game(config, 777);

    // 1. Unscripted Intent: Ask neighbor to reduce noise
    let res1 = engine.submit_living_intent("Ask the neighbor across the compound to reduce loud generator noise in the evening");
    assert!(res1.success);
    assert!(res1.narrative.contains("generator noise") || res1.headline.contains("Life"));

    // 2. Unscripted Intent: Search for missing schoolbook
    let res2 = engine.submit_living_intent("Search the bedroom shelves and study desk thoroughly for my missing chemistry textbook");
    assert!(res2.success);

    // 3. Unscripted Intent: Premature corporate incorporation under 18
    let mut young_engine = SimulationEngine::new_game(NewLifeConfig {
        starting_age: 0,
        ..Default::default()
    }, 123);
    let infant_corp_res = young_engine.submit_living_intent("Incorporate a new limited liability company with commercial authorities");
    assert!(!infant_corp_res.success);
    assert!(infant_corp_res.narrative.contains("infant") || infant_corp_res.narrative.contains("Developmental"));

    // 4. Intent: Cuddle with mother for infant
    let cuddle_res = young_engine.submit_living_intent("Cuddle close to mother on the sofa");
    assert!(cuddle_res.success);
    assert!(cuddle_res.narrative.contains("mother"));
}

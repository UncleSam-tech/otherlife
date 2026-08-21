use otherlife_simulation::SimulationEngine;
use otherlife_world::{ClimateType, NewLifeConfig, WealthTier};

#[test]
fn test_multi_seed_generation_across_global_cities() {
    let cities = [
        ("city:real:edinburgh", "country:real:united_kingdom", "£", ClimateType::OceanicMaritime),
        ("city:real:london", "country:real:united_kingdom", "£", ClimateType::OceanicMaritime),
        ("city:real:san_francisco", "country:real:united_states", "$", ClimateType::MediterraneanMarine),
        ("city:real:houston", "country:real:united_states", "$", ClimateType::HumidSubtropical),
        ("city:real:lagos", "country:real:nigeria", "₦", ClimateType::TropicalSavanna),
        ("city:real:kano", "country:real:nigeria", "₦", ClimateType::TropicalSavanna),
    ];

    for (seed, (loc, country, expected_curr, expected_climate)) in cities.iter().enumerate() {
        let config = NewLifeConfig {
            creation_mode: "ORGANIC_BIRTH".to_string(),
            starting_year: 2000 + seed as i32,
            country_id: country.to_string(),
            location_id: loc.to_string(),
            starting_age: 0,
            birth_year: Some(2000 + seed as i32),
            birth_month: Some(6),
            birth_day: Some(15),
            first_name: Some("Test".to_string()),
            last_name: Some("Subject".to_string()),
            sex: Some("Male".to_string()),
            household_income_tier: Some("MIDDLE".to_string()),
            ..Default::default()
        };

        let engine = SimulationEngine::new_game(config, (seed * 1000 + 42) as u64);
        let state = engine.get_living_state();

        assert_eq!(state.age, 0);
        assert_eq!(state.currency_symbol, *expected_curr);
        assert_eq!(engine.rule_pack.climate_type, *expected_climate);
        assert_eq!(state.cash, 0.0, "Newborn must not possess cash savings!");
        assert!(engine.events_ledger.iter().any(|e| e.event_type == "BIRTH"));
    }
}

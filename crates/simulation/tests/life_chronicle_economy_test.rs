use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;

fn adult_life(city: &str, country: &str, seed: u64) -> SimulationEngine {
    SimulationEngine::new_game(
        NewLifeConfig {
            starting_year: 2026,
            starting_age: 25,
            location_id: city.to_string(),
            country_id: country.to_string(),
            household_income_tier: Some("MIDDLE".to_string()),
            ..Default::default()
        },
        seed,
    )
}

#[test]
fn lagos_fares_use_naira_scale_distance_and_real_transport_modes() {
    let engine = adult_life("city:real:lagos", "country:real:nigeria", 601);
    let university = engine
        .get_world_map()
        .into_iter()
        .find(|place| place.id == "place:university")
        .unwrap();

    assert_eq!(engine.get_living_state().currency_code, "NGN");
    assert!(engine.get_player().resources.cash >= 100_000.0);
    assert!(university.distance_km > 5.0);
    assert!(university.public_transit_cost >= 500.0);
    assert!(university.taxi_cost > university.public_transit_cost);
    assert!(university.walk_minutes > university.taxi_minutes);
}

#[test]
fn mobility_quotes_and_currency_change_with_country() {
    let lagos = adult_life("city:real:lagos", "country:real:nigeria", 602);
    let london = adult_life("city:real:london", "country:real:united_kingdom", 603);
    let san_francisco = adult_life("city:real:san_francisco", "country:real:united_states", 604);

    let quote = |engine: &SimulationEngine| {
        engine
            .get_world_map()
            .into_iter()
            .find(|place| place.id == "place:office")
            .unwrap()
    };
    assert_eq!(lagos.get_living_state().currency_code, "NGN");
    assert_eq!(london.get_living_state().currency_code, "GBP");
    assert_eq!(san_francisco.get_living_state().currency_code, "USD");
    assert!(quote(&lagos).public_transit_cost > 100.0);
    assert!((2.0..10.0).contains(&quote(&london).public_transit_cost));
    assert!((2.0..10.0).contains(&quote(&san_francisco).public_transit_cost));
}

#[test]
fn age_up_advances_one_chapter_and_writes_the_chronicle() {
    let mut engine = adult_life("city:real:lagos", "country:real:nigeria", 605);
    let age_before = engine.get_player_age();
    let events_before = engine.events_ledger.len();

    let result = engine.age_up_one_year();
    let chronicle = engine.get_life_chronicle(10);

    assert!(result.success);
    assert_eq!(result.days_advanced, 365);
    assert_eq!(engine.get_player_age(), age_before + 1);
    assert_eq!(engine.events_ledger.len(), events_before + 1);
    assert_eq!(chronicle[0].event_type, "AGE_UP");
    assert_eq!(chronicle[0].age, age_before + 1);
}

#[test]
fn international_arrival_converts_the_remaining_balance_into_destination_currency() {
    let mut engine = adult_life("city:real:lagos", "country:real:nigeria", 606);
    engine.get_player_mut().resources.cash = 3_000_000.0;

    let result = engine.travel_to_location_detailed(
        "city:real:london",
        "Flight",
        30,
        "Atlantic Air",
        "Standard flexible",
        1_350_000.0,
        "Serviced Apartment",
        "Tomorrow at 08:30",
        "Work",
        "Skilled worker visa and residence permit",
    );

    assert!(result.success);
    assert_eq!(engine.get_living_state().currency_code, "GBP");
    assert_eq!(engine.get_living_state().currency_symbol, "£");
    assert!(
        engine.get_player().resources.cash > 800.0 && engine.get_player().resources.cash < 900.0
    );
    assert!(result.narrative.contains("converted from NGN to GBP"));
}

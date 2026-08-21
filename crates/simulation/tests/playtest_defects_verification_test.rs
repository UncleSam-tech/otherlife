use otherlife_simulation::SimulationEngine;
use otherlife_world::NewLifeConfig;

#[test]
fn test_playtest_defect_1_and_2_geography_accuracy_abuja_and_glasgow() {
    // 1. Verify Abuja creates an authentic Abuja life
    let abuja_config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 0,
        first_name: Some("Emeka".to_string()),
        last_name: Some("Okafor".to_string()),
        sex: Some("Male".to_string()),
        ..Default::default()
    };
    let abuja_engine = SimulationEngine::new_game(abuja_config, 101);
    let state = abuja_engine.get_living_state();
    assert_eq!(state.location_formatted, "Abuja, Nigeria");
    assert_eq!(abuja_engine.rule_pack.city_name, "Abuja");
    assert_eq!(state.currency_symbol, "₦");

    // 2. Verify Glasgow creates an authentic Scottish life in Glasgow
    let glasgow_config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2005,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:glasgow".to_string(),
        starting_age: 14,
        first_name: Some("Callum".to_string()),
        last_name: Some("Sinclair".to_string()),
        sex: Some("Male".to_string()),
        ..Default::default()
    };
    let glasgow_engine = SimulationEngine::new_game(glasgow_config, 102);
    let g_state = glasgow_engine.get_living_state();
    assert_eq!(g_state.location_formatted, "Glasgow, United Kingdom");
    assert_eq!(glasgow_engine.rule_pack.city_name, "Glasgow");
    assert_eq!(g_state.currency_symbol, "£");
}

#[test]
fn test_playtest_defect_3_age_25_birth_year_calculation() {
    // Starting in 2030 at Age 25 must record birth year as 2005 (not 2030)
    let config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2030,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 25,
        first_name: Some("Israel".to_string()),
        last_name: Some("Oyebamiji".to_string()),
        sex: Some("Male".to_string()),
        ..Default::default()
    };
    let engine = SimulationEngine::new_game(config, 103);
    let player = engine.get_player();
    assert_eq!(player.identity.birth_year, 2005);
    assert_eq!(engine.time.year, 2030);
    assert_eq!(player.identity.calculate_age(engine.time.year, engine.time.month, engine.time.day), 25);
}

#[test]
fn test_playtest_defect_4_parent_names_and_profession_alignment() {
    let config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 0,
        first_name: Some("Israel".to_string()),
        last_name: Some("Oyebamiji".to_string()),
        mother_name: Some("Mary".to_string()),
        mother_job: Some("Architect".to_string()),
        father_name: Some("David".to_string()),
        father_job: Some("Structural Engineer".to_string()),
        ..Default::default()
    };
    let engine = SimulationEngine::new_game(config, 104);

    let mother = engine.npcs.get("person:sim:mother").unwrap();
    let father = engine.npcs.get("person:sim:father").unwrap();

    // Check no duplicate surnames
    assert_eq!(mother.base.identity.first_name, "Mary");
    assert_eq!(mother.base.identity.last_name, "Oyebamiji");
    assert_eq!(mother.base.identity.full_name(), "Mary Oyebamiji");

    assert_eq!(father.base.identity.first_name, "David");
    assert_eq!(father.base.identity.last_name, "Oyebamiji");
    assert_eq!(father.base.identity.full_name(), "David Oyebamiji");

    // Check routine alignment
    assert_eq!(mother.base.occupation.as_deref(), Some("Architect"));
    assert!(mother.daily_routine.iter().any(|act| act.location_id == "place:drafting_studio"));

    assert_eq!(father.base.occupation.as_deref(), Some("Structural Engineer"));
    assert!(father.daily_routine.iter().any(|act| act.location_id == "place:office"));
}

#[test]
fn test_playtest_defect_5_exact_calendar_weekday_calculation() {
    let config = NewLifeConfig {
        creation_mode: "ORGANIC_BIRTH".to_string(),
        starting_year: 2005,
        country_id: "country:real:nigeria".to_string(),
        location_id: "city:real:abuja".to_string(),
        starting_age: 0,
        birth_month: Some(6),
        birth_day: Some(14),
        ..Default::default()
    };
    let engine = SimulationEngine::new_game(config, 105);
    // June 14, 2005 was a Tuesday
    assert_eq!(engine.time.weekday_name(), "Tuesday");
    assert_eq!(engine.time.formatted_full_date(), "Tuesday, June 14, 2005");
}

#[test]
fn test_playtest_defect_6_explicit_time_operations() {
    let config = NewLifeConfig {
        starting_year: 2025,
        starting_age: 18,
        ..Default::default()
    };
    let mut engine = SimulationEngine::new_game(config, 106);

    // 1. Wait 1 hour
    let initial_hour = engine.time.hour;
    let initial_day = engine.time.day;
    let res_hour = engine.advance_hours(1);
    assert_eq!(res_hour.hours_advanced, 1);
    assert_eq!(res_hour.days_advanced, 0);
    assert_eq!(engine.time.hour, initial_hour + 1);
    assert_eq!(engine.time.day, initial_day);

    // 2. Sleep until morning
    let res_sleep = engine.sleep_until_morning();
    assert_eq!(res_sleep.days_advanced, 1);
    assert_eq!(engine.time.hour, 7);

    // 3. Follow routine for 7 days
    let day_before_routine = engine.time.day;
    let res_routine = engine.follow_routine(7);
    assert_eq!(res_routine.days_advanced, 7);
    assert_eq!(engine.time.day, day_before_routine + 7);
}

#[test]
fn test_playtest_defect_7_structured_intentions_and_documents() {
    let config = NewLifeConfig {
        starting_year: 2025,
        starting_age: 22,
        ..Default::default()
    };
    let mut engine = SimulationEngine::new_game(config, 107);

    // 1. Pediatric / clinical checkup
    let med_res = engine.attend_medical_checkup();
    assert_eq!(med_res.hours_advanced, 2);
    assert_eq!(med_res.days_advanced, 0);
    assert_eq!(engine.get_player().biology.health_overall, 100.0);

    // 2. Official Birth Certificate generation
    let docs = engine.get_documents();
    let birth_cert = docs.iter().find(|d| d.document_type == "BIRTH_CERTIFICATE");
    assert!(birth_cert.is_some());
    let bc = birth_cert.unwrap();
    assert!(bc.fields.contains_key("Full Legal Name"));
    assert!(bc.fields.contains_key("Place of Birth"));
    assert!(bc.fields.contains_key("Mother"));
    assert!(bc.fields.contains_key("Father"));
    assert!(bc.fields.contains_key("Registration Status"));

    // 3. Official Company Incorporation
    let corp_res = engine.register_company(
        "AeroDynamics Ltd",
        "Private Limited Liability (LTD)",
        &["Dr. David Oyebamiji".to_string()],
        1000000.0,
    );
    assert!(corp_res.success);
    assert_eq!(corp_res.days_advanced, 3);

    let updated_docs = engine.get_documents();
    let inc_cert = updated_docs.iter().find(|d| d.document_type == "COMPANY_INCORPORATION");
    assert!(inc_cert.is_some());
    let ic = inc_cert.unwrap();
    assert_eq!(ic.fields.get("Company Name").map(|s| s.as_str()), Some("AeroDynamics Ltd"));
    assert!(ic.fields.get("Registration Number").unwrap().starts_with("RC-"));
}

#[test]
fn test_playtest_defect_8_save_and_load_persistence_roundtrip() {
    let config = NewLifeConfig {
        starting_year: 2025,
        starting_age: 20,
        first_name: Some("Kelechi".to_string()),
        last_name: Some("Nnamdi".to_string()),
        ..Default::default()
    };
    let mut engine = SimulationEngine::new_game(config, 108);
    engine.advance_days(14);
    engine.register_company("Kelechi Ventures Ltd", "Private Limited (LTD)", &[], 500000.0);

    // Serialize
    let json_save = engine.save_to_string().expect("Serialization must succeed");
    assert!(!json_save.is_empty());

    // Restore
    let restored_engine = SimulationEngine::load_from_string(&json_save).expect("Deserialization must succeed");
    assert_eq!(restored_engine.get_player().identity.full_name(), "Kelechi Nnamdi");
    assert_eq!(restored_engine.time.total_days, engine.time.total_days);
    assert_eq!(restored_engine.documents.len(), engine.documents.len());
    let docs = restored_engine.get_documents();
    assert!(docs.iter().any(|d| d.title.contains("Kelechi Ventures Ltd")));
}

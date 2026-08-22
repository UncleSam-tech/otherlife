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

#[test]
fn test_structured_phone_message_persists_without_skipping_a_day() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 20,
        ..Default::default()
    }, 109);
    let initial_days = engine.time.total_days;
    let initial_count = engine.get_phone_messages().len();

    let result = engine.send_phone_message("person:sim:mother", "I will be home this evening.");
    assert!(result.success);
    assert_eq!(result.days_advanced, 0);
    assert_eq!(engine.time.total_days, initial_days);
    assert_eq!(engine.get_phone_messages().len(), initial_count + 1);

    let restored = SimulationEngine::load_from_string(&engine.save_to_string().unwrap()).unwrap();
    assert!(restored.get_phone_messages().iter().any(|message| message.text == "I will be home this evening."));
}

#[test]
fn test_structured_job_application_creates_a_tracked_process_and_notice() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 22,
        ..Default::default()
    }, 110);

    let result = engine.apply_for_job(
        "job_dev",
        "company:apex",
        "Junior Software Engineer",
        "Apex Digital Systems",
    );
    assert!(result.success);
    assert_eq!(result.days_advanced, 0);
    assert_eq!(result.hours_advanced, 1);
    assert!(engine.active_processes.iter().any(|process| {
        process.id == "proc:job:job_dev" && process.status == "APPLICATION_SUBMITTED"
    }));
    assert!(engine.letters_inbox.iter().any(|letter| letter.subject.contains("Application received")));

    let duplicate = engine.apply_for_job(
        "job_dev",
        "company:apex",
        "Junior Software Engineer",
        "Apex Digital Systems",
    );
    assert!(!duplicate.success);
}

#[test]
fn test_detailed_job_application_persists_the_submitted_materials() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 22,
        ..Default::default()
    }, 113);

    let result = engine.apply_for_job_detailed(
        "job_analyst",
        "company:meridian",
        "Research Analyst",
        "Meridian Advisory",
        "Economics graduate with field research experience.",
        "I can turn complex evidence into clear decisions.",
        "Two weeks' notice",
    );
    assert!(result.success);
    let application = engine.get_documents().into_iter()
        .find(|document| document.document_type == "JOB_APPLICATION_RECORD")
        .unwrap();
    assert_eq!(application.fields.get("Resume Profile").map(String::as_str), Some("Economics graduate with field research experience."));
    assert_eq!(application.fields.get("Cover Letter").map(String::as_str), Some("I can turn complex evidence into clear decisions."));
    assert_eq!(application.fields.get("Availability").map(String::as_str), Some("Two weeks' notice"));
}

#[test]
fn test_structured_travel_moves_the_player_and_creates_an_itinerary() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 25,
        location_id: "city:real:lagos".to_string(),
        ..Default::default()
    }, 111);
    let initial_cash = engine.get_player().resources.cash;

    let result = engine.travel_to_location("city:real:abuja", "Intercity Bus", 7);
    assert!(result.success);
    assert_eq!(result.days_advanced, 0);
    assert_eq!(result.hours_advanced, 10);
    assert_eq!(engine.rule_pack.city_name, "Abuja");
    assert!(engine.get_player().resources.cash < initial_cash);
    assert!(engine.get_documents().iter().any(|document| {
        document.document_type == "TRAVEL_TICKET"
            && document.fields.get("Accommodation").map(String::as_str) == Some("7 night(s) reserved")
    }));
}

#[test]
fn test_detailed_booking_charges_selected_fare_and_persists_the_itinerary() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 25,
        location_id: "city:real:lagos".to_string(),
        ..Default::default()
    }, 114);
    let initial_cash = engine.get_player().resources.cash;

    let result = engine.travel_to_location_detailed(
        "city:real:abuja",
        "Intercity Bus",
        7,
        "ABC Intercity",
        "Comfort priority",
        96.0,
        "Business District Suites",
        "Tomorrow morning at 12:15",
        "Visit",
        "Visitor / tourist entry",
    );
    assert!(result.success);
    assert_eq!(engine.get_player().resources.cash, initial_cash - 96.0);
    let itinerary = engine.get_documents().into_iter()
        .find(|document| document.document_type == "TRAVEL_TICKET")
        .unwrap();
    assert_eq!(itinerary.fields.get("Operator").map(String::as_str), Some("ABC Intercity"));
    assert_eq!(itinerary.fields.get("Service").map(String::as_str), Some("Comfort priority"));
    assert_eq!(itinerary.fields.get("Departure").map(String::as_str), Some("Tomorrow morning at 12:15"));
    assert_eq!(itinerary.fields.get("Accommodation").map(String::as_str), Some("Business District Suites · 7 night(s)"));
}

#[test]
fn test_detailed_company_registration_persists_operating_details() {
    let mut engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 25,
        ..Default::default()
    }, 115);

    let result = engine.register_company_detailed(
        "Northstar Studios Ltd",
        "Private company limited by shares",
        &["Amara Okafor".to_string()],
        100_000.0,
        "Interactive media production",
        "14 Unity Crescent, Lagos",
    );
    assert!(result.success);
    let certificate = engine.get_documents().into_iter()
        .find(|document| document.document_type == "COMPANY_INCORPORATION")
        .unwrap();
    assert_eq!(certificate.fields.get("Business Activity").map(String::as_str), Some("Interactive media production"));
    assert_eq!(certificate.fields.get("Registered Office").map(String::as_str), Some("14 Unity Crescent, Lagos"));
}

#[test]
fn test_birth_certificate_issue_date_is_not_before_birth() {
    let engine = SimulationEngine::new_game(NewLifeConfig {
        starting_year: 2025,
        starting_age: 25,
        birth_year: Some(2000),
        birth_month: Some(9),
        birth_day: Some(30),
        ..Default::default()
    }, 112);
    let certificate = engine.get_documents().into_iter()
        .find(|document| document.document_type == "BIRTH_CERTIFICATE")
        .unwrap();
    assert_eq!(certificate.issue_date, "2000-10-30");
}

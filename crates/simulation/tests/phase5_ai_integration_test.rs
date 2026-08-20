use otherlife_actions::ActionPrimitive;
use otherlife_ai_bridge::{AIBridge, AIBridgeConfig, BiographyWriter, HallucinationValidator, MemorySummarizer};
use otherlife_simulation::SimulationEngine;
use otherlife_world::{EventRecord, NewLifeConfig};
use std::collections::HashMap;

#[test]
fn test_multi_domain_intent_parsing() {
    let bridge = AIBridge::new(AIBridgeConfig::default());

    // 1. Lie / Deception
    let payload1 = bridge.parse_intent("Tell Mum I'm studying math, but secretly go to training.", "player", Some("mum"));
    assert_eq!(payload1.action, ActionPrimitive::Deceive);

    // 2. Job Application
    let payload2 = bridge.parse_intent("Apply for a part-time junior vacancy.", "player", None);
    assert_eq!(payload2.action, ActionPrimitive::ApplyJob);

    // 3. Rent Apartment
    let payload3 = bridge.parse_intent("Search for a city apartment to rent.", "player", None);
    assert_eq!(payload3.action, ActionPrimitive::RentApartment);

    // 4. Medical Care
    let payload4 = bridge.parse_intent("Go to the hospital to seek medical treatment.", "player", None);
    assert_eq!(payload4.action, ActionPrimitive::SeekMedicalTreatment);

    // 5. Marriage
    let payload5 = bridge.parse_intent("Propose marriage to partner at the wedding venue.", "player", None);
    assert_eq!(payload5.action, ActionPrimitive::Marry);
}

#[test]
fn test_hallucination_validator() {
    // When action succeeds but text claims complete failure
    let hallucinated_text = "The action failed completely and was rejected.";
    let sanitized = HallucinationValidator::validate_and_sanitize(hallucinated_text, true, "Hired as associate");
    assert!(sanitized.contains("succeeded as intended"));

    // Valid text passes through
    let valid_text = "You signed the lease for the city apartment.";
    let checked = HallucinationValidator::validate_and_sanitize(valid_text, true, "Lease signed");
    assert_eq!(checked, valid_text);
}

#[test]
fn test_memory_summarization_and_biography() {
    let events = vec![
        EventRecord {
            id: "ev-1".to_string(),
            timestamp: "12 OCT 2026 · 09:00".to_string(),
            event_type: "NEW_LIFE".to_string(),
            actor_id: "person:sim:player".to_string(),
            target_id: None,
            summary: "Began timeline in London.".to_string(),
            metadata: serde_json::json!({}),
            causality_parent_id: None,
        },
        EventRecord {
            id: "ev-2".to_string(),
            timestamp: "14 OCT 2026 · 10:00".to_string(),
            event_type: "ApplyJob".to_string(),
            actor_id: "person:sim:player".to_string(),
            target_id: None,
            summary: "Submitted job application and was hired as Staff Associate.".to_string(),
            metadata: serde_json::json!({}),
            causality_parent_id: None,
        },
    ];

    let memories = MemorySummarizer::summarize_events(&events);
    assert_eq!(memories.len(), 2);
    assert_eq!(memories[0].importance, 1.0); // NEW_LIFE event has top importance

    let bio = BiographyWriter::generate_lifetime_biography("Alex Morgan", &events);
    assert!(bio.contains("# The Life Biography of Alex Morgan"));
    assert!(bio.contains("Chapter 1: Foundations and Origins"));
    assert!(bio.contains("Began timeline in London."));
}

#[test]
fn test_offline_fallback_pipeline_execution() {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:london".to_string(),
        starting_age: 18,
        first_name: Some("Oliver".to_string()),
        last_name: Some("Twist".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["writing".to_string()],
        goals: vec!["write_novel".to_string()],
    };

    let mut engine = SimulationEngine::new_game(config, 9999);
    assert!(!engine.ai_bridge.config.use_local_llm); // 100% offline fallback

    let payload = engine.ai_bridge.parse_intent(
        "Apply for a part-time job vacancy at the publishing firm.",
        "person:sim:player",
        None,
    );

    let res = engine.execute_player_action(payload);
    assert!(res.success);
    assert!(res.narrative.contains("job application"));

    let bio = engine.get_biography();
    assert!(bio.contains("Oliver Twist"));
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::{
    CanonicalEntity, EntityNamespace, EntityType, NewLifeConfig, ResolutionContext, ResolutionResult,
};
use std::collections::HashMap;

fn create_test_engine() -> SimulationEngine {
    let config = NewLifeConfig {
        creation_mode: "CUSTOM".to_string(),
        starting_year: 2026,
        country_id: "country:real:united_kingdom".to_string(),
        location_id: "city:real:manchester".to_string(),
        starting_age: 20,
        first_name: Some("Alex".to_string()),
        last_name: Some("Hunter".to_string()),
        sex: Some("Male".to_string()),
        household_income_tier: Some("MIDDLE".to_string()),
        traits: HashMap::new(),
        skills: HashMap::new(),
        interests: vec!["football".to_string()],
        goals: vec![],
    };

    SimulationEngine::new_game(config, 12345)
}

#[test]
fn test_1_manchester_contains_multiple_football_clubs() {
    let engine = create_test_engine();
    let clubs = engine.resolver.find_entities_near("city:real:manchester", Some(EntityType::Club), 20.0);

    assert!(clubs.len() >= 3, "Manchester should contain at least 3 clubs (real + generated)");
    let names: Vec<String> = clubs.iter().map(|c| c.name.clone()).collect();
    assert!(names.contains(&"Manchester United FC".to_string()));
    assert!(names.contains(&"Manchester City FC".to_string()));
    assert!(names.contains(&"Manchester Local Youth FC".to_string()));
}

#[test]
fn test_2_city_search_does_not_default_to_most_famous() {
    let engine = create_test_engine();
    let result = engine.resolver.search_clubs("Manchester", Some("city:real:manchester"), None);

    match result {
        ResolutionResult::Ambiguous { candidates, prompt } => {
            assert!(candidates.len() >= 2);
            assert!(prompt.contains("Which Manchester do you mean") || prompt.contains("Manchester"));
        }
        ResolutionResult::Resolved(entity) => {
            panic!("Should not silently resolve to most famous club '{}' without confidence", entity.name);
        }
        _ => panic!("Expected Ambiguous resolution for multi-club search in Manchester"),
    }
}

#[test]
fn test_3_contextual_disambiguation_for_united() {
    let engine = create_test_engine();

    // Context A: Newcastle United in recent conversation
    let mut ctx_newcastle = ResolutionContext::default();
    ctx_newcastle.recent_entities = vec!["club:real:newcastle_united".to_string()];

    let res_newcastle = engine.resolver.search_clubs("United", None, Some(&ctx_newcastle));
    let candidates_newcastle = match res_newcastle {
        ResolutionResult::Ambiguous { candidates, .. } => candidates,
        ResolutionResult::Resolved(e) => vec![otherlife_world::EntityCandidate {
            entity: e,
            score: 100.0,
            match_reasons: vec![],
        }],
        _ => panic!("Expected candidates"),
    };
    assert_eq!(candidates_newcastle[0].entity.id, "club:real:newcastle_united");

    // Context B: Manchester United in recent conversation
    let mut ctx_manchester = ResolutionContext::default();
    ctx_manchester.recent_entities = vec!["club:real:manchester_united".to_string()];

    let res_mcr = engine.resolver.search_clubs("United", None, Some(&ctx_manchester));
    let candidates_mcr = match res_mcr {
        ResolutionResult::Ambiguous { candidates, .. } => candidates,
        ResolutionResult::Resolved(e) => vec![otherlife_world::EntityCandidate {
            entity: e,
            score: 100.0,
            match_reasons: vec![],
        }],
        _ => panic!("Expected candidates"),
    };
    assert_eq!(candidates_mcr[0].entity.id, "club:real:manchester_united");
}

#[test]
fn test_4_ambiguous_person_names_return_candidates() {
    let mut engine = create_test_engine();

    engine.resolver.register_entity(CanonicalEntity {
        id: "person:sim:john_smith".to_string(),
        name: "John Smith".to_string(),
        entity_type: EntityType::Person,
        aliases: vec!["John".to_string()],
        location_id: Some("city:real:manchester".to_string()),
        parent_org_id: None,
        fame_score: 10.0,
        namespace: EntityNamespace::Sim,
    });

    engine.resolver.register_entity(CanonicalEntity {
        id: "person:sim:john_ferguson".to_string(),
        name: "John Ferguson".to_string(),
        entity_type: EntityType::Person,
        aliases: vec!["John".to_string()],
        location_id: Some("city:real:manchester".to_string()),
        parent_org_id: None,
        fame_score: 10.0,
        namespace: EntityNamespace::Sim,
    });

    let res = engine.resolver.search_people("John", Some("city:real:manchester"), None);
    match res {
        ResolutionResult::Ambiguous { candidates, .. } => {
            assert!(candidates.len() >= 2);
        }
        ResolutionResult::Resolved(person) => {
            panic!("Should not silently guess person named '{}' when multiple Johns exist", person.name);
        }
        _ => panic!("Expected Ambiguous result for John"),
    }
}

#[test]
fn test_5_role_resolution_my_manager() {
    let mut engine = create_test_engine();

    // Assign player employment at a company with manager ID "person:sim:boss_steve"
    if let Some(p) = engine.persons.get_mut("person:sim:player") {
        p.employment.employer_org_id = Some("org:sim:local_company".to_string());
    }

    engine.resolver.assign_role("org:sim:local_company", "manager", "person:sim:boss_steve", 2026);

    let resolved_manager = engine.resolve_role_for_person("my manager");
    assert_eq!(resolved_manager, Some("person:sim:boss_steve".to_string()));
}

#[test]
fn test_6_real_club_with_generated_manager() {
    let mut engine = create_test_engine();

    // Assign a generated NPC as manager of Manchester United
    engine.resolver.assign_role("club:real:manchester_united", "manager", "person:sim:gen_manager_99", 2026);

    let mgr = engine.resolver.resolve_role("club:real:manchester_united", "manager", 2026);
    assert_eq!(mgr, Some("person:sim:gen_manager_99".to_string()));
}

#[test]
fn test_7_timeline_divergence_uses_generated_manager() {
    let mut engine = create_test_engine();

    // Year 2026 (Initial seed manager: Erik ten Hag)
    let init_mgr = engine.resolver.resolve_role("club:real:manchester_united", "manager", 2026);
    assert_eq!(init_mgr, Some("person:real:ten_hag".to_string()));

    // 10 simulated years later (Timeline divergence: replaced by generated manager)
    engine.time.year = 2036;
    engine.resolver.assign_role("club:real:manchester_united", "manager", "person:sim:gen_manager_2036", 2036);

    let diverged_mgr = engine.resolver.resolve_role("club:real:manchester_united", "manager", 2036);
    assert_eq!(diverged_mgr, Some("person:sim:gen_manager_2036".to_string()));
}

#[test]
fn test_8_real_and_generated_orgs_in_same_location() {
    let engine = create_test_engine();
    let orgs = engine.resolver.find_entities_near("city:real:manchester", None, 50.0);

    let real_exists = orgs.iter().any(|e| e.namespace == EntityNamespace::Real);
    let sim_exists = orgs.iter().any(|e| e.namespace == EntityNamespace::Sim);

    assert!(real_exists, "Real organizations should appear in location search");
    assert!(sim_exists, "Generated organizations should appear in location search");
}

#[test]
fn test_9_llm_cannot_create_authoritative_entity() {
    let engine = create_test_engine();

    // Search for an entity absent from the world database
    let res = engine.resolver.search_entities("Atlantis Cyber FC", Some(EntityType::Club), None, None);
    match res {
        ResolutionResult::NotFound => {
            // Correct behavior: absent entities return NotFound
        }
        _ => panic!("Entities absent from world DB must return NotFound"),
    }
}

#[test]
fn test_10_low_confidence_requests_player_clarification() {
    let engine = create_test_engine();

    // Search query with multiple candidate matches and no single dominant score
    let res = engine.resolver.search_entities("Manchester", None, Some("city:real:manchester"), None);
    match res {
        ResolutionResult::Ambiguous { prompt, candidates } => {
            assert!(candidates.len() >= 2);
            assert!(!prompt.is_empty());
        }
        ResolutionResult::Resolved(e) => {
            panic!("Low confidence search should not silently resolve to '{}'", e.name);
        }
        _ => panic!("Expected Ambiguous result for low-confidence query"),
    }
}

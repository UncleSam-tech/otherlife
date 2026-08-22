use otherlife_simulation::SimulationEngine;
use otherlife_world::{NewLifeConfig, ProcessType};

fn adult_lagos_life(seed: u64) -> SimulationEngine {
    SimulationEngine::new_game(
        NewLifeConfig {
            starting_year: 2026,
            starting_age: 25,
            location_id: "city:real:lagos".to_string(),
            country_id: "country:real:nigeria".to_string(),
            ..Default::default()
        },
        seed,
    )
}

#[test]
fn city_map_starts_at_home_and_commute_changes_place_time_and_cash() {
    let mut engine = adult_lagos_life(201);
    let map = engine.get_world_map();

    assert!(map.len() >= 8);
    assert!(map.iter().any(|place| place.id == "place:home" && place.is_current));
    assert!(map.iter().any(|place| place.id == "place:university"));
    assert!(map.iter().any(|place| place.id == "place:office"));
    assert!(map.iter().any(|place| place.id == "place:transport_terminal"));

    let cash_before = engine.get_player().resources.cash;
    let hour_before = engine.time.hour;
    let result = engine.commute_to_place("place:cafe", "Public Transit");

    assert!(result.success);
    assert_eq!(engine.get_living_state().current_place_id, "place:cafe");
    assert!(engine.get_player().resources.cash < cash_before);
    assert!(engine.time.hour > hour_before);
    assert!(engine.get_world_map().iter().any(|place| place.id == "place:cafe" && place.is_current));
}

#[test]
fn people_only_appear_where_their_schedule_places_them_and_conversation_persists() {
    let mut engine = adult_lagos_life(202);

    assert!(!engine.get_surrounding_npcs().iter().any(|npc| npc.id == "person:city:regular"));
    assert!(engine.commute_to_place("place:cafe", "Walk").success);

    let nearby = engine.get_surrounding_npcs();
    let regular = nearby.iter().find(|npc| npc.id == "person:city:regular").expect("cafe regular should be physically present");
    assert_eq!(regular.relationship_type, "Stranger");
    assert!(regular.is_new_acquaintance);

    let result = engine.converse_with_npc("person:city:regular", "Hi, what are you designing today?");
    assert!(result.success);
    let relationship = engine.get_player().relationships.get("person:city:regular").expect("conversation should create a relationship");
    assert_eq!(relationship.relationship_type, "Acquaintance");
    assert!(relationship.memories.iter().any(|memory| memory.description.contains("what are you designing")));
    assert!(engine.get_phone_contacts().iter().any(|contact| {
        contact.id == "person:city:regular" && contact.relationship_type == "Acquaintance"
    }));
}

#[test]
fn phone_contacts_persist_independently_of_physical_location() {
    let mut engine = adult_lagos_life(206);
    assert!(engine.commute_to_place("place:cafe", "Walk").success);
    assert!(engine.converse_with_npc("person:city:regular", "Let us stay in touch.").success);
    assert!(engine.commute_to_place("place:office", "Public Transit").success);

    assert!(!engine.get_surrounding_npcs().iter().any(|npc| npc.id == "person:city:regular"));
    assert!(engine.get_phone_contacts().iter().any(|contact| contact.id == "person:city:regular"));
}

#[test]
fn university_requires_presence_and_preserves_the_selected_course() {
    let mut engine = adult_lagos_life(203);

    let rejected = engine.apply_to_university(
        "Metropolitan University",
        "Bachelor of Science",
        "Computer Science",
        "Full-time",
        "Family funding",
    );
    assert!(!rejected.success);

    assert!(engine.commute_to_place("place:university", "Public Transit").success);
    let submitted = engine.apply_to_university(
        "Metropolitan University",
        "Bachelor of Science",
        "Computer Science",
        "Full-time",
        "Family funding",
    );
    assert!(submitted.success);
    assert!(engine.active_processes.iter().any(|process| {
        process.process_type == ProcessType::UniversityAdmission
            && process.total_steps == 6
            && process.current_step == 1
    }));
    assert!(engine.get_documents().iter().any(|document| {
        document.document_type == "UNIVERSITY_APPLICATION"
            && document.fields.get("Primary Course").map(String::as_str) == Some("Computer Science")
    }));
}

#[test]
fn incorporated_company_has_location_bound_ongoing_operations() {
    let mut engine = adult_lagos_life(204);
    assert!(engine.register_company_detailed(
        "Lantern Works Ltd",
        "Private Limited Company",
        &[],
        100_000.0,
        "Software and digital services",
        "14 Marina Road, Lagos",
    ).success);

    let rejected = engine.advance_company_operation(
        "Lantern Works Ltd",
        "Interview first HR manager",
        "Ask about hiring ethics, payroll controls, and conflict resolution.",
    );
    assert!(!rejected.success);

    assert!(engine.commute_to_place("place:office", "Taxi").success);
    let result = engine.advance_company_operation(
        "Lantern Works Ltd",
        "Interview first HR manager",
        "Ask about hiring ethics, payroll controls, and conflict resolution.",
    );
    assert!(result.success);
    assert!(engine.active_processes.iter().any(|process| {
        process.process_type == ProcessType::BusinessOperations
            && process.total_steps == 8
    }));
    assert!(engine.get_documents().iter().any(|document| {
        document.document_type == "BUSINESS_OPERATION_RECORD"
            && document.fields.get("Plan / Response").map(String::as_str)
                == Some("Ask about hiring ethics, payroll controls, and conflict resolution.")
    }));
}

#[test]
fn international_relocation_creates_residency_work_instead_of_ending_at_arrival() {
    let mut engine = adult_lagos_life(205);
    let result = engine.travel_to_location_detailed(
        "city:real:london",
        "Flight",
        0,
        "Atlantic Air",
        "Economy",
        210.0,
        "Temporary serviced apartment",
        "Tomorrow at 09:00",
        "Permanent relocation",
        "Skilled worker residence pathway",
    );

    assert!(result.success);
    assert_eq!(engine.get_living_state().current_place_id, "place:transport_terminal");
    assert!(engine.active_processes.iter().any(|process| {
        process.process_type == ProcessType::ResidencyApplication
            && process.status == "ENTRY_STATUS_REVIEW_REQUIRED"
            && process.target_institution_id.as_deref() == Some("place:civic_center")
    }));
}

use otherlife_actions::{ActionPayload, ActionPrimitive, ActionValidator};
use otherlife_ai_bridge::{AIBridge, AIBridgeConfig};
use otherlife_relationships::{RelationshipMatrix, RelationshipVector};
use otherlife_rng::WorldRng;
use otherlife_world::{
    AcademicDegree, AcademicProgram, ActivityType, AgeGate, BeliefComponent, BusinessEntity, CanonicalEntity,
    CareerCrisis, CovertOperation, CosmicLegacy, CosmicMegastructure, CreatorChannel, CreativeRelease,
    CriminalRecord, CyberneticImplant, DigitalPost, EducationComponent, EmploymentComponent, EntityNamespace,
    EntityType, EnvironmentalRating, EventRecord, FaithMovement, FameComponent, FinancesComponent, FootballContract,
    FootballMatch, FootballPlayerAttributes, FootballRole, FootballScoutReport, GeopoliticalConflict,
    HealthComponent, HousingComponent, IdentityComponent, KnowledgeRecord, LegalStatus, LifePivot, LifeSituation,
    LifeSituationChoice, LifeStage, MacroEconomy, MedicalRecord, MilitaryRecord, MindUpload, NaturalDisaster,
    NewLifeConfig, NpcSchedule, NpcTier, OrganizationSubunit, Passport, Patent, Person, PersonalityComponent,
    Place, PolicyProposal, PoliticalCampaign, PostScarcityEconomy, PrisonSentence, ProcessChain, ProcessStep,
    Qualification, ReputationRecord, ResearchProject, ResolutionContext, ResolutionResult, ResourceAccess,
    RomanceComponent, SecretMembership, SecretSociety, SimTime, SituationCategory, SocialMediaAccount, SpaceAgency,
    SpaceMission, SurgicalProcedure, TodayChoice, TodayScene, ActiveDeadline, TravelRecord, Visa, WeatherEvent,
    WillAndTestament, WorldEntityResolver, WorldNewsItem,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub success: bool,
    pub narrative: String,
    pub causality_note: String,
    pub event_record: EventRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentDTO {
    pub title: String,
    pub description: String,
    pub urgency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarStateDTO {
    pub commitments: Vec<CommitmentDTO>,
    pub household_trust: f32,
    pub household_resentment: f32,
    pub active_interest: String,
    pub primary_skill_name: String,
    pub primary_skill_value: f32,
    pub life_stage: String,
    pub marital_status: String,
    pub job_title: String,
    pub monthly_salary: f64,
    pub fitness: f32,
    pub stress: f32,
    pub public_reputation: f32,
    pub channel_subscribers: u64,
    pub active_crises_count: u32,
    pub life_pivots_count: u32,
}

pub struct SimulationInvariantValidator;

impl SimulationInvariantValidator {
    pub fn validate(engine: &SimulationEngine) -> Result<(), String> {
        for (person_id, person) in &engine.persons {
            let age = engine.time.compute_age(
                person.identity.birth_year,
                person.identity.birth_month,
                person.identity.birth_day,
            );

            // Invariant 1: Age 0 cannot work or earn a salary
            if age == 0 {
                if let Some(ref title) = person.employment.job_title {
                    if title != "Unemployed / Infant" && title != "Unemployed / Student" && title != "Unemployed" {
                        return Err(format!(
                            "Invariant Failure: Person {} at age 0 has active job_title '{}'",
                            person_id, title
                        ));
                    }
                }
                if person.employment.monthly_salary > 0.0 {
                    return Err(format!(
                        "Invariant Failure: Person {} at age 0 has positive salary {}",
                        person_id, person.employment.monthly_salary
                    ));
                }
            }

            // Invariant 2: Age 0 cannot hold contracts or adult football roles
            if age == 0 {
                if person.football_role != FootballRole::None {
                    return Err(format!(
                        "Invariant Failure: Person {} at age 0 has football_role {:?}",
                        person_id, person.football_role
                    ));
                }
                if person.football_contract.is_some() {
                    return Err(format!(
                        "Invariant Failure: Person {} at age 0 has football_contract",
                        person_id
                    ));
                }
            }

            // Invariant 3: Skills must satisfy 0.0 <= skill <= 100.0
            for (skill_name, skill_val) in &person.skills {
                if *skill_val < 0.0 || *skill_val > 100.0 {
                    return Err(format!(
                        "Invariant Failure: Person {} skill '{}' value {:.2} out of range [0, 100]",
                        person_id, skill_name, skill_val
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEngine {
    pub time: SimTime,
    pub rng: WorldRng,
    pub persons: HashMap<String, Person>,
    pub places: HashMap<String, Place>,
    pub relationships: RelationshipMatrix,
    pub macro_economy: MacroEconomy,
    pub businesses: HashMap<String, BusinessEntity>,
    pub policy_proposals: Vec<PolicyProposal>,
    pub conflicts: Vec<GeopoliticalConflict>,
    pub weather_events: Vec<WeatherEvent>,
    pub active_disasters: Vec<NaturalDisaster>,
    pub environmental_rating: EnvironmentalRating,
    pub secret_societies: Vec<SecretSociety>,
    pub covert_operations: Vec<CovertOperation>,
    pub space_agencies: Vec<SpaceAgency>,
    pub space_missions: Vec<SpaceMission>,
    pub post_scarcity_economy: PostScarcityEconomy,
    pub cosmic_legacy: CosmicLegacy,
    pub cosmic_megastructures: Vec<CosmicMegastructure>,
    pub resolver: WorldEntityResolver,
    pub active_situations: Vec<LifeSituation>,
    pub active_processes: Vec<ProcessChain>,
    pub active_deadlines: Vec<ActiveDeadline>,
    pub academic_program: Option<AcademicProgram>,
    pub reputation: ReputationRecord,
    pub resources: ResourceAccess,
    pub creator_channel: Option<CreatorChannel>,
    pub active_crises: Vec<CareerCrisis>,
    pub life_pivots: Vec<LifePivot>,
    pub events: Vec<EventRecord>,
    pub world_news: Vec<WorldNewsItem>,
    pub ai_bridge: AIBridge,
}

impl SimulationEngine {
    pub fn new_game(config: NewLifeConfig, seed: u64) -> Self {
        let mut rng = WorldRng::new(seed);
        let time = SimTime::new(config.starting_year, 10, 12, 09, 00);

        let player_id = "person:sim:player".to_string();

        let birth_year = config.starting_year - (config.starting_age as i32);
        let first_name = config.first_name.unwrap_or_else(|| "Alex".to_string());
        let last_name = config.last_name.unwrap_or_else(|| "Morgan".to_string());
        let sex = config.sex.unwrap_or_else(|| "Non-binary".to_string());
        let income_tier = config.household_income_tier.unwrap_or_else(|| "MIDDLE".to_string());

        let starting_cash = if config.starting_age == 0 {
            0.0
        } else {
            match income_tier.as_str() {
                "HIGH" => 2500.0,
                "LOW" => 15.0,
                _ => 150.0,
            }
        };

        let mut skills = config.skills;
        if config.starting_age == 0 {
            skills.clear();
        } else if skills.is_empty() {
            skills.insert("communication".to_string(), 45.0);
            skills.insert("reading".to_string(), 50.0);
        }

        let mut interests_set = HashSet::new();
        for int_str in &config.interests {
            interests_set.insert(int_str.clone());
        }

        // NO AUTOMATIC CAREER/CONTRACT ASSIGNMENT FROM INTEREST!
        let football_role = FootballRole::None;
        let football_contract = None;

        let housing = if config.starting_age >= 18 {
            HousingComponent {
                housing_type: "Renting".to_string(),
                monthly_cost: 650.0,
                quality: 75.0,
            }
        } else {
            HousingComponent::default()
        };

        let employment = if config.starting_age == 0 {
            EmploymentComponent {
                job_title: Some("Unemployed / Infant".to_string()),
                employer_org_id: None,
                monthly_salary: 0.0,
                job_performance: 0.0,
                years_in_role: 0,
            }
        } else if config.starting_age >= 22 {
            EmploymentComponent {
                job_title: Some("Junior Associate".to_string()),
                employer_org_id: Some("org:sim:local_company".to_string()),
                monthly_salary: 2200.0,
                job_performance: 65.0,
                years_in_role: 1,
            }
        } else {
            EmploymentComponent::default()
        };

        let mut qualifications = Vec::new();
        if config.starting_age >= 18 {
            qualifications.push(Qualification {
                title: "High School Diploma".to_string(),
                field: "General".to_string(),
                year_obtained: birth_year + 18,
            });
        }

        // Resolve country geography from location_id or default to country_id
        let country_id = if config.location_id.contains("glasgow") || config.location_id.contains("london") || config.location_id.contains("manchester") {
            "country:real:united_kingdom".to_string()
        } else if config.location_id.contains("lagos") || config.location_id.contains("abuja") {
            "country:real:nigeria".to_string()
        } else if config.location_id.contains("new_york") {
            "country:real:united_states".to_string()
        } else if config.location_id.contains("paris") {
            "country:real:france".to_string()
        } else if config.location_id.contains("madrid") {
            "country:real:spain".to_string()
        } else {
            config.country_id.clone()
        };

        // Generate parent/guardian dynamically if starting age < 18
        let parent_id = if config.starting_age < 18 {
            Some(format!("person:sim:parent_{}", seed % 10000))
        } else {
            None
        };

        let player = Person {
            id: player_id.clone(),
            is_player: true,
            is_alive: true,
            tier: NpcTier::TierA,
            schedule: NpcSchedule {
                current_activity: ActivityType::Home,
                work_start_hour: 9,
                work_end_hour: 17,
                primary_location_id: config.location_id.clone(),
            },
            identity: IdentityComponent {
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                birth_year,
                birth_month: 4,
                birth_day: 12,
                sex,
                birth_location_id: config.location_id.clone(),
                current_location_id: config.location_id.clone(),
                nationalities: vec![country_id.clone()],
                citizenships: vec![country_id.clone()],
            },
            personality: PersonalityComponent {
                openness: *config.traits.get("creativity").unwrap_or(&0.5),
                conscientiousness: *config.traits.get("discipline").unwrap_or(&0.5),
                extraversion: *config.traits.get("sociability").unwrap_or(&0.5),
                agreeableness: *config.traits.get("empathy").unwrap_or(&0.5),
                neuroticism: 0.5,
                ambition: *config.traits.get("ambition").unwrap_or(&0.5),
                discipline: *config.traits.get("discipline").unwrap_or(&0.5),
                risk_tolerance: *config.traits.get("risk_tolerance").unwrap_or(&0.5),
            },
            skills,
            interests: interests_set,
            goals: config.goals.clone(),
            education: EducationComponent {
                school_id: if config.starting_age >= 5 && config.starting_age <= 18 {
                    Some("school:sim:local_school".to_string())
                } else {
                    None
                },
                grade_level: (config.starting_age as u32).saturating_sub(5),
                academic_performance: 65.0,
                attendance_rate: 92.0,
                qualifications,
                degree_program: None,
            },
            employment,
            housing,
            health: HealthComponent::default(),
            romance: RomanceComponent::default(),
            finances: FinancesComponent {
                cash: starting_cash,
                monthly_allowance: if config.starting_age < 18 && config.starting_age > 5 { 10.0 } else { 0.0 },
                household_income_tier: income_tier.clone(),
                monthly_expenses: if config.starting_age < 18 { 0.0 } else { 50.0 },
            },
            football_role,
            football_attributes: FootballPlayerAttributes::default(),
            football_contract,
            owned_business_ids: Vec::new(),
            political_party_id: None,
            political_office_title: None,
            active_campaign: None,
            fame: FameComponent::default(),
            creative_releases: Vec::new(),
            legal_status: LegalStatus::Clean,
            criminal_records: Vec::new(),
            prison_sentence: None,
            academic_degrees: Vec::new(),
            research_projects: Vec::new(),
            patents: Vec::new(),
            belief: BeliefComponent::default(),
            founded_movements: Vec::new(),
            passports: Vec::new(),
            visas: Vec::new(),
            travel_history: Vec::new(),
            military_record: None,
            medical_history: Vec::new(),
            surgical_history: Vec::new(),
            will_and_testament: None,
            social_media_accounts: Vec::new(),
            digital_posts: Vec::new(),
            secret_memberships: Vec::new(),
            space_missions: Vec::new(),
            cybernetic_implants: Vec::new(),
            mind_uploads: Vec::new(),
            cosmic_megastructures: Vec::new(),
            location_id: config.location_id.clone(),
            parent_ids: parent_id.clone().into_iter().collect(),
            child_ids: Vec::new(),
            active_roles: Vec::new(),
            knowledge: HashSet::new(),
            secrets: Vec::new(),
            memories: Vec::new(),
        };

        let mut persons = HashMap::new();
        persons.insert(player_id.clone(), player);

        let mut relationships = RelationshipMatrix::new();

        if let Some(ref pid) = parent_id {
            let parent_person = Person {
                id: pid.clone(),
                is_player: false,
                is_alive: true,
                tier: NpcTier::TierA,
                schedule: NpcSchedule {
                    current_activity: ActivityType::Work,
                    work_start_hour: 8,
                    work_end_hour: 16,
                    primary_location_id: config.location_id.clone(),
                },
                identity: IdentityComponent {
                    first_name: "Sarah".to_string(),
                    last_name: last_name.clone(),
                    birth_year: birth_year - 28,
                    birth_month: 8,
                    birth_day: 24,
                    sex: "Female".to_string(),
                    birth_location_id: config.location_id.clone(),
                    current_location_id: config.location_id.clone(),
                    nationalities: vec![country_id.clone()],
                    citizenships: vec![country_id.clone()],
                },
                personality: PersonalityComponent::default(),
                skills: HashMap::new(),
                interests: HashSet::new(),
                goals: Vec::new(),
                education: EducationComponent::default(),
                employment: EmploymentComponent {
                    job_title: Some("Civil Servant".to_string()),
                    employer_org_id: Some("org:sim:local_gov".to_string()),
                    monthly_salary: 2800.0,
                    job_performance: 75.0,
                    years_in_role: 5,
                },
                housing: HousingComponent::default(),
                health: HealthComponent::default(),
                romance: RomanceComponent::default(),
                finances: FinancesComponent {
                    cash: 5000.0,
                    monthly_allowance: 0.0,
                    household_income_tier: income_tier.clone(),
                    monthly_expenses: 500.0,
                },
                football_role: FootballRole::None,
                football_attributes: FootballPlayerAttributes::default(),
                football_contract: None,
                owned_business_ids: Vec::new(),
                political_party_id: None,
                political_office_title: None,
                active_campaign: None,
                fame: FameComponent::default(),
                creative_releases: Vec::new(),
                legal_status: LegalStatus::Clean,
                criminal_records: Vec::new(),
                prison_sentence: None,
                academic_degrees: Vec::new(),
                research_projects: Vec::new(),
                patents: Vec::new(),
                belief: BeliefComponent::default(),
                founded_movements: Vec::new(),
                passports: Vec::new(),
                visas: Vec::new(),
                travel_history: Vec::new(),
                military_record: None,
                medical_history: Vec::new(),
                surgical_history: Vec::new(),
                will_and_testament: None,
                social_media_accounts: Vec::new(),
                digital_posts: Vec::new(),
                secret_memberships: Vec::new(),
                space_missions: Vec::new(),
                cybernetic_implants: Vec::new(),
                mind_uploads: Vec::new(),
                cosmic_megastructures: Vec::new(),
                location_id: config.location_id.clone(),
                parent_ids: Vec::new(),
                child_ids: vec![player_id.clone()],
                active_roles: vec!["Parent".to_string()],
                knowledge: HashSet::new(),
                secrets: Vec::new(),
                memories: Vec::new(),
            };

            persons.insert(pid.clone(), parent_person);
            relationships.set_link(
                pid.clone(),
                player_id.clone(),
                RelationshipVector::new_parent_child(),
            );
        }

        let mut resolver = WorldEntityResolver::new();

        // 1. Real Cities
        resolver.register_entity(CanonicalEntity {
            id: "city:real:manchester".to_string(),
            name: "Manchester".to_string(),
            entity_type: EntityType::City,
            aliases: vec!["Mcr".to_string()],
            location_id: Some("country:real:united_kingdom".to_string()),
            parent_org_id: None,
            fame_score: 90.0,
            namespace: EntityNamespace::Real,
        });

        resolver.register_entity(CanonicalEntity {
            id: "city:real:glasgow".to_string(),
            name: "Glasgow".to_string(),
            entity_type: EntityType::City,
            aliases: vec!["Glaschu".to_string()],
            location_id: Some("country:real:united_kingdom".to_string()),
            parent_org_id: None,
            fame_score: 85.0,
            namespace: EntityNamespace::Real,
        });

        resolver.register_entity(CanonicalEntity {
            id: "city:real:newcastle".to_string(),
            name: "Newcastle".to_string(),
            entity_type: EntityType::City,
            aliases: vec!["Newcastle upon Tyne".to_string()],
            location_id: Some("country:real:united_kingdom".to_string()),
            parent_org_id: None,
            fame_score: 80.0,
            namespace: EntityNamespace::Real,
        });

        // 2. Football Clubs in Manchester (Real + Generated)
        resolver.register_entity(CanonicalEntity {
            id: "club:real:manchester_united".to_string(),
            name: "Manchester United FC".to_string(),
            entity_type: EntityType::Club,
            aliases: vec!["Manchester United".to_string(), "Man Utd".to_string(), "United".to_string(), "Red Devils".to_string()],
            location_id: Some("city:real:manchester".to_string()),
            parent_org_id: None,
            fame_score: 98.0,
            namespace: EntityNamespace::Real,
        });

        resolver.register_entity(CanonicalEntity {
            id: "club:real:manchester_city".to_string(),
            name: "Manchester City FC".to_string(),
            entity_type: EntityType::Club,
            aliases: vec!["Manchester City".to_string(), "Man City".to_string(), "City".to_string(), "Cityzens".to_string()],
            location_id: Some("city:real:manchester".to_string()),
            parent_org_id: None,
            fame_score: 97.0,
            namespace: EntityNamespace::Real,
        });

        resolver.register_entity(CanonicalEntity {
            id: "club:sim:manchester_local_youth".to_string(),
            name: "Manchester Local Youth FC".to_string(),
            entity_type: EntityType::Club,
            aliases: vec!["Mcr Youth".to_string(), "Manchester Local FC".to_string()],
            location_id: Some("city:real:manchester".to_string()),
            parent_org_id: None,
            fame_score: 25.0,
            namespace: EntityNamespace::Sim,
        });

        resolver.register_entity(CanonicalEntity {
            id: "club:sim:manchester_amateurs".to_string(),
            name: "Manchester Amateur Athletic Club".to_string(),
            entity_type: EntityType::Club,
            aliases: vec!["Mcr Amateurs".to_string()],
            location_id: Some("city:real:manchester".to_string()),
            parent_org_id: None,
            fame_score: 15.0,
            namespace: EntityNamespace::Sim,
        });

        // 3. Other United clubs
        resolver.register_entity(CanonicalEntity {
            id: "club:real:newcastle_united".to_string(),
            name: "Newcastle United FC".to_string(),
            entity_type: EntityType::Club,
            aliases: vec!["Newcastle United".to_string(), "Newcastle".to_string(), "United".to_string(), "Magpies".to_string()],
            location_id: Some("city:real:newcastle".to_string()),
            parent_org_id: None,
            fame_score: 90.0,
            namespace: EntityNamespace::Real,
        });

        // 4. Role Assignments & Managers
        let mgr_mcr_utd = "person:real:ten_hag".to_string();
        resolver.assign_role("club:real:manchester_united", "manager", &mgr_mcr_utd, config.starting_year);

        let mgr_newcastle = "person:real:eddie_howe".to_string();
        resolver.assign_role("club:real:newcastle_united", "manager", &mgr_newcastle, config.starting_year);

        // Subunits hierarchy example
        resolver.register_subunit(OrganizationSubunit {
            id: "subunit:manchester_city_academy".to_string(),
            name: "Manchester City Academy".to_string(),
            subunit_type: "Academy".to_string(),
            parent_id: "club:real:manchester_city".to_string(),
            roles: Vec::new(),
        });

        let ai_bridge = AIBridge::new(AIBridgeConfig::default());

        let mut engine = Self {
            time,
            rng,
            persons,
            places: HashMap::new(),
            relationships,
            macro_economy: MacroEconomy::default(),
            businesses: HashMap::new(),
            policy_proposals: Vec::new(),
            conflicts: Vec::new(),
            weather_events: Vec::new(),
            active_disasters: Vec::new(),
            environmental_rating: EnvironmentalRating::default(),
            secret_societies: Vec::new(),
            covert_operations: Vec::new(),
            space_agencies: Vec::new(),
            space_missions: Vec::new(),
            post_scarcity_economy: PostScarcityEconomy::default(),
            cosmic_legacy: CosmicLegacy::default(),
            cosmic_megastructures: Vec::new(),
            resolver,
            active_situations: Vec::new(),
            active_processes: Vec::new(),
            active_deadlines: Vec::new(),
            academic_program: None,
            reputation: ReputationRecord::default(),
            resources: ResourceAccess::default(),
            creator_channel: None,
            active_crises: Vec::new(),
            life_pivots: Vec::new(),
            events: Vec::new(),
            world_news: Vec::new(),
            ai_bridge,
        };

        let player_age = config.starting_age;
        if (14..=17).contains(&player_age) && config.interests.contains(&"football".to_string()) {
            let deadline_day = engine.time.total_days() + 30;
            engine.active_deadlines.push(ActiveDeadline {
                id: "football_trial_youth".to_string(),
                title: "Regional Youth Football Trial".to_string(),
                description: "Open trial session with scouts in the region.".to_string(),
                deadline_day_total: deadline_day,
                category: "FOOTBALL".to_string(),
            });
        }

        engine.generate_active_situations();

        engine.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: engine.time.formatted(),
            headline: "Local Economy and Community Digest Updated".to_string(),
            body: "City authorities published the quarterly local development report.".to_string(),
            category: "LOCAL".to_string(),
            source_event_id: "init-world-news".to_string(),
        });

        engine
    }

    pub fn new_vertical_slice_fixture(seed: u64) -> Self {
        let mut config = NewLifeConfig {
            creation_mode: "CUSTOM".to_string(),
            starting_year: 2029,
            country_id: "country:real:united_kingdom".to_string(),
            location_id: "city:real:glasgow".to_string(),
            starting_age: 14,
            first_name: Some("James".to_string()),
            last_name: Some("Morrison".to_string()),
            sex: Some("Male".to_string()),
            household_income_tier: Some("MIDDLE".to_string()),
            traits: HashMap::new(),
            skills: HashMap::new(),
            interests: vec!["football".to_string()],
            goals: vec!["play_pro_football".to_string()],
        };

        config.skills.insert("football_control".to_string(), 70.0);
        config.skills.insert("athleticism".to_string(), 72.0);
        config.skills.insert("mathematics".to_string(), 42.0);

        let mut engine = Self::new_game(config, seed);
        if let Some(p) = engine.persons.get_mut("person:sim:player") {
            p.education.academic_performance = 42.0;
            p.finances.cash = 24.0;
        }

        engine
    }

    pub fn step_time_forward(&mut self, days: u32) {
        self.time.advance_days(days);
        self.update_deadlines_and_education(days);
        SimulationInvariantValidator::validate(self).expect("Simulation invariants broken during time advance");
    }

    pub fn update_deadlines_and_education(&mut self, days_passed: u32) {
        let current_day_total = self.time.total_days();
        let player_id = "person:sim:player".to_string();

        // 1. Process and expire deadlines
        let mut remaining_deadlines = Vec::new();
        for dl in self.active_deadlines.drain(..) {
            if current_day_total >= dl.deadline_day_total {
                // Deadline expired naturally
                let event = EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "OPPORTUNITY_EXPIRED".to_string(),
                    actor_id: player_id.clone(),
                    target_id: None,
                    summary: format!("The window for '{}' has closed.", dl.title),
                    metadata: serde_json::json!({ "deadline_id": dl.id }),
                    causality_parent_id: None,
                };
                self.events.push(event);
            } else {
                remaining_deadlines.push(dl);
            }
        }
        self.active_deadlines = remaining_deadlines;

        // 2. Multi-year university progression
        if let Some(ref mut prog) = self.academic_program {
            if !prog.is_graduated {
                if days_passed >= 30 {
                    if self.time.month == 6 && prog.current_semester == 1 {
                        prog.current_semester = 2;
                        self.events.push(EventRecord {
                            id: uuid::Uuid::new_v4().to_string(),
                            timestamp: self.time.formatted(),
                            event_type: "ACADEMIC_SEMESTER_PASSED".to_string(),
                            actor_id: player_id.clone(),
                            target_id: None,
                            summary: format!("Completed Semester 1 examinations for Year {} in {} at {}.", prog.current_year, prog.degree_title, prog.university_name),
                            metadata: serde_json::json!({ "year": prog.current_year, "semester": 1 }),
                            causality_parent_id: None,
                        });
                    } else if self.time.month == 12 && prog.current_semester == 2 {
                        if prog.current_year < prog.total_years {
                            prog.current_year += 1;
                            prog.current_semester = 1;
                            self.events.push(EventRecord {
                                id: uuid::Uuid::new_v4().to_string(),
                                timestamp: self.time.formatted(),
                                event_type: "ACADEMIC_YEAR_ADVANCED".to_string(),
                                actor_id: player_id.clone(),
                                target_id: None,
                                summary: format!("Advanced to Year {} of {} at {}!", prog.current_year, prog.degree_title, prog.university_name),
                                metadata: serde_json::json!({ "year": prog.current_year }),
                                causality_parent_id: None,
                            });
                        } else {
                            prog.is_graduated = true;
                            if let Some(p) = self.persons.get_mut(&player_id) {
                                p.education.qualifications.push(Qualification {
                                    title: prog.degree_title.clone(),
                                    field: prog.faculty.clone(),
                                    year_obtained: self.time.year,
                                });
                                p.education.degree_program = Some(prog.degree_title.clone());
                            }
                            self.events.push(EventRecord {
                                id: uuid::Uuid::new_v4().to_string(),
                                timestamp: self.time.formatted(),
                                event_type: "UNIVERSITY_GRADUATION".to_string(),
                                actor_id: player_id.clone(),
                                target_id: None,
                                summary: format!("Graduated from {} with a {} in {}!", prog.university_name, prog.degree_title, prog.faculty),
                                metadata: serde_json::json!({ "degree": prog.degree_title, "faculty": prog.faculty }),
                                causality_parent_id: None,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn resolve_role_for_person(&self, role_title: &str) -> Option<String> {
        let player = self.persons.get("person:sim:player")?;
        let role_lower = role_title.to_lowercase();

        if role_lower.contains("manager") || role_lower.contains("boss") || role_lower.contains("ceo") {
            if let Some(ref emp_org) = player.employment.employer_org_id {
                if let Some(p_id) = self.resolver.resolve_role(emp_org, "manager", self.time.year) {
                    return Some(p_id);
                }
            }
            if let Some(ref contract) = player.football_contract {
                if let Some(p_id) = self.resolver.resolve_role(&contract.club_id, "manager", self.time.year) {
                    return Some(p_id);
                }
            }
        }

        if role_lower.contains("coach") {
            if let Some(ref contract) = player.football_contract {
                if let Some(p_id) = self.resolver.resolve_role(&contract.club_id, "head_coach", self.time.year) {
                    return Some(p_id);
                }
            }
        }

        None
    }

    pub fn build_resolution_context(&self, recent_entities: Vec<String>) -> ResolutionContext {
        let player = self.persons.get("person:sim:player").unwrap();
        ResolutionContext {
            player_id: player.id.clone(),
            player_location_id: player.location_id.clone(),
            current_domain: player.interests.iter().next().cloned(),
            recent_entities,
            current_year: self.time.year,
            player_age: (self.time.year - player.identity.birth_year) as u32,
            player_gender: Some(player.identity.sex.clone()),
            player_skills: player.skills.clone(),
            current_employer_id: player.employment.employer_org_id.clone(),
            current_club_id: player.football_contract.as_ref().map(|c| c.club_id.clone()),
            relationships: Vec::new(),
        }
    }

    pub fn distribute_universal_basic_dividend(&mut self, dividend_amount: f64) -> f64 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.finances.cash += dividend_amount;
        self.post_scarcity_economy.universal_basic_dividend = dividend_amount;

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "UNIVERSAL_BASIC_DIVIDEND".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Received automated Post-Scarcity Universal Basic Dividend of £{:.2}.", dividend_amount),
            metadata: serde_json::json!({ "dividend": dividend_amount }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: "Post-Scarcity Economy Dividend Dispatched".to_string(),
            body: format!("Universal basic income dividend (£{:.2}) deposited to all citizens.", dividend_amount),
            category: "COSMIC_LEGACY".to_string(),
            source_event_id: "ubd-news".to_string(),
        });

        dividend_amount
    }

    pub fn construct_cosmic_megastructure(&mut self, name: &str, structure_type: &str, energy_output_gw: f64) -> CosmicMegastructure {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let mega = CosmicMegastructure {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            structure_type: structure_type.to_string(),
            completion_pct: 100.0,
            energy_output_gw,
            built_year: current_year,
        };

        player.cosmic_megastructures.push(mega.clone());
        self.cosmic_megastructures.push(mega.clone());
        self.cosmic_legacy.civilization_kardashev_tier = "TYPE_II".to_string();

        let summary = format!("COSMIC MEGASTRUCTURE COMPLETED! Built '{}' ({}) outputting {:.0} GW energy!", name, structure_type, energy_output_gw);

        self.events.push(EventRecord {
            id: mega.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "COSMIC_MEGASTRUCTURE_BUILT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "name": name, "type": structure_type, "energy_gw": energy_output_gw }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Cosmic Megastructure Milestone: {}", name),
            body: summary,
            category: "COSMIC_LEGACY".to_string(),
            source_event_id: mega.id.clone(),
        });

        mega
    }

    pub fn establish_interstellar_colony(&mut self, planet_name: &str, settlers_count: u32) -> u32 {
        self.cosmic_legacy.interstellar_colonies_count += 1;
        let total_colonies = self.cosmic_legacy.interstellar_colonies_count;

        let player = self.persons.get_mut("person:sim:player").unwrap();

        let summary = format!("INTERSTELLAR COLONIZATION SUCCESS! Established colony on planet '{}' with {} settlers.", planet_name, settlers_count);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "INTERSTELLAR_COLONY_ESTABLISHED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "planet": planet_name, "settlers": settlers_count, "total_colonies": total_colonies }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Galactic Expansion: Colony on {}", planet_name),
            body: summary,
            category: "COSMIC_LEGACY".to_string(),
            source_event_id: "interstellar-colony".to_string(),
        });

        total_colonies
    }

    pub fn evaluate_cosmic_legacy(&self) -> CosmicLegacy {
        self.cosmic_legacy.clone()
    }

    pub fn install_cybernetic_implant(&mut self, name: &str, implant_type: &str, augmentation_level: f32, cost: f64) -> Result<CyberneticImplant, String> {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < cost {
            return Err("Insufficient cash funds to pay cybernetic augmentation procedure fees.".to_string());
        }

        player.finances.cash -= cost;

        let implant = CyberneticImplant {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            implant_type: implant_type.to_string(),
            augmentation_level,
            installation_year: current_year,
            maintenance_cost: cost * 0.05,
        };

        player.cybernetic_implants.push(implant.clone());
        player.personality.openness = (player.personality.openness + 0.10).min(1.0);

        self.events.push(EventRecord {
            id: implant.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "CYBERNETIC_IMPLANT_INSTALLED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Installed cybernetic augmentation '{}' ({}, +{:.1} level).", name, implant_type, augmentation_level),
            metadata: serde_json::json!({ "implant": name, "type": implant_type, "level": augmentation_level }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Transhumanism Advancement: Cybernetic {}", implant_type),
            body: format!("Patient completed cybernetic implant surgery: '{}'.", name),
            category: "CYBERNETICS".to_string(),
            source_event_id: implant.id.clone(),
        });

        Ok(implant)
    }

    pub fn upload_mind_to_digital_avatar(&mut self, avatar_name: &str, substrate: &str) -> MindUpload {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let upload = MindUpload {
            id: uuid::Uuid::new_v4().to_string(),
            digital_avatar_name: avatar_name.to_string(),
            upload_fidelity: 99.4,
            substrate: substrate.to_string(),
            year_uploaded: current_year,
            status: "ONLINE_ACTIVE".to_string(),
        };

        player.mind_uploads.push(upload.clone());

        let summary = format!("CONSCIOUSNESS UPLOAD COMPLETE! Mind uploaded to digital avatar '{}' on {} substrate.", avatar_name, substrate);

        self.events.push(EventRecord {
            id: upload.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "MIND_UPLOAD_COMPLETED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "avatar": avatar_name, "substrate": substrate }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Bio-Digital Immortality Achieved: {}", avatar_name),
            body: summary,
            category: "CYBERNETICS".to_string(),
            source_event_id: upload.id.clone(),
        });

        upload
    }

    pub fn upgrade_avatar_substrate(&mut self, mind_upload_id: &str, new_substrate: &str) -> String {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut substrate_str = new_substrate.to_string();

        if let Some(up) = player.mind_uploads.iter_mut().find(|m| m.id == mind_upload_id) {
            up.substrate = new_substrate.to_string();
            up.upload_fidelity = 99.9;
            substrate_str = up.substrate.clone();
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "SUBSTRATE_UPGRADED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Upgraded digital avatar mind substrate to {}.", substrate_str),
            metadata: serde_json::json!({ "substrate": substrate_str }),
            causality_parent_id: None,
        });

        substrate_str
    }

    pub fn fund_space_agency(&mut self, name: &str, agency_type: &str, seed_capital: f64) -> Result<SpaceAgency, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < seed_capital {
            return Err("Insufficient cash funds to found aerospace agency.".to_string());
        }

        player.finances.cash -= seed_capital;

        let agency = SpaceAgency {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            agency_type: agency_type.to_string(),
            reputation: 75.0,
            budget: seed_capital,
        };

        self.space_agencies.push(agency.clone());

        self.events.push(EventRecord {
            id: agency.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "FUNDED_SPACE_AGENCY".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Founded aerospace organization '{}' ({}) with £{:.2} seed capital.", name, agency_type, seed_capital),
            metadata: serde_json::json!({ "agency": name, "type": agency_type, "capital": seed_capital }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("New Aerospace Agency Founded: {}", name),
            body: format!("Aerospace venture '{}' registered for orbital exploration missions.", name),
            category: "SPACE".to_string(),
            source_event_id: agency.id.clone(),
        });

        Ok(agency)
    }

    pub fn launch_space_mission(&mut self, mission_name: &str, mission_type: &str, destination: &str, budget: f64) -> SpaceMission {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let mission = SpaceMission {
            id: uuid::Uuid::new_v4().to_string(),
            name: mission_name.to_string(),
            mission_type: mission_type.to_string(),
            destination: destination.to_string(),
            launch_year: current_year,
            budget,
            success_rate: 0.92,
            status: "ORBIT_SUCCESS".to_string(),
        };

        player.space_missions.push(mission.clone());
        self.space_missions.push(mission.clone());

        let summary = format!("SPACE MISSION LAUNCH SUCCESS! '{}' ({}) launched to {} (Budget: £{:.2}).", mission_name, mission_type, destination, budget);

        self.events.push(EventRecord {
            id: mission.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "SPACE_MISSION_LAUNCH".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "name": mission_name, "destination": destination, "budget": budget }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Space Exploration Digest: {}", mission_name),
            body: summary,
            category: "SPACE".to_string(),
            source_event_id: mission.id.clone(),
        });

        mission
    }

    pub fn deploy_satellite(&mut self, satellite_name: &str, orbit_type: &str) -> SpaceMission {
        self.launch_space_mission(satellite_name, "ORBITAL_SATELLITE", orbit_type, 45000.0)
    }

    pub fn register_space_patent(&mut self, title: &str, valuation: f64) -> Patent {
        self.file_patent(title, "Aerospace Tech", valuation)
    }

    pub fn join_secret_society(&mut self, society_name: &str, society_type: &str, secret_password: &str) -> SecretMembership {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let soc_id = format!("soc:sim:{}", uuid::Uuid::new_v4().to_string());

        let soc = SecretSociety {
            id: soc_id.clone(),
            name: society_name.to_string(),
            society_type: society_type.to_string(),
            founder_id: player.id.clone(),
            secrecy_level: 95.0,
            member_count: 33,
        };
        self.secret_societies.push(soc);

        let mem = SecretMembership {
            society_id: soc_id,
            society_name: society_name.to_string(),
            rank: "INITIATE".to_string(),
            covert_reputation: 15.0,
            secret_password: secret_password.to_string(),
        };

        player.secret_memberships.push(mem.clone());

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "SECRET_SOCIETY_INITIATION".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Initiated into secret order '{}' ({}) with cipher password.", society_name, society_type),
            metadata: serde_json::json!({ "society": society_name, "type": society_type }),
            causality_parent_id: None,
        });

        mem
    }

    pub fn perform_covert_ritual(&mut self, society_id: &str, ritual_name: &str) -> f32 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut rep = 25.0;

        if let Some(mem) = player.secret_memberships.iter_mut().find(|m| m.society_id == society_id) {
            mem.covert_reputation = (mem.covert_reputation + 20.0).min(100.0);
            rep = mem.covert_reputation;
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "COVERT_RITUAL".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Performed secret occult/esoteric ritual '{}'. Covert reputation boosted.", ritual_name),
            metadata: serde_json::json!({ "ritual": ritual_name, "reputation": rep }),
            causality_parent_id: None,
        });

        rep
    }

    pub fn launch_covert_operation(&mut self, society_id: &str, operation_name: &str, target_id: &str) -> CovertOperation {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let op = CovertOperation {
            id: uuid::Uuid::new_v4().to_string(),
            society_id: society_id.to_string(),
            operation_name: operation_name.to_string(),
            target_entity_id: target_id.to_string(),
            success_rate: 0.85,
            status: "SUCCESSFUL".to_string(),
        };

        self.covert_operations.push(op.clone());

        self.events.push(EventRecord {
            id: op.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "COVERT_OPERATION".to_string(),
            actor_id: player.id.clone(),
            target_id: Some(target_id.to_string()),
            summary: format!("Executed clandestine intelligence operation '{}' targeting {}.", operation_name, target_id),
            metadata: serde_json::json!({ "operation": operation_name, "target": target_id }),
            causality_parent_id: None,
        });

        op
    }

    pub fn advance_society_rank(&mut self, society_id: &str) -> String {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut new_rank = "ADEPT".to_string();

        if let Some(mem) = player.secret_memberships.iter_mut().find(|m| m.society_id == society_id) {
            new_rank = match mem.rank.as_str() {
                "INITIATE" => "ADEPT".to_string(),
                "ADEPT" => "MASTER".to_string(),
                _ => "GRAND_MASTER".to_string(),
            };
            mem.rank = new_rank.clone();
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "SOCIETY_RANK_ADVANCEMENT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Elevated rank to {} within secret society.", new_rank),
            metadata: serde_json::json!({ "rank": new_rank }),
            causality_parent_id: None,
        });

        new_rank
    }

    pub fn simulate_weather_turn(&mut self) -> WeatherEvent {
        let current_year = self.time.year;
        let month = self.time.month;

        let (season, condition, temp) = match month {
            3 | 4 | 5 => ("SPRING", "SUNNY", 14.5),
            6 | 7 | 8 => ("SUMMER", "HEATWAVE", 26.0),
            9 | 10 | 11 => ("AUTUMN", "RAIN", 11.0),
            _ => ("WINTER", "SNOW", 2.0),
        };

        let weather = WeatherEvent {
            id: uuid::Uuid::new_v4().to_string(),
            season: season.to_string(),
            condition: condition.to_string(),
            temperature_celsius: temp,
            air_quality_index: 42,
            year: current_year,
        };

        self.weather_events.push(weather.clone());

        self.events.push(EventRecord {
            id: weather.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "WEATHER_CHANGE".to_string(),
            actor_id: "system:environment".to_string(),
            target_id: None,
            summary: format!("Seasonal weather shift to {} ({}, {:.1}°C).", season, condition, temp),
            metadata: serde_json::json!({ "season": season, "condition": condition, "temperature": temp }),
            causality_parent_id: None,
        });

        weather
    }

    pub fn trigger_natural_disaster(&mut self, disaster_type: &str, severity: f32) -> NaturalDisaster {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let disaster = NaturalDisaster {
            id: uuid::Uuid::new_v4().to_string(),
            disaster_type: disaster_type.to_string(),
            severity,
            city_id: player.location_id.clone(),
            damage_cost: (severity as f64) * 50000.0,
            is_active: true,
            year: current_year,
        };

        self.active_disasters.push(disaster.clone());
        player.health.stress = (player.health.stress + 30.0).min(100.0);

        let summary = format!("NATURAL DISASTER EMERGENCY: Severe {} hit {} (Damage: £{:.2}).", disaster_type, player.location_id, disaster.damage_cost);

        self.events.push(EventRecord {
            id: disaster.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "NATURAL_DISASTER".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "type": disaster_type, "severity": severity, "damage": disaster.damage_cost }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Emergency Alert: {} Natural Disaster", disaster_type),
            body: summary,
            category: "ENVIRONMENT".to_string(),
            source_event_id: disaster.id.clone(),
        });

        disaster
    }

    pub fn rebuild_infrastructure(&mut self, disaster_id: &str, relief_funding: f64) -> Result<f64, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < relief_funding {
            return Err("Insufficient cash funds for disaster infrastructure rebuilding.".to_string());
        }

        player.finances.cash -= relief_funding;

        if let Some(d) = self.active_disasters.iter_mut().find(|dis| dis.id == disaster_id) {
            d.is_active = false;
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "REBUILD_INFRASTRUCTURE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Allocated £{:.2} relief funding to rebuild disaster damaged infrastructure.", relief_funding),
            metadata: serde_json::json!({ "relief_funding": relief_funding }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: "Infrastructure Rebuilding Underway".to_string(),
            body: format!("Relief funding (£{:.2}) dispatched to repair regional disaster damage.", relief_funding),
            category: "ENVIRONMENT".to_string(),
            source_event_id: disaster_id.to_string(),
        });

        Ok(relief_funding)
    }

    pub fn evaluate_environmental_impact(&self) -> EnvironmentalRating {
        self.environmental_rating.clone()
    }

    pub fn create_social_media_account(&mut self, platform: &str, handle: &str) -> SocialMediaAccount {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let acc = SocialMediaAccount {
            id: uuid::Uuid::new_v4().to_string(),
            platform: platform.to_string(),
            handle: handle.to_string(),
            follower_count: 120,
            subscriber_count: 50,
            engagement_rate: 4.5,
            influencer_tier: "NANO".to_string(),
        };

        player.social_media_accounts.push(acc.clone());

        self.events.push(EventRecord {
            id: acc.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "CREATED_SOCIAL_ACCOUNT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Created official digital profile @{} on {}.", handle, platform),
            metadata: serde_json::json!({ "platform": platform, "handle": handle }),
            causality_parent_id: None,
        });

        acc
    }

    pub fn post_digital_content(&mut self, platform: &str, caption: &str) -> DigitalPost {
        let current_year = self.time.year;
        let is_viral = self.rng.gen_range_f32(0.0, 1.0) > 0.70;
        let likes = if is_viral { 85000 } else { 450 };
        let shares = if is_viral { 12000 } else { 35 };

        let player = self.persons.get_mut("person:sim:player").unwrap();

        if let Some(acc) = player.social_media_accounts.iter_mut().find(|a| a.platform == platform) {
            acc.follower_count += if is_viral { 15000 } else { 120 };
            if acc.follower_count > 100000 {
                acc.influencer_tier = "MACRO".to_string();
            } else if acc.follower_count > 10000 {
                acc.influencer_tier = "MICRO".to_string();
            }
        }

        let post = DigitalPost {
            id: uuid::Uuid::new_v4().to_string(),
            platform: platform.to_string(),
            caption: caption.to_string(),
            likes,
            shares,
            impressions: likes * 3,
            is_viral,
            posted_year: current_year,
        };

        player.digital_posts.push(post.clone());

        let summary = if is_viral {
            format!("VIRAL CONTENT! Post on {} went viral with {} likes and {} shares!", platform, likes, shares)
        } else {
            format!("Published post on {}: '{}' ({} likes).", platform, caption, likes)
        };

        self.events.push(EventRecord {
            id: post.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "DIGITAL_POST".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "platform": platform, "likes": likes, "is_viral": is_viral }),
            causality_parent_id: None,
        });

        if is_viral {
            self.world_news.push(WorldNewsItem {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: self.time.formatted(),
                headline: format!("Viral Digital Trend on {}", platform),
                body: summary,
                category: "DIGITAL".to_string(),
                source_event_id: post.id.clone(),
            });
        }

        post
    }

    pub fn accept_brand_sponsorship(&mut self, platform: &str, fee: f64) -> Result<f64, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let acc_opt = player.social_media_accounts.iter().find(|a| a.platform == platform);

        if acc_opt.is_none() {
            return Err("No social media account found on specified platform for brand sponsorship.".to_string());
        }

        player.finances.cash += fee;

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "BRAND_SPONSORSHIP".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Executed commercial brand sponsorship deal on {} earning £{:.2}.", platform, fee),
            metadata: serde_json::json!({ "platform": platform, "fee": fee }),
            causality_parent_id: None,
        });

        Ok(fee)
    }

    pub fn handle_cyber_attack(&mut self, attack_type: &str) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.fame.public_reputation = (player.fame.public_reputation - 10.0).max(0.0);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "CYBER_ATTACK".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Targeted by online {} incident. Digital reputation impacted.", attack_type),
            metadata: serde_json::json!({ "type": attack_type }),
            causality_parent_id: None,
        });
    }

    pub fn diagnose_condition(&mut self, condition_name: &str, severity: &str) -> MedicalRecord {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let rec = MedicalRecord {
            id: uuid::Uuid::new_v4().to_string(),
            condition_name: condition_name.to_string(),
            severity: severity.to_string(),
            diagnosed_year: current_year,
            is_chronic: severity == "SEVERE" || severity == "CRITICAL",
            is_cured: false,
        };

        player.medical_history.push(rec.clone());
        player.health.conditions.push(condition_name.to_string());
        player.health.fitness = (player.health.fitness - 20.0).max(10.0);

        self.events.push(EventRecord {
            id: rec.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "MEDICAL_DIAGNOSIS".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Diagnosed with medical condition '{}' ({}) by clinical physician.", condition_name, severity),
            metadata: serde_json::json!({ "condition": condition_name, "severity": severity }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Healthcare Digest: Diagnosis for {}", condition_name),
            body: format!("Patient diagnosed with {} medical condition in hospital clinic.", condition_name),
            category: "HEALTHCARE".to_string(),
            source_event_id: rec.id.clone(),
        });

        rec
    }

    pub fn undergo_surgery(&mut self, procedure_name: &str, cost: f64) -> Result<SurgicalProcedure, String> {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < cost {
            return Err("Insufficient cash funds to pay surgical procedure fees.".to_string());
        }

        player.finances.cash -= cost;
        player.health.fitness = (player.health.fitness + 35.0).min(100.0);

        for cond in player.medical_history.iter_mut() {
            cond.is_cured = true;
        }
        player.health.conditions.clear();

        let surg = SurgicalProcedure {
            id: uuid::Uuid::new_v4().to_string(),
            procedure_name: procedure_name.to_string(),
            hospital_name: "Queen Elizabeth University Hospital".to_string(),
            success_rate: 0.96,
            cost,
            performed_year: current_year,
        };

        player.surgical_history.push(surg.clone());

        self.events.push(EventRecord {
            id: surg.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "SURGICAL_PROCEDURE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Underwent successful surgical operation '{}' (£{:.2}). Health restored.", procedure_name, cost),
            metadata: serde_json::json!({ "procedure": procedure_name, "cost": cost }),
            causality_parent_id: None,
        });

        Ok(surg)
    }

    pub fn draft_will_and_testament(&mut self, beneficiaries: Vec<String>, summary: &str) -> WillAndTestament {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let will = WillAndTestament {
            id: uuid::Uuid::new_v4().to_string(),
            beneficiary_ids: beneficiaries,
            estate_distribution_summary: summary.to_string(),
            executor_person_id: "person:sim:mum".to_string(),
        };

        player.will_and_testament = Some(will.clone());

        self.events.push(EventRecord {
            id: will.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "DRAFTED_WILL".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Drafted legally binding Will and Testament: {}.", summary),
            metadata: serde_json::json!({ "summary": summary }),
            causality_parent_id: None,
        });

        will
    }

    pub fn evaluate_epidemic_exposure(&mut self, virus_name: &str) -> bool {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.health.stress = (player.health.stress + 15.0).min(100.0);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "EPIDEMIC_EXPOSURE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Exposed to public epidemic outbreak of virus '{}'. Enforced home quarantine.", virus_name),
            metadata: serde_json::json!({ "virus": virus_name }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Public Health Advisory: {} Epidemic", virus_name),
            body: format!("Health authorities issued quarantine guidelines due to {} outbreak.", virus_name),
            category: "HEALTHCARE".to_string(),
            source_event_id: "epidemic-outbreak".to_string(),
        });

        true
    }

    pub fn enlist_military(&mut self, branch: &str) -> MilitaryRecord {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let rec = MilitaryRecord {
            id: uuid::Uuid::new_v4().to_string(),
            branch: branch.to_string(),
            rank: "PRIVATE".to_string(),
            years_served: 1,
            combat_deployments_count: 0,
            medals: Vec::new(),
            is_active_duty: true,
            is_veteran: false,
            monthly_pension: 0.0,
        };

        player.military_record = Some(rec.clone());
        player.employment.job_title = Some(format!("Armed Forces Enlisted Soldier ({})", branch));
        player.employment.monthly_salary = 2400.0;

        self.events.push(EventRecord {
            id: rec.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "MILITARY_ENLISTMENT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Enlisted in Armed Forces branch '{}' as Private.", branch),
            metadata: serde_json::json!({ "branch": branch, "rank": "PRIVATE" }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Armed Forces Enlistment: {}", branch),
            body: format!("Recruit enlisted into active service in the {} branch.", branch),
            category: "MILITARY".to_string(),
            source_event_id: rec.id.clone(),
        });

        rec
    }

    pub fn promote_military_rank(&mut self) -> String {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut new_rank = "SERGEANT".to_string();

        if let Some(ref mut mil) = player.military_record {
            new_rank = match mil.rank.as_str() {
                "PRIVATE" => "SERGEANT".to_string(),
                "SERGEANT" => "LIEUTENANT".to_string(),
                "LIEUTENANT" => "CAPTAIN".to_string(),
                _ => "GENERAL".to_string(),
            };
            mil.rank = new_rank.clone();
            mil.years_served += 2;
            player.employment.monthly_salary += 800.0;
            player.employment.job_title = Some(format!("Military Officer: {} ({})", new_rank, mil.branch));
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "MILITARY_PROMOTION".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Promoted to military rank of {}.", new_rank),
            metadata: serde_json::json!({ "rank": new_rank }),
            causality_parent_id: None,
        });

        new_rank
    }

    pub fn deploy_to_combat(&mut self, conflict_name: &str) -> u32 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut count = 1;

        if let Some(ref mut mil) = player.military_record {
            mil.combat_deployments_count += 1;
            count = mil.combat_deployments_count;
            mil.medals.push(format!("Combat Commendation: Operation {}", conflict_name));
        }

        player.health.stress = (player.health.stress + 20.0).min(100.0);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "COMBAT_DEPLOYMENT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Deployed to active combat zone in Operation '{}'. Awarded Combat Commendation Medal.", conflict_name),
            metadata: serde_json::json!({ "conflict": conflict_name, "deployments": count }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Military Combat Mission: {}", conflict_name),
            body: format!("Armed forces battalion completed active duty mission in operation {}.", conflict_name),
            category: "MILITARY".to_string(),
            source_event_id: "combat-deployment-news".to_string(),
        });

        count
    }

    pub fn discharge_military_veteran(&mut self) -> f64 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut pension = 1200.0;

        if let Some(ref mut mil) = player.military_record {
            mil.is_active_duty = false;
            mil.is_veteran = true;
            mil.monthly_pension = 1200.0;
            pension = mil.monthly_pension;
        }

        player.employment.job_title = Some("Honorable Military Veteran".to_string());
        player.employment.monthly_salary = pension;

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "MILITARY_DISCHARGE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Received Honorable Discharge from armed forces. Granted £{:.2}/mo veteran pension.", pension),
            metadata: serde_json::json!({ "pension": pension }),
            causality_parent_id: None,
        });

        pension
    }

    pub fn issue_passport(&mut self, country_id: &str) -> Passport {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let passport = Passport {
            id: uuid::Uuid::new_v4().to_string(),
            country_id: country_id.to_string(),
            issued_year: current_year,
            expiry_year: current_year + 10,
            is_valid: true,
        };

        player.passports.push(passport.clone());

        self.events.push(EventRecord {
            id: passport.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "PASSPORT_ISSUED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Issued official national passport for {}.", country_id),
            metadata: serde_json::json!({ "country_id": country_id }),
            causality_parent_id: None,
        });

        passport
    }

    pub fn apply_visa(&mut self, target_country_id: &str, visa_type: &str) -> Visa {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let visa = Visa {
            id: uuid::Uuid::new_v4().to_string(),
            target_country_id: target_country_id.to_string(),
            visa_type: visa_type.to_string(),
            expiry_year: current_year + 3,
        };

        player.visas.push(visa.clone());

        self.events.push(EventRecord {
            id: visa.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "VISA_APPROVED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Approved {} visa for {}.", visa_type, target_country_id),
            metadata: serde_json::json!({ "target_country": target_country_id, "type": visa_type }),
            causality_parent_id: None,
        });

        visa
    }

    pub fn book_and_take_flight(&mut self, dest_city_id: &str, dest_country_id: &str, cost: f64) -> Result<TravelRecord, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < cost {
            return Err("Insufficient cash funds for international flight ticket.".to_string());
        }

        player.finances.cash -= cost;
        player.location_id = dest_city_id.to_string();

        let travel = TravelRecord {
            id: uuid::Uuid::new_v4().to_string(),
            destination_city_id: dest_city_id.to_string(),
            destination_country_id: dest_country_id.to_string(),
            travel_date: self.time.formatted(),
            purpose: "Leisure & Travel".to_string(),
            cost,
        };

        player.travel_history.push(travel.clone());

        self.events.push(EventRecord {
            id: travel.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "INTERNATIONAL_FLIGHT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Flew to {} ({}) for £{:.2}.", dest_city_id, dest_country_id, cost),
            metadata: serde_json::json!({ "city": dest_city_id, "country": dest_country_id, "cost": cost }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("International Travel Digest: Destination {}", dest_city_id),
            body: format!("Passenger arrived in {} on international flight.", dest_city_id),
            category: "TRAVEL".to_string(),
            source_event_id: travel.id.clone(),
        });

        Ok(travel)
    }

    pub fn relocate_residence(&mut self, dest_city_id: &str, dest_country_id: &str) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.location_id = dest_city_id.to_string();
        player.identity.current_location_id = dest_city_id.to_string();
        if !player.identity.nationalities.contains(&dest_country_id.to_string()) {
            player.identity.nationalities.push(dest_country_id.to_string());
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "CROSS_BORDER_RELOCATION".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Relocated permanent residence to {} ({}).", dest_city_id, dest_country_id),
            metadata: serde_json::json!({ "city": dest_city_id, "country": dest_country_id }),
            causality_parent_id: None,
        });
    }

    pub fn convert_faith(&mut self, faith_id: &str, faith_name: &str) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.belief.faith_id = faith_id.to_string();
        player.belief.faith_name = faith_name.to_string();
        player.belief.devotion_level = 35.0;

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "FAITH_CONVERSION".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Converted spiritual & philosophical beliefs to {}.", faith_name),
            metadata: serde_json::json!({ "faith_id": faith_id, "faith_name": faith_name }),
            causality_parent_id: None,
        });
    }

    pub fn attend_worship_service(&mut self) -> f32 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.belief.devotion_level = (player.belief.devotion_level + 8.0).min(100.0);
        player.health.stress = (player.health.stress - 15.0).max(0.0);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "ATTENDED_WORSHIP".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Attended worship service for {}. Devotion increased (+8.0) and stress reduced (-15.0).", player.belief.faith_name),
            metadata: serde_json::json!({ "faith": player.belief.faith_name }),
            causality_parent_id: None,
        });

        player.belief.devotion_level
    }

    pub fn donate_tithe(&mut self, amount: f64) -> Result<f64, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < amount {
            return Err("Insufficient funds for tithe donation.".to_string());
        }

        player.finances.cash -= amount;
        player.belief.tithes_donated += amount;

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "DONATED_TITHE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Donated £{:.2} tithe contribution to {}.", amount, player.belief.faith_name),
            metadata: serde_json::json!({ "amount": amount }),
            causality_parent_id: None,
        });

        Ok(player.belief.tithes_donated)
    }

    pub fn found_faith_movement(&mut self, name: &str, doctrine: &str) -> FaithMovement {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let movement = FaithMovement {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            founder_person_id: player.id.clone(),
            doctrine_summary: doctrine.to_string(),
            congregation_size: 45,
            treasury: 500.0,
        };

        player.belief.faith_id = "CUSTOM".to_string();
        player.belief.faith_name = name.to_string();
        player.belief.spiritual_rank = "LEADER".to_string();
        player.founded_movements.push(movement.clone());

        self.events.push(EventRecord {
            id: movement.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "FOUNDED_FAITH_MOVEMENT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Founded spiritual/philosophical movement '{}' as Spiritual Leader.", name),
            metadata: serde_json::json!({ "name": name, "doctrine": doctrine }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("New Spiritual Movement Founded: {}", name),
            body: format!("Founder registered new congregation '{}' based on doctrine: {}.", name, doctrine),
            category: "RELIGION".to_string(),
            source_event_id: movement.id.clone(),
        });

        movement
    }

    pub fn enroll_university_program(&mut self, degree_type: &str, field: &str, university_name: &str) -> AcademicDegree {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let degree = AcademicDegree {
            degree_type: degree_type.to_string(),
            field_of_study: field.to_string(),
            university_name: university_name.to_string(),
            graduation_year: current_year + 3,
            gpa: 3.85,
        };

        player.academic_degrees.push(degree.clone());
        player.education.degree_program = Some(format!("{} in {}", degree_type, field));

        self.events.push(EventRecord {
            id: degree.graduation_year.to_string(),
            timestamp: self.time.formatted(),
            event_type: "UNIVERSITY_ENROLLMENT".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Enrolled in {} program in {} at {}.", degree_type, field, university_name),
            metadata: serde_json::json!({ "degree": degree_type, "field": field, "university": university_name }),
            causality_parent_id: None,
        });

        degree
    }

    pub fn conduct_scientific_research(&mut self, title: &str, field: &str, grant: f64) -> ResearchProject {
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let project = ResearchProject {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            field_of_study: field.to_string(),
            funding_grant: grant,
            progress_pct: 100.0,
            lead_researcher_id: player.id.clone(),
            status: "IN_PROGRESS".to_string(),
            citation_count: 0,
        };

        player.research_projects.push(project.clone());

        self.events.push(EventRecord {
            id: project.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "RESEARCH_PROJECT_STARTED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Launched scientific research project '{}' with £{:.2} grant funding.", title, grant),
            metadata: serde_json::json!({ "title": title, "field": field, "grant": grant }),
            causality_parent_id: None,
        });

        project
    }

    pub fn publish_paper(&mut self, project_id: &str) -> u32 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut citations = 0;

        if let Some(proj) = player.research_projects.iter_mut().find(|p| p.id == project_id) {
            proj.status = "PUBLISHED".to_string();
            proj.citation_count = 142;
            citations = proj.citation_count;
        }

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "PAPER_PUBLISHED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Published peer-reviewed scientific paper for project. Earned {} citations.", citations),
            metadata: serde_json::json!({ "citations": citations }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: "Scientific Research Discovery Published".to_string(),
            body: format!("Lead researcher published breakthrough academic paper with {} citations.", citations),
            category: "SCIENCE".to_string(),
            source_event_id: project_id.to_string(),
        });

        citations
    }

    pub fn file_patent(&mut self, title: &str, field: &str, valuation: f64) -> Patent {
        let current_year = self.time.year;
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let patent = Patent {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            field: field.to_string(),
            inventor_person_id: player.id.clone(),
            filed_year: current_year,
            estimated_valuation: valuation,
        };

        player.patents.push(patent.clone());

        self.events.push(EventRecord {
            id: patent.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "PATENT_FILED".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Filed patent '{}' in {} with an estimated valuation of £{:.2}.", title, field, valuation),
            metadata: serde_json::json!({ "title": title, "field": field, "valuation": valuation }),
            causality_parent_id: None,
        });

        patent
    }

    pub fn commit_crime(&mut self, crime_type: &str, stolen_value: f64) -> bool {
        let roll = self.rng.gen_range_f32(0.0, 1.0);
        let player = self.persons.get_mut("person:sim:player").unwrap();

        let success = roll > 0.40;
        let rec = CriminalRecord {
            id: uuid::Uuid::new_v4().to_string(),
            crime_type: crime_type.to_string(),
            severity: 0.7,
            stolen_value,
            is_unsolved: !success,
        };

        player.criminal_records.push(rec.clone());

        if success {
            player.finances.cash += stolen_value;
            self.events.push(EventRecord {
                id: rec.id.clone(),
                timestamp: self.time.formatted(),
                event_type: "CRIME_COMMITTED".to_string(),
                actor_id: player.id.clone(),
                target_id: None,
                summary: format!("Executed {} crime successfully. Gained £{:.2} loot.", crime_type, stolen_value),
                metadata: serde_json::json!({ "crime": crime_type, "stolen": stolen_value }),
                causality_parent_id: None,
            });
            true
        } else {
            player.legal_status = LegalStatus::UnderInvestigation;
            self.events.push(EventRecord {
                id: rec.id.clone(),
                timestamp: self.time.formatted(),
                event_type: "CRIME_FAILED".to_string(),
                actor_id: player.id.clone(),
                target_id: None,
                summary: format!("Attempted {} crime failed. Police launched formal investigation.", crime_type),
                metadata: serde_json::json!({ "crime": crime_type }),
                causality_parent_id: None,
            });
            false
        }
    }

    pub fn conduct_court_trial(&mut self, lawyer_skill: f32) -> bool {
        let roll = self.rng.gen_range_f32(0.0, 1.0);
        let guilty = roll > (lawyer_skill / 100.0);

        let player = self.persons.get_mut("person:sim:player").unwrap();
        if guilty {
            player.legal_status = LegalStatus::Imprisoned;
            player.prison_sentence = Some(PrisonSentence {
                crime_type: "BURGLARY".to_string(),
                months_total: 12,
                months_served: 0,
                facility_name: "HMP Barlinnie".to_string(),
            });

            self.events.push(EventRecord {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: self.time.formatted(),
                event_type: "TRIAL_CONVICTION".to_string(),
                actor_id: player.id.clone(),
                target_id: None,
                summary: "GUILTY VERDICT. Convicted in court and sentenced to 12 months imprisonment.".to_string(),
                metadata: serde_json::json!({ "sentence_months": 12 }),
                causality_parent_id: None,
            });

            self.world_news.push(WorldNewsItem {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: self.time.formatted(),
                headline: "Court Trial Verdict Delivered".to_string(),
                body: "Defendant convicted in judicial court and handed 12-month custodial sentence.".to_string(),
                category: "CRIME".to_string(),
                source_event_id: "trial-verdict".to_string(),
            });
            false
        } else {
            player.legal_status = LegalStatus::Clean;
            self.events.push(EventRecord {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: self.time.formatted(),
                event_type: "TRIAL_ACQUITTAL".to_string(),
                actor_id: player.id.clone(),
                target_id: None,
                summary: "NOT GUILTY VERDICT! Acquitted of all charges in court.".to_string(),
                metadata: serde_json::json!({}),
                causality_parent_id: None,
            });
            true
        }
    }

    pub fn serve_prison_turn(&mut self) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if let Some(ref mut sentence) = player.prison_sentence {
            sentence.months_served += 1;
            player.health.stress = (player.health.stress + 5.0).min(100.0);
            if sentence.months_served >= sentence.months_total {
                player.legal_status = LegalStatus::Parole;
                player.prison_sentence = None;

                self.events.push(EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "PRISON_RELEASE".to_string(),
                    actor_id: player.id.clone(),
                    target_id: None,
                    summary: "Served custodial sentence. Released from prison on parole status.".to_string(),
                    metadata: serde_json::json!({}),
                    causality_parent_id: None,
                });
            }
        }
    }

    pub fn produce_creative_release(&mut self, title: &str, medium: &str) -> CreativeRelease {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let quality = 60.0 + self.rng.gen_range_f32(5.0, 30.0);

        let release = CreativeRelease {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            medium: medium.to_string(),
            creator_person_id: player.id.clone(),
            quality_rating: quality,
            sales_volume: 500,
            chart_position: 85,
        };

        player.creative_releases.push(release.clone());

        self.events.push(EventRecord {
            id: release.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "PRODUCED_CREATIVE_RELEASE".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Released new {} '{}' with a quality score of {:.1}/100.", medium, title, quality),
            metadata: serde_json::json!({ "title": title, "medium": medium, "quality": quality }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("New {} Release: {}", medium, title),
            body: format!("Artist launched creative project '{}' to positive reception.", title),
            category: "ENTERTAINMENT".to_string(),
            source_event_id: release.id.clone(),
        });

        release
    }

    pub fn promote_release(&mut self, release_id: &str) -> f64 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let mut royalties = 0.0;

        if let Some(rel) = player.creative_releases.iter_mut().find(|r| r.id == release_id) {
            rel.sales_volume += 12000;
            rel.chart_position = (rel.chart_position.saturating_sub(15)).max(1);
            royalties = (rel.sales_volume as f64) * 0.15;
            player.finances.cash += royalties;
            player.fame.fame_level = (player.fame.fame_level + 15.0).min(100.0);
            player.fame.fanbase_count += 8500;
        }

        royalties
    }

    pub fn handle_media_scandal(&mut self, scandal_description: &str) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.fame.public_reputation = (player.fame.public_reputation - 25.0).max(0.0);

        let summary = format!("MEDIA SCANDAL: Press published report: {}", scandal_description);

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "MEDIA_SCANDAL".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({ "scandal": scandal_description }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: "Entertainment Media Scandal Break".to_string(),
            body: summary,
            category: "ENTERTAINMENT".to_string(),
            source_event_id: "scandal-news".to_string(),
        });
    }

    pub fn join_political_party(&mut self, party_id: &str) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.political_party_id = Some(party_id.to_string());

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "JOINED_POLITICAL_PARTY".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Officially joined political party '{}'.", party_id),
            metadata: serde_json::json!({ "party_id": party_id }),
            causality_parent_id: None,
        });
    }

    pub fn launch_political_campaign(&mut self, office_id: &str, office_title: &str, initial_funds: f64) -> Result<PoliticalCampaign, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < initial_funds {
            return Err("Insufficient funds for campaign launch.".to_string());
        }

        player.finances.cash -= initial_funds;

        let campaign = PoliticalCampaign {
            id: uuid::Uuid::new_v4().to_string(),
            office_id: office_id.to_string(),
            office_title: office_title.to_string(),
            candidate_person_id: player.id.clone(),
            party_id: player.political_party_id.clone(),
            campaign_funds: initial_funds,
            polling_pct: 20.0,
        };

        player.active_campaign = Some(campaign.clone());

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "LAUNCHED_CAMPAIGN".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Launched election campaign for '{}' with £{:.2} seed funds.", office_title, initial_funds),
            metadata: serde_json::json!({ "office": office_title }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Election Campaign Announced for {}", office_title),
            body: format!("Candidate launched their official campaign for {}.", office_title),
            category: "POLITICS".to_string(),
            source_event_id: campaign.id.clone(),
        });

        Ok(campaign)
    }

    pub fn hold_campaign_rally(&mut self) -> f32 {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if let Some(ref mut campaign) = player.active_campaign {
            if campaign.campaign_funds >= 100.0 {
                campaign.campaign_funds -= 100.0;
                let boost = self.rng.gen_range_f32(4.0, 12.0);
                campaign.polling_pct = (campaign.polling_pct + boost).min(95.0);
                campaign.polling_pct
            } else {
                campaign.polling_pct
            }
        } else {
            0.0
        }
    }

    pub fn simulate_election(&mut self) -> bool {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if let Some(campaign) = player.active_campaign.take() {
            let win_probability = campaign.polling_pct / 100.0;
            let roll = self.rng.gen_range_f32(0.0, 1.0);
            let won = roll < win_probability;

            if won {
                player.political_office_title = Some(campaign.office_title.clone());
                player.employment.job_title = Some(format!("Public Official: {}", campaign.office_title));
                player.employment.monthly_salary = 5500.0;

                let summary = format!("ELECTION VICTORY! Won election for '{}' with {:.1}% of the vote!", campaign.office_title, campaign.polling_pct);
                self.events.push(EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "ELECTION_WON".to_string(),
                    actor_id: player.id.clone(),
                    target_id: None,
                    summary: summary.clone(),
                    metadata: serde_json::json!({ "office": campaign.office_title, "polling": campaign.polling_pct }),
                    causality_parent_id: None,
                });

                self.world_news.push(WorldNewsItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    headline: format!("Election Result: {}", campaign.office_title),
                    body: summary,
                    category: "POLITICS".to_string(),
                    source_event_id: campaign.id,
                });
                true
            } else {
                let summary = format!("Lost election for '{}' after polling {:.1}%.", campaign.office_title, campaign.polling_pct);
                self.events.push(EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "ELECTION_LOST".to_string(),
                    actor_id: player.id.clone(),
                    target_id: None,
                    summary,
                    metadata: serde_json::json!({ "office": campaign.office_title }),
                    causality_parent_id: None,
                });
                false
            }
        } else {
            false
        }
    }

    pub fn set_economic_cycle(&mut self, cycle: &str) {
        self.macro_economy.economic_cycle = cycle.to_string();
        match cycle {
            "BOOM" => {
                self.macro_economy.inflation_rate = 0.040;
                self.macro_economy.interest_rate = 0.060;
            }
            "RECESSION" => {
                self.macro_economy.inflation_rate = 0.010;
                self.macro_economy.interest_rate = 0.020;
            }
            "RECOVERY" => {
                self.macro_economy.inflation_rate = 0.020;
                self.macro_economy.interest_rate = 0.035;
            }
            _ => {
                self.macro_economy.inflation_rate = 0.025;
                self.macro_economy.interest_rate = 0.045;
            }
        }
    }

    pub fn found_business(&mut self, name: &str, industry: &str, initial_capital: f64) -> Result<BusinessEntity, String> {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        if player.finances.cash < initial_capital {
            return Err("Insufficient cash funds to cover business startup capital.".to_string());
        }

        player.finances.cash -= initial_capital;

        let biz_id = format!("biz:sim:{}", uuid::Uuid::new_v4().to_string());
        let biz = BusinessEntity {
            id: biz_id.clone(),
            name: name.to_string(),
            industry: industry.to_string(),
            owner_person_id: player.id.clone(),
            valuation: initial_capital * 1.5,
            monthly_revenue: initial_capital * 0.20,
            monthly_expenses: initial_capital * 0.12,
            cash_reserve: initial_capital,
            debt: 0.0,
            employee_count: 2,
            equity_owned_pct: 100.0,
        };

        player.owned_business_ids.push(biz_id.clone());
        self.businesses.insert(biz_id, biz.clone());

        self.resolver.register_entity(CanonicalEntity {
            id: biz.id.clone(),
            name: name.to_string(),
            entity_type: EntityType::Company,
            aliases: Vec::new(),
            location_id: Some(player.location_id.clone()),
            parent_org_id: None,
            fame_score: 30.0,
            namespace: EntityNamespace::Sim,
        });

        self.events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "FOUNDED_BUSINESS".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: format!("Founded new venture '{}' in the {} sector with £{:.2} capital.", name, industry, initial_capital),
            metadata: serde_json::json!({ "name": name, "industry": industry, "capital": initial_capital }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("New Venture Launched: {}", name),
            body: format!("Local entrepreneur registered {} in the {} market.", name, industry),
            category: "ECONOMY".to_string(),
            source_event_id: biz.id.clone(),
        });

        Ok(biz)
    }

    pub fn operate_business_turn(&mut self, business_id: &str) -> f64 {
        let cycle = self.macro_economy.economic_cycle.clone();
        let multiplier = match cycle.as_str() {
            "BOOM" => 1.3,
            "RECESSION" => 0.7,
            "RECOVERY" => 1.1,
            _ => 1.0,
        };

        let biz = self.businesses.get_mut(business_id).unwrap();
        biz.monthly_revenue *= multiplier;
        let profit = biz.monthly_revenue - biz.monthly_expenses;

        if profit > 0.0 {
            let dividend = profit * 0.4;
            biz.cash_reserve += profit - dividend;
            biz.valuation += profit * 2.0;

            let player = self.persons.get_mut(&biz.owner_person_id).unwrap();
            player.finances.cash += dividend;
            dividend
        } else {
            biz.cash_reserve += profit;
            0.0
        }
    }

    pub fn handle_business_bankruptcy(&mut self, business_id: &str) -> bool {
        let biz = match self.businesses.get(business_id) {
            Some(b) => b,
            None => return false,
        };

        if biz.cash_reserve < 0.0 && biz.debt > biz.valuation {
            let owner_id = biz.owner_person_id.clone();
            let name = biz.name.clone();

            self.businesses.remove(business_id);
            if let Some(player) = self.persons.get_mut(&owner_id) {
                player.owned_business_ids.retain(|id| id != business_id);
            }

            self.events.push(EventRecord {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: self.time.formatted(),
                event_type: "BUSINESS_BANKRUPT".to_string(),
                actor_id: owner_id,
                target_id: None,
                summary: format!("Venture '{}' filed for bankruptcy and was liquidated.", name),
                metadata: serde_json::json!({ "name": name }),
                causality_parent_id: None,
            });
            true
        } else {
            false
        }
    }

    pub fn simulate_football_match(&mut self, home_name: &str, away_name: &str) -> FootballMatch {
        let home_goals = self.rng.gen_range_u32(0, 4);
        let away_goals = self.rng.gen_range_u32(0, 3);

        let player = self.persons.get_mut("person:sim:player").unwrap();
        let rating = (6.0 + self.rng.gen_range_f32(0.5, 3.5)).min(10.0);

        let match_record = FootballMatch {
            id: uuid::Uuid::new_v4().to_string(),
            match_date: self.time.formatted(),
            home_club_id: "club:real:celtic".to_string(),
            home_club_name: home_name.to_string(),
            away_club_id: "club:real:rangers".to_string(),
            away_club_name: away_name.to_string(),
            home_score: home_goals,
            away_score: away_goals,
            player_rating: rating,
        };

        let summary = format!(
            "Match Completed: {} {} - {} {}. Player Match Rating: {:.1}/10.",
            home_name, home_goals, away_goals, away_name, rating
        );

        self.events.push(EventRecord {
            id: match_record.id.clone(),
            timestamp: self.time.formatted(),
            event_type: "FOOTBALL_MATCH".to_string(),
            actor_id: player.id.clone(),
            target_id: None,
            summary: summary.clone(),
            metadata: serde_json::json!({
                "home": home_name,
                "away": away_name,
                "score": format!("{}-{}", home_goals, away_goals),
                "rating": rating
            }),
            causality_parent_id: None,
        });

        self.world_news.push(WorldNewsItem {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            headline: format!("Match Report: {} vs {}", home_name, away_name),
            body: summary,
            category: "FOOTBALL".to_string(),
            source_event_id: match_record.id.clone(),
        });

        match_record
    }

    pub fn generate_scout_report(&self, target_player_id: &str) -> FootballScoutReport {
        let player = self.persons.get(target_player_id).unwrap();
        let ca = (player.football_attributes.ball_control + player.football_attributes.pace) / 2.0;
        let pa = (ca + 15.0).min(99.0);

        FootballScoutReport {
            id: uuid::Uuid::new_v4().to_string(),
            target_player_id: target_player_id.to_string(),
            scout_id: "scout:sim:celtic".to_string(),
            current_ability: ca,
            potential_rating: pa,
            recommended_transfer_fee: (ca as f64) * 25000.0,
            notes: format!("Displays excellent natural technique ({:.0}) and composure under pressure.", ca),
        }
    }

    pub fn negotiate_football_contract(&mut self, club_name: &str, weekly_wage: f64, years: u32) -> FootballContract {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        let contract = FootballContract {
            club_id: format!("club:sim:{}", club_name.to_lowercase().replace(' ', "_")),
            club_name: club_name.to_string(),
            weekly_wage,
            years_remaining: years,
            release_clause: weekly_wage * 100.0,
            goal_bonus: weekly_wage * 0.1,
            agent_id: None,
        };

        player.football_role = FootballRole::Player;
        player.football_contract = Some(contract.clone());
        player.employment.monthly_salary = weekly_wage * 4.0;
        player.employment.job_title = Some(format!("Professional Footballer ({})", club_name));

        contract
    }

    pub fn transition_football_role(&mut self, new_role: FootballRole) {
        let player = self.persons.get_mut("person:sim:player").unwrap();
        player.football_role = new_role.clone();
        player.employment.job_title = Some(format!("Football {:?}", new_role));
    }

    pub fn tick_npc_simulation(&mut self) {
        let player_id = "person:sim:player".to_string();
        let timestamp = self.time.formatted();

        let mut npc_events = Vec::new();
        let mut news_items = Vec::new();

        for (id, npc) in self.persons.iter_mut() {
            if id == &player_id || !npc.is_alive {
                continue;
            }

            let current_hour = self.time.hour;
            if npc.schedule.current_activity != ActivityType::Socializing {
                if current_hour >= 9 && current_hour <= 17 {
                    npc.schedule.current_activity = ActivityType::Work;
                } else if current_hour >= 18 && current_hour <= 21 {
                    npc.schedule.current_activity = ActivityType::Socializing;
                } else {
                    npc.schedule.current_activity = ActivityType::Home;
                }
            }

            let event_roll = self.rng.gen_range_f32(0.0, 1.0);

            if event_roll < 0.40 && npc.employment.job_title.is_some() {
                npc.employment.job_performance = (npc.employment.job_performance + 5.0).min(100.0);
                if npc.employment.job_performance > 85.0 {
                    let old_title = npc.employment.job_title.clone().unwrap();
                    let new_title = if old_title.starts_with("Lead ") { old_title } else { format!("Lead {}", old_title) };
                    npc.employment.job_title = Some(new_title.clone());
                    npc.employment.monthly_salary += 500.0;

                    let summary = format!("{} received a career promotion to {}!", npc.identity.first_name, new_title);
                    npc_events.push(EventRecord {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: timestamp.clone(),
                        event_type: "NPC_PROMOTION".to_string(),
                        actor_id: id.clone(),
                        target_id: None,
                        summary: summary.clone(),
                        metadata: serde_json::json!({ "new_title": new_title }),
                        causality_parent_id: None,
                    });

                    news_items.push(WorldNewsItem {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: timestamp.clone(),
                        headline: format!("Promotions Announced at {}", npc.identity.last_name),
                        body: summary,
                        category: "CAREER".to_string(),
                        source_event_id: id.clone(),
                    });
                }
            }

            if npc.schedule.current_activity == ActivityType::Socializing {
                for secret in npc.secrets.iter_mut() {
                    if secret.is_secret && !secret.known_by_ids.contains(&player_id) {
                        if self.rng.gen_range_f32(0.0, 1.0) < 0.60 {
                            secret.known_by_ids.insert(player_id.clone());
                            npc_events.push(EventRecord {
                                id: uuid::Uuid::new_v4().to_string(),
                                timestamp: timestamp.clone(),
                                event_type: "SECRET_PROPAGATED".to_string(),
                                actor_id: id.clone(),
                                target_id: Some(player_id.clone()),
                                summary: format!("You overheard a secret: {}", secret.description),
                                metadata: serde_json::json!({ "topic": secret.topic_id }),
                                causality_parent_id: None,
                            });
                        }
                    }
                }
            }
        }

        self.events.extend(npc_events);
        self.world_news.extend(news_items);
    }

    pub fn generate_active_situations(&mut self) {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => p,
            None => return,
        };

        if !player.is_alive {
            self.active_situations = vec![LifeSituation {
                id: "sit_passed_away".to_string(),
                category: SituationCategory::Milestone,
                title: "Journey's End".to_string(),
                narrative: format!("{} has passed away. The timeline of this life has concluded.", player.identity.first_name),
                choices: vec![LifeSituationChoice {
                    id: "view_legacy".to_string(),
                    label: "Reflect on this life's biography and memories".to_string(),
                    consequence_hint: None,
                }],
                min_age: 0,
                max_age: None,
                location_id: Some(player.location_id.clone()),
                expires_in_days: None,
                generated_year: self.time.year,
                process_id: None,
            }];
            return;
        }

        let age = (self.time.year - player.identity.birth_year) as u32;
        let location_name = player.location_id.replace("city:real:", "").replace("city:sim:", "").replace('_', " ");
        let location_name_title = location_name.chars().enumerate().map(|(i, c)| if i == 0 || location_name.chars().nth(i-1) == Some(' ') { c.to_ascii_uppercase() } else { c }).collect::<String>();
        let country_id = player.identity.nationalities.first().cloned().unwrap_or_else(|| "country:real:united_kingdom".to_string());
        let currency_symbol = if country_id.contains("nigeria") { "₦" } else if country_id.contains("united_states") { "$" } else { "£" };

        let mut situations = Vec::new();

        // 1. Process Chains in progress
        for proc in &self.active_processes {
            if proc.is_active && (proc.current_step as usize) < proc.steps.len() {
                let step = &proc.steps[proc.current_step as usize];
                situations.push(LifeSituation {
                    id: format!("proc_{}_{}", proc.id, proc.current_step),
                    category: SituationCategory::Opportunity,
                    title: format!("{}: Step {}/{}", proc.title, proc.current_step + 1, proc.total_steps),
                    narrative: format!("{}\n\nCurrent phase: {}", proc.title, step.description),
                    choices: vec![
                        LifeSituationChoice {
                            id: "advance_process".to_string(),
                            label: format!("Complete: {}", step.title),
                            consequence_hint: Some("Moves this process toward its next stage".to_string()),
                        },
                        LifeSituationChoice {
                            id: "delay_process".to_string(),
                            label: "Take more time to prepare".to_string(),
                            consequence_hint: Some("Delays the process without forfeiting".to_string()),
                        },
                        LifeSituationChoice {
                            id: "cancel_process".to_string(),
                            label: "Withdraw and cancel".to_string(),
                            consequence_hint: Some("Cancels this application/pathway".to_string()),
                        },
                    ],
                    min_age: age,
                    max_age: None,
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: Some(30),
                    generated_year: self.time.year,
                    process_id: Some(proc.id.clone()),
                });
            }
        }

        // 2. Age-bracket situation generators
        if age <= 3 {
            // INFANCY / TODDLERHOOD (Age 0-3)
            let parent_first = player.parent_ids.first()
                .and_then(|pid| self.persons.get(pid))
                .map(|p| p.identity.first_name.clone())
                .unwrap_or_else(|| "Your mother".to_string());

            match age {
                0 => {
                    situations.push(LifeSituation {
                        id: format!("infancy_cradle_{}_{}", self.time.year, self.time.month),
                        category: SituationCategory::Routine,
                        title: "Early Days in the Family Home".to_string(),
                        narrative: format!(
                            "You lie in your cot in {}. {} gently rocks you to sleep while family chatter fills the next room. The sounds and sights of your early home surround you.",
                            location_name_title, parent_first
                        ),
                        choices: vec![
                            LifeSituationChoice {
                                id: "rest_peacefully".to_string(),
                                label: "Drift off to sleep peacefully".to_string(),
                                consequence_hint: Some("Nourishes health and emotional stability".to_string()),
                            },
                            LifeSituationChoice {
                                id: "reach_out".to_string(),
                                label: "Reach for your parent's hand and babble".to_string(),
                                consequence_hint: Some("Strengthens family closeness".to_string()),
                            },
                            LifeSituationChoice {
                                id: "observe_room".to_string(),
                                label: "Look around curiously at lights and shapes".to_string(),
                                consequence_hint: Some("Early sensory curiosity development".to_string()),
                            },
                        ],
                        min_age: 0,
                        max_age: Some(1),
                        location_id: Some(player.location_id.clone()),
                        expires_in_days: None,
                        generated_year: self.time.year,
                        process_id: None,
                    });
                }
                1 | 2 => {
                    situations.push(LifeSituation {
                        id: format!("toddler_first_steps_{}", self.time.year),
                        category: SituationCategory::Milestone,
                        title: "Exploring the Household".to_string(),
                        narrative: format!(
                            "You are on your feet, toddling across the living room carpet in {}. {} watches with an encouraging smile as you discover toys and household objects.",
                            location_name_title, parent_first
                        ),
                        choices: vec![
                            LifeSituationChoice {
                                id: "explore_household".to_string(),
                                label: "Explore every corner and climb onto a chair".to_string(),
                                consequence_hint: Some("Builds physical confidence (+athleticism)".to_string()),
                            },
                            LifeSituationChoice {
                                id: "play_with_blocks".to_string(),
                                label: "Sit quietly and stack colourful building blocks".to_string(),
                                consequence_hint: Some("Develops patience and focus (+cognition)".to_string()),
                            },
                            LifeSituationChoice {
                                id: "mimic_speech".to_string(),
                                label: "Try to repeat words your parents are saying".to_string(),
                                consequence_hint: Some("Early language development (+communication)".to_string()),
                            },
                        ],
                        min_age: 1,
                        max_age: Some(3),
                        location_id: Some(player.location_id.clone()),
                        expires_in_days: None,
                        generated_year: self.time.year,
                        process_id: None,
                    });
                }
                _ => {
                    situations.push(LifeSituation {
                        id: format!("nursery_curiosity_{}", self.time.year),
                        category: SituationCategory::Relationship,
                        title: "Curiosity and Play".to_string(),
                        narrative: format!(
                            "At age 3 in {}, your vocabulary has expanded rapidly. You ask questions about everything outside the window and love listening to bedtime stories.",
                            location_name_title
                        ),
                        choices: vec![
                            LifeSituationChoice {
                                id: "listen_to_stories".to_string(),
                                label: "Ask for another storybook reading".to_string(),
                                consequence_hint: Some("Sparks imagination and early reading (+reading)".to_string()),
                            },
                            LifeSituationChoice {
                                id: "play_outdoors".to_string(),
                                label: "Run around in the garden or courtyard".to_string(),
                                consequence_hint: Some("Builds stamina and joyful energy (+fitness)".to_string()),
                            },
                            LifeSituationChoice {
                                id: "draw_pictures".to_string(),
                                label: "Scribble pictures with bright crayons".to_string(),
                                consequence_hint: Some("Expresses creativity (+creativity)".to_string()),
                            },
                        ],
                        min_age: 2,
                        max_age: Some(3),
                        location_id: Some(player.location_id.clone()),
                        expires_in_days: None,
                        generated_year: self.time.year,
                        process_id: None,
                    });
                }
            }
        } else if age >= 4 && age <= 12 {
            // CHILDHOOD (Age 4-12)
            situations.push(LifeSituation {
                id: format!("child_school_routine_{}", self.time.year),
                category: SituationCategory::Routine,
                title: "Primary School Days".to_string(),
                narrative: format!(
                    "School is in full swing in {}. Your teacher introduces a new group reading and problem-solving project this term. Classmates are forming study pairs.",
                    location_name_title
                ),
                choices: vec![
                    LifeSituationChoice {
                        id: "study_hard".to_string(),
                        label: "Throw yourself into the coursework enthusiastically".to_string(),
                        consequence_hint: Some("Improves academic performance (+academics)".to_string()),
                    },
                    LifeSituationChoice {
                        id: "help_friend".to_string(),
                        label: "Pair up with a classmate who is finding it difficult".to_string(),
                        consequence_hint: Some("Builds empathy and strong friendship (+trust)".to_string()),
                    },
                    LifeSituationChoice {
                        id: "focus_creative".to_string(),
                        label: "Add creative drawings and stories to your submission".to_string(),
                        consequence_hint: Some("Highlights individual creativity (+creativity)".to_string()),
                    },
                    LifeSituationChoice {
                        id: "do_minimum".to_string(),
                        label: "Finish quickly so you can go play at break time".to_string(),
                        consequence_hint: Some("Prioritises fun and playground games (+sociability)".to_string()),
                    },
                ],
                min_age: 4,
                max_age: Some(12),
                location_id: Some(player.location_id.clone()),
                expires_in_days: None,
                generated_year: self.time.year,
                process_id: None,
            });

            // If player has football / sports interest
            if player.interests.contains("football") || player.interests.contains("sports") {
                situations.push(LifeSituation {
                    id: format!("child_neighborhood_match_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "After-School Games in the Community".to_string(),
                    narrative: format!(
                        "Kids from the neighborhood are setting up jumpers for goalposts on the local pitch in {}. Someone brought a proper leather ball.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "play_match".to_string(),
                            label: "Join the game and play until sundown".to_string(),
                            consequence_hint: Some("Improves natural ball control and fitness (+football_control)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "practice_alone".to_string(),
                            label: "Drill keepie-uppies and free kicks on the side".to_string(),
                            consequence_hint: Some("Focused technique practice (+discipline)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "watch_and_cheer".to_string(),
                            label: "Cheer on friends from the side and socialize".to_string(),
                            consequence_hint: Some("Deepens social bonds (+sociability)".to_string()),
                        },
                    ],
                    min_age: 6,
                    max_age: Some(12),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            } else if player.interests.contains("music") || player.interests.contains("arts") {
                situations.push(LifeSituation {
                    id: format!("child_music_discovery_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "School Music & Arts Corner".to_string(),
                    narrative: format!(
                        "The school hall in {} has opened up instrument and choir practice after class for interested pupils.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "join_choir".to_string(),
                            label: "Join choir practice and sing with the group".to_string(),
                            consequence_hint: Some("Develops ear for pitch and harmony (+music)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "practice_instrument".to_string(),
                            label: "Try your hand at playing an instrument".to_string(),
                            consequence_hint: Some("Builds musical foundations (+instrument)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "draw_art".to_string(),
                            label: "Spend time in the art studio drawing".to_string(),
                            consequence_hint: Some("Develops visual arts skills (+creativity)".to_string()),
                        },
                    ],
                    min_age: 6,
                    max_age: Some(12),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            }
        } else if age >= 13 && age <= 18 {
            // ADOLESCENCE (Age 13-18)
            situations.push(LifeSituation {
                id: format!("adol_exams_decision_{}", self.time.year),
                category: SituationCategory::Decision,
                title: "Academic Focus & Term Examinations".to_string(),
                narrative: format!(
                    "Mid-term examination season is approaching at your secondary school in {}. The results will dictate your academic standing and teacher recommendations.",
                    location_name_title
                ),
                choices: vec![
                    LifeSituationChoice {
                        id: "dedicated_study".to_string(),
                        label: "Dedicate evenings to revision and past exam papers".to_string(),
                        consequence_hint: Some("Substantially raises academic grade (+academics, +stress)".to_string()),
                    },
                    LifeSituationChoice {
                        id: "balanced_approach".to_string(),
                        label: "Maintain a steady balance between study and personal life".to_string(),
                        consequence_hint: Some("Steady academic progress while preserving well-being".to_string()),
                    },
                    LifeSituationChoice {
                        id: "pursue_passions".to_string(),
                        label: "Study the minimum required and focus on your outside passions".to_string(),
                        consequence_hint: Some("Accelerates hobbies/skills but risks lower grades".to_string()),
                    },
                ],
                min_age: 13,
                max_age: Some(18),
                location_id: Some(player.location_id.clone()),
                expires_in_days: None,
                generated_year: self.time.year,
                process_id: None,
            });

            // Opportunity: Football trial in the area
            if player.interests.contains("football") {
                let ball_control = player.skills.get("football_control").copied().unwrap_or(0.0);
                let trial_desc = if ball_control > 60.0 {
                    format!("A youth scout from a regional club has noticed your performances in {} and invited you to an open trial session.", location_name_title)
                } else {
                    format!("A local grassroots youth team in {} is holding open trials for next season's youth league squad.", location_name_title)
                };

                situations.push(LifeSituation {
                    id: format!("adol_football_trial_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Regional Youth Football Trial Opportunity".to_string(),
                    narrative: trial_desc,
                    choices: vec![
                        LifeSituationChoice {
                            id: "attend_trial_prepared".to_string(),
                            label: "Attend the trial and give it everything you've got".to_string(),
                            consequence_hint: Some("Judged on current ability, athleticism, and match composure".to_string()),
                        },
                        LifeSituationChoice {
                            id: "train_extra_first".to_string(),
                            label: "Spend extra days doing fitness drills before deciding".to_string(),
                            consequence_hint: Some("Boosts fitness and composure before testing yourself".to_string()),
                        },
                        LifeSituationChoice {
                            id: "decline_trial".to_string(),
                            label: "Decline the trial — keep football as a casual hobby".to_string(),
                            consequence_hint: Some("Keeps focus on education and other pursuits".to_string()),
                        },
                    ],
                    min_age: 13,
                    max_age: Some(18),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: Some(30),
                    generated_year: self.time.year,
                    process_id: None,
                });
            }

            // Opportunity: Music / Arts showcase
            if player.interests.contains("music") || player.interests.contains("writing") || player.interests.contains("arts") {
                situations.push(LifeSituation {
                    id: format!("adol_music_showcase_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Youth Music & Creative Talent Showcase".to_string(),
                    narrative: format!(
                        "A local venue in {} is hosting a creative youth open-stage showcase, inviting young musicians, singers, and songwriters to perform.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "practice_singing".to_string(),
                            label: "Practice singing and songwriting to prepare a track".to_string(),
                            consequence_hint: Some("Hones vocal and composition skills (+music, +creativity)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "join_band".to_string(),
                            label: "Form a youth music band with talented school friends".to_string(),
                            consequence_hint: Some("Collaborative musicianship (+sociability, +music)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "attend_showcase_audience".to_string(),
                            label: "Attend the show as an audience member".to_string(),
                            consequence_hint: Some("Enjoys the performances and connects with peers".to_string()),
                        },
                    ],
                    min_age: 13,
                    max_age: Some(18),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: Some(30),
                    generated_year: self.time.year,
                    process_id: None,
                });
            }

            // Opportunity: Politics / Debating forum
            if player.interests.contains("politics") || player.interests.contains("social_causes") {
                situations.push(LifeSituation {
                    id: format!("adol_politics_forum_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Youth Parliament & Civic Debating Assembly".to_string(),
                    narrative: format!(
                        "The regional student union in {} is holding a public youth debate on community policy, public infrastructure, and youth representation.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "attend_political_debate".to_string(),
                            label: "Attend and speak at the local political debate".to_string(),
                            consequence_hint: Some("Delivers an impactful speech (+communication, +confidence)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "draft_policy_essay".to_string(),
                            label: "Write and submit a policy essay on local reforms".to_string(),
                            consequence_hint: Some("Sharpens analytic writing (+writing, +discipline)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "listen_to_debate".to_string(),
                            label: "Observe the discussions from the gallery".to_string(),
                            consequence_hint: Some("Expands understanding of civic governance".to_string()),
                        },
                    ],
                    min_age: 14,
                    max_age: Some(18),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: Some(30),
                    generated_year: self.time.year,
                    process_id: None,
                });
            }

            // Opportunity: Part-time work exploration (if age >= 16)
            if age >= 16 && player.employment.job_title.is_none() {
                situations.push(LifeSituation {
                    id: format!("adol_part_time_work_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Part-Time Student Job Opening".to_string(),
                    narrative: format!(
                        "A local cafe and retail store in {} has posted a weekend assistant notice looking for energetic students.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "apply_part_time".to_string(),
                            label: format!("Submit an application (Earn ~{}150/mo)", currency_symbol),
                            consequence_hint: Some("Starts a job application process for pocket money".to_string()),
                        },
                        LifeSituationChoice {
                            id: "inquire_in_person".to_string(),
                            label: "Visit the shop in person and speak to the manager".to_string(),
                            consequence_hint: Some("Demonstrates initiative (+confidence)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "pass_on_job".to_string(),
                            label: "Pass on the job to focus fully on schooling".to_string(),
                            consequence_hint: Some("More time for studies and relaxation".to_string()),
                        },
                    ],
                    min_age: 16,
                    max_age: Some(18),
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: Some(21),
                    generated_year: self.time.year,
                    process_id: None,
                });
            }
        } else {
            // ADULTHOOD (Age 19+)
            if player.employment.job_title.is_none() {
                situations.push(LifeSituation {
                    id: format!("adult_job_search_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Career & Employment Search".to_string(),
                    narrative: format!(
                        "You are currently unemployed in {}. Local employers across various sectors have listed entry-level and intermediate vacancies.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "start_job_search_process".to_string(),
                            label: "Begin structured job search & send CVs".to_string(),
                            consequence_hint: Some("Initiates a formal multi-step recruitment process".to_string()),
                        },
                        LifeSituationChoice {
                            id: "freelance_work".to_string(),
                            label: "Take on independent odd jobs and gig work".to_string(),
                            consequence_hint: Some("Earns immediate modest cash without long-term contracts".to_string()),
                        },
                        LifeSituationChoice {
                            id: "apply_higher_education".to_string(),
                            label: "Apply for University / College Degree programs".to_string(),
                            consequence_hint: Some("Initiates university admission process".to_string()),
                        },
                    ],
                    min_age: 18,
                    max_age: None,
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            } else {
                let job = player.employment.job_title.clone().unwrap_or_default();
                situations.push(LifeSituation {
                    id: format!("adult_workplace_dynamic_{}", self.time.year),
                    category: SituationCategory::Routine,
                    title: format!("Professional Responsibilities: {}", job),
                    narrative: format!(
                        "Your routine as a {} in {} continues. Quarterly performance reviews are on the horizon, and your team is tackling key deliverables.",
                        job, location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "overtime_work".to_string(),
                            label: "Take on overtime and spearhead a project".to_string(),
                            consequence_hint: Some("Boosts job performance and bonus prospects (+performance, +stress)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "steady_work".to_string(),
                            label: "Execute tasks diligently while maintaining work-life balance".to_string(),
                            consequence_hint: Some("Steady career progression with healthy stress levels".to_string()),
                        },
                        LifeSituationChoice {
                            id: "network_colleagues".to_string(),
                            label: "Organize social drinks with colleagues and industry peers".to_string(),
                            consequence_hint: Some("Expands professional network and trust (+sociability)".to_string()),
                        },
                    ],
                    min_age: 18,
                    max_age: None,
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            }

            // Adult Politics & Civic Engagement
            if player.interests.contains("politics") || player.interests.contains("social_causes") {
                situations.push(LifeSituation {
                    id: format!("adult_politics_engagement_{}", self.time.year),
                    category: SituationCategory::Opportunity,
                    title: "Constituency Policy Forum & Political Assembly".to_string(),
                    narrative: format!(
                        "The civic policy forum in {} is convening local members to debate public policy proposals, community initiatives, and party constituency matters.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "attend_political_debate".to_string(),
                            label: "Attend and participate in the policy debate".to_string(),
                            consequence_hint: Some("Engages with civic leaders and delivers remarks (+fame, +confidence)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "draft_policy_essay".to_string(),
                            label: "Author a published policy memorandum on regional development".to_string(),
                            consequence_hint: Some("Expands political credibility and analytical standing (+writing)".to_string()),
                        },
                        LifeSituationChoice {
                            id: "network_political_members".to_string(),
                            label: "Network with party committee delegates".to_string(),
                            consequence_hint: Some("Deepens political alliances (+trust)".to_string()),
                        },
                    ],
                    min_age: 18,
                    max_age: None,
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            }

            // Housing Situation
            if player.housing.housing_type == "FamilyHome" {
                situations.push(LifeSituation {
                    id: format!("adult_housing_move_{}", self.time.year),
                    category: SituationCategory::Decision,
                    title: "Housing Independence & Rental Search".to_string(),
                    narrative: format!(
                        "You are living at home with family in {}. Several apartment listings and flatshares have opened up across the city.",
                        location_name_title
                    ),
                    choices: vec![
                        LifeSituationChoice {
                            id: "start_housing_search".to_string(),
                            label: "Search for a rental apartment to move into".to_string(),
                            consequence_hint: Some("Initiates apartment viewing and lease signing process".to_string()),
                        },
                        LifeSituationChoice {
                            id: "save_living_at_home".to_string(),
                            label: "Stay living at home to accumulate savings".to_string(),
                            consequence_hint: Some("Saves monthly rent money to build financial buffer".to_string()),
                        },
                    ],
                    min_age: 19,
                    max_age: None,
                    location_id: Some(player.location_id.clone()),
                    expires_in_days: None,
                    generated_year: self.time.year,
                    process_id: None,
                });
            }
        }

        // Keep top 2-3 most relevant situations
        situations.truncate(3);
        self.active_situations = situations;
    }

    pub fn resolve_situation_choice(&mut self, situation_id: &str, choice_id: &str) -> StepResult {
        let player_id = "person:sim:player".to_string();
        let player = match self.persons.get(&player_id) {
            Some(p) => p.clone(),
            None => {
                return StepResult {
                    success: false,
                    narrative: "No active player found.".to_string(),
                    causality_note: "Player entity missing.".to_string(),
                    event_record: EventRecord {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: self.time.formatted(),
                        event_type: "ERROR".to_string(),
                        actor_id: player_id,
                        target_id: None,
                        summary: "Player missing.".to_string(),
                        metadata: serde_json::json!({}),
                        causality_parent_id: None,
                    },
                }
            }
        };

        let age = (self.time.year - player.identity.birth_year) as u32;
        let mut narrative = String::new();
        let mut causality_note = String::new();
        let mut success = true;
        let mut days_to_advance = 7u32;

        // Check if situation belongs to a ProcessChain
        let mut completed_process_id: Option<String> = None;
        if let Some(proc) = self.active_processes.iter_mut().find(|p| p.is_active && situation_id.contains(&p.id)) {
            match choice_id {
                "advance_process" => {
                    proc.current_step += 1;
                    if proc.current_step >= proc.total_steps {
                        proc.is_active = false;
                        completed_process_id = Some(proc.id.clone());
                        narrative = format!("Successfully completed all requirements for {}!", proc.title);
                        causality_note = format!("Process '{}' completed successfully.", proc.title);
                    } else {
                        narrative = format!("Completed the current phase of {}. Moving to next requirement.", proc.title);
                        causality_note = format!("Advanced to step {}/{} in {}.", proc.current_step + 1, proc.total_steps, proc.title);
                    }
                }
                "delay_process" => {
                    narrative = format!("You took additional time to prepare for {}.", proc.title);
                    causality_note = "Process delayed to allow extra preparation.".to_string();
                    days_to_advance = 14;
                }
                "cancel_process" => {
                    proc.is_active = false;
                    narrative = format!("You chose to withdraw from {}.", proc.title);
                    causality_note = "Process cancelled by player decision.".to_string();
                }
                _ => {}
            }
        } else {
            // Standard situational choices
            match choice_id {
                // Infancy choices
                "rest_peacefully" => {
                    narrative = "You drifted off to a peaceful sleep, comforted by the gentle sounds of your home.".to_string();
                    causality_note = "Restful sleep supported healthy infant growth and emotional stability.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.fitness = (p.health.fitness + 2.0).min(100.0);
                        p.health.stress = (p.health.stress - 5.0).max(0.0);
                    }
                    days_to_advance = 14;
                }
                "reach_out" => {
                    narrative = "You reached out your small hand and babbled cheerfully. Your parent smiled and held your fingers tightly.".to_string();
                    causality_note = "Deepened emotional connection with your parent.".to_string();
                    if let Some(pid) = player.parent_ids.first() {
                        self.relationships.modify_link(pid.clone(), player_id.clone(), |rel| {
                            rel.trust = (rel.trust + 0.08).min(1.0);
                        });
                    }
                    days_to_advance = 14;
                }
                "observe_room" => {
                    narrative = "You stared intently around the room, tracking dust motes in the sunlight and noticing the bright colours of the curtains.".to_string();
                    causality_note = "Early sensory exploration sparked early cognitive curiosity.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.personality.openness = (p.personality.openness + 0.02).min(1.0);
                    }
                    days_to_advance = 14;
                }
                "explore_household" => {
                    narrative = "You wobbled on two feet and proudly walked from one side of the room to the other, clinging briefly to a coffee table.".to_string();
                    causality_note = "Developed motor coordination and physical confidence.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("athleticism".to_string()).or_insert(10.0);
                        *entry += 3.0;
                    }
                    days_to_advance = 30;
                }
                "play_with_blocks" => {
                    narrative = "You spent an afternoon carefully stacking coloured wooden blocks, clapping every time the tower stood tall.".to_string();
                    causality_note = "Patience and fine motor skills developed naturally through play.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.personality.conscientiousness = (p.personality.conscientiousness + 0.02).min(1.0);
                    }
                    days_to_advance = 30;
                }
                "mimic_speech" => {
                    narrative = "You listened closely to your family's conversation and echoed words back, earning proud laughter and encouragement.".to_string();
                    causality_note = "Language comprehension and expressive vocabulary blossomed.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("communication".to_string()).or_insert(15.0);
                        *entry += 4.0;
                    }
                    days_to_advance = 30;
                }
                "listen_to_stories" => {
                    narrative = "Curled up on the sofa, you listened wide-eyed to an illustrated story about distant adventures.".to_string();
                    causality_note = "Early exposure to storytelling built imagination and pre-reading skills.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("reading".to_string()).or_insert(20.0);
                        *entry += 4.0;
                        p.personality.openness = (p.personality.openness + 0.03).min(1.0);
                    }
                    days_to_advance = 30;
                }
                "play_outdoors" => {
                    narrative = "You ran through the outdoor courtyard, chasing butterflies and enjoying the warm breeze.".to_string();
                    causality_note = "Active play strengthened physical stamina and elevated mood.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.fitness = (p.health.fitness + 3.0).min(100.0);
                    }
                    days_to_advance = 30;
                }
                "draw_pictures" => {
                    narrative = "Armed with wax crayons, you filled sheets of paper with colourful depictions of your family and home.".to_string();
                    causality_note = "Creative expression flourished.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("creativity".to_string()).or_insert(20.0);
                        *entry += 3.0;
                    }
                    days_to_advance = 30;
                }

                // Childhood choices
                "study_hard" => {
                    narrative = "You sat down with your school books every afternoon, preparing your assignments thoroughly. Your teacher praised your diligence.".to_string();
                    causality_note = "Academic performance increased noticeably (+6.0).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.academic_performance = (p.education.academic_performance + 6.0).min(100.0);
                        p.personality.discipline = (p.personality.discipline + 0.02).min(1.0);
                    }
                    days_to_advance = 14;
                }
                "help_friend" => {
                    narrative = "You sat beside a classmate and patiently walked through the lesson questions together. Your friendship blossomed.".to_string();
                    causality_note = "Demonstrated empathy and built strong peer trust.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("communication".to_string()).or_insert(30.0);
                        *entry += 3.0;
                        p.personality.agreeableness = (p.personality.agreeableness + 0.03).min(1.0);
                    }
                    days_to_advance = 14;
                }
                "focus_creative" => {
                    narrative = "You poured your energy into illustrating and framing your project uniquely. The teacher pinned your work to the classroom board.".to_string();
                    causality_note = "Creative flair recognized and encouraged.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("creativity".to_string()).or_insert(30.0);
                        *entry += 4.0;
                    }
                    days_to_advance = 14;
                }
                "do_minimum" => {
                    narrative = "You wrapped up the schoolwork quickly and spent the rest of break running around playing games with friends.".to_string();
                    causality_note = "Maintained high energy and great social enjoyment.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.stress = (p.health.stress - 10.0).max(0.0);
                    }
                    days_to_advance = 14;
                }
                "play_match" => {
                    narrative = "You joined in the neighborhood football game. After a few scrappy minutes, you found your rhythm, completing sharp passes and enjoying every touch.".to_string();
                    causality_note = "Recreational match experience improved ball control (+3.0) and stamina.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("football_control".to_string()).or_insert(30.0);
                        *entry = (*entry + 3.0).min(100.0);
                        p.football_attributes.ball_control = (p.football_attributes.ball_control + 3.0).min(99.0);
                        p.health.fitness = (p.health.fitness + 2.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "practice_alone" => {
                    narrative = "You stayed by the wall, juggling the ball repeatedly to test your touch. By the end of the session, your control felt much sharper.".to_string();
                    causality_note = "Disciplined solo practice honed technical touch (+2.5).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("football_control".to_string()).or_insert(30.0);
                        *entry = (*entry + 2.5).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "watch_and_cheer" => {
                    narrative = "You sat with friends on the grass, shouting encouragement and debating favourite teams.".to_string();
                    causality_note = "Shared laughs and relaxed community time.".to_string();
                    days_to_advance = 7;
                }

                // Adolescent choices
                "dedicated_study" => {
                    narrative = "You spent two weeks in rigorous revision. When exam day arrived, you turned over the paper and answered with confidence.".to_string();
                    causality_note = "Intensive study significantly boosted exam marks (+8.0).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.academic_performance = (p.education.academic_performance + 8.0).min(100.0);
                        p.health.stress = (p.health.stress + 5.0).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "balanced_approach" => {
                    narrative = "You kept a sensible revision schedule, making steady progress without burning yourself out.".to_string();
                    causality_note = "Balanced study maintained solid academic growth (+4.0).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.academic_performance = (p.education.academic_performance + 4.0).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "pursue_passions" => {
                    narrative = "You gave the books a quick skim and spent the rest of your time focusing on your personal interests.".to_string();
                    causality_note = "Prioritized personal hobbies over exam performance.".to_string();
                    days_to_advance = 14;
                }
                "attend_trial_prepared" => {
                    let control = player.skills.get("football_control").copied().unwrap_or(40.0);
                    let trial_roll = self.rng.gen_range_f32(0.8, 1.2) * control;
                    if trial_roll > 55.0 {
                        narrative = "You attended the trial and played with composure. The coaches took your details and invited you to attend formal squad training next month.".to_string();
                        causality_note = "Trial performance earned positive scout feedback. Youth academy pathway unlocked.".to_string();
                        if let Some(p) = self.persons.get_mut(&player_id) {
                            p.football_role = FootballRole::AcademyProspect;
                        }
                    } else {
                        narrative = "You worked hard during the trial matches, though the pace was intense. The coaches gave constructive feedback on areas to improve.".to_string();
                        causality_note = "Trial was challenging. Valuable competitive experience gained (+2.0 control).".to_string();
                        if let Some(p) = self.persons.get_mut(&player_id) {
                            let entry = p.skills.entry("football_control".to_string()).or_insert(40.0);
                            *entry = (*entry + 2.0).min(100.0);
                        }
                    }
                    days_to_advance = 7;
                }
                "train_extra_first" => {
                    narrative = "You put in extra sprint and stamina sessions across the week to get yourself in peak physical shape.".to_string();
                    causality_note = "Intensive physical preparation boosted athleticism (+2.5).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.fitness = (p.health.fitness + 4.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "decline_trial" => {
                    narrative = "You decided not to attend the trial, preferring to keep your pursuits casual and pressure-free.".to_string();
                    causality_note = "Chose a relaxed path focused on daily life.".to_string();
                    days_to_advance = 7;
                }
                "apply_part_time" => {
                    narrative = "You submitted your CV and completed an introductory interview. The shop manager offered you weekend shifts.".to_string();
                    causality_note = "Hired as Weekend Retail Assistant (+£140/mo).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_title = Some("Weekend Assistant".to_string());
                        p.employment.monthly_salary = 140.0;
                    }
                    days_to_advance = 7;
                }
                "inquire_in_person" => {
                    narrative = "You walked in smartly dressed, introduced yourself to the manager, and handed over your details. They appreciated your direct approach.".to_string();
                    causality_note = "Direct initiative impressed employer (+confidence).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("communication".to_string()).or_insert(40.0);
                        *entry += 2.0;
                        p.employment.job_title = Some("Store Assistant".to_string());
                        p.employment.monthly_salary = 160.0;
                    }
                    days_to_advance = 7;
                }
                "practice_singing" => {
                    narrative = "You spent dedicated hours practicing vocal scales, lyric writing, and melodic arrangement. Your creative confidence grew significantly.".to_string();
                    causality_note = "Vocal practice and songwriting refined creative musicianship (+music, +creativity).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("music".to_string()).or_insert(30.0);
                        *entry = (*entry + 4.0).min(100.0);
                        let c_entry = p.skills.entry("creativity".to_string()).or_insert(30.0);
                        *c_entry = (*c_entry + 3.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "join_band" => {
                    narrative = "You met up with fellow aspiring musicians in the neighborhood garage. The chemistry was energetic as you jammed together.".to_string();
                    causality_note = "Collaborative band rehearsals fostered musical teamwork (+sociability).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("music".to_string()).or_insert(30.0);
                        *entry = (*entry + 3.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "attend_political_debate" => {
                    narrative = "You took the microphone at the civic assembly and articulated your perspective clearly before the audience, drawing thoughtful nods and applause.".to_string();
                    causality_note = "Public debate participation raised civic standing and oratory eloquence (+communication).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("communication".to_string()).or_insert(35.0);
                        *entry = (*entry + 4.0).min(100.0);
                        p.fame.fame_level = (p.fame.fame_level + 2.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "draft_policy_essay" => {
                    narrative = "You spent several evenings researching municipal data and drafted a structured policy proposal on youth development and community services.".to_string();
                    causality_note = "In-depth policy analysis developed analytical writing (+writing, +discipline).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("writing".to_string()).or_insert(35.0);
                        *entry = (*entry + 4.0).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "network_political_members" => {
                    narrative = "You engaged in thoughtful conversations with community representatives and local organizers, discussing regional civic initiatives.".to_string();
                    causality_note = "Built connections within local civic networks.".to_string();
                    days_to_advance = 7;
                }
                "pass_on_job" => {
                    narrative = "You chose to focus your free hours on your education and personal projects.".to_string();
                    causality_note = "Dedicated time fully to schooling.".to_string();
                    days_to_advance = 7;
                }

                // Adult choices
                "start_job_search_process" => {
                    // Create a multi-step job search process
                    let proc_id = format!("proc_job_{}", self.rng.gen_range_u32(1000, 9999));
                    self.active_processes.push(ProcessChain {
                        id: proc_id,
                        process_type: "JOB_SEARCH".to_string(),
                        title: "Employment Application Pathway".to_string(),
                        current_step: 0,
                        total_steps: 3,
                        steps: vec![
                            ProcessStep {
                                step_index: 0,
                                title: "Tailor CV & Apply to Shortlisted Vacancies".to_string(),
                                description: "Refine your resume and submit formal job applications to local organizations.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 1,
                                title: "Attend Formal Interview".to_string(),
                                description: "Meet with hiring managers to discuss your qualifications and suitability.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 2,
                                title: "Review Contract Offer & Onboarding".to_string(),
                                description: "Sign employment terms, agree on salary, and complete onboarding documentation.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                        ],
                        outcome_payload: Some(serde_json::json!({
                            "job_title": "Associate Specialist",
                            "monthly_salary": 2200.0
                        })),
                        is_active: true,
                    });
                    narrative = "You set up a professional portfolio and initiated a structured job search process.".to_string();
                    causality_note = "Started formal 3-step employment recruitment pathway.".to_string();
                    days_to_advance = 7;
                }
                "freelance_work" => {
                    narrative = "You took on independent local gig projects over the fortnight, earning immediate cash in hand.".to_string();
                    causality_note = "Earned £350 from independent contract work.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.finances.cash += 350.0;
                    }
                    days_to_advance = 14;
                }
                "apply_higher_education" => {
                    let proc_id = format!("proc_uni_{}", self.rng.gen_range_u32(1000, 9999));
                    self.active_processes.push(ProcessChain {
                        id: proc_id,
                        process_type: "UNIVERSITY_APPLICATION".to_string(),
                        title: "University Degree Admission Pathway".to_string(),
                        current_step: 0,
                        total_steps: 3,
                        steps: vec![
                            ProcessStep {
                                step_index: 0,
                                title: "Submit Academic Transcripts & Personal Statement".to_string(),
                                description: "Compile examination records and submit admission papers to university admissions board.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 1,
                                title: "Entrance Assessment & Funding Verification".to_string(),
                                description: "Verify tuition support or student loan funding and sit faculty entrance test.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 2,
                                title: "Formal University Matriculation".to_string(),
                                description: "Collect student ID, attend orientation, and register for inaugural semester modules.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                        ],
                        outcome_payload: None,
                        is_active: true,
                    });
                    narrative = "You initiated your application for higher education degree studies.".to_string();
                    causality_note = "Started university admission process.".to_string();
                    days_to_advance = 7;
                }
                "overtime_work" => {
                    narrative = "You stayed late at the office, coordinating deliverables and ensuring key targets were met.".to_string();
                    causality_note = "High work output boosted job performance (+4.0) and earned £220 bonus.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_performance = (p.employment.job_performance + 4.0).min(100.0);
                        p.finances.cash += 220.0;
                        p.health.stress = (p.health.stress + 6.0).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "steady_work" => {
                    narrative = "You completed your workplace tasks steadily, maintaining quality without unnecessary stress.".to_string();
                    causality_note = "Reliable performance and healthy work-life balance.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_performance = (p.employment.job_performance + 1.5).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "network_colleagues" => {
                    narrative = "You went out for dinner and drinks with teammates, having engaging conversations about career goals and life.".to_string();
                    causality_note = "Stronger interpersonal connections with colleagues (+sociability).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.stress = (p.health.stress - 8.0).max(0.0);
                    }
                    days_to_advance = 7;
                }
                "start_housing_search" => {
                    let proc_id = format!("proc_house_{}", self.rng.gen_range_u32(1000, 9999));
                    self.active_processes.push(ProcessChain {
                        id: proc_id,
                        process_type: "HOUSING_SEARCH".to_string(),
                        title: "Independent Rental Accommodation Search".to_string(),
                        current_step: 0,
                        total_steps: 3,
                        steps: vec![
                            ProcessStep {
                                step_index: 0,
                                title: "Browse Listings & Attend Property Viewings".to_string(),
                                description: "Inspect shortlisted rental flats and meet letting agents.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 1,
                                title: "Reference Checks & Deposit Payment".to_string(),
                                description: "Provide employer references and pay holding deposit.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                            ProcessStep {
                                step_index: 2,
                                title: "Sign Tenancy Agreement & Collect Keys".to_string(),
                                description: "Sign legal lease and move into your independent home.".to_string(),
                                target_date: self.time.formatted(),
                                is_completed: false,
                            },
                        ],
                        outcome_payload: Some(serde_json::json!({
                            "housing_type": "Renting",
                            "monthly_cost": 650.0
                        })),
                        is_active: true,
                    });
                    narrative = "You registered with local letting agencies to begin searching for an apartment.".to_string();
                    causality_note = "Initiated 3-step rental search and tenancy pathway.".to_string();
                    days_to_advance = 7;
                }
                "save_living_at_home" => {
                    narrative = "You decided to remain living at home for the moment, putting extra income directly into your savings buffer.".to_string();
                    causality_note = "Living at home enabled maximum savings accumulation.".to_string();
                    days_to_advance = 30;
                }
                "apply_university_science" => {
                    let uni_name = if player.location_id.contains("nigeria") {
                        "University of Abuja"
                    } else if player.location_id.contains("glasgow") || player.location_id.contains("united_kingdom") {
                        "University of Glasgow"
                    } else if player.location_id.contains("new_york") {
                        "Columbia University"
                    } else {
                        "National University"
                    };
                    self.academic_program = Some(AcademicProgram {
                        university_name: uni_name.to_string(),
                        faculty: "School of Computing & Mathematical Sciences".to_string(),
                        degree_title: "Bachelor of Science (B.Sc.) in Computer Science".to_string(),
                        current_year: 1,
                        total_years: 4,
                        current_semester: 1,
                        gpa: 3.65,
                        is_graduated: false,
                    });
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.degree_program = Some("B.Sc. Computer Science".to_string());
                    }
                    narrative = format!("You were formally accepted into the B.Sc. Computer Science program at {}! Classes commence this academic semester.", uni_name);
                    causality_note = "Enrolled in 4-year undergraduate degree program.".to_string();
                    days_to_advance = 14;
                }
                "apply_university_law" => {
                    let uni_name = if player.location_id.contains("nigeria") {
                        "University of Abuja"
                    } else if player.location_id.contains("glasgow") || player.location_id.contains("united_kingdom") {
                        "University of Glasgow"
                    } else {
                        "National University"
                    };
                    self.academic_program = Some(AcademicProgram {
                        university_name: uni_name.to_string(),
                        faculty: "Faculty of Law & Jurisprudence".to_string(),
                        degree_title: "Bachelor of Laws (LL.B.)".to_string(),
                        current_year: 1,
                        total_years: 5,
                        current_semester: 1,
                        gpa: 3.50,
                        is_graduated: false,
                    });
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.degree_program = Some("LL.B. Law".to_string());
                    }
                    narrative = format!("You were admitted to study Law at {}! You received your matriculation package.", uni_name);
                    causality_note = "Enrolled in 5-year undergraduate legal curriculum.".to_string();
                    days_to_advance = 14;
                }
                "attend_lectures" => {
                    narrative = "You attended all scheduled morning and afternoon lectures, participating in tutorial problem sets.".to_string();
                    causality_note = "Steady academic attendance improved course mastery (+cognition).".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("cognition".to_string()).or_insert(50.0);
                        *entry = (*entry + 1.5).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "study_library_uni" => {
                    narrative = "You spent quiet evenings in the university library reading research journals and completing assignments.".to_string();
                    causality_note = "Diligent coursework revision boosted academic performance.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.academic_performance = (p.education.academic_performance + 2.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "campus_social" => {
                    narrative = "You attended student union discussions and relaxed with friends on campus, forming meaningful friendships.".to_string();
                    causality_note = "Active social life strengthened emotional well-being.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.stress = (p.health.stress - 6.0).max(0.0);
                    }
                    days_to_advance = 7;
                }
                "action_football_trial_youth" => {
                    // Scout assessment
                    let football_skill = player.skills.get("football_control").copied().unwrap_or(50.0);
                    let athleticism = player.skills.get("athleticism").copied().unwrap_or(50.0);
                    let score = (football_skill * 0.6) + (athleticism * 0.4);

                    // Remove trial deadline
                    self.active_deadlines.retain(|d| d.id != "football_trial_youth");

                    if score >= 65.0 {
                        narrative = "You performed brilliantly at the regional trial! Scouts praised your technical flair and offered you a youth academy training agreement.".to_string();
                        causality_note = "Exceptional trial performance resulted in regional academy invitation.".to_string();
                        if let Some(p) = self.persons.get_mut(&player_id) {
                            p.football_contract = Some(FootballContract {
                                club_id: "club:sim:regional_youth_fc".to_string(),
                                club_name: "Regional Youth Academy FC".to_string(),
                                weekly_wage: 80.0,
                                years_remaining: 2,
                                release_clause: 5000.0,
                                goal_bonus: 20.0,
                                agent_id: None,
                            });
                        }
                    } else {
                        narrative = "You gave your all at the trial. The scouts commended your work ethic and gave you clear technical areas to develop before next season.".to_string();
                        causality_note = "Valuable match trial experience gained with constructive scout feedback.".to_string();
                        if let Some(p) = self.persons.get_mut(&player_id) {
                            let entry = p.skills.entry("football_control".to_string()).or_insert(50.0);
                            *entry = (*entry + 4.0).min(100.0);
                        }
                    }
                    days_to_advance = 7;
                }
                "launch_creator_channel" => {
                    let handle = format!("@{}", player.identity.first_name.to_lowercase());
                    self.creator_channel = Some(CreatorChannel {
                        platform_name: "VideoNet".to_string(),
                        channel_handle: handle.clone(),
                        content_niche: "Tech, Creativity & Life".to_string(),
                        subscriber_count: 850,
                        total_views: 12500,
                        monthly_ad_revenue: 45.0,
                        brand_deals_count: 0,
                        burnout_level: 10.0,
                    });
                    narrative = format!("You published your first edited video series under '{}'. Early viewers began subscribing and commenting!", handle);
                    causality_note = "Launched independent digital content channel.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("creativity".to_string()).or_insert(30.0);
                        *entry = (*entry + 5.0).min(100.0);
                    }
                    days_to_advance = 14;
                }
                "produce_creator_video" => {
                    if let Some(ref mut ch) = self.creator_channel {
                        let new_subs = self.rng.gen_range_u32(1200, 4800) as u64;
                        let new_views = (new_subs * 12) + 5000;
                        ch.subscriber_count += new_subs;
                        ch.total_views += new_views;
                        ch.monthly_ad_revenue += (new_subs as f64) * 0.08;
                        ch.burnout_level = (ch.burnout_level + 15.0).min(100.0);

                        if ch.burnout_level >= 75.0 && self.active_crises.is_empty() {
                            self.active_crises.push(CareerCrisis {
                                id: "crisis_creator_burnout".to_string(),
                                crisis_type: "CREATIVE_BURNOUT".to_string(),
                                description: "The relentless schedule and constant algorithm pressure have caused severe creative exhaustion.".to_string(),
                                severity: 4,
                                unresolved: true,
                            });
                        }

                        narrative = format!("Your new video performed strongly, gaining +{} subscribers and bringing your total audience to {}!", new_subs.to_string(), ch.subscriber_count.to_string());
                        causality_note = "Audience growth accompanied by rising workload pressure.".to_string();
                    } else {
                        narrative = "You recorded and edited a new video project.".to_string();
                        causality_note = "Content creation practice.".to_string();
                    }
                    days_to_advance = 7;
                }
                "handle_burnout_break" => {
                    if let Some(ref mut ch) = self.creator_channel {
                        ch.burnout_level = 5.0;
                    }
                    self.active_crises.retain(|c| c.crisis_type != "CREATIVE_BURNOUT");
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.stress = (p.health.stress - 30.0).max(0.0);
                    }
                    narrative = "You stepped away from the screen for two restorative months. Traveling and resting with family completely renewed your creative energy.".to_string();
                    causality_note = "Sabbatical resolved burnout and restored health.".to_string();
                    days_to_advance = 60;
                }
                "handle_burnout_pivot_production" => {
                    if let Some(ref mut ch) = self.creator_channel {
                        ch.burnout_level = 10.0;
                    }
                    self.active_crises.retain(|c| c.crisis_type != "CREATIVE_BURNOUT");
                    self.life_pivots.push(LifePivot {
                        former_identity: "Solo Content Creator".to_string(),
                        new_path: "Digital Media Studio Executive".to_string(),
                        year_of_pivot: self.time.year,
                        rationale: "Hired an editing team and transitioned from solo recording into media production.".to_string(),
                    });
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_title = Some("Managing Director (Media Agency)".to_string());
                        p.employment.monthly_salary = 4200.0;
                    }
                    narrative = "You founded a boutique digital production agency, hiring talented young editors and writers. You now direct projects sustainably!".to_string();
                    causality_note = "Pivoted from solo burnout into sustainable media enterprise.".to_string();
                    days_to_advance = 30;
                }
                "start_saturday_match" => {
                    let match_rating = self.rng.gen_range_u32(65, 92) as f32 / 10.0;
                    narrative = format!("You started in the weekend competitive match, earning a match rating of {:.1}/10. Coaches noted your tactical maturity.", match_rating);
                    causality_note = "Competitive match experience gained.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.health.fitness = (p.health.fitness + 1.0).min(100.0);
                    }
                    days_to_advance = 7;
                }
                "football_pivot_coaching" => {
                    self.life_pivots.push(LifePivot {
                        former_identity: "Youth Player".to_string(),
                        new_path: "Academy Coach & Talent Scout".to_string(),
                        year_of_pivot: self.time.year,
                        rationale: "Transitioned from on-pitch player to certified academy coaching staff.".to_string(),
                    });
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_title = Some("Youth Academy Coach".to_string());
                        p.employment.monthly_salary = 1850.0;
                        p.football_contract = None;
                    }
                    narrative = "You completed your foundational Coaching License. Regional clubs welcomed your sharp tactical mind as a Youth Academy Coach!".to_string();
                    causality_note = "Successful career pivot into coaching and talent identification.".to_string();
                    days_to_advance = 30;
                }
                "launch_startup_venture" => {
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_title = Some("Founder & Managing Director".to_string());
                        p.employment.monthly_salary = 1500.0;
                        p.finances.cash = (p.finances.cash - 800.0).max(0.0);
                    }
                    narrative = "You incorporated your independent enterprise, signing initial consulting clients in the city!".to_string();
                    causality_note = "Founded new commercial business enterprise.".to_string();
                    days_to_advance = 30;
                }
                "handle_controversy_apology" => {
                    self.reputation.public_standing = (self.reputation.public_standing + 0.35).min(1.0);
                    self.reputation.active_controversies.clear();
                    narrative = "You delivered a heartfelt, transparent public address addressing past misunderstandings. Public trust began to heal steadily.".to_string();
                    causality_note = "Accountability restored public credibility and peer respect.".to_string();
                    days_to_advance = 14;
                }
                "organize_community_townhall" => {
                    self.reputation.public_standing = (self.reputation.public_standing + 0.20).min(1.0);
                    self.reputation.peer_respect = (self.reputation.peer_respect + 0.15).min(1.0);
                    narrative = "You organized a packed town hall debate on youth opportunities and public infrastructure. Community elders commended your leadership!".to_string();
                    causality_note = "Civic leadership expanded public reputation.".to_string();
                    days_to_advance = 14;
                }
                _ => {
                    narrative = format!("You chose to: {}.", choice_id);
                    causality_note = "Situational choice executed.".to_string();
                    days_to_advance = 7;
                }
            }
        }

        // Apply completed process effects if any
        if let Some(pid) = completed_process_id {
            if let Some(proc) = self.active_processes.iter().find(|p| p.id == pid) {
                if proc.process_type == "JOB_SEARCH" {
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.employment.job_title = Some("Associate Specialist".to_string());
                        p.employment.monthly_salary = 2200.0;
                        p.employment.employer_org_id = Some("org:sim:local_enterprise".to_string());
                    }
                } else if proc.process_type == "HOUSING_SEARCH" {
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.housing.housing_type = "Renting".to_string();
                        p.housing.monthly_cost = 650.0;
                    }
                } else if proc.process_type == "UNIVERSITY_APPLICATION" {
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.education.degree_program = Some("Bachelor of Arts/Science".to_string());
                        p.education.grade_level = 1;
                    }
                }
            }
        }

        // Record event
        let event_record = EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "LIFE_SITUATION".to_string(),
            actor_id: player_id.clone(),
            target_id: None,
            summary: narrative.clone(),
            metadata: serde_json::json!({
                "situation_id": situation_id,
                "choice_id": choice_id,
                "success": success,
                "causality_note": causality_note,
            }),
            causality_parent_id: None,
        };

        self.events.push(event_record.clone());

        // Advance time
        self.step_time_forward(days_to_advance);
        self.tick_npc_simulation();

        // Regenerate active situations for current state
        self.generate_active_situations();

        StepResult {
            success,
            narrative,
            causality_note,
            event_record,
        }
    }

    pub fn advance_time_with_events(&mut self, days: u32) -> StepResult {
        let player_id = "person:sim:player".to_string();
        let player_exists = self.persons.contains_key(&player_id);

        if !player_exists {
            return StepResult {
                success: false,
                narrative: "No active player.".to_string(),
                causality_note: "No active player.".to_string(),
                event_record: EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "ERROR".to_string(),
                    actor_id: player_id,
                    target_id: None,
                    summary: "Player missing.".to_string(),
                    metadata: serde_json::json!({}),
                    causality_parent_id: None,
                },
            };
        }

        let old_month = self.time.month;
        let old_year = self.time.year;
        self.step_time_forward(days);
        self.tick_npc_simulation();

        let player = self.persons.get_mut(&player_id).unwrap();
        let age = (self.time.year - player.identity.birth_year) as u32;
        let location_name = player.location_id.replace("city:real:", "").replace("city:sim:", "").replace('_', " ");
        let loc_title = location_name.chars().enumerate().map(|(i, c)| if i == 0 || location_name.chars().nth(i-1) == Some(' ') { c.to_ascii_uppercase() } else { c }).collect::<String>();

        // Monthly financial cycle
        if self.time.month != old_month || self.time.year != old_year {
            if player.employment.monthly_salary > 0.0 {
                player.finances.cash += player.employment.monthly_salary;
            }
            if player.finances.monthly_allowance > 0.0 {
                player.finances.cash += player.finances.monthly_allowance;
            }
            if player.housing.monthly_cost > 0.0 {
                player.finances.cash = (player.finances.cash - player.housing.monthly_cost).max(0.0);
            }
            if player.finances.monthly_expenses > 0.0 {
                player.finances.cash = (player.finances.cash - player.finances.monthly_expenses).max(0.0);
            }
        }

        let narrative = if days == 1 {
            format!("A quiet day passed in {}.", loc_title)
        } else if days <= 7 {
            format!("A week passed in {}. Routine daily life continued steadily.", loc_title)
        } else {
            format!("{} days passed in {}. Seasons shifted gently across the city as time moved forward.", days, loc_title)
        };

        let causality_note = format!("Time advanced by {} days.", days);

        let event_record = EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: "TIME_ADVANCED".to_string(),
            actor_id: player_id,
            target_id: None,
            summary: narrative.clone(),
            metadata: serde_json::json!({ "days_advanced": days }),
            causality_parent_id: None,
        };

        self.events.push(event_record.clone());
        self.generate_active_situations();

        StepResult {
            success: true,
            narrative,
            causality_note,
            event_record,
        }
    }

    pub fn get_suggested_actions(&self) -> Vec<String> {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => p,
            None => return vec!["Explore local neighborhood.".to_string()],
        };

        let age = (self.time.year - player.identity.birth_year) as u32;

        let mut suggestions = Vec::new();

        // 1. Collect choices from active situations
        for sit in &self.active_situations {
            for choice in &sit.choices {
                suggestions.push(choice.label.clone());
            }
        }

        // 2. Add contextual interest actions if player has established interests
        if player.interests.contains("music") {
            suggestions.push("Practice singing and songwriting in your room.".to_string());
        }
        if player.interests.contains("politics") {
            suggestions.push("Attend a local political constituency debate and policy discussion.".to_string());
        }
        if player.interests.contains("writing") {
            suggestions.push("Write and draft an essay on local community developments.".to_string());
        }

        if suggestions.is_empty() {
            if age <= 3 {
                suggestions.push("Observe your surroundings at home.".to_string());
                suggestions.push("Rest and sleep.".to_string());
            } else if age <= 12 {
                suggestions.push("Focus on school lessons and reading.".to_string());
                suggestions.push("Play outdoors with friends.".to_string());
            } else if age <= 18 {
                suggestions.push("Study for upcoming exams.".to_string());
                suggestions.push("Spend time on personal interests.".to_string());
            } else {
                suggestions.push("Advance your career projects.".to_string());
                suggestions.push("Manage personal finances and home.".to_string());
            }
        }

        suggestions
    }

    pub fn get_sidebar_state(&self) -> SidebarStateDTO {
        let player_id = "person:sim:player".to_string();
        let player = match self.persons.get(&player_id) {
            Some(p) => p,
            None => return SidebarStateDTO {
                commitments: Vec::new(),
                household_trust: 0.8,
                household_resentment: 0.0,
                active_interest: "General Life".to_string(),
                primary_skill_name: "General".to_string(),
                primary_skill_value: 50.0,
                life_stage: "Infancy".to_string(),
                marital_status: "Single".to_string(),
                job_title: "None".to_string(),
                monthly_salary: 0.0,
                fitness: 70.0,
                stress: 20.0,
                public_reputation: 0.2,
                channel_subscribers: 0,
                active_crises_count: 0,
                life_pivots_count: 0,
            },
        };

        let age = (self.time.year - player.identity.birth_year) as u32;
        let stage = LifeStage::from_age(age, player.is_alive);

        let parent_trust = player.parent_ids.first()
            .map(|pid| self.relationships.get_link(pid, &player_id).trust)
            .unwrap_or(0.85);

        let parent_resentment = player.parent_ids.first()
            .map(|pid| self.relationships.get_link(pid, &player_id).resentment)
            .unwrap_or(0.0);

        let mut commitments = Vec::new();
        if let Some(ref title) = player.employment.job_title {
            if player.employment.monthly_salary > 0.0 {
                commitments.push(CommitmentDTO {
                    title: format!("Work Schedule: {}", title),
                    description: "Active employment obligations.".to_string(),
                    urgency: "MEDIUM".to_string(),
                });
            }
        }

        if age >= 5 && age <= 18 {
            commitments.push(CommitmentDTO {
                title: "School Term Attendance".to_string(),
                description: "Daily classes and academic curriculum.".to_string(),
                urgency: "MEDIUM".to_string(),
            });
        }

        for proc in &self.active_processes {
            if proc.is_active {
                commitments.push(CommitmentDTO {
                    title: proc.title.clone(),
                    description: format!("Step {} of {}", proc.current_step + 1, proc.total_steps),
                    urgency: "HIGH".to_string(),
                });
            }
        }

        if commitments.is_empty() {
            commitments.push(CommitmentDTO {
                title: "Daily Life".to_string(),
                description: "Personal routine and growth.".to_string(),
                urgency: "LOW".to_string(),
            });
        }

        let active_interest = player
            .interests
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "General Life".to_string());

        let (top_skill_name, top_skill_val) = player
            .skills
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, v)| (k.clone(), *v))
            .unwrap_or_else(|| ("communication".to_string(), 40.0));

        let subs = self.creator_channel.as_ref().map(|c| c.subscriber_count).unwrap_or(0);

        SidebarStateDTO {
            commitments,
            household_trust: parent_trust,
            household_resentment: parent_resentment,
            active_interest,
            primary_skill_name: top_skill_name,
            primary_skill_value: top_skill_val,
            life_stage: format!("{:?}", stage),
            marital_status: player.romance.marital_status.clone(),
            job_title: player.employment.job_title.clone().unwrap_or_else(|| "Unemployed".to_string()),
            monthly_salary: player.employment.monthly_salary,
            fitness: player.health.fitness,
            stress: player.health.stress,
            public_reputation: self.reputation.public_standing,
            channel_subscribers: subs,
            active_crises_count: self.active_crises.len() as u32,
            life_pivots_count: self.life_pivots.len() as u32,
        }
    }

    pub fn get_biography(&self) -> String {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => format!("{} {}", p.identity.first_name, p.identity.last_name),
            None => "Player".to_string(),
        };
        otherlife_ai_bridge::BiographyWriter::generate_lifetime_biography(&player, &self.events)
    }

    pub fn generate_today_scene(&mut self) -> TodayScene {
        let current_total_days = self.time.total_days();
        self.update_deadlines_and_education(0);

        let player = match self.persons.get("person:sim:player") {
            Some(p) => p.clone(),
            None => {
                return TodayScene {
                    greeting: "No active life.".to_string(),
                    date_formatted: self.time.literary_date(),
                    location_formatted: "Unknown".to_string(),
                    age: 0,
                    headline: "Simulation Idle".to_string(),
                    narrative: "No active character exists in this timeline.".to_string(),
                    circumstances: Vec::new(),
                    choices: Vec::new(),
                    pending_deadlines: Vec::new(),
                    life_stage: "Unknown".to_string(),
                };
            }
        };

        let age = (self.time.year - player.identity.birth_year) as u32;
        let loc_clean = player.location_id.replace("city:real:", "").replace("city:sim:", "").replace('_', " ");
        let loc_title = loc_clean.chars().enumerate().map(|(i, c)| if i == 0 || loc_clean.chars().nth(i-1) == Some(' ') { c.to_ascii_uppercase() } else { c }).collect::<String>();
        let country_id = player.identity.nationalities.first().cloned().unwrap_or_else(|| "country:real:united_kingdom".to_string());
        let country_clean = country_id.replace("country:real:", "").replace('_', " ");
        let country_title = country_clean.chars().enumerate().map(|(i, c)| if i == 0 || country_clean.chars().nth(i-1) == Some(' ') { c.to_ascii_uppercase() } else { c }).collect::<String>();
        let full_location = format!("{}, {}", loc_title, country_title);

        let parent_name = player.parent_ids.first()
            .and_then(|pid| self.persons.get(pid))
            .map(|p| p.identity.first_name.clone())
            .unwrap_or_else(|| "Your family".to_string());

        let mut circumstances = Vec::new();
        let mut choices = Vec::new();
        let mut narrative_lines = Vec::new();
        let headline: String;

        let stage_str = if age <= 3 { "Infancy" }
            else if age <= 12 { "Childhood" }
            else if age <= 18 { "Adolescence" }
            else if age <= 29 { "Early Adulthood" }
            else if age <= 64 { "Adulthood" }
            else { "Senior Years" };

        if !player.is_alive {
            return TodayScene {
                greeting: "Journey's End".to_string(),
                date_formatted: self.time.literary_date(),
                location_formatted: full_location,
                age,
                headline: format!("Reflecting on the Life of {} {}", player.identity.first_name, player.identity.last_name),
                narrative: format!("The story of {} has drawn to its quiet close. Memories of your journey remain in the chronicle.", player.identity.first_name),
                circumstances: vec!["This lifetime has concluded.".to_string()],
                choices: vec![TodayChoice {
                    id: "view_chronicle".to_string(),
                    label: "Review the full chronicle of your journey".to_string(),
                    consequence_hint: None,
                    category: "ROUTINE".to_string(),
                    remaining_days: None,
                }],
                pending_deadlines: Vec::new(),
                life_stage: "Passed Away".to_string(),
            };
        }

        match age {
            0..=3 => {
                headline = format!("Morning at Home in {}", loc_title);
                if age == 0 {
                    narrative_lines.push(format!("You wake to soft light filtering into your nursery. {} is humming gently while preparing your morning feed.", parent_name));
                    narrative_lines.push("The familiar warmth of your home provides comfort as another peaceful day begins.".to_string());
                    circumstances.push("Family household care & early infancy".to_string());
                    choices.push(TodayChoice { id: "sleep_calmly".to_string(), label: "Rest comfortably in your crib".to_string(), consequence_hint: Some("Promotes health and growth".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "reach_out".to_string(), label: format!("Reach toward {} and smile", parent_name), consequence_hint: Some("Strengthens family bond".to_string()), category: "FAMILY".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "listen_sounds".to_string(), label: "Listen to the sounds of morning outside".to_string(), consequence_hint: Some("Early sensory awareness".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                } else {
                    narrative_lines.push(format!("You are up early, exploring the living room in {}. Toys and household books are scattered nearby.", loc_title));
                    circumstances.push("Toddler exploration & home routine".to_string());
                    choices.push(TodayChoice { id: "play_building".to_string(), label: "Stack colourful wooden building blocks".to_string(), consequence_hint: Some("Builds focus and spatial curiosity".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "learn_words".to_string(), label: format!("Ask {} to read a picture storybook", parent_name), consequence_hint: Some("Language development".to_string()), category: "FAMILY".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "garden_walk".to_string(), label: "Walk around the courtyard watching birds".to_string(), consequence_hint: Some("Physical motor coordination".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                }
            }
            4..=12 => {
                headline = format!("School Day in {}", loc_title);
                narrative_lines.push(format!("Morning bells ring across {} as you prepare for primary school. Your classmates are gathering in the courtyard.", loc_title));
                circumstances.push(format!("Enrolled in Primary School (Academic Performance: {:.0}%)", player.education.academic_performance));
                
                choices.push(TodayChoice { id: "attend_school".to_string(), label: "Attend classes and participate actively".to_string(), consequence_hint: Some("Strengthens academic understanding".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "play_friends".to_string(), label: "Play football and games with friends at break".to_string(), consequence_hint: Some("Builds athleticism and peer friendships".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "study_library".to_string(), label: "Spend the afternoon reading library books".to_string(), consequence_hint: Some("Expands reading comprehension".to_string()), category: "ROUTINE".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "family_evening".to_string(), label: format!("Help {} with evening household chores", parent_name), consequence_hint: Some("Fosters family trust and responsibility".to_string()), category: "FAMILY".to_string(), remaining_days: None });
            }
            13..=18 => {
                headline = format!("Youth & Aspirations in {}", loc_title);
                narrative_lines.push(format!("You wake up early in {}. Term coursework is underway, and thoughts about your future path are becoming more frequent.", loc_title));
                circumstances.push(format!("Secondary School Classes (Academic Performance: {:.0}%)", player.education.academic_performance));

                // Active Crisis
                if let Some(crisis) = self.active_crises.first() {
                    narrative_lines.push(format!("URGENT CHALLENGE: {}", crisis.description));
                    if crisis.crisis_type == "CREATIVE_BURNOUT" {
                        choices.push(TodayChoice { id: "handle_burnout_break".to_string(), label: "Take a 2-month mental health sabbatical".to_string(), consequence_hint: Some("Recovers stress and creative passion".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                        choices.push(TodayChoice { id: "handle_burnout_pivot_production".to_string(), label: "Hire editors and transition into a production studio".to_string(), consequence_hint: Some("Pivots to managing director role".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                    }
                }

                // Digital Creator Pathway
                if let Some(ref ch) = self.creator_channel {
                    circumstances.push(format!("Channel: {} ({} subs · ₦{:.0}/mo)", ch.channel_handle, ch.subscriber_count, ch.monthly_ad_revenue));
                    choices.push(TodayChoice { id: "produce_creator_video".to_string(), label: "Script, record and edit new video episode".to_string(), consequence_hint: Some("Expands subscriber base & ad revenue (+burnout)".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                } else if player.skills.get("creativity").copied().unwrap_or(0.0) >= 15.0 || player.identity.first_name.len() > 0 {
                    choices.push(TodayChoice { id: "launch_creator_channel".to_string(), label: "Launch independent digital video channel".to_string(), consequence_hint: Some("Begins creating digital media for online audience".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                }

                // Football Pathway
                if let Some(ref contract) = player.football_contract {
                    circumstances.push(format!("Signed: {} (Weekly Wage: £{:.0})", contract.club_name, contract.weekly_wage));
                    choices.push(TodayChoice { id: "start_saturday_match".to_string(), label: "Start in Saturday's competitive academy fixture".to_string(), consequence_hint: Some("Competitive match sharpness & scout evaluation".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "football_pivot_coaching".to_string(), label: "Enroll in Youth Coaching & Talent Scouting License".to_string(), consequence_hint: Some("Transitions towards academy management staff".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                }

                for dl in &self.active_deadlines {
                    let rem = (dl.deadline_day_total - current_total_days).max(0);
                    narrative_lines.push(format!("Active Matter: {} has {} days remaining.", dl.title, rem));
                    choices.push(TodayChoice {
                        id: format!("action_{}", dl.id),
                        label: format!("{} ({}d remaining)", dl.title, rem),
                        consequence_hint: Some("Takes action before the deadline closes".to_string()),
                        category: "OPPORTUNITY".to_string(),
                        remaining_days: Some(rem),
                    });
                }

                choices.push(TodayChoice { id: "school_revision".to_string(), label: "Dedicate hours to exam revision and past papers".to_string(), consequence_hint: Some("Raises academic standing".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "train_football".to_string(), label: "Head to the pitch for intensive football drills".to_string(), consequence_hint: Some("Sharpens match fitness and technical control".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "civic_debate".to_string(), label: "Attend the youth student assembly and debate".to_string(), consequence_hint: Some("Builds public speaking and confidence".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                choices.push(TodayChoice { id: "family_future".to_string(), label: format!("Talk with {} about your ambitions after graduation", parent_name), consequence_hint: Some("Deepens parental understanding".to_string()), category: "FAMILY".to_string(), remaining_days: None });

                if age >= 17 && self.academic_program.is_none() {
                    choices.push(TodayChoice {
                        id: "apply_university_science".to_string(),
                        label: "Apply for University Degree in Computer Science (4-Year B.Sc.)".to_string(),
                        consequence_hint: Some("Begins 4-year higher education journey".to_string()),
                        category: "OPPORTUNITY".to_string(),
                        remaining_days: None,
                    });
                    choices.push(TodayChoice {
                        id: "apply_university_law".to_string(),
                        label: "Apply for University Degree in Law & Jurisprudence (5-Year LL.B.)".to_string(),
                        consequence_hint: Some("Begins legal qualification pathway".to_string()),
                        category: "OPPORTUNITY".to_string(),
                        remaining_days: None,
                    });
                }
            }
            _ => {
                // Crisis in Adulthood
                if let Some(crisis) = self.active_crises.first() {
                    narrative_lines.push(format!("CRITICAL SITUATION: {}", crisis.description));
                    if crisis.crisis_type == "CREATIVE_BURNOUT" {
                        choices.push(TodayChoice { id: "handle_burnout_break".to_string(), label: "Take a 2-month mental health sabbatical".to_string(), consequence_hint: Some("Recovers stress and creative passion".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                        choices.push(TodayChoice { id: "handle_burnout_pivot_production".to_string(), label: "Hire editors and transition into a production studio".to_string(), consequence_hint: Some("Pivots to managing director role".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                    }
                }

                if let Some(ref prog) = self.academic_program {
                    if !prog.is_graduated {
                        headline = format!("University Life · Year {}, Semester {} at {}", prog.current_year, prog.current_semester, prog.university_name);
                        narrative_lines.push(format!("Campus life is bustling at {}. You are currently in Year {}, Semester {} of your {} program in {}.", prog.university_name, prog.current_year, prog.current_semester, prog.degree_title, prog.faculty));
                        circumstances.push(format!("Enrolled: {} ({}) · Year {}/{}", prog.degree_title, prog.university_name, prog.current_year, prog.total_years));
                        choices.push(TodayChoice { id: "attend_lectures".to_string(), label: "Attend university lectures and tutorials".to_string(), consequence_hint: Some("Progresses semester coursework".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                        choices.push(TodayChoice { id: "study_library_uni".to_string(), label: "Spend the evening studying research papers".to_string(), consequence_hint: Some("Boosts GPA and academic mastery".to_string()), category: "ROUTINE".to_string(), remaining_days: None });
                        choices.push(TodayChoice { id: "campus_social".to_string(), label: "Join campus student association activities".to_string(), consequence_hint: Some("Expands networking and social ties".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                    } else {
                        headline = format!("Professional Life in {}", loc_title);
                        narrative_lines.push(format!("As a graduate with a {} in {}, you navigate career opportunities in {}.", prog.degree_title, prog.faculty, loc_title));
                    }
                } else if let Some(ref job) = player.employment.job_title {
                    headline = format!("Workday in {}", loc_title);
                    narrative_lines.push(format!("You head into work as a {}. Projects and workplace expectations require your attention today.", job));
                    let curr_sym = if country_id.contains("nigeria") { "₦" } else { "£" };
                    circumstances.push(format!("Employed: {} (Monthly Salary: {}{:.0})", job, curr_sym, player.employment.monthly_salary));
                    choices.push(TodayChoice { id: "work_shift".to_string(), label: format!("Complete your work responsibilities as {}", job), consequence_hint: Some("Advances job standing and career reputation".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "career_network".to_string(), label: "Network with professional colleagues after hours".to_string(), consequence_hint: Some("Builds industry connections".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                } else {
                    headline = format!("Life & Opportunity in {}", loc_title);
                    narrative_lines.push(format!("You are currently seeking employment or new directions in {}. The city offers varied paths to explore.", loc_title));
                    circumstances.push("Seeking new career direction".to_string());
                    choices.push(TodayChoice { id: "apply_entry_job".to_string(), label: "Apply for junior professional openings in the city".to_string(), consequence_hint: Some("Initiates employment opportunities".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "launch_startup_venture".to_string(), label: "Found and incorporate an independent consultancy".to_string(), consequence_hint: Some("Embarks on commercial entrepreneurship".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "organize_community_townhall".to_string(), label: "Organize a public community town hall forum".to_string(), consequence_hint: Some("Establishes civic leadership and public standing".to_string()), category: "PERSONAL".to_string(), remaining_days: None });
                }

                if let Some(ref ch) = self.creator_channel {
                    circumstances.push(format!("Channel: {} ({} subs · ₦{:.0}/mo)", ch.channel_handle, ch.subscriber_count, ch.monthly_ad_revenue));
                    choices.push(TodayChoice { id: "produce_creator_video".to_string(), label: "Script, record and edit new video episode".to_string(), consequence_hint: Some("Expands subscriber base & ad revenue (+burnout)".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                }

                if let Some(ref contract) = player.football_contract {
                    circumstances.push(format!("Club: {} (Weekly Wage: £{:.0})", contract.club_name, contract.weekly_wage));
                    choices.push(TodayChoice { id: "start_saturday_match".to_string(), label: "Start in weekend competitive fixture".to_string(), consequence_hint: Some("Match sharpness & tactical execution".to_string()), category: "IMMEDIATE".to_string(), remaining_days: None });
                    choices.push(TodayChoice { id: "football_pivot_coaching".to_string(), label: "Enroll in Coaching License & Scouting Staff".to_string(), consequence_hint: Some("Transitions towards coaching & management".to_string()), category: "OPPORTUNITY".to_string(), remaining_days: None });
                }

                for dl in &self.active_deadlines {
                    let rem = (dl.deadline_day_total - current_total_days).max(0);
                    narrative_lines.push(format!("Active Matter: {} ({} days remaining).", dl.title, rem));
                    choices.push(TodayChoice {
                        id: format!("action_{}", dl.id),
                        label: format!("{} ({}d remaining)", dl.title, rem),
                        consequence_hint: Some("Addresses this urgent deadline".to_string()),
                        category: "OPPORTUNITY".to_string(),
                        remaining_days: Some(rem),
                    });
                }
            }
        }

        // Standard time progression choices
        choices.push(TodayChoice { id: "pass_day".to_string(), label: "Let a quiet day pass in routine (+1 Day)".to_string(), consequence_hint: Some("Time moves forward 1 day".to_string()), category: "ROUTINE".to_string(), remaining_days: None });
        choices.push(TodayChoice { id: "pass_week".to_string(), label: "Let a week pass steadily (+1 Week)".to_string(), consequence_hint: Some("Time moves forward 7 days".to_string()), category: "ROUTINE".to_string(), remaining_days: None });
        choices.push(TodayChoice { id: "pass_month".to_string(), label: "Advance through the month (+1 Month)".to_string(), consequence_hint: Some("Time moves forward 30 days".to_string()), category: "ROUTINE".to_string(), remaining_days: None });

        let narrative = narrative_lines.join(" ");

        TodayScene {
            greeting: format!("OTHERLIFE · Age {}", age),
            date_formatted: self.time.literary_date(),
            location_formatted: full_location,
            age,
            headline,
            narrative,
            circumstances,
            choices,
            pending_deadlines: self.active_deadlines.clone(),
            life_stage: stage_str.to_string(),
        }
    }

    pub fn execute_player_action(&mut self, action_payload: ActionPayload) -> StepResult {
        let player_id = "person:sim:player".to_string();
        let mum_id = "person:sim:mum".to_string();

        let player = self.persons.get(&player_id).unwrap();
        let validation = ActionValidator::validate(player, &action_payload);
        if !validation.is_valid {
            return StepResult {
                success: false,
                narrative: format!("Action failed validation: {}", validation.reason.unwrap_or_default()),
                causality_note: "Validation check failed.".to_string(),
                event_record: EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "FAILED_ACTION".to_string(),
                    actor_id: player_id,
                    target_id: None,
                    summary: "Attempted invalid action.".to_string(),
                    metadata: serde_json::json!({}),
                    causality_parent_id: None,
                },
            };
        }

        let mut success = true;
        let causality_note: String;

        match action_payload.action {
            ActionPrimitive::ApplyJob => {
                causality_note = "Submitted job application. Hired as Associate. (+£1,800/mo salary)".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.employment.job_title = Some("Staff Associate".to_string());
                    p.employment.employer_org_id = Some("org:sim:company".to_string());
                    p.employment.monthly_salary = 1800.0;
                    p.employment.job_performance = 60.0;
                }
            }
            ActionPrimitive::WorkShift => {
                let salary = self.persons.get(&player_id).map(|p| p.employment.monthly_salary).unwrap_or(0.0);
                let shift_earnings = if salary > 0.0 { salary / 15.0 } else { 80.0 };
                causality_note = format!("Completed work shift. Earned £{:.2} and boosted job performance (+2.0).", shift_earnings);
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.finances.cash += shift_earnings;
                    p.employment.job_performance = (p.employment.job_performance + 2.0).min(100.0);
                }
            }
            ActionPrimitive::RentApartment => {
                causality_note = "Signed lease for city apartment (£550/mo rent). Moved out of family home.".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.housing.housing_type = "Renting".to_string();
                    p.housing.monthly_cost = 550.0;
                    p.finances.cash -= 550.0;
                }
            }
            ActionPrimitive::BuyProperty => {
                let cash = self.persons.get(&player_id).map(|p| p.finances.cash).unwrap_or(0.0);
                if cash >= 15000.0 {
                    causality_note = "Purchased residential property outright (£15,000). Housing secure.".to_string();
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.finances.cash -= 15000.0;
                        p.housing.housing_type = "Ownership".to_string();
                        p.housing.monthly_cost = 0.0;
                    }
                } else {
                    success = false;
                    causality_note = "Property purchase failed due to insufficient funds.".to_string();
                }
            }
            ActionPrimitive::Date => {
                causality_note = "Went on a date. Met a compatible partner and began dating.".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.romance.marital_status = "Dating".to_string();
                    p.romance.partner_id = Some("person:sim:partner".to_string());
                    p.romance.relationship_satisfaction = 75.0;
                }
            }
            ActionPrimitive::Marry => {
                causality_note = "Held marriage ceremony. Officially married partner!".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.romance.marital_status = "Married".to_string();
                    p.romance.relationship_satisfaction = 90.0;
                }
            }
            ActionPrimitive::Divorce => {
                causality_note = "Finalized divorce proceedings. Returned to Single status.".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.romance.marital_status = "Divorced".to_string();
                    p.romance.partner_id = None;
                    p.romance.relationship_satisfaction = 20.0;
                }
            }
            ActionPrimitive::HaveChild => {
                let child_id = format!("person:sim:child_{}", self.rng.gen_range_u32(1000, 9999));
                causality_note = format!("Welcomed a newborn child ({}) into the family!", child_id);
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.child_ids.push(child_id.clone());
                }
            }
            ActionPrimitive::SeekMedicalTreatment => {
                causality_note = "Received medical treatment. Stress reduced (-25.0) and fitness restored (+10.0).".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.health.stress = (p.health.stress - 25.0).max(0.0);
                    p.health.fitness = (p.health.fitness + 10.0).min(100.0);
                    p.finances.cash -= 40.0;
                }
            }
            ActionPrimitive::Deceive => {
                let persuasion_roll = self.rng.gen_range_f32(0.3, 0.9);
                let mum_trust = self.relationships.get_link(&mum_id, &player_id).trust;

                if persuasion_roll + mum_trust > 0.8 {
                    success = true;
                    causality_note = format!(
                        "Deception succeeded based on trust baseline ({:.0}%). Practice session completed successfully.",
                        mum_trust * 100.0
                    );

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("football_control".to_string()).or_insert(50.0);
                        *entry += 2.5;
                        p.football_attributes.ball_control = (p.football_attributes.ball_control + 2.5).min(99.0);
                    }
                    self.relationships.modify_link(mum_id.clone(), player_id.clone(), |rel| {
                        rel.trust -= 0.05;
                        rel.resentment += 0.04;
                    });
                } else {
                    success = false;
                    causality_note = "Claim was questioned due to recent school performance history.".to_string();
                    self.relationships.modify_link(mum_id.clone(), player_id.clone(), |rel| {
                        rel.trust -= 0.15;
                        rel.resentment += 0.12;
                    });
                }
            }
            ActionPrimitive::AttendActivity => {
                causality_note = "Attended training session. Improved ball control (+3.0) and pace (+1.5).".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    let entry = p.skills.entry("football_control".to_string()).or_insert(50.0);
                    *entry += 3.0;
                    p.football_attributes.ball_control = (p.football_attributes.ball_control + 3.0).min(99.0);
                    p.football_attributes.pace = (p.football_attributes.pace + 1.5).min(99.0);
                }
            }
            ActionPrimitive::Study => {
                causality_note = "Spent 2 hours studying and reviewing concepts. Academic performance improved.".to_string();
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.education.academic_performance += 4.0;
                }
            }
            _ => {
                causality_note = "General action executed.".to_string();
            }
        }

        let narrative = self.ai_bridge.render_narrative(
            &action_payload.action,
            success,
            self.persons.get(&player_id).unwrap(),
            Some("Mum"),
            &causality_note,
        );

        let event_record = EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: self.time.formatted(),
            event_type: format!("{:?}", action_payload.action),
            actor_id: player_id.clone(),
            target_id: action_payload.target_id,
            summary: narrative.clone(),
            metadata: serde_json::json!({
                "success": success,
                "causality_note": causality_note,
            }),
            causality_parent_id: None,
        };

        self.events.push(event_record.clone());
        self.time.advance_days(2);

        self.tick_npc_simulation();

        if let Some(p) = self.persons.get_mut(&player_id) {
            let age = (self.time.year - p.identity.birth_year) as u32;
            if age >= 85 {
                p.is_alive = false;
                let death_ev = EventRecord {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: self.time.formatted(),
                    event_type: "DEATH".to_string(),
                    actor_id: player_id,
                    target_id: None,
                    summary: format!("{} passed away peacefully of old age at age {}.", p.identity.first_name, age),
                    metadata: serde_json::json!({ "cause": "old_age" }),
                    causality_parent_id: None,
                };
                self.events.push(death_ev);
            }
        }

        StepResult {
            success,
            narrative,
            causality_note,
            event_record,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_slice_engine_deception_turn() {
        let mut engine = SimulationEngine::new_vertical_slice_fixture(42);
        let payload = engine.ai_bridge.parse_intent(
            "Tell Mum I'm going to James's house to study math, but secretly go to training.",
            "person:sim:player",
            Some("person:sim:mum"),
        );

        let res = engine.execute_player_action(payload);
        assert!(res.narrative.contains("lied") || res.narrative.contains("convincingly") || res.narrative.contains("failed"));
        assert_eq!(engine.events.len(), 1);
        assert_eq!(engine.time.day, 14);
    }
}

use otherlife_actions::{ActionPayload, ActionPrimitive, ActionValidator};
use otherlife_ai_bridge::{AIBridge, AIBridgeConfig};
use otherlife_relationships::{RelationshipMatrix, RelationshipVector};
use otherlife_rng::WorldRng;
use otherlife_world::{
    AcademicDegree, ActivityType, BeliefComponent, BusinessEntity, CanonicalEntity, CovertOperation, CosmicLegacy,
    CosmicMegastructure, CreativeRelease, CriminalRecord, CyberneticImplant, DigitalPost, EducationComponent,
    EmploymentComponent, EntityNamespace, EntityType, EnvironmentalRating, EventRecord, FaithMovement, FameComponent,
    FinancesComponent, FootballContract, FootballMatch, FootballPlayerAttributes, FootballRole, FootballScoutReport,
    GeopoliticalConflict, HealthComponent, HousingComponent, IdentityComponent, KnowledgeRecord, LegalStatus,
    LifeStage, MacroEconomy, MedicalRecord, MilitaryRecord, MindUpload, NaturalDisaster, NewLifeConfig,
    NpcSchedule, NpcTier, OrganizationSubunit, Passport, Patent, Person, PersonalityComponent, Place, PolicyProposal,
    PoliticalCampaign, PostScarcityEconomy, PrisonSentence, Qualification, ResearchProject, ResolutionContext,
    ResolutionResult, RomanceComponent, SecretMembership, SecretSociety, SimTime, SocialMediaAccount, SpaceAgency,
    SpaceMission, SurgicalProcedure, TravelRecord, Visa, WeatherEvent, WillAndTestament, WorldEntityResolver,
    WorldNewsItem,
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
}

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
    pub events: Vec<EventRecord>,
    pub world_news: Vec<WorldNewsItem>,
    pub ai_bridge: AIBridge,
}

impl SimulationEngine {
    pub fn new_game(config: NewLifeConfig, seed: u64) -> Self {
        let mut rng = WorldRng::new(seed);
        let time = SimTime::new(config.starting_year, 10, 12, 09, 00);

        let player_id = "person:sim:player".to_string();
        let mum_id = "person:sim:mum".to_string();

        let birth_year = config.starting_year - (config.starting_age as i32);
        let first_name = config.first_name.unwrap_or_else(|| "Alex".to_string());
        let last_name = config.last_name.unwrap_or_else(|| "Morgan".to_string());
        let sex = config.sex.unwrap_or_else(|| "Non-binary".to_string());
        let income_tier = config.household_income_tier.unwrap_or_else(|| "MIDDLE".to_string());

        let starting_cash = match income_tier.as_str() {
            "HIGH" => 2500.0,
            "LOW" => 15.0,
            _ => 150.0,
        };

        let mut skills = config.skills;
        if skills.is_empty() {
            skills.insert("communication".to_string(), 45.0);
            skills.insert("reading".to_string(), 50.0);
        }

        let mut interests_set = HashSet::new();
        for int_str in &config.interests {
            interests_set.insert(int_str.clone());
        }

        let is_footballer = interests_set.contains("football");
        let football_role = if is_footballer {
            if config.starting_age < 18 {
                FootballRole::AcademyProspect
            } else {
                FootballRole::Player
            }
        } else {
            FootballRole::None
        };

        let football_contract = if is_footballer && config.starting_age >= 16 {
            Some(FootballContract {
                club_id: "club:real:celtic".to_string(),
                club_name: "Celtic FC".to_string(),
                weekly_wage: 450.0,
                years_remaining: 3,
                release_clause: 500000.0,
                goal_bonus: 250.0,
                agent_id: None,
            })
        } else {
            None
        };

        let housing = if config.starting_age >= 18 {
            HousingComponent {
                housing_type: "Renting".to_string(),
                monthly_cost: 650.0,
                quality: 75.0,
            }
        } else {
            HousingComponent::default()
        };

        let employment = if config.starting_age >= 22 {
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
                country_id: config.country_id.clone(),
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
                school_id: Some("school:sim:local_school".to_string()),
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
                monthly_allowance: 25.0,
                household_income_tier: income_tier.clone(),
                monthly_expenses: 50.0,
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
            parent_ids: vec![mum_id.clone()],
            child_ids: Vec::new(),
            active_roles: Vec::new(),
            knowledge: HashSet::new(),
            secrets: Vec::new(),
            memories: Vec::new(),
        };

        let mum = Person {
            id: mum_id.clone(),
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
                first_name: "Eleanor".to_string(),
                last_name: last_name.clone(),
                birth_year: birth_year - 28,
                birth_month: 8,
                birth_day: 24,
                sex: "Female".to_string(),
                country_id: config.country_id.clone(),
            },
            personality: PersonalityComponent::default(),
            skills: HashMap::new(),
            interests: HashSet::new(),
            goals: Vec::new(),
            education: EducationComponent {
                school_id: None,
                grade_level: 0,
                academic_performance: 80.0,
                attendance_rate: 100.0,
                qualifications: Vec::new(),
                degree_program: None,
            },
            employment: EmploymentComponent {
                job_title: Some("Senior Administrator".to_string()),
                employer_org_id: Some("org:sim:city_admin".to_string()),
                monthly_salary: 3200.0,
                job_performance: 75.0,
                years_in_role: 6,
            },
            housing: HousingComponent::default(),
            health: HealthComponent::default(),
            romance: RomanceComponent {
                marital_status: "Married".to_string(),
                partner_id: None,
                relationship_satisfaction: 70.0,
            },
            finances: FinancesComponent {
                cash: starting_cash * 10.0,
                monthly_allowance: 0.0,
                household_income_tier: income_tier,
                monthly_expenses: 200.0,
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
            location_id: config.location_id,
            parent_ids: Vec::new(),
            child_ids: vec![player_id.clone()],
            active_roles: vec!["Parent".to_string()],
            knowledge: HashSet::new(),
            secrets: vec![KnowledgeRecord {
                topic_id: "secret:family_inheritance".to_string(),
                description: "Family estate trust fund savings.".to_string(),
                certainty: 1.0,
                is_secret: true,
                known_by_ids: vec![mum_id.clone()].into_iter().collect(),
            }],
            memories: Vec::new(),
        };

        let mut persons = HashMap::new();
        persons.insert(player_id.clone(), player);
        persons.insert(mum_id.clone(), mum);

        let mut relationships = RelationshipMatrix::new();
        relationships.set_link(
            mum_id,
            player_id,
            RelationshipVector::new_parent_child(),
        );

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
            events: Vec::new(),
            world_news: Vec::new(),
            ai_bridge,
        };

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
        player.identity.country_id = dest_country_id.to_string();

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

    pub fn get_suggested_actions(&self) -> Vec<String> {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => p,
            None => return vec!["Explore local neighborhood.".to_string()],
        };

        let mut suggestions = Vec::new();
        let age = self.time.year - player.identity.birth_year;

        if player.interests.contains("politics") {
            suggestions.push("Attend a local political constituency debate.".to_string());
        }
        if player.interests.contains("music") {
            suggestions.push("Practice singing and songwriting in your bedroom.".to_string());
        }
        if player.interests.contains("football") {
            suggestions.push("Tell Mum I'm going to James's house to study math, but secretly go to football training.".to_string());
        }

        if age >= 16 && player.employment.job_title.is_none() {
            suggestions.push("Apply for a part-time job vacancy.".to_string());
        } else if player.employment.job_title.is_some() {
            suggestions.push("Work a shift to earn salary and build career experience.".to_string());
        }

        if age >= 18 && player.housing.housing_type == "FamilyHome" {
            suggestions.push("Search for an apartment to rent.".to_string());
        }

        if player.romance.marital_status == "Single" && age >= 16 {
            suggestions.push("Go on a date to meet new romantic partners.".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Explore the city and talk to people.".to_string());
        }

        suggestions.truncate(3);
        suggestions
    }

    pub fn get_sidebar_state(&self) -> SidebarStateDTO {
        let player_id = "person:sim:player".to_string();
        let mum_id = "person:sim:mum".to_string();

        let player = self.persons.get(&player_id).unwrap();
        let mum_rel = self.relationships.get_link(&mum_id, &player_id);

        let age = (self.time.year - player.identity.birth_year) as u32;
        let stage = LifeStage::from_age(age, player.is_alive);

        let mut commitments = Vec::new();
        if let Some(ref title) = player.employment.job_title {
            commitments.push(CommitmentDTO {
                title: format!("Work Shift: {}", title),
                description: "Upcoming scheduled work shift.".to_string(),
                urgency: "HIGH".to_string(),
            });
        } else if player.interests.contains("football") {
            commitments.push(CommitmentDTO {
                title: "Saturday Youth Match".to_string(),
                description: "Regional scout attending youth match.".to_string(),
                urgency: "HIGH".to_string(),
            });
        } else {
            commitments.push(CommitmentDTO {
                title: "Daily Life Schedule".to_string(),
                description: "Personal goals and routine commitments.".to_string(),
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
            .unwrap_or_else(|| ("communication".to_string(), 45.0));

        SidebarStateDTO {
            commitments,
            household_trust: mum_rel.trust,
            household_resentment: mum_rel.resentment,
            active_interest,
            primary_skill_name: top_skill_name,
            primary_skill_value: top_skill_val,
            life_stage: format!("{:?}", stage),
            marital_status: player.romance.marital_status.clone(),
            job_title: player.employment.job_title.clone().unwrap_or_else(|| "Unemployed / Student".to_string()),
            monthly_salary: player.employment.monthly_salary,
            fitness: player.health.fitness,
            stress: player.health.stress,
        }
    }

    pub fn get_biography(&self) -> String {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => format!("{} {}", p.identity.first_name, p.identity.last_name),
            None => "Player".to_string(),
        };
        otherlife_ai_bridge::BiographyWriter::generate_lifetime_biography(&player, &self.events)
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

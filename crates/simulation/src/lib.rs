use otherlife_ai_bridge::{AIBridge, AIBridgeConfig};
use otherlife_rng::WorldRng;
use otherlife_world::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEngine {
    pub time: TimeState,
    pub rng: WorldRng,
    pub persons: HashMap<String, HumanEntity>,
    pub npcs: HashMap<String, AutonomousNPC>,
    pub households: HashMap<String, HouseholdEntity>,
    pub places: HashMap<String, WorldPlace>,
    pub institutions: HashMap<String, InstitutionEntity>,
    pub accounts: HashMap<String, FinancialAccount>,
    pub active_processes: Vec<LifeProcess>,
    pub active_opportunities: Vec<OpportunityRecord>,
    pub letters_inbox: Vec<LetterNotification>,
    pub events_ledger: Vec<EventRecord>,
    pub rule_pack: RegionalRulePack,
    pub ai_bridge: AIBridge,
}

impl SimulationEngine {
    pub fn new_game(config: NewLifeConfig, seed: u64) -> Self {
        let mut rng = WorldRng::new(seed);

        let birth_year = config.birth_year.unwrap_or(2005);
        let birth_month = config.birth_month.unwrap_or(6).clamp(1, 12);
        let birth_day = config.birth_day.unwrap_or(14).clamp(1, 30);
        let start_age = config.starting_age;

        let current_year = birth_year + start_age as i32;
        let time = TimeState::new(current_year, birth_month, birth_day);

        // 1. Regional Rule Pack Resolution
        let rule_pack = Self::resolve_rule_pack(&config.location_id, &config.country_id);

        let first_name = config.first_name.unwrap_or_else(|| "Israel".to_string());
        let last_name = config.last_name.unwrap_or_else(|| "Adeyemi".to_string());
        let sex = config.sex.unwrap_or_else(|| "Male".to_string());
        let wealth = WealthTier::from_str(config.household_income_tier.as_deref().unwrap_or("MIDDLE"));

        let player_id = "person:sim:player".to_string();

        // 2. Player Entity Creation
        let mut initial_cash = 0.0; // Newborns start with 0 cash!
        if start_age >= 18 {
            initial_cash = match wealth {
                WealthTier::Poverty => 50.0,
                WealthTier::WorkingClass => 300.0,
                WealthTier::MiddleClass => 1200.0,
                WealthTier::UpperMiddle => 3500.0,
                WealthTier::Wealthy => 10000.0,
            };
        } else if start_age >= 13 {
            initial_cash = match wealth {
                WealthTier::Poverty => 5.0,
                WealthTier::WorkingClass => 20.0,
                WealthTier::MiddleClass => 60.0,
                WealthTier::UpperMiddle => 150.0,
                WealthTier::Wealthy => 400.0,
            };
        }

        let mut skills = HashMap::new();
        for (k, v) in config.skills {
            skills.insert(k, SkillMastery {
                level: v,
                experience: 0.0,
                natural_affinity: 1.0,
                last_practiced_day: time.total_days,
            });
        }

        let player_entity = HumanEntity {
            id: player_id.clone(),
            identity: IdentityProfile {
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                birth_year,
                birth_month,
                birth_day,
                sex: sex.clone(),
                birthplace_id: rule_pack.city_id.clone(),
                nationality: rule_pack.country_name.clone(),
                culture: rule_pack.region_name.clone(),
                primary_language: rule_pack.primary_language.clone(),
            },
            biology: BiologicalProfile {
                is_alive: true,
                death_year: None,
                death_reason: None,
                health_overall: 95.0,
                fitness: if start_age >= 13 { 55.0 } else { 30.0 },
                energy_level: 90.0,
                chronic_conditions: Vec::new(),
            },
            psychology: PsychologicalProfile {
                discipline: 0.50,
                curiosity: 0.70,
                creativity: 0.60,
                confidence: 0.55,
                risk_tolerance: 0.40,
                stress_level: 10.0,
                resilience: 0.60,
            },
            reputation: ReputationProfile::default(),
            skills,
            resources: HumanResources {
                cash: initial_cash,
                household_wealth_tier: wealth.clone(),
                living_arrangement: "FAMILY_HOME".to_string(),
                tools_available: if start_age >= 13 { vec!["BOOKS".to_string(), "FAMILY_DESKTOP".to_string()] } else { vec!["CRIB_TOYS".to_string()] },
            },
            relationships: HashMap::new(),
            occupation: None,
            is_player: true,
        };

        let mut persons = HashMap::new();
        persons.insert(player_id.clone(), player_entity);

        // 3. Parents and Immediate Family Setup
        let mother_name = config.mother_name.unwrap_or_else(|| {
            if rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Fiona".to_string()
            } else if rule_pack.city_id.contains("san_francisco") || rule_pack.city_id.contains("houston") {
                "Eleanor".to_string()
            } else {
                "Sarah".to_string()
            }
        });
        let mother_job = config.mother_job.unwrap_or_else(|| {
            if rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Staff Nurse (NHS)".to_string()
            } else if rule_pack.city_id.contains("san_francisco") {
                "Biotech Research Scientist".to_string()
            } else {
                "Healthcare Officer".to_string()
            }
        });

        let father_name = config.father_name.unwrap_or_else(|| {
            if rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Duncan".to_string()
            } else if rule_pack.city_id.contains("san_francisco") || rule_pack.city_id.contains("houston") {
                "Arthur".to_string()
            } else {
                "David".to_string()
            }
        });
        let father_job = config.father_job.unwrap_or_else(|| {
            if rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Civil Engineer".to_string()
            } else if rule_pack.city_id.contains("san_francisco") {
                "Software Architect".to_string()
            } else {
                "Senior Ministry Administrator".to_string()
            }
        });

        let mut npcs = HashMap::new();

        // Mother
        let mother_id = "person:sim:mother".to_string();
        npcs.insert(mother_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: mother_id.clone(),
                identity: IdentityProfile {
                    first_name: mother_name.clone(),
                    last_name: last_name.clone(),
                    birth_year: current_year - (28 + start_age as i32),
                    birth_month: 4,
                    birth_day: 12,
                    sex: "Female".to_string(),
                    birthplace_id: rule_pack.city_id.clone(),
                    nationality: rule_pack.country_name.clone(),
                    culture: rule_pack.region_name.clone(),
                    primary_language: rule_pack.primary_language.clone(),
                },
                biology: BiologicalProfile {
                    is_alive: true,
                    death_year: None,
                    death_reason: None,
                    health_overall: 90.0,
                    fitness: 60.0,
                    energy_level: 80.0,
                    chronic_conditions: Vec::new(),
                },
                psychology: PsychologicalProfile {
                    discipline: 0.80,
                    curiosity: 0.65,
                    creativity: 0.50,
                    confidence: 0.75,
                    risk_tolerance: 0.30,
                    stress_level: 25.0,
                    resilience: 0.85,
                },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources {
                    cash: 45000.0,
                    household_wealth_tier: wealth.clone(),
                    living_arrangement: "FAMILY_HOME".to_string(),
                    tools_available: vec!["VEHICLE".to_string(), "MOBILE_PHONE".to_string()],
                },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord {
                    title: mother_job.clone(),
                    employer_org_id: Some("org:sim:health_center".to_string()),
                    monthly_earnings: 180000.0,
                    start_year: current_year - 6,
                }),
                is_player: false,
            },
            primary_role: NpcRole::Parent,
            personality: PersonalityProfile {
                warmth: 0.95,
                patience: 0.85,
                strictness: 0.50,
                ambition: 0.60,
                risk_tolerance: 0.25,
                communication_style: CommunicationStyle::Nurturing,
                core_values: vec!["Family Integrity".to_string(), "Education".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 7, end_hour: 16, activity_name: format!("Hospital Ward Shift: {}", mother_job), location_id: "place:clinic".to_string() },
                DailyRoutineBlock { start_hour: 17, end_hour: 22, activity_name: "Family Home Evening Care".to_string(), location_id: "place:home".to_string() },
            ],
            life_goal: "Provide a prosperous, nurturing future for children".to_string(),
            subjective_memories_of_player: Vec::new(),
            monthly_income: 180000.0,
            stress_level: 20.0,
            last_active_day: time.total_days,
        });

        // Father
        let father_id = "person:sim:father".to_string();
        npcs.insert(father_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: father_id.clone(),
                identity: IdentityProfile {
                    first_name: father_name.clone(),
                    last_name: last_name.clone(),
                    birth_year: current_year - (30 + start_age as i32),
                    birth_month: 9,
                    birth_day: 22,
                    sex: "Male".to_string(),
                    birthplace_id: rule_pack.city_id.clone(),
                    nationality: rule_pack.country_name.clone(),
                    culture: rule_pack.region_name.clone(),
                    primary_language: rule_pack.primary_language.clone(),
                },
                biology: BiologicalProfile {
                    is_alive: true,
                    death_year: None,
                    death_reason: None,
                    health_overall: 88.0,
                    fitness: 55.0,
                    energy_level: 75.0,
                    chronic_conditions: Vec::new(),
                },
                psychology: PsychologicalProfile {
                    discipline: 0.85,
                    curiosity: 0.60,
                    creativity: 0.45,
                    confidence: 0.80,
                    risk_tolerance: 0.35,
                    stress_level: 30.0,
                    resilience: 0.80,
                },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources {
                    cash: 60000.0,
                    household_wealth_tier: wealth.clone(),
                    living_arrangement: "FAMILY_HOME".to_string(),
                    tools_available: vec!["VEHICLE".to_string(), "LAPTOP".to_string()],
                },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord {
                    title: father_job.clone(),
                    employer_org_id: Some("org:sim:gov_ministry".to_string()),
                    monthly_earnings: 220000.0,
                    start_year: current_year - 8,
                }),
                is_player: false,
            },
            primary_role: NpcRole::Parent,
            personality: PersonalityProfile {
                warmth: 0.80,
                patience: 0.70,
                strictness: 0.65,
                ambition: 0.75,
                risk_tolerance: 0.30,
                communication_style: CommunicationStyle::Disciplinarian,
                core_values: vec!["Diligence".to_string(), "Academic Excellence".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 8, end_hour: 17, activity_name: format!("Civil Ministry Duties: {}", father_job), location_id: "place:cbd".to_string() },
                DailyRoutineBlock { start_hour: 18, end_hour: 22, activity_name: "Home Study & Financial Review".to_string(), location_id: "place:home".to_string() },
            ],
            life_goal: "Instill discipline, excellence, and strong moral character".to_string(),
            subjective_memories_of_player: Vec::new(),
            monthly_income: 220000.0,
            stress_level: 25.0,
            last_active_day: time.total_days,
        });

        // Add relationships to player
        if let Some(player) = persons.get_mut(&player_id) {
            player.relationships.insert(mother_id.clone(), RelationshipVector {
                source_person_id: player_id.clone(),
                target_person_id: mother_id.clone(),
                relationship_type: RelationshipType::Mother,
                trust: 0.95,
                affection: 0.95,
                respect: 0.90,
                resentment: 0.0,
                history: RelationshipHistory::default(),
                is_active: true,
            });
            player.relationships.insert(father_id.clone(), RelationshipVector {
                source_person_id: player_id.clone(),
                target_person_id: father_id.clone(),
                relationship_type: RelationshipType::Father,
                trust: 0.90,
                affection: 0.90,
                respect: 0.92,
                resentment: 0.0,
                history: RelationshipHistory::default(),
                is_active: true,
            });
        }

        // Additional age-appropriate NPCs (Teacher, Coach, Peer) if starting at older age
        if start_age >= 6 {
            let teacher_id = "person:sim:teacher".to_string();
            npcs.insert(teacher_id.clone(), AutonomousNPC {
                base: HumanEntity {
                    id: teacher_id.clone(),
                    identity: IdentityProfile {
                        first_name: "Oladipo".to_string(),
                        last_name: "Johnson".to_string(),
                        birth_year: current_year - 40,
                        birth_month: 3,
                        birth_day: 15,
                        sex: "Male".to_string(),
                        birthplace_id: rule_pack.city_id.clone(),
                        nationality: rule_pack.country_name.clone(),
                        culture: rule_pack.region_name.clone(),
                        primary_language: rule_pack.primary_language.clone(),
                    },
                    biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 85.0, fitness: 50.0, energy_level: 80.0, chronic_conditions: Vec::new() },
                    psychology: PsychologicalProfile { discipline: 0.90, curiosity: 0.80, creativity: 0.70, confidence: 0.85, risk_tolerance: 0.20, stress_level: 20.0, resilience: 0.80 },
                    reputation: ReputationProfile::default(),
                    skills: HashMap::new(),
                    resources: HumanResources { cash: 30000.0, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "APARTMENT".to_string(), tools_available: vec!["BOOKS".to_string()] },
                    relationships: HashMap::new(),
                    occupation: Some(OccupationRecord { title: "Senior Mathematics & Science Tutor".to_string(), employer_org_id: Some("org:sim:school".to_string()), monthly_earnings: 120000.0, start_year: current_year - 12 }),
                    is_player: false,
                },
                primary_role: NpcRole::Teacher,
                personality: PersonalityProfile { warmth: 0.75, patience: 0.85, strictness: 0.70, ambition: 0.65, risk_tolerance: 0.20, communication_style: CommunicationStyle::Inspirational, core_values: vec!["Academic Rigor".to_string()] },
                daily_schedule: vec![DailyRoutineBlock { start_hour: 8, end_hour: 15, activity_name: "Classroom Instruction".to_string(), location_id: "place:school".to_string() }],
                life_goal: "Inspire generation of critical thinkers".to_string(),
                subjective_memories_of_player: Vec::new(),
                monthly_income: 120000.0,
                stress_level: 15.0,
                last_active_day: time.total_days,
            });
            if let Some(player) = persons.get_mut(&player_id) {
                player.relationships.insert(teacher_id.clone(), RelationshipVector {
                    source_person_id: player_id.clone(),
                    target_person_id: teacher_id.clone(),
                    relationship_type: RelationshipType::Teacher,
                    trust: 0.75,
                    affection: 0.60,
                    respect: 0.85,
                    resentment: 0.0,
                    history: RelationshipHistory::default(),
                    is_active: true,
                });
            }
        }

        if start_age >= 10 {
            let coach_id = "person:sim:coach".to_string();
            npcs.insert(coach_id.clone(), AutonomousNPC {
                base: HumanEntity {
                    id: coach_id.clone(),
                    identity: IdentityProfile {
                        first_name: "Kunle".to_string(),
                        last_name: "Balogun".to_string(),
                        birth_year: current_year - 38,
                        birth_month: 7,
                        birth_day: 20,
                        sex: "Male".to_string(),
                        birthplace_id: rule_pack.city_id.clone(),
                        nationality: rule_pack.country_name.clone(),
                        culture: rule_pack.region_name.clone(),
                        primary_language: rule_pack.primary_language.clone(),
                    },
                    biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 95.0, fitness: 88.0, energy_level: 90.0, chronic_conditions: Vec::new() },
                    psychology: PsychologicalProfile { discipline: 0.88, curiosity: 0.50, creativity: 0.60, confidence: 0.85, risk_tolerance: 0.50, stress_level: 20.0, resilience: 0.85 },
                    reputation: ReputationProfile::default(),
                    skills: HashMap::new(),
                    resources: HumanResources { cash: 25000.0, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "APARTMENT".to_string(), tools_available: vec!["SPORTS_KIT".to_string()] },
                    relationships: HashMap::new(),
                    occupation: Some(OccupationRecord { title: "Youth Academy Head Coach".to_string(), employer_org_id: Some("org:sim:sports_club".to_string()), monthly_earnings: 110000.0, start_year: current_year - 9 }),
                    is_player: false,
                },
                primary_role: NpcRole::Coach,
                personality: PersonalityProfile { warmth: 0.70, patience: 0.65, strictness: 0.80, ambition: 0.85, risk_tolerance: 0.50, communication_style: CommunicationStyle::Direct, core_values: vec!["Work Rate".to_string(), "Tactical Discipline".to_string()] },
                daily_schedule: vec![DailyRoutineBlock { start_hour: 15, end_hour: 19, activity_name: "Youth Squad Tactical Training".to_string(), location_id: "place:pitch".to_string() }],
                life_goal: "Develop world-class athletic talent".to_string(),
                subjective_memories_of_player: Vec::new(),
                monthly_income: 110000.0,
                stress_level: 15.0,
                last_active_day: time.total_days,
            });
            if let Some(player) = persons.get_mut(&player_id) {
                player.relationships.insert(coach_id.clone(), RelationshipVector {
                    source_person_id: player_id.clone(),
                    target_person_id: coach_id.clone(),
                    relationship_type: RelationshipType::Coach,
                    trust: 0.70,
                    affection: 0.55,
                    respect: 0.80,
                    resentment: 0.0,
                    history: RelationshipHistory::default(),
                    is_active: true,
                });
            }
        }

        // 4. Spatial Places & Institutions
        let mut places = HashMap::new();
        let home_id = "place:home".to_string();
        places.insert(home_id.clone(), WorldPlace {
            id: home_id.clone(),
            name: "Family Residence".to_string(),
            place_type: PlaceType::Residence,
            parent_place_id: Some(rule_pack.city_id.clone()),
            country_id: rule_pack.country_id.clone(),
            climate_zone: format!("{:?}", rule_pack.climate_type),
            cost_of_living_index: 1.0,
            culture_tags: vec![rule_pack.region_name.clone()],
        });

        let mut institutions = HashMap::new();
        let mut events_ledger = Vec::new();

        // 5. Authentic Birth Event in the Ledger
        let birth_headline = if start_age == 0 {
            format!("The Birth of {} {}", first_name, last_name)
        } else {
            format!("The Early Life & Genesis of {} {}", first_name, last_name)
        };
        let birth_narrative = if start_age == 0 {
            format!("On {}, you were born in {}, {}. Welcomed by your mother {} and father {}, your life in the living world begins.", time.literary_date(), rule_pack.city_name, rule_pack.country_name, mother_name, father_name)
        } else {
            format!("You were born in {}, {} on {}. Growing up under the care of your parents {} and {}, you stand at Age {} ready to make your way in the world.", rule_pack.city_name, rule_pack.country_name, time.literary_date(), mother_name, father_name, start_age)
        };

        events_ledger.push(EventRecord {
            id: "ev:initial:birth".to_string(),
            timestamp: time.literary_date(),
            day_total: time.total_days,
            event_type: "BIRTH".to_string(),
            actor_id: player_id.clone(),
            location_id: home_id.clone(),
            headline: birth_headline,
            narrative: birth_narrative,
            causality_note: format!("Life began with authentic family roots in {}.", rule_pack.country_name),
            success: true,
        });

        Self {
            time,
            rng,
            persons,
            npcs,
            households: HashMap::new(),
            places,
            institutions,
            accounts: HashMap::new(),
            active_processes: Vec::new(),
            active_opportunities: Vec::new(),
            letters_inbox: Vec::new(),
            events_ledger,
            rule_pack,
            ai_bridge: AIBridge::new(AIBridgeConfig::default()),
        }
    }

    pub fn resolve_rule_pack(location_id: &str, country_id: &str) -> RegionalRulePack {
        let loc = location_id.to_lowercase();
        let c = country_id.to_lowercase();

        if loc.contains("edinburgh") || c.contains("scotland") {
            RegionalRulePack {
                city_id: "city:real:edinburgh".to_string(),
                city_name: "Edinburgh".to_string(),
                region_name: "Scotland".to_string(),
                country_id: "country:real:united_kingdom".to_string(),
                country_name: "United Kingdom".to_string(),
                currency_symbol: "£".to_string(),
                currency_code: "GBP".to_string(),
                climate_type: ClimateType::OceanicMaritime,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::BritishStandard,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 950.0, base_groceries_cost: 320.0, average_working_salary: 2600.0 },
            }
        } else if loc.contains("london") || c.contains("united_kingdom") || c.contains("uk") {
            RegionalRulePack {
                city_id: "city:real:london".to_string(),
                city_name: "London".to_string(),
                region_name: "Greater London".to_string(),
                country_id: "country:real:united_kingdom".to_string(),
                country_name: "United Kingdom".to_string(),
                currency_symbol: "£".to_string(),
                currency_code: "GBP".to_string(),
                climate_type: ClimateType::OceanicMaritime,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::BritishStandard,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 1600.0, base_groceries_cost: 380.0, average_working_salary: 3200.0 },
            }
        } else if loc.contains("san_francisco") {
            RegionalRulePack {
                city_id: "city:real:san_francisco".to_string(),
                city_name: "San Francisco".to_string(),
                region_name: "California".to_string(),
                country_id: "country:real:united_states".to_string(),
                country_name: "United States".to_string(),
                currency_symbol: "$".to_string(),
                currency_code: "USD".to_string(),
                climate_type: ClimateType::MediterraneanMarine,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::AmericanK12,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 2400.0, base_groceries_cost: 500.0, average_working_salary: 5500.0 },
            }
        } else if loc.contains("houston") || c.contains("united_states") || c.contains("usa") {
            RegionalRulePack {
                city_id: "city:real:houston".to_string(),
                city_name: "Houston".to_string(),
                region_name: "Texas".to_string(),
                country_id: "country:real:united_states".to_string(),
                country_name: "United States".to_string(),
                currency_symbol: "$".to_string(),
                currency_code: "USD".to_string(),
                climate_type: ClimateType::HumidSubtropical,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::AmericanK12,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 1300.0, base_groceries_cost: 420.0, average_working_salary: 4200.0 },
            }
        } else if loc.contains("kano") {
            RegionalRulePack {
                city_id: "city:real:kano".to_string(),
                city_name: "Kano".to_string(),
                region_name: "Northern Nigeria".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "Hausa / English".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 40000.0, base_groceries_cost: 35000.0, average_working_salary: 90000.0 },
            }
        } else {
            // Default: Lagos / Nigeria
            RegionalRulePack {
                city_id: "city:real:lagos".to_string(),
                city_name: "Lagos".to_string(),
                region_name: "Lagos State".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "Yoruba / English".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 85000.0, base_groceries_cost: 60000.0, average_working_salary: 150000.0 },
            }
        }
    }

    pub fn get_living_state(&self) -> LivingStateDTO {
        let player = self.persons.get("person:sim:player").unwrap();
        let age = player.identity.calculate_age(self.time.year, self.time.month, self.time.day);
        let stage = LifeStage::from_age(age);
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);

        LivingStateDTO {
            player_name: player.identity.full_name(),
            age,
            life_stage: stage.display_name().to_string(),
            time_formatted: self.time.literary_date(),
            location_formatted: format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name),
            weather_name: weather.name,
            weather_description: weather.description,
            cash: player.resources.cash,
            currency_symbol: self.rule_pack.currency_symbol.clone(),
            household_tier: format!("{:?}", player.resources.household_wealth_tier),
            energy_level: player.biology.energy_level,
            stress_level: player.psychology.stress_level,
            fitness: player.biology.fitness,
            occupation: player.occupation.as_ref().map(|o| o.title.clone()).unwrap_or_else(|| {
                if age < 4 { "Infant at Home".to_string() }
                else if age < 11 { "Primary School Student".to_string() }
                else if age < 16 { "Secondary School Student".to_string() }
                else if age < 18 { "Senior Secondary Scholar".to_string() }
                else { "Independent Citizen".to_string() }
            }),
            active_processes_count: self.active_processes.len(),
            surrounding_npcs_count: self.npcs.len(),
        }
    }

    pub fn get_situation(&self) -> SituationDTO {
        let player = self.persons.get("person:sim:player").unwrap();
        let age = player.identity.calculate_age(self.time.year, self.time.month, self.time.day);
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);

        let (place_name, atmosphere, objects, pressures, suggestions) = if age < 4 {
            (
                "Family Living Room & Nursery",
                format!("Gentle afternoon sunlight fills the living room. Outside, {}. Your parents are nearby watching over your rest and play.", weather.description.to_lowercase()),
                vec!["Soft baby blanket".to_string(), "Wooden toy blocks".to_string(), "Picture book on the rug".to_string()],
                vec!["Physical motor exploration".to_string(), "Emotional bonding with parents".to_string()],
                vec![
                    "Cuddle close to your mother on the sofa".to_string(),
                    "Try to stand and take first steps toward your father".to_string(),
                    "Point at the picture book and babble words".to_string(),
                    "Rest peacefully in your crib".to_string(),
                ]
            )
        } else if age < 13 {
            (
                "Family Home & Neighborhood Courtyard",
                format!("Morning air is crisp and active. Outside, {}. Schoolbooks lie stacked on the desk while children play in the courtyard.", weather.description.to_lowercase()),
                vec!["Primary arithmetic notebook".to_string(), "Leather football".to_string(), "Shared family computer".to_string()],
                vec!["School homework assignments".to_string(), "Childhood friendships & sports drills".to_string()],
                vec![
                    "Complete arithmetic homework exercises at the desk".to_string(),
                    "Head to the community field to play football with friends".to_string(),
                    "Help your parents with evening household chores".to_string(),
                    "Explore basic programming logic on the family computer".to_string(),
                ]
            )
        } else if age < 18 {
            (
                "Study Room & Senior Academy Grounds",
                format!("Evening air settles quietly over the neighborhood. Outside, {}. National examination revision papers and prospectus brochures lie open under the study lamp.", weather.description.to_lowercase()),
                vec!["Past examination papers (WAEC / JAMB / GCSE)".to_string(), "Football boots & kit".to_string(), "Personal mobile phone".to_string()],
                vec!["Upcoming national certificate examinations".to_string(), "Youth athletic scouting trials & career ambitions".to_string()],
                vec![
                    "Dedicate intensive evening study to past examination papers".to_string(),
                    "Train at the sports academy grounds under coach observation".to_string(),
                    "Sit down with your parents to discuss future academic goals".to_string(),
                    "Spend time with close friends or your romantic partner".to_string(),
                    "Ask for pocket money allowance for school supplies".to_string(),
                ]
            )
        } else {
            (
                "City Horizon & Independent Quarters",
                format!("The city pulse hums outside your window. {}. Opportunities for enterprise, employment, higher studies, and personal independence await.", weather.description),
                vec!["Personal smartphone & bank app".to_string(), "Academic certificates & credentials".to_string(), "Personal wallet & identity documents".to_string()],
                vec!["Career advancement & financial independence".to_string(), "Rent, living expenses & long-term ambitions".to_string()],
                vec![
                    "Search and apply for open professional career positions".to_string(),
                    "Draft an executive business plan and incorporate a company".to_string(),
                    "Enroll in higher degree seminars and university lectures".to_string(),
                    "Apply for international travel visas and flight reservations".to_string(),
                    "Manage personal savings and high-yield investments".to_string(),
                ]
            )
        };

        SituationDTO {
            current_room_or_place: place_name.to_string(),
            atmosphere_description: atmosphere,
            present_people: self.npcs.values().map(|n| format!("{} ({:?})", n.base.identity.full_name(), n.primary_role)).collect(),
            available_objects: objects,
            immediate_pressures: pressures,
            suggested_intentions: suggestions,
        }
    }

    pub fn submit_living_intent(&mut self, intent_text: &str) -> StepResolutionDTO {
        let player_id = "person:sim:player".to_string();
        let player = self.persons.get(&player_id).unwrap();
        let age = player.identity.calculate_age(self.time.year, self.time.month, self.time.day);
        let input_lower = intent_text.to_lowercase();

        // 1. Capability & Developmental Checks
        if age < 4 {
            // Infant actions only
            if input_lower.contains("business") || input_lower.contains("company") || input_lower.contains("job") || input_lower.contains("invest") || input_lower.contains("allowance") || input_lower.contains("money") || input_lower.contains("advice") {
                return StepResolutionDTO {
                    success: false,
                    days_advanced: 0,
                    hours_advanced: 1,
                    headline: "Developmental Limitation".to_string(),
                    narrative: "As an infant, your thoughts and capabilities are centered on warmth, bonding, and exploring your immediate surroundings. You cannot perform financial or adult career actions.".to_string(),
                    causality_note: "Physical and cognitive infancy precludes adult economic actions.".to_string(),
                    milestone_achieved: None,
                    world_consequences: Vec::new(),
                    financial_delta: 0.0,
                };
            }

            let (days_adv, hours_adv, hd, narr, caus, milestone, cons) = if input_lower.contains("cuddle") || input_lower.contains("hug") || input_lower.contains("mother") || input_lower.contains("parent") || input_lower.contains("hold") || input_lower.contains("book") {
                self.time.advance_hours(2);
                if let Some(p) = self.persons.get_mut(&player_id) {
                    p.psychology.stress_level = (p.psychology.stress_level - 10.0).max(0.0);
                    p.psychology.confidence = (p.psychology.confidence + 0.05).min(1.0);
                    if let Some(rel) = p.relationships.get_mut("person:sim:mother") {
                        rel.affection = (rel.affection + 0.05).min(1.0);
                        rel.trust = (rel.trust + 0.05).min(1.0);
                    }
                }
                (
                    0, 2,
                    "Warm Family Bonding".to_string(),
                    "You reached out toward your mother. She smiled tenderly, held you close, and sang a gentle lullaby as you rested in comforting warmth.".to_string(),
                    "Maternal affection nurtured emotional security and deepened trust.".to_string(),
                    None,
                    vec!["Mother affection increased (+5%)".to_string()]
                )
            } else if input_lower.contains("step") || input_lower.contains("walk") || input_lower.contains("stand") || input_lower.contains("crawl") {
                self.time.advance_days(7);
                if let Some(p) = self.persons.get_mut(&player_id) {
                    let entry = p.skills.entry("motor_coordination".to_string()).or_insert(SkillMastery { level: 10.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: self.time.total_days });
                    entry.level = (entry.level + 8.0).min(100.0);
                    p.biology.fitness = (p.biology.fitness + 3.0).min(100.0);
                }
                (
                    7, 0,
                    "First Confident Steps".to_string(),
                    "You pulled yourself up against the sofa and took wobbly, determined steps toward your father. He cheered with delighted pride as your mother clapped with joy.".to_string(),
                    "Physical motor coordination developed through self-directed movement.".to_string(),
                    Some("Took First Independent Steps".to_string()),
                    vec!["Motor Coordination mastery increased".to_string()]
                )
            } else {
                self.time.advance_days(3);
                (
                    3, 0,
                    "Peaceful Infant Day".to_string(),
                    "You spent the days exploring colorful nursery toys and resting peacefully while your parents cared for your needs.".to_string(),
                    "Safe home upbringing supported early childhood development.".to_string(),
                    None,
                    Vec::new()
                )
            };

            self.events_ledger.push(EventRecord {
                id: format!("ev:infant:{}", self.events_ledger.len() + 1),
                timestamp: self.time.literary_date(),
                day_total: self.time.total_days,
                event_type: "INFANCY_EXPERIENCE".to_string(),
                actor_id: player_id,
                location_id: self.rule_pack.city_name.clone(),
                headline: hd.clone(),
                narrative: narr.clone(),
                causality_note: caus.clone(),
                success: true,
            });

            return StepResolutionDTO {
                success: true,
                days_advanced: days_adv,
                hours_advanced: hours_adv,
                headline: hd,
                narrative: narr,
                causality_note: caus,
                milestone_achieved: milestone,
                world_consequences: cons,
                financial_delta: 0.0,
            };
        }

        // 2. Childhood & Adolescence & Adulthood Actions
        let mut days_advanced = 7;
        let mut hours_advanced = 0;
        let mut financial_delta = 0.0;
        let mut milestone = None;
        let headline;
        let narrative;
        let causality;

        if input_lower.contains("allowance") || input_lower.contains("pocket money") {
            let allowance = match player.resources.household_wealth_tier {
                WealthTier::Poverty => 10.0,
                WealthTier::WorkingClass => 30.0,
                WealthTier::MiddleClass => 100.0,
                WealthTier::UpperMiddle => 300.0,
                WealthTier::Wealthy => 800.0,
            };
            financial_delta = allowance;
            if let Some(p) = self.persons.get_mut(&player_id) {
                p.resources.cash += allowance;
                if let Some(rel) = p.relationships.get_mut("person:sim:mother") {
                    rel.affection = (rel.affection + 0.03).min(1.0);
                }
            }
            headline = "Pocket Money Received".to_string();
            narrative = format!("You asked your parents for a pocket money allowance. They smiled warmly, handed you {}{:.0}, and reminded you to manage your personal savings carefully.", self.rule_pack.currency_symbol, allowance);
            causality = "Received parental allowance based on household income.".to_string();
        } else if input_lower.contains("programming") || input_lower.contains("code") {
            days_advanced = if input_lower.contains("six months") { 56 } else if input_lower.contains("four weeks") { 28 } else { 14 };
            if let Some(p) = self.persons.get_mut(&player_id) {
                let entry = p.skills.entry("programming".to_string()).or_insert(SkillMastery { level: 10.0, experience: 0.0, natural_affinity: 1.3, last_practiced_day: self.time.total_days });
                entry.level = (entry.level + 12.0).min(100.0);
                p.psychology.curiosity = (p.psychology.curiosity + 0.04).min(1.0);
                p.reputation.creativity = (p.reputation.creativity + 8.0).min(100.0);
            }
            headline = "Software & Algorithmic Practice".to_string();
            narrative = "You spent time studying algorithmic logic, data structures, and building computer software on the family computer with deep focus.".to_string();
            causality = "Self-directed programming practice developed computational problem solving capability.".to_string();
        } else if input_lower.contains("apply") && input_lower.contains("university") {
            days_advanced = 28;
            if self.active_processes.iter().all(|p| p.process_type != ProcessType::UniversityAdmission) {
                self.active_processes.push(LifeProcess {
                    id: "proc:uni_admission".to_string(),
                    person_id: player_id.clone(),
                    process_type: ProcessType::UniversityAdmission,
                    title: "University Undergraduate Admission Application".to_string(),
                    institution_id: Some("org:real:university".to_string()),
                    current_step: 4,
                    total_steps: 4,
                    target_completion_day: self.time.total_days + 30,
                    requirements_met: true,
                    status: ProcessStatus::Succeeded,
                    payload: HashMap::new(),
                });
            }
            headline = "University Application Submitted".to_string();
            narrative = "You completed and submitted official matriculation forms and academic credentials for undergraduate admissions.".to_string();
            causality = "Formal university application processed through institutional registry.".to_string();
        } else if input_lower.contains("study") || input_lower.contains("exam") || input_lower.contains("waec") || input_lower.contains("math") || input_lower.contains("homework") {
            days_advanced = if input_lower.contains("four weeks") || input_lower.contains("4 weeks") { 28 } else { 14 };
            if let Some(p) = self.persons.get_mut(&player_id) {
                let entry = p.skills.entry("academics".to_string()).or_insert(SkillMastery { level: 25.0, experience: 0.0, natural_affinity: 1.1, last_practiced_day: self.time.total_days });
                entry.level = (entry.level + 6.0).min(100.0);
                p.psychology.discipline = (p.psychology.discipline + 0.04).min(1.0);
                p.reputation.academic_reputation = (p.reputation.academic_reputation + 5.0).min(100.0);
            }

            if age >= 15 && age <= 17 && input_lower.contains("waec") {
                milestone = Some("Completed National Certificate Examinations (WAEC & JAMB)".to_string());
                headline = "National Examination Results Ratified".to_string();
                narrative = "You sat for the national examinations in the academy halls. Weeks of disciplined preparation yielded excellent results: 7 Distinctions on your Senior Secondary Certificate and an outstanding JAMB UTME score of 288, qualifying you for higher university admissions.".to_string();
                causality = "Academic mastery unlocked official university entrance qualifications.".to_string();
            } else {
                headline = "Diligent Academic Study".to_string();
                narrative = "You dedicated evenings to working through curriculum problem sets and textbook chapters, strengthening your conceptual understanding.".to_string();
                causality = "Consistent study reinforced academic discipline and subject mastery.".to_string();
            }
        } else if input_lower.contains("football") || input_lower.contains("sports") || input_lower.contains("train") || input_lower.contains("coach") {
            days_advanced = 14;
            if let Some(p) = self.persons.get_mut(&player_id) {
                let entry = p.skills.entry("football_skill".to_string()).or_insert(SkillMastery { level: 20.0, experience: 0.0, natural_affinity: 1.3, last_practiced_day: self.time.total_days });
                entry.level = (entry.level + 7.0).min(100.0);
                p.biology.fitness = (p.biology.fitness + 4.0).min(100.0);
                p.reputation.athletic_reputation = (p.reputation.athletic_reputation + 6.0).min(100.0);
            }
            headline = "Athletic Drills & Tactical Training".to_string();
            narrative = "You trained on the sports pitch with focus, practicing quick passing drills and stamina runs under coach observation.".to_string();
            causality = "Regular athletic training enhanced physical fitness and technical capability.".to_string();
        } else if input_lower.contains("talk") || input_lower.contains("converse") || input_lower.contains("spend time") || input_lower.contains("advice") || input_lower.contains("tuition") || input_lower.contains("funding") {
            hours_advanced = 3;
            days_advanced = 0;
            if let Some(p) = self.persons.get_mut(&player_id) {
                p.psychology.stress_level = (p.psychology.stress_level - 12.0).max(0.0);
                if let Some(rel) = p.relationships.get_mut("person:sim:father") {
                    rel.trust = (rel.trust + 0.05).min(1.0);
                    rel.affection = (rel.affection + 0.05).min(1.0);
                }
            }
            headline = "Heartfelt Family Deliberation".to_string();
            narrative = "You sat down with your father in the evening for a serious conversation about higher education financing and career ambitions. Your father listened intently, pledging full family backing for your academic path.".to_string();
            causality = "Meaningful family dialogue reinforced mutual trust and clarified sponsorship.".to_string();
        } else if input_lower.contains("business") || input_lower.contains("company") || input_lower.contains("incorporate") {
            if age < 18 {
                return StepResolutionDTO {
                    success: false,
                    days_advanced: 0,
                    hours_advanced: 1,
                    headline: "Legal Age Requirement".to_string(),
                    narrative: "You must be at least 18 years of age to legally incorporate a business entity or register a limited liability company. You can prepare by drafting business plans and building skills.".to_string(),
                    causality_note: "Corporate incorporation laws require legal age of majority.".to_string(),
                    milestone_achieved: None,
                    world_consequences: Vec::new(),
                    financial_delta: 0.0,
                };
            }
            days_advanced = 21;
            financial_delta = -150.0;
            if let Some(p) = self.persons.get_mut(&player_id) {
                p.resources.cash -= 150.0;
            }
            milestone = Some("Incorporated First Commercial Company".to_string());
            headline = "Company Formally Incorporated".to_string();
            narrative = "You submitted articles of incorporation, paid administrative filing fees, and received your official certificate of incorporation. Your new enterprise is officially registered.".to_string();
            causality = "Completed legal company incorporation with commercial authorities.".to_string();
        } else {
            days_advanced = 7;
            headline = "Life Unfolds".to_string();
            narrative = format!("You pursued your intention: \"{}\". The days passed naturally within the rhythm of your environment and community.", intent_text);
            causality = "Carried out personal intention in the living world.".to_string();
        }

        if days_advanced > 0 {
            self.time.advance_days(days_advanced);
        }
        if hours_advanced > 0 {
            self.time.advance_hours(hours_advanced);
        }

        // Tick autonomous NPCs
        self.tick_autonomous_npcs(days_advanced);

        // Record in Event Ledger
        self.events_ledger.push(EventRecord {
            id: format!("ev:intent:{}", self.time.total_days),
            timestamp: self.time.literary_date(),
            day_total: self.time.total_days,
            event_type: "INTENTION_RESOLVED".to_string(),
            actor_id: player_id,
            location_id: "place:home".to_string(),
            headline: headline.clone(),
            narrative: narrative.clone(),
            causality_note: causality.clone(),
            success: true,
        });

        StepResolutionDTO {
            success: true,
            days_advanced,
            hours_advanced,
            headline,
            narrative,
            causality_note: causality,
            milestone_achieved: milestone,
            world_consequences: vec![format!("Advanced to {}", self.time.literary_date())],
            financial_delta,
        }
    }

    pub fn tick_autonomous_npcs(&mut self, days_elapsed: u32) {
        for npc in self.npcs.values_mut() {
            npc.last_active_day = self.time.total_days;
            if days_elapsed >= 28 && npc.monthly_income > 0.0 {
                npc.base.resources.cash += npc.monthly_income;
                npc.base.resources.cash = (npc.base.resources.cash - (npc.monthly_income * 0.45)).max(0.0);
            }
        }
    }

    pub fn generate_today_scene(&self) -> TodaySceneDTO {
        let situation = self.get_situation();
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);

        TodaySceneDTO {
            headline: format!("Life in {}", self.rule_pack.city_name),
            narrative: situation.atmosphere_description,
            weather_name: weather.name,
            weather_description: weather.description,
            location_name: format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name),
            present_people: situation.present_people,
            environmental_objects: situation.available_objects,
            subtle_details: situation.immediate_pressures.clone(),
            immediate_pressures: situation.immediate_pressures,
        }
    }

    pub fn get_surrounding_npcs(&self) -> Vec<ContextNpcDTO> {
        let player = self.persons.get("person:sim:player").unwrap();
        let age = player.identity.calculate_age(self.time.year, self.time.month, self.time.day);

        let mut list = Vec::new();
        for (id, npc) in &self.npcs {
            if age < 4 && !matches!(npc.primary_role, NpcRole::Parent | NpcRole::Sibling) {
                continue;
            }
            if age < 10 && matches!(npc.primary_role, NpcRole::Coach | NpcRole::Colleague | NpcRole::Employer) {
                continue;
            }

            let role_label = match npc.primary_role {
                NpcRole::Parent => {
                    if npc.base.identity.sex == "Female" { "Mother".to_string() } else { "Father".to_string() }
                }
                NpcRole::Teacher => "Teacher & Mentor".to_string(),
                NpcRole::Coach => "Sports Coach & Scout".to_string(),
                NpcRole::Friend => "Friend & Peer".to_string(),
                NpcRole::Classmate => "Classmate".to_string(),
                NpcRole::Partner => "Romantic Partner".to_string(),
                _ => format!("{:?}", npc.primary_role),
            };

            let current_act = npc.daily_schedule.first().map(|s| s.activity_name.clone()).unwrap_or_else(|| "Resting at home".to_string());

            list.push(ContextNpcDTO {
                id: id.clone(),
                name: npc.base.identity.full_name(),
                relationship_type: role_label,
                trust_description: "Deep familial trust and affection".to_string(),
                current_activity: current_act,
            });
        }
        list
    }

    pub fn get_active_processes(&self) -> Vec<ContextProcessDTO> {
        self.active_processes.iter().map(|p| {
            let pct = if p.total_steps > 0 { (p.current_step * 100) / p.total_steps } else { 0 };
            ContextProcessDTO {
                id: p.id.clone(),
                title: p.title.clone(),
                current_step: p.current_step,
                total_steps: p.total_steps,
                progress_percent: pct,
                status: format!("{:?}", p.status),
            }
        }).collect()
    }

    pub fn get_biography(&self) -> String {
        let mut bio = String::new();
        for ev in &self.events_ledger {
            bio.push_str(&format!("## {}\n{}\n*{}*\n\n", ev.headline, ev.narrative, ev.timestamp));
        }
        if bio.is_empty() {
            "Life is just beginning to unfold...".to_string()
        } else {
            bio
        }
    }
}

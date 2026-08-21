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
    pub active_processes: Vec<LifeProcess>,
    pub letters_inbox: Vec<LetterNotification>,
    pub documents: HashMap<String, DocumentRecord>,
    pub phone_messages: Vec<PhoneMessage>,
    pub active_call: Option<PhoneCallState>,
    pub events_ledger: Vec<EventRecord>,
    pub rule_pack: RegionalRulePack,
    #[serde(skip, default = "default_ai_bridge")]
    pub ai_bridge: AIBridge,
}

fn default_ai_bridge() -> AIBridge {
    AIBridge::new(AIBridgeConfig::default())
}

impl SimulationEngine {
    pub fn new_game(config: NewLifeConfig, seed: u64) -> Self {
        let rng = WorldRng::new(seed);
        let start_age = config.starting_age;

        // 1. Exact Birthdate & Current Time Calculation
        let birth_year = config.birth_year.unwrap_or(config.starting_year - start_age as i32);
        let birth_month = config.birth_month.unwrap_or(6).clamp(1, 12);
        let birth_day = config.birth_day.unwrap_or(14).clamp(1, 30);
        let current_year = config.starting_year;
        let mut time = TimeState::new(current_year, birth_month, birth_day);

        // 2. Regional Rule Pack Resolution
        let rule_pack = Self::resolve_rule_pack(&config.location_id, &config.country_id);

        let first_name = config.first_name.unwrap_or_else(|| "Israel".to_string());
        let last_name = config.last_name.unwrap_or_else(|| "Oyebamiji".to_string());
        let sex = config.sex.unwrap_or_else(|| "Male".to_string());
        let wealth = WealthTier::from_str(config.household_income_tier.as_deref().unwrap_or("MIDDLE"));

        let player_id = "person:sim:player".to_string();

        // 3. Player Cash Gated by Age and Wealth
        let initial_cash = if start_age >= 18 {
            match wealth {
                WealthTier::Poverty => 50.0,
                WealthTier::WorkingClass => 300.0,
                WealthTier::MiddleClass => 1200.0,
                WealthTier::UpperMiddle => 3500.0,
                WealthTier::Wealthy => 10000.0,
            }
        } else if start_age >= 13 {
            match wealth {
                WealthTier::Poverty => 5.0,
                WealthTier::WorkingClass => 20.0,
                WealthTier::MiddleClass => 60.0,
                WealthTier::UpperMiddle => 150.0,
                WealthTier::Wealthy => 400.0,
            }
        } else {
            0.0 // Age 0-12 starts with 0 cash!
        };

        let mut skills = HashMap::new();
        for (k, v) in config.skills {
            skills.insert(k, SkillMastery {
                level: v,
                experience: 0.0,
                natural_affinity: 1.0,
                last_practiced_day: time.total_days,
            });
        }

        let mut player_relationships = HashMap::new();
        player_relationships.insert("person:sim:mother".to_string(), RelationshipEdge {
            target_entity_id: "person:sim:mother".to_string(),
            target_name: "Mother".to_string(),
            relationship_type: "Mother".to_string(),
            affinity: 0.95,
            trust: 0.95,
            respect: 0.90,
            memories: Vec::new(),
        });
        player_relationships.insert("person:sim:father".to_string(), RelationshipEdge {
            target_entity_id: "person:sim:father".to_string(),
            target_name: "Father".to_string(),
            relationship_type: "Father".to_string(),
            affinity: 0.92,
            trust: 0.92,
            respect: 0.90,
            memories: Vec::new(),
        });

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
            psychology: PsychologicalProfile::default(),
            reputation: ReputationProfile::default(),
            skills,
            resources: HumanResources {
                cash: initial_cash,
                household_wealth_tier: wealth.clone(),
                living_arrangement: "FAMILY_HOME".to_string(),
                tools_available: if start_age >= 13 {
                    vec!["BOOKS".to_string(), "FAMILY_DESKTOP".to_string(), "SMARTPHONE".to_string()]
                } else {
                    vec!["CRIB_TOYS".to_string()]
                },
            },
            relationships: player_relationships,
            occupation: if start_age >= 22 { Some("Junior Associate".to_string()) } else { None },
            is_player: true,
        };

        let mut persons = HashMap::new();
        persons.insert(player_id.clone(), player_entity);

        // 4. Parents Setup with Regionalization and Clean First Names
        let raw_mother = config.mother_name.unwrap_or_else(|| {
            if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") {
                "Fiona".to_string()
            } else if rule_pack.city_id.contains("london") || rule_pack.city_id.contains("manchester") {
                "Eleanor".to_string()
            } else if rule_pack.city_id.contains("san_francisco") || rule_pack.city_id.contains("houston") || rule_pack.city_id.contains("new_york") {
                "Sarah".to_string()
            } else {
                "Blessing".to_string()
            }
        });
        let mother_first = if raw_mother.contains(' ') {
            raw_mother.split_whitespace().next().unwrap_or(&raw_mother).to_string()
        } else {
            raw_mother.clone()
        };

        let mother_job = config.mother_job.unwrap_or_else(|| {
            if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Senior Nurse (NHS)".to_string()
            } else if rule_pack.city_id.contains("san_francisco") {
                "Biotech Research Scientist".to_string()
            } else {
                "Senior Healthcare Officer".to_string()
            }
        });

        let raw_father = config.father_name.unwrap_or_else(|| {
            if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") {
                "Callum".to_string()
            } else if rule_pack.city_id.contains("london") || rule_pack.city_id.contains("manchester") {
                "Arthur".to_string()
            } else if rule_pack.city_id.contains("san_francisco") || rule_pack.city_id.contains("houston") || rule_pack.city_id.contains("new_york") {
                "Robert".to_string()
            } else {
                "David".to_string()
            }
        });
        let father_first = if raw_father.contains(' ') {
            raw_father.split_whitespace().next().unwrap_or(&raw_father).to_string()
        } else {
            raw_father.clone()
        };

        let father_job = config.father_job.unwrap_or_else(|| {
            if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") || rule_pack.city_id.contains("london") {
                "Civil Structural Engineer".to_string()
            } else if rule_pack.city_id.contains("san_francisco") {
                "Software Architect".to_string()
            } else {
                "Senior Ministry Administrator".to_string()
            }
        });

        let mut npcs = HashMap::new();

        // Mother NPC
        let mother_id = "person:sim:mother".to_string();
        let mother_routine = Self::generate_routine_for_job(&mother_job, &rule_pack.city_id);
        npcs.insert(mother_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: mother_id.clone(),
                identity: IdentityProfile {
                    first_name: mother_first.clone(),
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
                biology: BiologicalProfile::default(),
                psychology: PsychologicalProfile::default(),
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources {
                    cash: 4500.0,
                    household_wealth_tier: wealth.clone(),
                    living_arrangement: "FAMILY_HOME".to_string(),
                    tools_available: vec![],
                },
                relationships: HashMap::new(),
                occupation: Some(mother_job.clone()),
                is_player: false,
            },
            daily_routine: mother_routine,
            communication_style: CommunicationStyle::Nurturing,
            personality: NpcPersonality {
                communication_style: CommunicationStyle::Nurturing,
                strictness: 0.35,
            },
            current_goal: format!("Provide loving family stability in {}", rule_pack.city_name),
            last_active_day: time.total_days,
        });

        // Father NPC
        let father_id = "person:sim:father".to_string();
        let father_routine = Self::generate_routine_for_job(&father_job, &rule_pack.city_id);
        npcs.insert(father_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: father_id.clone(),
                identity: IdentityProfile {
                    first_name: father_first.clone(),
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
                biology: BiologicalProfile::default(),
                psychology: PsychologicalProfile::default(),
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources {
                    cash: 5200.0,
                    household_wealth_tier: wealth.clone(),
                    living_arrangement: "FAMILY_HOME".to_string(),
                    tools_available: vec![],
                },
                relationships: HashMap::new(),
                occupation: Some(father_job.clone()),
                is_player: false,
            },
            daily_routine: father_routine,
            communication_style: CommunicationStyle::Disciplinarian,
            personality: NpcPersonality {
                communication_style: CommunicationStyle::Disciplinarian,
                strictness: 0.70,
            },
            current_goal: format!("Support family career and education goals in {}", rule_pack.city_name),
            last_active_day: time.total_days,
        });

        // Mentor / Teacher NPC (Regionalized)
        if start_age >= 4 {
            let (teacher_first, teacher_last, teacher_title) = if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") {
                ("Hamish", "MacGregor", "Head Teacher")
            } else if rule_pack.city_id.contains("london") || rule_pack.city_id.contains("manchester") {
                ("David", "Harrison", "Senior Form Tutor")
            } else if rule_pack.city_id.contains("san_francisco") || rule_pack.city_id.contains("houston") || rule_pack.city_id.contains("new_york") {
                ("Marcus", "Davis", "Principal Instructor")
            } else {
                ("Babatunde", "Balogun", "Senior Master")
            };

            let teacher_id = "person:sim:teacher".to_string();
            npcs.insert(teacher_id.clone(), AutonomousNPC {
                base: HumanEntity {
                    id: teacher_id.clone(),
                    identity: IdentityProfile {
                        first_name: teacher_first.to_string(),
                        last_name: teacher_last.to_string(),
                        birth_year: current_year - 42,
                        birth_month: 2,
                        birth_day: 15,
                        sex: "Male".to_string(),
                        birthplace_id: rule_pack.city_id.clone(),
                        nationality: rule_pack.country_name.clone(),
                        culture: rule_pack.region_name.clone(),
                        primary_language: rule_pack.primary_language.clone(),
                    },
                    biology: BiologicalProfile::default(),
                    psychology: PsychologicalProfile::default(),
                    reputation: ReputationProfile::default(),
                    skills: HashMap::new(),
                    resources: HumanResources {
                        cash: 3000.0,
                        household_wealth_tier: WealthTier::MiddleClass,
                        living_arrangement: "APARTMENT".to_string(),
                        tools_available: vec![],
                    },
                    relationships: HashMap::new(),
                    occupation: Some(teacher_title.to_string()),
                    is_player: false,
                },
                daily_routine: vec![ScheduledActivity {
                    start_hour: 8,
                    end_hour: 16,
                    location_id: "place:school".to_string(),
                    activity_name: "Teaching Class".to_string(),
                    description: "Conducting academic lectures and evaluating student progress.".to_string(),
                }],
                communication_style: CommunicationStyle::Inspirational,
                personality: NpcPersonality {
                    communication_style: CommunicationStyle::Inspirational,
                    strictness: 0.50,
                },
                current_goal: "Mentor students toward exceptional academic achievement".to_string(),
                last_active_day: time.total_days,
            });

            // Sports Coach NPC
            let coach_id = "person:sim:coach".to_string();
            npcs.insert(coach_id.clone(), AutonomousNPC {
                base: HumanEntity {
                    id: coach_id.clone(),
                    identity: IdentityProfile {
                        first_name: "Segun".to_string(),
                        last_name: "Okafor".to_string(),
                        birth_year: current_year - 38,
                        birth_month: 7,
                        birth_day: 19,
                        sex: "Male".to_string(),
                        birthplace_id: rule_pack.city_id.clone(),
                        nationality: rule_pack.country_name.clone(),
                        culture: rule_pack.region_name.clone(),
                        primary_language: rule_pack.primary_language.clone(),
                    },
                    biology: BiologicalProfile::default(),
                    psychology: PsychologicalProfile::default(),
                    reputation: ReputationProfile::default(),
                    skills: HashMap::new(),
                    resources: HumanResources {
                        cash: 2800.0,
                        household_wealth_tier: WealthTier::MiddleClass,
                        living_arrangement: "APARTMENT".to_string(),
                        tools_available: vec![],
                    },
                    relationships: HashMap::new(),
                    occupation: Some("Head Football Coach".to_string()),
                    is_player: false,
                },
                daily_routine: vec![ScheduledActivity {
                    start_hour: 15,
                    end_hour: 19,
                    location_id: "place:sports_academy".to_string(),
                    activity_name: "Football Drills".to_string(),
                    description: "Conducting tactical pitch drills and athletic conditioning.".to_string(),
                }],
                communication_style: CommunicationStyle::Direct,
                personality: NpcPersonality {
                    communication_style: CommunicationStyle::Direct,
                    strictness: 0.80,
                },
                current_goal: "Develop elite sporting discipline and technical skill".to_string(),
                last_active_day: time.total_days,
            });
        }

        // 5. World Places Creation
        let mut places = HashMap::new();
        let home_id = "place:home".to_string();
        places.insert(home_id.clone(), WorldPlace {
            id: home_id.clone(),
            name: format!("{} Family Home", last_name),
            place_type: PlaceType::Residence,
            city_id: rule_pack.city_id.clone(),
            district_name: "Residential District".to_string(),
            required_min_age: 0,
            affords_activities: vec!["REST".to_string(), "FAMILY_BONDING".to_string(), "QUIET_STUDY".to_string()],
        });

        // 6. Documents Generator: Authentic Birth Certificate
        let mut documents = HashMap::new();
        let birth_cert_id = "doc:birth_certificate".to_string();
        let reg_number = format!("{}/{}/BC-{:05}", rule_pack.country_id.split(':').last().unwrap_or("NG").to_uppercase(), birth_year, seed % 90000 + 10000);
        let issuing_authority = if rule_pack.city_id.contains("glasgow") || rule_pack.city_id.contains("edinburgh") {
            "National Records of Scotland (NRS)".to_string()
        } else if rule_pack.city_id.contains("london") || rule_pack.city_id.contains("manchester") {
            "General Register Office (GRO England & Wales)".to_string()
        } else if rule_pack.city_id.contains("san_francisco") {
            "California Department of Public Health (Vital Records)".to_string()
        } else if rule_pack.city_id.contains("houston") {
            "Texas Department of State Health Services (Vital Statistics)".to_string()
        } else {
            "National Population Commission (NPC Nigeria)".to_string()
        };

        let mut birth_fields = HashMap::new();
        birth_fields.insert("Full Legal Name".to_string(), format!("{} {}", first_name, last_name));
        birth_fields.insert("Sex".to_string(), sex.clone());
        birth_fields.insert("Date of Birth".to_string(), format!("{} {} {}", birth_day, match birth_month {
            1 => "January", 2 => "February", 3 => "March", 4 => "April", 5 => "May", 6 => "June",
            7 => "July", 8 => "August", 9 => "September", 10 => "October", 11 => "November", _ => "December"
        }, birth_year));
        birth_fields.insert("Place of Birth".to_string(), format!("{}, {}", rule_pack.city_name, rule_pack.country_name));
        birth_fields.insert("Mother".to_string(), format!("{} {}", mother_first, last_name));
        birth_fields.insert("Father".to_string(), format!("{} {}", father_first, last_name));
        birth_fields.insert("Registration Number".to_string(), reg_number.clone());
        birth_fields.insert("Issuing Authority".to_string(), issuing_authority.clone());
        birth_fields.insert("Status".to_string(), "OFFICIALLY_REGISTERED".to_string());
        birth_fields.insert("Registration Status".to_string(), "OFFICIALLY_REGISTERED".to_string());

        documents.insert(birth_cert_id.clone(), DocumentRecord {
            id: birth_cert_id,
            title: "Official Certificate of Birth".to_string(),
            document_type: "BIRTH_CERTIFICATE".to_string(),
            issue_date: format!("{}-06-15", birth_year),
            issuing_authority,
            registration_number: reg_number,
            fields: birth_fields,
            is_verified: true,
        });

        // 7. Initial Events Ledger
        let mut events_ledger = Vec::new();
        events_ledger.push(EventRecord {
            id: "event:genesis".to_string(),
            timestamp: time.literary_date(),
            day_total: time.total_days,
            event_type: "BIRTH".to_string(),
            actor_id: player_id.clone(),
            location_id: home_id.clone(),
            headline: format!("Life Commences in {}", rule_pack.city_name),
            narrative: format!("A newborn child, {} {}, is welcomed into the world in {}, {}.", first_name, last_name, rule_pack.city_name, rule_pack.country_name),
            causality_note: format!("Rooted authentically in the {} regional rule pack.", rule_pack.country_name),
            success: true,
        });

        Self {
            time,
            rng,
            persons,
            npcs,
            households: HashMap::new(),
            places,
            institutions: HashMap::new(),
            active_processes: Vec::new(),
            letters_inbox: Vec::new(),
            documents,
            phone_messages: Vec::new(),
            active_call: None,
            events_ledger,
            rule_pack,
            ai_bridge: AIBridge::new(AIBridgeConfig::default()),
        }
    }

    fn generate_routine_for_job(job: &str, city_id: &str) -> Vec<ScheduledActivity> {
        let job_lower = job.to_lowercase();
        if job_lower.contains("nurse") || job_lower.contains("doctor") || job_lower.contains("healthcare") {
            vec![
                ScheduledActivity {
                    start_hour: 8,
                    end_hour: 16,
                    location_id: "place:clinic".to_string(),
                    activity_name: "Clinical Ward Shift".to_string(),
                    description: "Attending to patients and administering treatments at the hospital.".to_string(),
                },
                ScheduledActivity {
                    start_hour: 17,
                    end_hour: 22,
                    location_id: "place:home".to_string(),
                    activity_name: "Family Dinner & Rest".to_string(),
                    description: "At home preparing family meals and relaxing.".to_string(),
                },
            ]
        } else if job_lower.contains("architect") {
            vec![
                ScheduledActivity {
                    start_hour: 9,
                    end_hour: 17,
                    location_id: "place:drafting_studio".to_string(),
                    activity_name: "Architectural Drafting & Review".to_string(),
                    description: "Drafting structural floor plans and consulting clients at the design studio.".to_string(),
                },
                ScheduledActivity {
                    start_hour: 18,
                    end_hour: 22,
                    location_id: "place:home".to_string(),
                    activity_name: "Home Relaxation".to_string(),
                    description: "Reviewing sketchbooks and spending time with family.".to_string(),
                },
            ]
        } else if job_lower.contains("engineer") || job_lower.contains("software") {
            vec![
                ScheduledActivity {
                    start_hour: 9,
                    end_hour: 17,
                    location_id: "place:office".to_string(),
                    activity_name: "Systems Engineering Work".to_string(),
                    description: "Writing architecture specifications and supervising project builds.".to_string(),
                },
                ScheduledActivity {
                    start_hour: 18,
                    end_hour: 22,
                    location_id: "place:home".to_string(),
                    activity_name: "Evening Family Time".to_string(),
                    description: "Resting and discussing goals with the family.".to_string(),
                },
            ]
        } else {
            vec![
                ScheduledActivity {
                    start_hour: 8,
                    end_hour: 16,
                    location_id: "place:ministry".to_string(),
                    activity_name: "Public Administration Duty".to_string(),
                    description: "Managing civic department records and civil service meetings.".to_string(),
                },
                ScheduledActivity {
                    start_hour: 17,
                    end_hour: 22,
                    location_id: "place:home".to_string(),
                    activity_name: "Evening Rest".to_string(),
                    description: "At home with family.".to_string(),
                },
            ]
        }
    }

    pub fn resolve_rule_pack(location_id: &str, country_id: &str) -> RegionalRulePack {
        let loc = location_id.to_lowercase();
        let c = country_id.to_lowercase();

        if loc.contains("abuja") {
            RegionalRulePack {
                city_id: "city:real:abuja".to_string(),
                city_name: "Abuja".to_string(),
                region_name: "Federal Capital Territory".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "English / Hausa".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 75000.0, base_groceries_cost: 45000.0, average_working_salary: 140000.0 },
            }
        } else if loc.contains("glasgow") {
            RegionalRulePack {
                city_id: "city:real:glasgow".to_string(),
                city_name: "Glasgow".to_string(),
                region_name: "Scotland".to_string(),
                country_id: "country:real:united_kingdom".to_string(),
                country_name: "United Kingdom".to_string(),
                currency_symbol: "£".to_string(),
                currency_code: "GBP".to_string(),
                climate_type: ClimateType::OceanicMaritime,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::BritishStandard,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 850.0, base_groceries_cost: 290.0, average_working_salary: 2400.0 },
            }
        } else if loc.contains("edinburgh") {
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
        } else if loc.contains("london") {
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
        } else if loc.contains("manchester") {
            RegionalRulePack {
                city_id: "city:real:manchester".to_string(),
                city_name: "Manchester".to_string(),
                region_name: "Greater Manchester".to_string(),
                country_id: "country:real:united_kingdom".to_string(),
                country_name: "United Kingdom".to_string(),
                currency_symbol: "£".to_string(),
                currency_code: "GBP".to_string(),
                climate_type: ClimateType::OceanicMaritime,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::BritishStandard,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 900.0, base_groceries_cost: 300.0, average_working_salary: 2500.0 },
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
        } else if loc.contains("houston") {
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
        } else if loc.contains("new_york") {
            RegionalRulePack {
                city_id: "city:real:new_york".to_string(),
                city_name: "New York City".to_string(),
                region_name: "New York".to_string(),
                country_id: "country:real:united_states".to_string(),
                country_name: "United States".to_string(),
                currency_symbol: "$".to_string(),
                currency_code: "USD".to_string(),
                climate_type: ClimateType::HumidSubtropical,
                primary_language: "English".to_string(),
                school_system: SchoolSystemType::AmericanK12,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 2600.0, base_groceries_cost: 520.0, average_working_salary: 5200.0 },
            }
        } else if loc.contains("kano") {
            RegionalRulePack {
                city_id: "city:real:kano".to_string(),
                city_name: "Kano".to_string(),
                region_name: "Kano State".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "Hausa / English".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 40000.0, base_groceries_cost: 35000.0, average_working_salary: 90000.0 },
            }
        } else if loc.contains("ibadan") {
            RegionalRulePack {
                city_id: "city:real:ibadan".to_string(),
                city_name: "Ibadan".to_string(),
                region_name: "Oyo State".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "Yoruba / English".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 45000.0, base_groceries_cost: 38000.0, average_working_salary: 95000.0 },
            }
        } else if loc.contains("port_harcourt") {
            RegionalRulePack {
                city_id: "city:real:port_harcourt".to_string(),
                city_name: "Port Harcourt".to_string(),
                region_name: "Rivers State".to_string(),
                country_id: "country:real:nigeria".to_string(),
                country_name: "Nigeria".to_string(),
                currency_symbol: "₦".to_string(),
                currency_code: "NGN".to_string(),
                climate_type: ClimateType::TropicalSavanna,
                primary_language: "English / Pidgin".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 65000.0, base_groceries_cost: 42000.0, average_working_salary: 130000.0 },
            }
        } else if c.contains("united_kingdom") || c.contains("uk") || c.contains("scotland") {
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
        } else if c.contains("united_states") || c.contains("usa") {
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
                primary_language: "English / Yoruba".to_string(),
                school_system: SchoolSystemType::Nigerian6_3_3_4,
                starting_costs: HouseholdEconomyProfile { base_monthly_rent: 60000.0, base_groceries_cost: 40000.0, average_working_salary: 120000.0 },
            }
        }
    }

    // =========================================================================
    // EXPLICIT TIME OPERATIONS (No keyword guesswork!)
    // =========================================================================

    pub fn advance_hours(&mut self, hours: u32) -> StepResolutionDTO {
        self.time.advance_hours(hours as u8);
        self.get_player_mut().biology.energy_level = (self.get_player().biology.energy_level - (hours as f32 * 2.0)).clamp(10.0, 100.0);

        let headline = format!("{} Passed Quietly", if hours == 1 { "An Hour".to_string() } else { format!("{} Hours", hours) });
        let narrative = format!("You spend {} attending to quiet moments in {}.", if hours == 1 { "one hour".to_string() } else { format!("{} hours", hours) }, self.rule_pack.city_name);

        self.record_event("TIME_WAIT", &headline, &narrative, &format!("Advanced time by exactly {} hour(s).", hours), true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: hours as u8,
            headline,
            narrative,
            causality_note: format!("Clock advanced by {} hour(s).", hours),
            milestone_achieved: None,
            world_consequences: vec![],
            financial_delta: 0.0,
        }
    }

    pub fn sleep_until_morning(&mut self) -> StepResolutionDTO {
        self.time.advance_days(1);
        self.time.hour = 7;
        self.time.minute = 0;
        self.get_player_mut().biology.energy_level = 100.0;
        self.get_player_mut().psychology.stress_level = (self.get_player().psychology.stress_level - 15.0).clamp(0.0, 100.0);

        let headline = "Awakening to Morning Light".to_string();
        let narrative = format!("You wake refreshed at 7:00 AM after a peaceful night's rest in {}. The morning air is calm.", self.rule_pack.city_name);

        self.record_event("SLEEP", &headline, &narrative, "Restored energy to 100% and reduced accumulated stress.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 1,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: "Slept until 7:00 AM morning.".to_string(),
            milestone_achieved: None,
            world_consequences: vec!["Energy fully restored".to_string()],
            financial_delta: 0.0,
        }
    }

    pub fn advance_days(&mut self, days: u32) -> StepResolutionDTO {
        self.time.advance_days(days);
        let current_days = self.time.total_days;
        for npc in self.npcs.values_mut() {
            npc.last_active_day = current_days;
        }
        let headline = format!("{} Day(s) Passed", days);
        let narrative = format!("{} calendar day(s) have passed as you attended to life and routines in {}.", days, self.rule_pack.city_name);

        self.record_event("TIME_ADVANCE", &headline, &narrative, &format!("Advanced exactly {} calendar day(s).", days), true);

        StepResolutionDTO {
            success: true,
            days_advanced: days,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: format!("Advanced simulation by {} day(s).", days),
            milestone_achieved: None,
            world_consequences: vec![],
            financial_delta: 0.0,
        }
    }

    pub fn follow_routine(&mut self, days: u32) -> StepResolutionDTO {
        self.time.advance_days(days);
        self.get_player_mut().psychology.discipline = (self.get_player().psychology.discipline + (days as f32 * 0.01)).clamp(0.0, 1.0);

        let headline = format!("Followed Daily Routine for {} Days", days);
        let narrative = format!("You maintained consistent daily discipline, attending to your household responsibilities, studies, and family relationships in {}.", self.rule_pack.city_name);

        self.record_event("ROUTINE", &headline, &narrative, "Built personal discipline through structured daily consistency.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: days,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: format!("Completed {} days of daily routine.", days),
            milestone_achieved: None,
            world_consequences: vec!["Discipline reinforced".to_string()],
            financial_delta: 0.0,
        }
    }

    // =========================================================================
    // STRUCTURED INTENTIONS & ACTIONS
    // =========================================================================

    pub fn submit_living_intent(&mut self, text_or_cmd: &str) -> StepResolutionDTO {
        let lower = text_or_cmd.trim().to_lowercase();

        // 1. Check for explicit time actions
        if lower == "wait 1 hour" || lower == "wait one hour" || lower.starts_with("i spend an hour quietly reading") {
            return self.advance_hours(1);
        }
        if lower.starts_with("i sleep peacefully") || lower == "sleep" || lower == "sleep until morning" {
            return self.sleep_until_morning();
        }
        if lower.starts_with("i follow my daily routine") || lower.contains("follow routine") {
            return self.follow_routine(7);
        }
        if lower.starts_with("i spend the entire day") || lower == "advance 1 day" {
            return self.advance_days(1);
        }

        // 1. Developmental and Legal Age Gating
        let age = self.get_player_age();
        if age < 18 && (lower.contains("incorporate a new") || lower.contains("limited liability company") || lower.contains("register a company")) {
            let headline = "Developmental & Legal Age Constraint".to_string();
            let narrative = format!("As an infant at age {} in {}, you lack the developmental and legal capacity to execute commercial incorporation filings.", age, self.rule_pack.city_name);
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline,
                narrative,
                causality_note: "Action barred by developmental capability and legal age.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        // 2. Healthcare Checkup Action
        if lower.contains("pediatric health checkup") || lower.contains("vaccination") || lower.contains("routine health checkup") {
            return self.attend_medical_checkup();
        }

        // 3. University Admission Application
        if lower.contains("undergraduate admission") || lower.contains("apply for university") || lower.contains("college application") {
            self.time.advance_days(28);
            let proc_id = format!("proc:uni_adm_{}", self.active_processes.len() + 1);
            self.active_processes.push(LifeProcess {
                id: proc_id,
                process_type: ProcessType::UniversityAdmission,
                title: "University Undergraduate Admission Application".to_string(),
                target_institution_id: Some("inst:university".to_string()),
                current_step: 4,
                total_steps: 4,
                progress_percent: 100,
                status: "SUBMITTED_UNDER_FACULTY_REVIEW".to_string(),
                missing_requirements: vec![],
                next_appointment_day: Some(self.time.total_days + 30),
            });

            let headline = "University Application Formally Submitted".to_string();
            let narrative = format!("You completed all requisite documentation, academic transcripts, and verification fees for undergraduate admission in {}. The application is now lodged under faculty review.", self.rule_pack.city_name);
            self.record_event("EDUCATION", &headline, &narrative, "Lodged official university admission application.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 28,
                hours_advanced: 0,
                headline,
                narrative,
                causality_note: "University application lodged.".to_string(),
                milestone_achieved: Some("University Application Submitted".to_string()),
                world_consequences: vec!["Application registered".to_string()],
                financial_delta: -50.0,
            };
        }

        // 4. Programming Learning Intent (6 months weekends = 56 days)
        if lower.contains("programming") && (lower.contains("six months") || lower.contains("weekend")) {
            self.time.advance_days(56);
            let current_total_days = self.time.total_days;
            for npc in self.npcs.values_mut() {
                npc.last_active_day = current_total_days;
            }
            let mastery = self.get_player_mut().skills.entry("programming".to_string()).or_insert(SkillMastery {
                level: 10.0,
                experience: 0.0,
                natural_affinity: 1.0,
                last_practiced_day: current_total_days,
            });
            mastery.level += 25.0;
            mastery.last_practiced_day = current_total_days;

            let headline = "Mastered Systems Programming Principles".to_string();
            let narrative = format!("Over six months of dedicated weekend study in {}, you built command of algorithmic logic, data structures, and software architecture.", self.rule_pack.city_name);
            self.record_event("SKILL_DEVELOPMENT", &headline, &narrative, "Gained programming proficiency.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 56,
                hours_advanced: 0,
                headline,
                narrative,
                causality_note: "Six months of weekend coding mastery completed.".to_string(),
                milestone_achieved: Some("Programming Foundations Mastered".to_string()),
                world_consequences: vec!["Programming skill level increased".to_string()],
                financial_delta: 0.0,
            };
        }

        // 5. WAEC & Academic National Examinations
        if lower.contains("waec") || lower.contains("national examination") || lower.contains("study arithmetic") || lower.contains("math problems") || lower.contains("science every evening") {
            let days = if lower.contains("four weeks") { 28 } else { 7 };
            self.time.advance_days(days);
            let current_total_days = self.time.total_days;
            for npc in self.npcs.values_mut() {
                npc.last_active_day = current_total_days;
            }
            self.get_player_mut().reputation.academic_reputation += 0.35;
            let headline = "Intensive Academic Curriculum Study".to_string();
            let narrative = format!("You completed diligent academic study and examination revisions in {}. Your curriculum understanding and problem-solving mastery deepened.", self.rule_pack.city_name);
            self.record_event("ACADEMICS", &headline, &narrative, "Enhanced academic reputation and curriculum readiness.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: days,
                hours_advanced: 0,
                headline,
                narrative,
                causality_note: "Academic examination study completed.".to_string(),
                milestone_achieved: None,
                world_consequences: vec!["Academic standing elevated".to_string()],
                financial_delta: 0.0,
            };
        }

        // 6. Sports & Football Training
        if lower.contains("football") || lower.contains("coach") || lower.contains("sports grounds") || lower.contains("sports pitch") {
            self.time.advance_days(7);
            let current_total_days = self.time.total_days;
            for npc in self.npcs.values_mut() {
                npc.last_active_day = current_total_days;
            }
            self.get_player_mut().reputation.athletic_reputation += 0.35;
            self.get_player_mut().biology.fitness = (self.get_player().biology.fitness + 5.0).min(100.0);

            let headline = "Structured Sports Academy Training".to_string();
            let narrative = format!("You attended tactical football training drills and athletic conditioning on the sports pitch in {}. The coach noted your technical sharpness and discipline.", self.rule_pack.city_name);
            self.record_event("ATHLETICS", &headline, &narrative, "Athletic fitness and scout standing progressed.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 7,
                hours_advanced: 0,
                headline,
                narrative,
                causality_note: "Completed weekly football training session.".to_string(),
                milestone_achieved: None,
                world_consequences: vec!["Fitness and athletic standing increased".to_string()],
                financial_delta: 0.0,
            };
        }

        // 7. Infancy Motor & Bonding
        if lower.contains("first steps") {
            self.time.advance_hours(1);
            let headline = "First Independent Steps".to_string();
            let narrative = format!("With determined balance, you let go of the family coffee table and took your very first steps across the living room in {}.", self.rule_pack.city_name);
            self.record_event("MOTOR_MILESTONE", &headline, &narrative, "Achieved physical walking milestone.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 0,
                hours_advanced: 1,
                headline,
                narrative,
                causality_note: "Took first infant steps.".to_string(),
                milestone_achieved: Some("First Steps Taken".to_string()),
                world_consequences: vec!["Motor capability unlocked".to_string()],
                financial_delta: 0.0,
            };
        }

        if lower.contains("cuddle") {
            self.time.advance_hours(1);
            let headline = "Comforting Family Warmth".to_string();
            let narrative = format!("You cuddle close to your mother on the living room sofa in {}. A sense of total safety and gentle love fills the afternoon.", self.rule_pack.city_name);
            self.record_event("FAMILY_BOND", &headline, &narrative, "Strengthened motherly attachment bond.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 0,
                hours_advanced: 1,
                headline,
                narrative,
                causality_note: "Spent loving time with mother.".to_string(),
                milestone_achieved: None,
                world_consequences: vec!["Emotional security strengthened".to_string()],
                financial_delta: 0.0,
            };
        }

        // 8. Allowance Request
        if lower.contains("allowance") || lower.contains("pocket money") {
            self.time.advance_hours(1);
            self.get_player_mut().resources.cash += 15.0;
            let headline = "Pocket Money Allowance".to_string();
            let narrative = format!("You asked your parents for a pocket money allowance. After reviewing your conduct and diligence, they handed you spending money with advice on saving in {}.", self.rule_pack.city_name);
            self.record_event("ALLOWANCE", &headline, &narrative, "Received pocket money allowance.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 0,
                hours_advanced: 1,
                headline,
                narrative,
                causality_note: "Received allowance from parents.".to_string(),
                milestone_achieved: None,
                world_consequences: vec!["Pocket money received".to_string()],
                financial_delta: 15.0,
            };
        }

        // 9. Family Backing for University Funding
        if lower.contains("university funding") || lower.contains("tuition") {
            self.time.advance_hours(2);
            let headline = "Family Deliberation on Higher Education".to_string();
            let narrative = format!("You sat down with your father to discuss university degree options and tuition fees. He listened with deep pride and agreed, pledging full family backing for your academic ambitions in {}.", self.rule_pack.city_name);
            self.record_event("FAMILY_DELIBERATION", &headline, &narrative, "Secured father's backing for university funding.", true);

            return StepResolutionDTO {
                success: true,
                days_advanced: 0,
                hours_advanced: 2,
                headline,
                narrative,
                causality_note: "Family committed to tuition support.".to_string(),
                milestone_achieved: Some("Family University Sponsorship Secured".to_string()),
                world_consequences: vec!["Tuition support confirmed".to_string()],
                financial_delta: 0.0,
            };
        }

        // 10. Conversation Action
        if text_or_cmd.starts_with("I say to ") || lower.contains("converse") || lower.contains("talk to ") || lower.contains("ask ") {
            return self.handle_dialogue_intent(text_or_cmd);
        }

        // 11. Default structured execution
        self.advance_days(1);
        let headline = "Engaged in Meaningful Undertaking".to_string();
        let narrative = format!("You spent the day focusing on: \"{}\". Your actions resonated through your daily life in {}.", text_or_cmd, self.rule_pack.city_name);

        self.record_event("ACTION", &headline, &narrative, "Personal intent progressed.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 1,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: "Time progressed with intention.".to_string(),
            milestone_achieved: None,
            world_consequences: vec![],
            financial_delta: 0.0,
        }
    }

    pub fn attend_medical_checkup(&mut self) -> StepResolutionDTO {
        self.time.advance_hours(2);
        self.get_player_mut().biology.health_overall = 100.0;

        let headline = "Pediatric Health & Growth Review".to_string();
        let narrative = format!(
            "Your mother accompanied you to the neighborhood health clinic in {}. The pediatrician completed a thorough growth assessment and administered the scheduled routine immunization. Your development is right on track.",
            self.rule_pack.city_name
        );

        self.record_event("HEALTHCARE", &headline, &narrative, "Health and vital immunization up to date.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 2,
            headline,
            narrative,
            causality_note: "Completed official clinical pediatric checkup.".to_string(),
            milestone_achieved: Some("Clinical Health Review Completed".to_string()),
            world_consequences: vec!["Health rating set to 100%".to_string()],
            financial_delta: 0.0,
        }
    }

    pub fn handle_dialogue_intent(&mut self, dialogue_text: &str) -> StepResolutionDTO {
        self.time.advance_hours(1); // Conversations advance minutes/hours, NOT 7 days!
        
        let headline = "Heartfelt Conversation".to_string();
        let narrative = format!("You engaged in a thoughtful conversation: \"{}\". Sincere words were exchanged, strengthening mutual understanding.", dialogue_text);

        self.record_event("CONVERSATION", &headline, &narrative, "Strengthened relationship bond.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 1,
            headline,
            narrative,
            causality_note: "Spent one hour in conversation.".to_string(),
            milestone_achieved: None,
            world_consequences: vec!["Relationship trust increased".to_string()],
            financial_delta: 0.0,
        }
    }

    pub fn register_company(&mut self, name: &str, structure: &str, partners: &[String], authorized_capital: f64) -> StepResolutionDTO {
        let fee = 250.0;
        let founder_name = self.get_player().identity.full_name();
        let current_cash = self.get_player().resources.cash;
        self.get_player_mut().resources.cash = (current_cash - fee).max(0.0);

        let doc_id = format!("doc:company_{}", self.documents.len() + 1);
        let reg_number = format!("RC-{:06}", self.rng.gen_range_u32(100000, 999999));
        
        let mut fields = HashMap::new();
        fields.insert("Company Name".to_string(), name.to_string());
        fields.insert("Registration Number".to_string(), reg_number.clone());
        fields.insert("Corporate Structure".to_string(), structure.to_string());
        fields.insert("Jurisdiction".to_string(), format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name));
        fields.insert("Authorized Capital".to_string(), format!("{}{:.2}", self.rule_pack.currency_symbol, authorized_capital));
        fields.insert("Principal Founder".to_string(), founder_name);
        fields.insert("Co-Founders".to_string(), if partners.is_empty() { "None (100% Equity)".to_string() } else { partners.join(", ") });
        fields.insert("Status".to_string(), "ACTIVE_INCORPORATED".to_string());

        self.documents.insert(doc_id.clone(), DocumentRecord {
            id: doc_id,
            title: format!("Certificate of Incorporation — {}", name),
            document_type: "COMPANY_INCORPORATION".to_string(),
            issue_date: self.time.literary_date(),
            issuing_authority: format!("Corporate Affairs Commission ({})", self.rule_pack.country_name),
            registration_number: reg_number.clone(),
            fields,
            is_verified: true,
        });

        let headline = format!("Company Successfully Incorporated: {}", name);
        let narrative = format!("You officially registered {} as a {} under commercial authorities in {}. Registration number {} was issued.", name, structure, self.rule_pack.city_name, reg_number);

        self.record_event("COMPANY_INCORPORATION", &headline, &narrative, &format!("Incorporated {} with filing number {}.", name, reg_number), true);

        StepResolutionDTO {
            success: true,
            days_advanced: 3,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: format!("Company {} incorporated under {}.", name, structure),
            milestone_achieved: Some(format!("Incorporated {}", name)),
            world_consequences: vec![format!("Entity {} established", name)],
            financial_delta: -fee,
        }
    }

    pub fn travel_to_location(&mut self, destination_city_id: &str, transport_mode: &str) -> StepResolutionDTO {
        let new_rule_pack = Self::resolve_rule_pack(destination_city_id, &self.rule_pack.country_id);
        let old_city = self.rule_pack.city_name.clone();
        self.rule_pack = new_rule_pack;

        let headline = format!("Arrived in {}", self.rule_pack.city_name);
        let narrative = format!("You completed your journey from {} to {} via {}. The local environment and opportunities have updated.", old_city, self.rule_pack.city_name, transport_mode);

        self.record_event("TRAVEL", &headline, &narrative, &format!("Relocated to {}.", self.rule_pack.city_name), true);

        StepResolutionDTO {
            success: true,
            days_advanced: 1,
            hours_advanced: 4,
            headline,
            narrative,
            causality_note: format!("Traveled to {}.", self.rule_pack.city_name),
            milestone_achieved: None,
            world_consequences: vec![format!("Location updated to {}", self.rule_pack.city_name)],
            financial_delta: -80.0,
        }
    }

    // =========================================================================
    // UTILITIES & STATE RETRIEVAL
    // =========================================================================

    pub fn get_player(&self) -> &HumanEntity {
        self.persons.get("person:sim:player").expect("Player entity must exist")
    }

    pub fn get_player_mut(&mut self) -> &mut HumanEntity {
        self.persons.get_mut("person:sim:player").expect("Player entity must exist")
    }

    pub fn get_player_age(&self) -> u32 {
        let player = self.get_player();
        player.identity.calculate_age(self.time.year, self.time.month, self.time.day)
    }

    pub fn get_living_state(&self) -> LivingStateDTO {
        let player = self.get_player();
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);
        let age = self.get_player_age();

        LivingStateDTO {
            player_name: player.identity.full_name(),
            age,
            life_stage: format!("{:?}", LifeStage::from_age(age)),
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
            occupation: player.occupation.clone().unwrap_or_else(|| {
                if age < 4 { "Infancy & Growth".to_string() }
                else if age < 13 { "Primary School Student".to_string() }
                else if age < 18 { "Secondary Student".to_string() }
                else { "Independent Citizen".to_string() }
            }),
            active_processes_count: self.active_processes.len(),
            surrounding_npcs_count: self.npcs.len(),
        }
    }

    pub fn generate_today_scene(&self) -> TodaySceneDTO {
        let age = self.get_player_age();
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);

        let headline = if age < 4 {
            format!("Morning in the Nursery — {}", self.rule_pack.city_name)
        } else if age < 13 {
            format!("School Term Morning in {}", self.rule_pack.city_name)
        } else if age < 18 {
            format!("Adolescent Aspirations in {}", self.rule_pack.city_name)
        } else {
            format!("Civic Life in {}", self.rule_pack.city_name)
        };

        let narrative = if age < 4 {
            format!("Morning sunshine warms the living room rug in {}. Your mother and father are close by, attending to breakfast and household rhythms.", self.rule_pack.city_name)
        } else if age < 13 {
            format!("The morning bell sounds across the neighborhood in {}. Textbooks and notebooks rest on your desk ready for the day's lessons.", self.rule_pack.city_name)
        } else {
            format!("The city avenues of {} are active with morning commerce, university students, and professionals commuting to work.", self.rule_pack.city_name)
        };

        let present_people: Vec<String> = self.npcs.values().map(|npc| npc.base.identity.full_name()).collect();
        let environmental_objects = if age < 4 {
            vec!["Wooden Blocks".to_string(), "Picture Book".to_string(), "Family Sofa".to_string(), "Warm Blanket".to_string()]
        } else if age < 13 {
            vec!["Arithmetic Exercise Books".to_string(), "Leather Football".to_string(), "Family Desktop".to_string(), "School Bag".to_string()]
        } else {
            vec!["Smartphone".to_string(), "Personal Computer".to_string(), "Study Library".to_string(), "Corporate Registry".to_string()]
        };

        TodaySceneDTO {
            headline,
            narrative,
            weather_name: weather.name,
            weather_description: weather.description,
            location_name: format!("Family Home · {}", self.rule_pack.city_name),
            present_people,
            environmental_objects,
            subtle_details: vec!["Gentle sunlight through curtains".to_string(), "Faint city morning sounds".to_string()],
            immediate_pressures: vec![],
            location_formatted: Some(format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name)),
            life_stage: Some(format!("{:?}", LifeStage::from_age(age))),
            age: Some(age),
            circumstances: Some(vec!["Peaceful household morning".to_string()]),
        }
    }

    pub fn get_surrounding_npcs(&self) -> Vec<ContextNpcDTO> {
        self.npcs.values().map(|npc| ContextNpcDTO {
            id: npc.base.id.clone(),
            name: npc.base.identity.full_name(),
            relationship_type: if npc.base.id.contains("mother") { "Mother".to_string() }
                else if npc.base.id.contains("father") { "Father".to_string() }
                else { "Mentor / Teacher".to_string() },
            trust_description: "High Trust & Mutual Respect".to_string(),
            current_activity: npc.daily_routine.first().map(|r| r.activity_name.clone()).unwrap_or_else(|| "At home".to_string()),
        }).collect()
    }

    pub fn get_active_processes(&self) -> Vec<ContextProcessDTO> {
        self.active_processes.iter().map(|p| ContextProcessDTO {
            id: p.id.clone(),
            title: p.title.clone(),
            current_step: p.current_step,
            total_steps: p.total_steps,
            progress_percent: p.progress_percent,
            status: p.status.clone(),
        }).collect()
    }

    pub fn get_biography(&self) -> String {
        let player = self.get_player();
        format!(
            "Chronicle of {} {}\n\nBorn in {} in the year {}. Rooted in the rich cultural heritage of {}.\n\nMilestones:\n- Life began with loving family care in {}.\n- Current age: {} years.\n- Overall Health: {:.0}% | Personal Discipline: {:.0}%.",
            player.identity.first_name,
            player.identity.last_name,
            self.rule_pack.city_name,
            player.identity.birth_year,
            self.rule_pack.country_name,
            self.rule_pack.city_name,
            self.get_player_age(),
            player.biology.health_overall,
            player.psychology.discipline * 100.0
        )
    }

    pub fn get_documents(&self) -> Vec<DocumentDTO> {
        self.documents.values().map(|d| DocumentDTO {
            id: d.id.clone(),
            title: d.title.clone(),
            document_type: d.document_type.clone(),
            issue_date: d.issue_date.clone(),
            issuing_authority: d.issuing_authority.clone(),
            registration_number: d.registration_number.clone(),
            fields: d.fields.clone(),
            is_verified: d.is_verified,
        }).collect()
    }

    fn record_event(&mut self, event_type: &str, headline: &str, narrative: &str, causality: &str, success: bool) {
        self.events_ledger.push(EventRecord {
            id: format!("event:{}", self.events_ledger.len() + 1),
            timestamp: self.time.literary_date(),
            day_total: self.time.total_days,
            event_type: event_type.to_string(),
            actor_id: "person:sim:player".to_string(),
            location_id: "place:home".to_string(),
            headline: headline.to_string(),
            narrative: narrative.to_string(),
            causality_note: causality.to_string(),
            success,
        });
    }

    pub fn save_to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn load_from_string(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

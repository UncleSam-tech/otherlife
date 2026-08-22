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
    #[serde(default = "default_current_place_id")]
    pub current_place_id: String,
    #[serde(skip, default = "default_ai_bridge")]
    pub ai_bridge: AIBridge,
}

fn default_ai_bridge() -> AIBridge {
    AIBridge::new(AIBridgeConfig::default())
}

fn default_current_place_id() -> String {
    "place:home".to_string()
}

#[derive(Debug, Clone, Copy)]
struct MobilityProfile {
    transit_boarding: f64,
    transit_per_km: f64,
    taxi_boarding: f64,
    taxi_per_km: f64,
    rounding_increment: f64,
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
        let monthly_income = rule_pack.starting_costs.average_working_salary;
        let initial_cash = if start_age >= 18 {
            let savings_months = match wealth {
                WealthTier::Poverty => 0.05,
                WealthTier::WorkingClass => 0.35,
                WealthTier::MiddleClass => 1.0,
                WealthTier::UpperMiddle => 2.5,
                WealthTier::Wealthy => 8.0,
            };
            monthly_income * savings_months
        } else if start_age >= 13 {
            let allowance_share = match wealth {
                WealthTier::Poverty => 0.002,
                WealthTier::WorkingClass => 0.008,
                WealthTier::MiddleClass => 0.025,
                WealthTier::UpperMiddle => 0.06,
                WealthTier::Wealthy => 0.18,
            };
            monthly_income * allowance_share
        } else {
            0.0
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

        if start_age >= 13 {
            let public_people = if rule_pack.country_id.contains("nigeria") {
                vec![
                    ("person:city:student", "Adaeze", "Nwosu", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                    ("person:city:coworker", "Zainab", "Musa", "Operations Associate", "place:office", "Preparing a team briefing"),
                    ("person:city:regular", "Malik", "Bello", "Freelance Designer", "place:cafe", "Working over coffee"),
                    ("person:city:runner", "Tomi", "Adebayo", "Fitness Coach", "place:park", "Leading an evening running group"),
                ]
            } else if rule_pack.country_id.contains("united_kingdom") {
                vec![
                    ("person:city:student", "Isla", "Campbell", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                    ("person:city:coworker", "Amelia", "Clarke", "Operations Associate", "place:office", "Preparing a team briefing"),
                    ("person:city:regular", "Noah", "Bennett", "Freelance Designer", "place:cafe", "Working over coffee"),
                    ("person:city:runner", "Oliver", "Reid", "Fitness Coach", "place:park", "Leading an evening running group"),
                ]
            } else {
                vec![
                    ("person:city:student", "Maya", "Chen", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                    ("person:city:coworker", "Jordan", "Brooks", "Operations Associate", "place:office", "Preparing a team briefing"),
                    ("person:city:regular", "Elias", "Rivera", "Freelance Designer", "place:cafe", "Working over coffee"),
                    ("person:city:runner", "Avery", "Morgan", "Fitness Coach", "place:park", "Leading an evening running group"),
                ]
            };
            for (id, first, last, occupation, location, activity) in public_people {
                let npc = Self::make_city_npc(
                    id,
                    first,
                    last,
                    occupation,
                    location,
                    activity,
                    &rule_pack,
                    current_year,
                    time.total_days,
                );
                npcs.insert(id.to_string(), npc);
            }
        }

        // 5. Persistent city places used by the map, schedules, and local scenes.
        let places = Self::build_city_places(&rule_pack, &last_name);
        let home_id = "place:home".to_string();

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

        let registration_month = if birth_month == 12 { 12 } else { birth_month + 1 };
        documents.insert(birth_cert_id.clone(), DocumentRecord {
            id: birth_cert_id,
            title: "Official Certificate of Birth".to_string(),
            document_type: "BIRTH_CERTIFICATE".to_string(),
            issue_date: format!("{}-{:02}-{:02}", birth_year, registration_month, birth_day),
            issuing_authority,
            registration_number: reg_number,
            fields: birth_fields,
            is_verified: true,
        });

        // 7. Initial Events Ledger
        let mut events_ledger = Vec::new();
        let birth_time = TimeState::new(birth_year, birth_month, birth_day);
        events_ledger.push(EventRecord {
            id: "event:genesis".to_string(),
            timestamp: birth_time.literary_date(),
            day_total: birth_time.total_days,
            event_type: "BIRTH".to_string(),
            actor_id: player_id.clone(),
            location_id: home_id.clone(),
            headline: format!("Life Commences in {}", rule_pack.city_name),
            narrative: format!("A newborn child, {} {}, is welcomed into the world in {}, {}.", first_name, last_name, rule_pack.city_name, rule_pack.country_name),
            causality_note: format!("Rooted authentically in the {} regional rule pack.", rule_pack.country_name),
            success: true,
        });
        if start_age > 0 {
            events_ledger.push(EventRecord {
                id: "event:life_start".to_string(),
                timestamp: time.literary_date(),
                day_total: time.total_days,
                event_type: "LIFE_START".to_string(),
                actor_id: player_id.clone(),
                location_id: home_id.clone(),
                headline: format!("Your Story Resumes at Age {}", start_age),
                narrative: format!(
                    "You begin this playable chapter as a {}-year-old in {}, carrying an existing family, education, finances, relationships, and personal history into every decision ahead.",
                    start_age, rule_pack.city_name
                ),
                causality_note: "An adult-start life preserves a prior history instead of treating the character as a newborn.".to_string(),
                success: true,
            });
        }

        let initial_message_timestamp = time.literary_date();

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
            phone_messages: vec![
                PhoneMessage {
                    id: "message:mother:welcome".to_string(),
                    sender_id: "person:sim:mother".to_string(),
                    sender_name: format!("{} {}", mother_first, last_name),
                    recipient_id: player_id.clone(),
                    text: "Please remember to take care of yourself today. I am proud of you.".to_string(),
                    timestamp: initial_message_timestamp.clone(),
                    is_read: false,
                    is_delivered: true,
                },
                PhoneMessage {
                    id: "message:father:welcome".to_string(),
                    sender_id: "person:sim:father".to_string(),
                    sender_name: format!("{} {}", father_first, last_name),
                    recipient_id: player_id.clone(),
                    text: "Let me know if you need any guidance with your plans.".to_string(),
                    timestamp: initial_message_timestamp,
                    is_read: false,
                    is_delivered: true,
                },
            ],
            active_call: None,
            events_ledger,
            rule_pack,
            current_place_id: home_id,
            ai_bridge: AIBridge::new(AIBridgeConfig::default()),
        }
    }

    fn build_city_places(rule_pack: &RegionalRulePack, household_name: &str) -> HashMap<String, WorldPlace> {
        let city_id = rule_pack.city_id.clone();
        let city = rule_pack.city_name.clone();
        let specs = vec![
            ("place:home", format!("{} Family Home", household_name), PlaceType::Residence, "Residential District", 0, vec!["REST", "FAMILY_BONDING", "USE_DEVICES"]),
            ("place:office", format!("{} Business District", city), PlaceType::Workplace, "Central Business District", 18, vec!["WORK_SHIFT", "JOB_INTERVIEW", "BUSINESS_MEETING"]),
            ("place:university", format!("{} Metropolitan University", city), PlaceType::Education, "University Quarter", 16, vec!["PROGRAM_APPLICATION", "LECTURE", "MEET_STUDENTS"]),
            ("place:cafe", "Junction Café & Social House".to_string(), PlaceType::CommercialVenue, "Cultural Quarter", 13, vec!["ORDER_MEAL", "SOCIALIZE", "INFORMAL_MEETING"]),
            ("place:civic_center", format!("{} Civic & Immigration Centre", city), PlaceType::CivicCenter, "Government Quarter", 18, vec!["COMPANY_FILING", "VISA_APPOINTMENT", "RESIDENCY_APPLICATION"]),
            ("place:clinic", format!("{} Community Hospital", city), PlaceType::MedicalClinic, "Health District", 0, vec!["CHECKUP", "TREATMENT", "VISIT_PERSON"]),
            ("place:park", "Unity Park & Recreation Grounds".to_string(), PlaceType::AthleticField, "Riverside District", 0, vec!["EXERCISE", "SOCIALIZE", "FOOTBALL"]),
            ("place:transport_terminal", format!("{} Transport Terminal", city), PlaceType::TrainStation, "Transit District", 0, vec!["CITY_TRAVEL", "INTERCITY_BOOKING", "ARRIVAL"]),
            ("place:school", format!("{} District School", city), PlaceType::Education, "School District", 4, vec!["ATTEND_CLASS", "STUDY", "MEET_STUDENTS"]),
            ("place:sports_academy", format!("{} Sports Academy", city), PlaceType::AthleticField, "Stadium District", 8, vec!["TRAIN", "TRIAL", "MATCH"]),
        ];

        specs.into_iter().map(|(id, name, place_type, district, min_age, actions)| {
            let id = id.to_string();
            (id.clone(), WorldPlace {
                id,
                name,
                place_type,
                city_id: city_id.clone(),
                district_name: district.to_string(),
                required_min_age: min_age,
                affords_activities: actions.into_iter().map(str::to_string).collect(),
            })
        }).collect()
    }

    fn make_city_npc(
        id: &str,
        first_name: &str,
        last_name: &str,
        occupation: &str,
        location_id: &str,
        activity: &str,
        rule_pack: &RegionalRulePack,
        current_year: i32,
        current_day: i64,
    ) -> AutonomousNPC {
        AutonomousNPC {
            base: HumanEntity {
                id: id.to_string(),
                identity: IdentityProfile {
                    first_name: first_name.to_string(),
                    last_name: last_name.to_string(),
                    birth_year: current_year - 24,
                    birth_month: 5,
                    birth_day: 18,
                    sex: "Unspecified".to_string(),
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
                    cash: 1400.0,
                    household_wealth_tier: WealthTier::MiddleClass,
                    living_arrangement: "CITY_APARTMENT".to_string(),
                    tools_available: vec!["SMARTPHONE".to_string()],
                },
                relationships: HashMap::new(),
                occupation: Some(occupation.to_string()),
                is_player: false,
            },
            daily_routine: vec![ScheduledActivity {
                start_hour: 7,
                end_hour: 23,
                location_id: location_id.to_string(),
                activity_name: activity.to_string(),
                description: format!("{} is currently at this location.", first_name),
            }],
            communication_style: CommunicationStyle::Supportive,
            personality: NpcPersonality {
                communication_style: CommunicationStyle::Supportive,
                strictness: 0.35,
            },
            current_goal: format!("Build a meaningful life in {}", rule_pack.city_name),
            last_active_day: current_day,
        }
    }

    fn ensure_world_places(&mut self) {
        let household_name = self.get_player().identity.last_name.clone();
        let canonical = Self::build_city_places(&self.rule_pack, &household_name);
        for (id, place) in canonical {
            self.places.entry(id).or_insert(place);
        }
        if !self.places.contains_key(&self.current_place_id) {
            self.current_place_id = default_current_place_id();
        }
    }

    fn ensure_city_people(&mut self) {
        if self.get_player_age() < 13 {
            return;
        }
        let specs = if self.rule_pack.country_id.contains("nigeria") {
            vec![
                ("person:city:student", "Adaeze", "Nwosu", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                ("person:city:coworker", "Zainab", "Musa", "Operations Associate", "place:office", "Preparing a team briefing"),
                ("person:city:regular", "Malik", "Bello", "Freelance Designer", "place:cafe", "Working over coffee"),
                ("person:city:runner", "Tomi", "Adebayo", "Fitness Coach", "place:park", "Leading an evening running group"),
            ]
        } else if self.rule_pack.country_id.contains("united_kingdom") {
            vec![
                ("person:city:student", "Isla", "Campbell", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                ("person:city:coworker", "Amelia", "Clarke", "Operations Associate", "place:office", "Preparing a team briefing"),
                ("person:city:regular", "Noah", "Bennett", "Freelance Designer", "place:cafe", "Working over coffee"),
                ("person:city:runner", "Oliver", "Reid", "Fitness Coach", "place:park", "Leading an evening running group"),
            ]
        } else {
            vec![
                ("person:city:student", "Maya", "Chen", "Undergraduate Student", "place:university", "Studying in the campus commons"),
                ("person:city:coworker", "Jordan", "Brooks", "Operations Associate", "place:office", "Preparing a team briefing"),
                ("person:city:regular", "Elias", "Rivera", "Freelance Designer", "place:cafe", "Working over coffee"),
                ("person:city:runner", "Avery", "Morgan", "Fitness Coach", "place:park", "Leading an evening running group"),
            ]
        };
        for (id, first, last, occupation, location, activity) in specs {
            if self.npcs.contains_key(id) {
                continue;
            }
            let npc = Self::make_city_npc(
                id,
                first,
                last,
                occupation,
                location,
                activity,
                &self.rule_pack,
                self.time.year,
                self.time.total_days,
            );
            self.npcs.insert(id.to_string(), npc);
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

    pub fn age_up_one_year(&mut self) -> StepResolutionDTO {
        let previous_age = self.get_player_age();
        let previous_stage = LifeStage::from_age(previous_age);
        self.time.year += 1;
        self.time.total_days += 365;
        let current_days = self.time.total_days;
        for npc in self.npcs.values_mut() {
            npc.last_active_day = current_days;
        }

        let new_age = self.get_player_age();
        let new_stage = LifeStage::from_age(new_age);
        let player = self.get_player_mut();
        player.biology.energy_level = 90.0;
        player.psychology.stress_level = (player.psychology.stress_level * 0.8 + 4.0).clamp(0.0, 100.0);
        player.psychology.confidence = (player.psychology.confidence + 0.01).clamp(0.0, 1.0);
        if new_age > 50 {
            player.biology.health_overall = (player.biology.health_overall - 0.35).clamp(0.0, 100.0);
        }

        let mut progressed = Vec::new();
        for process in &mut self.active_processes {
            if process.status != "COMPLETED" && process.current_step < process.total_steps {
                process.current_step += 1;
                process.progress_percent = process.current_step * 100 / process.total_steps.max(1);
                if process.current_step >= process.total_steps {
                    process.status = "COMPLETED".to_string();
                }
                progressed.push(process.title.clone());
            }
        }

        let headline = format!("Age {} — A New Chapter", new_age);
        let narrative = if previous_stage != new_stage {
            format!("A full year passed in {}. You turned {} and entered the {:?} stage of life; new responsibilities, relationships, and opportunities are now available.", self.rule_pack.city_name, new_age, new_stage)
        } else if progressed.is_empty() {
            format!("A full year passed in {}. You turned {}, carrying your relationships, health, money, education, and choices into the next chapter.", self.rule_pack.city_name, new_age)
        } else {
            format!("You turned {} in {}. Time advanced your ongoing commitments: {}.", new_age, self.rule_pack.city_name, progressed.join(", "))
        };
        let milestone = if previous_stage != new_stage { Some(format!("Entered {:?}", new_stage)) } else { None };
        self.record_event("AGE_UP", &headline, &narrative, "Advanced exactly one calendar year and progressed every active life process once.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 365,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: "One year passed; persistent stats, processes, relationships, and resources were retained.".to_string(),
            milestone_achieved: milestone,
            world_consequences: progressed.into_iter().map(|title| format!("Progressed: {}", title)).collect(),
            financial_delta: 0.0,
        }
    }

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

    pub fn send_phone_message(&mut self, recipient_id: &str, text: &str) -> StepResolutionDTO {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Message Not Sent".to_string(),
                narrative: "Write a message before pressing send.".to_string(),
                causality_note: "Empty messages are not stored.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        let recipient_name = self.npcs.get(recipient_id)
            .map(|npc| npc.base.identity.full_name())
            .unwrap_or_else(|| "Unknown contact".to_string());
        if recipient_name == "Unknown contact" {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Contact Unavailable".to_string(),
                narrative: "That person is not in your saved contacts.".to_string(),
                causality_note: "Recipient ID did not resolve to a known person.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        let message = PhoneMessage {
            id: format!("message:{}", self.phone_messages.len() + 1),
            sender_id: "person:sim:player".to_string(),
            sender_name: self.get_player().identity.full_name(),
            recipient_id: recipient_id.to_string(),
            text: trimmed.to_string(),
            timestamp: self.time.literary_date(),
            is_read: true,
            is_delivered: true,
        };
        self.phone_messages.push(message);

        let headline = format!("Message Sent to {}", recipient_name);
        let narrative = format!("Your message to {} was delivered and saved in the conversation history.", recipient_name);
        self.record_event("PHONE_MESSAGE", &headline, &narrative, "Stored a delivered phone message without advancing a full day.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 0,
            headline,
            narrative,
            causality_note: "Message persisted in the local simulation save.".to_string(),
            milestone_achieved: None,
            world_consequences: vec!["Conversation history updated".to_string()],
            financial_delta: 0.0,
        }
    }

    fn place_map_profile(place_id: &str) -> (f32, f32, u32, f64, &'static str, u8, u8) {
        match place_id {
            "place:home" => (16.0, 72.0, 0, 0.0, "Your household, personal devices, documents, and family routines.", 0, 24),
            "place:office" => (62.0, 28.0, 34, 4.0, "Offices, employers, meeting rooms, and the commercial life of the city.", 7, 20),
            "place:university" => (32.0, 24.0, 29, 3.0, "Faculties, admissions, lecture rooms, libraries, and student life.", 7, 21),
            "place:cafe" => (51.0, 54.0, 18, 2.0, "A social venue for meals, chance encounters, informal work, and conversation.", 7, 23),
            "place:civic_center" => (79.0, 50.0, 41, 5.0, "Government counters for registration, immigration, residence, and civic records.", 8, 17),
            "place:clinic" => (23.0, 47.0, 22, 3.0, "Medical consultation, treatment, hospital work, and visiting hours.", 0, 24),
            "place:park" => (48.0, 78.0, 16, 1.5, "Public green space for exercise, sport, leisure, and meeting people.", 5, 22),
            "place:transport_terminal" => (86.0, 78.0, 46, 6.0, "Local connections, intercity departures, arrivals, and travel services.", 0, 24),
            "place:school" => (19.0, 24.0, 24, 2.5, "Classrooms, teachers, examinations, and school activities.", 7, 17),
            "place:sports_academy" => (68.0, 76.0, 31, 3.0, "Training grounds, team sessions, competitive trials, and matches.", 6, 22),
            _ => (50.0, 50.0, 20, 2.0, "A place within the living city.", 0, 24),
        }
    }

    fn mobility_profile(&self) -> MobilityProfile {
        match self.rule_pack.currency_code.as_str() {
            "NGN" => MobilityProfile { transit_boarding: 400.0, transit_per_km: 80.0, taxi_boarding: 800.0, taxi_per_km: 350.0, rounding_increment: 50.0 },
            "GBP" => MobilityProfile { transit_boarding: 2.0, transit_per_km: 0.16, taxi_boarding: 3.8, taxi_per_km: 2.2, rounding_increment: 0.05 },
            _ => MobilityProfile { transit_boarding: if self.rule_pack.city_name == "Houston" { 1.25 } else { 2.50 }, transit_per_km: 0.06, taxi_boarding: 4.0, taxi_per_km: 2.35, rounding_increment: 0.05 },
        }
    }

    fn round_local_money(value: f64, increment: f64) -> f64 {
        (value / increment).round() * increment
    }

    fn local_journey_quote(&self, destination_id: &str, transport_mode: &str) -> (f64, u32, f64) {
        let (origin_x, origin_y, _, _, _, _, _) = Self::place_map_profile(&self.current_place_id);
        let (destination_x, destination_y, _, _, _, _, _) = Self::place_map_profile(destination_id);
        let map_distance = (((destination_x - origin_x).powi(2) + (destination_y - origin_y).powi(2)) as f64).sqrt();
        let distance_km = if destination_id == self.current_place_id { 0.0 } else { (map_distance * 0.18 * 1.18).max(0.8) };
        let economy = self.mobility_profile();
        match transport_mode.to_lowercase().as_str() {
            "walk" => (distance_km, ((distance_km / 4.8) * 60.0).ceil().max(5.0) as u32, 0.0),
            "taxi" => {
                let fare = Self::round_local_money(economy.taxi_boarding + economy.taxi_per_km * distance_km, economy.rounding_increment);
                (distance_km, (5.0 + distance_km / 28.0 * 60.0).ceil() as u32, fare)
            }
            _ => {
                let fare = Self::round_local_money(economy.transit_boarding + economy.transit_per_km * distance_km, economy.rounding_increment);
                (distance_km, (8.0 + distance_km / 22.0 * 60.0).ceil() as u32, fare)
            }
        }
    }

    fn format_local_money(&self, amount: f64) -> String {
        if self.rule_pack.currency_code == "NGN" {
            format!("{}{:.0} {}", self.rule_pack.currency_symbol, amount, self.rule_pack.currency_code)
        } else {
            format!("{}{:.2} {}", self.rule_pack.currency_symbol, amount, self.rule_pack.currency_code)
        }
    }

    fn npc_activity_at<'a>(&self, npc: &'a AutonomousNPC) -> (String, String) {
        if let Some(activity) = npc.daily_routine.iter().find(|activity| {
            self.time.hour >= activity.start_hour && self.time.hour < activity.end_hour
        }) {
            return (activity.location_id.clone(), activity.activity_name.clone());
        }
        ("place:home".to_string(), "Off schedule / at home".to_string())
    }

    pub fn get_world_map(&self) -> Vec<WorldMapPlaceDTO> {
        let age = self.get_player_age();
        let mut map: Vec<WorldMapPlaceDTO> = self.places.values()
            .filter(|place| age >= place.required_min_age)
            .map(|place| {
                let (map_x, map_y, _, _, description, open_hour, close_hour) = Self::place_map_profile(&place.id);
                let (distance_km, walk_minutes, _) = self.local_journey_quote(&place.id, "Walk");
                let (_, public_transit_minutes, public_transit_cost) = self.local_journey_quote(&place.id, "Public Transit");
                let (_, taxi_minutes, taxi_cost) = self.local_journey_quote(&place.id, "Taxi");
                let present_people_count = self.npcs.values()
                    .filter(|npc| self.npc_activity_at(npc).0 == place.id)
                    .count();
                WorldMapPlaceDTO {
                    id: place.id.clone(),
                    name: place.name.clone(),
                    category: format!("{:?}", place.place_type),
                    district_name: place.district_name.clone(),
                    description: description.to_string(),
                    map_x,
                    map_y,
                    travel_minutes: public_transit_minutes,
                    travel_cost: public_transit_cost,
                    distance_km,
                    walk_minutes,
                    public_transit_minutes,
                    public_transit_cost,
                    taxi_minutes,
                    taxi_cost,
                    is_current: self.current_place_id == place.id,
                    is_open: open_hour == 0 && close_hour == 24 || self.time.hour >= open_hour && self.time.hour < close_hour,
                    present_people_count,
                }
            })
            .collect();
        map.sort_by(|a, b| a.id.cmp(&b.id));
        map
    }

    pub fn commute_to_place(&mut self, place_id: &str, transport_mode: &str) -> StepResolutionDTO {
        self.ensure_world_places();
        let Some(place) = self.places.get(place_id).cloned() else {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Destination Unavailable".to_string(),
                narrative: "That destination is not part of the current city map.".to_string(),
                causality_note: "Movement was rejected because the place does not exist.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        };
        if place.id == self.current_place_id {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Already Here".to_string(),
                narrative: format!("You are already at {}.", place.name),
                causality_note: "No travel time or money was spent.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        if self.get_player_age() < place.required_min_age {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Cannot Travel Alone".to_string(),
                narrative: "This destination requires an accompanying adult at your current age.".to_string(),
                causality_note: "Age-appropriate movement rule enforced.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        let (distance_km, minutes, cost) = self.local_journey_quote(place_id, transport_mode);
        if self.get_player().resources.cash < cost {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Journey Payment Declined".to_string(),
                narrative: format!("You need {} for this journey.", self.format_local_money(cost)),
                causality_note: "The route cannot overdraw local funds.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        self.get_player_mut().resources.cash -= cost;
        let hours = ((minutes + 59) / 60).clamp(1, 12) as u8;
        self.time.advance_hours(hours);
        self.current_place_id = place.id.clone();
        let headline = format!("Arrived at {}", place.name);
        let fare_description = if cost == 0.0 { "free".to_string() } else { self.format_local_money(cost) };
        let narrative = format!("You travelled {:.1} km through {} by {}. The journey took about {} minutes and cost {}; the people and actions around you now reflect this location and time.", distance_km, self.rule_pack.city_name, transport_mode, minutes, fare_description);
        self.record_event("LOCAL_TRAVEL", &headline, &narrative, &format!("Moved to {} and advanced local time.", place.id), true);
        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: hours,
            headline,
            narrative,
            causality_note: format!("Current physical place is now {}.", place.name),
            milestone_achieved: None,
            world_consequences: vec![format!("Entered {}", place.district_name)],
            financial_delta: -cost,
        }
    }

    pub fn apply_to_university(
        &mut self,
        institution: &str,
        degree_program: &str,
        primary_course: &str,
        study_mode: &str,
        funding_plan: &str,
    ) -> StepResolutionDTO {
        if self.current_place_id != "place:university" {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Visit the University First".to_string(),
                narrative: "Admissions cannot be completed from the current location. Travel to the university campus or use its admissions portal.".to_string(),
                causality_note: "Institutional action requires physical or device context.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        if self.get_player_age() < 16 {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Admission Stage Not Yet Available".to_string(),
                narrative: "You have not yet reached the minimum stage for this higher-education application.".to_string(),
                causality_note: "Age and education-stage requirements enforced.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        let fee = 25.0;
        if self.get_player().resources.cash < fee {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Application Fee Unavailable".to_string(),
                narrative: format!("The application costs {}{:.2}.", self.rule_pack.currency_symbol, fee),
                causality_note: "Application payment is required before submission.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        let process_id = format!("proc:university:{}", degree_program.to_lowercase().replace(' ', "_"));
        if self.active_processes.iter().any(|process| process.id == process_id) {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Application Already Active".to_string(),
                narrative: format!("Your {} application is already being tracked.", degree_program),
                causality_note: "Duplicate programme applications are prevented.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        self.get_player_mut().resources.cash -= fee;
        self.time.advance_hours(2);
        self.active_processes.push(LifeProcess {
            id: process_id,
            process_type: ProcessType::UniversityAdmission,
            title: format!("{} — {}", degree_program, institution),
            target_institution_id: Some("place:university".to_string()),
            current_step: 1,
            total_steps: 6,
            progress_percent: 16,
            status: "APPLICATION_SUBMITTED".to_string(),
            missing_requirements: vec!["Academic records review".to_string(), "Admissions decision".to_string(), "Offer acceptance".to_string()],
            next_appointment_day: Some(self.time.total_days + 7),
        });
        let reference = format!("UNI-{:06}", self.rng.gen_range_u32(100000, 999999));
        let mut fields = HashMap::new();
        fields.insert("Institution".to_string(), institution.to_string());
        fields.insert("Degree Programme".to_string(), degree_program.to_string());
        fields.insert("Primary Course".to_string(), primary_course.to_string());
        fields.insert("Study Mode".to_string(), study_mode.to_string());
        fields.insert("Funding Plan".to_string(), funding_plan.to_string());
        fields.insert("Status".to_string(), "APPLICATION_SUBMITTED".to_string());
        self.documents.insert(format!("doc:{}", reference), DocumentRecord {
            id: format!("doc:{}", reference),
            title: format!("University Application — {}", degree_program),
            document_type: "UNIVERSITY_APPLICATION".to_string(),
            issue_date: self.time.literary_date(),
            issuing_authority: institution.to_string(),
            registration_number: reference,
            fields,
            is_verified: true,
        });
        let headline = format!("Application Submitted: {}", degree_program);
        let narrative = format!("You chose {} as your primary course within {} at {}. Your application now proceeds through records review, decision, offer, enrollment, and the academic timetable.", primary_course, degree_program, institution);
        self.record_event("UNIVERSITY_APPLICATION", &headline, &narrative, "Created a course-specific six-stage admissions process.", true);
        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 2,
            headline,
            narrative,
            causality_note: "Programme, course, study mode, funding plan, fee, and admissions status persisted.".to_string(),
            milestone_achieved: Some("Entered university admissions".to_string()),
            world_consequences: vec!["Admissions review scheduled".to_string()],
            financial_delta: -fee,
        }
    }

    pub fn converse_with_npc(&mut self, npc_id: &str, dialogue: &str) -> StepResolutionDTO {
        let Some(npc) = self.npcs.get(npc_id).cloned() else {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Person Not Found".to_string(),
                narrative: "That person is not currently part of this world.".to_string(),
                causality_note: "Conversation target validation failed.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        };
        let (npc_location, activity) = self.npc_activity_at(&npc);
        if npc_location != self.current_place_id {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: format!("{} Is Elsewhere", npc.base.identity.first_name),
                narrative: format!("You cannot begin an in-person conversation because {} is currently {}.", npc.base.identity.full_name(), activity),
                causality_note: "NPC schedules and physical presence are enforced.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        let npc_name = npc.base.identity.full_name();
        let was_new = !self.get_player().relationships.contains_key(npc_id);
        let memory = EpisodicMemoryRecord {
            day_occurred: self.time.total_days,
            headline: format!("Conversation at {}", self.places.get(&self.current_place_id).map(|place| place.name.as_str()).unwrap_or("the current place")),
            description: dialogue.to_string(),
            emotional_valence: 0.25,
            importance: 0.35,
        };
        let relationship = self.get_player_mut().relationships.entry(npc_id.to_string()).or_insert(RelationshipEdge {
            target_entity_id: npc_id.to_string(),
            target_name: npc_name.clone(),
            relationship_type: "Acquaintance".to_string(),
            affinity: 0.15,
            trust: 0.10,
            respect: 0.15,
            memories: vec![],
        });
        relationship.affinity = (relationship.affinity + 0.03).min(1.0);
        relationship.trust = (relationship.trust + 0.02).min(1.0);
        relationship.memories.push(memory);
        self.time.advance_hours(1);
        let headline = if was_new { format!("Met {}", npc_name) } else { format!("Spoke with {}", npc_name) };
        let narrative = format!("You spoke with {} while they were {}. The conversation became part of your shared relationship memory.", npc_name, activity.to_lowercase());
        self.record_event("IN_PERSON_CONVERSATION", &headline, &narrative, "Updated relationship affinity, trust, and episodic memory.", true);
        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 1,
            headline,
            narrative,
            causality_note: "Conversation and relationship state persisted locally.".to_string(),
            milestone_achieved: if was_new { Some(format!("Met {}", npc_name)) } else { None },
            world_consequences: vec![format!("Relationship with {} developed", npc_name)],
            financial_delta: 0.0,
        }
    }

    pub fn apply_for_job(&mut self, job_id: &str, company_id: &str, title: &str, company_name: &str) -> StepResolutionDTO {
        self.apply_for_job_detailed(
            job_id,
            company_id,
            title,
            company_name,
            "General résumé on file",
            "I would like to be considered for this role.",
            "Available by arrangement",
        )
    }

    pub fn apply_for_job_detailed(
        &mut self,
        job_id: &str,
        company_id: &str,
        title: &str,
        company_name: &str,
        resume_summary: &str,
        cover_letter: &str,
        availability: &str,
    ) -> StepResolutionDTO {
        let process_id = format!("proc:job:{}", job_id);
        if self.active_processes.iter().any(|process| process.id == process_id) {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Application Already Submitted".to_string(),
                narrative: format!("Your application for {} at {} is already being tracked.", title, company_name),
                causality_note: "Duplicate job applications are prevented.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        self.time.advance_hours(1);
        self.active_processes.push(LifeProcess {
            id: process_id,
            process_type: ProcessType::JobApplication,
            title: format!("{} — {}", title, company_name),
            target_institution_id: Some(company_id.to_string()),
            current_step: 1,
            total_steps: 4,
            progress_percent: 25,
            status: "APPLICATION_SUBMITTED".to_string(),
            missing_requirements: vec!["Await employer screening".to_string()],
            next_appointment_day: Some(self.time.total_days + 3),
        });
        let application_reference = format!("APP-{:06}", self.rng.gen_range_u32(100000, 999999));
        let mut application_fields = HashMap::new();
        application_fields.insert("Role".to_string(), title.to_string());
        application_fields.insert("Employer".to_string(), company_name.to_string());
        application_fields.insert("Resume Profile".to_string(), resume_summary.to_string());
        application_fields.insert("Cover Letter".to_string(), cover_letter.to_string());
        application_fields.insert("Availability".to_string(), availability.to_string());
        application_fields.insert("Status".to_string(), "APPLICATION_SUBMITTED".to_string());
        self.documents.insert(format!("doc:job_application:{}", job_id), DocumentRecord {
            id: format!("doc:job_application:{}", job_id),
            title: format!("Job Application — {}", title),
            document_type: "JOB_APPLICATION_RECORD".to_string(),
            issue_date: self.time.literary_date(),
            issuing_authority: company_name.to_string(),
            registration_number: application_reference,
            fields: application_fields,
            is_verified: true,
        });
        self.letters_inbox.push(LetterNotification {
            id: format!("letter:job:{}", job_id),
            sender: company_name.to_string(),
            subject: format!("Application received — {}", title),
            body: format!("We received your application for {}. The next stage is employer screening; expect an update within three days.", title),
            is_read: false,
            date_received: self.time.literary_date(),
        });

        let headline = format!("Application Submitted: {}", title);
        let narrative = format!("You completed and submitted a formal application to {}. A tracked four-stage hiring process now appears in your active processes.", company_name);
        self.record_event("JOB_APPLICATION", &headline, &narrative, "Created a durable hiring process and confirmation notice.", true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 1,
            headline,
            narrative,
            causality_note: "Application, screening status, and employer acknowledgement persisted.".to_string(),
            milestone_achieved: Some("Entered a hiring process".to_string()),
            world_consequences: vec!["Employer screening scheduled".to_string()],
            financial_delta: 0.0,
        }
    }

    pub fn register_company(&mut self, name: &str, structure: &str, partners: &[String], authorized_capital: f64) -> StepResolutionDTO {
        let current_city = format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name);
        self.register_company_detailed(
            name,
            structure,
            partners,
            authorized_capital,
            "General commercial services",
            &current_city,
        )
    }

    pub fn register_company_detailed(
        &mut self,
        name: &str,
        structure: &str,
        partners: &[String],
        authorized_capital: f64,
        business_activity: &str,
        registered_address: &str,
    ) -> StepResolutionDTO {
        let fee = 250.0;
        if self.get_player_age() < 18 {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Company Registration Unavailable".to_string(),
                narrative: "You must be at least 18 to complete this incorporation filing yourself.".to_string(),
                causality_note: "Legal-age rule enforced by the simulation engine.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        if self.get_player().resources.cash < fee {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Insufficient Filing Funds".to_string(),
                narrative: format!("The incorporation filing costs {}{:.2}; your current balance is too low.", self.rule_pack.currency_symbol, fee),
                causality_note: "Registration cannot silently overdraw the player account.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        let founder_name = self.get_player().identity.full_name();
        let current_cash = self.get_player().resources.cash;
        self.get_player_mut().resources.cash = current_cash - fee;
        self.time.advance_days(3);

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
        fields.insert("Business Activity".to_string(), business_activity.to_string());
        fields.insert("Registered Office".to_string(), registered_address.to_string());
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

        self.active_processes.push(LifeProcess {
            id: format!("proc:company:{}", reg_number),
            process_type: ProcessType::CompanyRegistration,
            title: format!("Company registration — {}", name),
            target_institution_id: None,
            current_step: 4,
            total_steps: 4,
            progress_percent: 100,
            status: "INCORPORATED_ACTIVE".to_string(),
            missing_requirements: vec![],
            next_appointment_day: None,
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

    pub fn advance_company_operation(&mut self, company_name: &str, operation: &str, plan: &str) -> StepResolutionDTO {
        let owns_company = self.documents.values().any(|document| {
            document.document_type == "COMPANY_INCORPORATION"
                && document.fields.get("Company Name").map(String::as_str) == Some(company_name)
        });
        if !owns_company {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "No Operating Company".to_string(),
                narrative: "Incorporate or acquire a company before attempting business operations.".to_string(),
                causality_note: "Business ownership validation failed.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        if self.current_place_id != "place:office" {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Go to the Business District".to_string(),
                narrative: "This operational meeting requires you to be at the office and business district.".to_string(),
                causality_note: "Physical workplace context enforced.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }
        self.time.advance_hours(2);
        let process_id = format!("proc:business_operations:{}", company_name.to_lowercase().replace(' ', "_"));
        if let Some(process) = self.active_processes.iter_mut().find(|process| process.id == process_id) {
            process.current_step = (process.current_step + 1).min(process.total_steps);
            process.progress_percent = process.current_step * 100 / process.total_steps;
            process.status = format!("{}_IN_PROGRESS", operation.to_uppercase().replace(' ', "_"));
            process.next_appointment_day = Some(self.time.total_days + 2);
        } else {
            self.active_processes.push(LifeProcess {
                id: process_id,
                process_type: ProcessType::BusinessOperations,
                title: format!("Operating {}", company_name),
                target_institution_id: Some("place:office".to_string()),
                current_step: 1,
                total_steps: 8,
                progress_percent: 12,
                status: format!("{}_IN_PROGRESS", operation.to_uppercase().replace(' ', "_")),
                missing_requirements: vec!["Build a team".to_string(), "Win customers".to_string(), "Manage cash flow".to_string(), "Deliver products or services".to_string()],
                next_appointment_day: Some(self.time.total_days + 2),
            });
        }
        let reference = format!("BIZ-{:06}", self.rng.gen_range_u32(100000, 999999));
        let mut fields = HashMap::new();
        fields.insert("Company".to_string(), company_name.to_string());
        fields.insert("Operation".to_string(), operation.to_string());
        fields.insert("Plan / Response".to_string(), plan.to_string());
        fields.insert("Status".to_string(), "IN_PROGRESS".to_string());
        self.documents.insert(format!("doc:{}", reference), DocumentRecord {
            id: format!("doc:{}", reference),
            title: format!("Business Activity — {}", operation),
            document_type: "BUSINESS_OPERATION_RECORD".to_string(),
            issue_date: self.time.literary_date(),
            issuing_authority: company_name.to_string(),
            registration_number: reference,
            fields,
            is_verified: true,
        });
        let headline = format!("{}: {}", company_name, operation);
        let narrative = format!("You spent two hours on {}. This moved the company's operating process forward but did not guarantee a hire, investment, customer, or successful product.", operation.to_lowercase());
        self.record_event("BUSINESS_OPERATION", &headline, &narrative, "Business plan response and ongoing operational process persisted.", true);
        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: 2,
            headline,
            narrative,
            causality_note: "Incorporation now leads into an ongoing company operating cycle.".to_string(),
            milestone_achieved: None,
            world_consequences: vec![format!("{} operations advanced", company_name)],
            financial_delta: 0.0,
        }
    }

    pub fn travel_to_location(&mut self, destination_city_id: &str, transport_mode: &str, stay_days: u32) -> StepResolutionDTO {
        let base_fare = self.base_travel_fare(destination_city_id, transport_mode);
        self.travel_to_location_detailed(
            destination_city_id,
            transport_mode,
            stay_days,
            "OTHERLIFE Travel Desk",
            "Standard flexible",
            base_fare,
            if stay_days > 0 { "Accommodation reserved" } else { "No accommodation reservation" },
            "Next available departure",
            "Visit",
            "Visitor / tourist entry",
        )
    }

    fn currency_units_per_usd(currency_code: &str) -> f64 {
        match currency_code { "NGN" => 1500.0, "GBP" => 0.77, _ => 1.0 }
    }

    fn base_travel_fare(&self, destination_city_id: &str, transport_mode: &str) -> f64 {
        let destination = Self::resolve_rule_pack(destination_city_id, &self.rule_pack.country_id);
        let international = destination.country_id != self.rule_pack.country_id;
        let usd_fare = if international {
            match transport_mode.to_lowercase().as_str() { "flight" => 900.0, _ => 1200.0 }
        } else {
            match transport_mode.to_lowercase().as_str() { "flight" => 120.0, "train" => 55.0, "private car" => 75.0, _ => 25.0 }
        };
        let raw = usd_fare * Self::currency_units_per_usd(&self.rule_pack.currency_code);
        let increment = if self.rule_pack.currency_code == "NGN" { 1000.0 } else { 1.0 };
        Self::round_local_money(raw, increment)
    }

    pub fn travel_to_location_detailed(
        &mut self,
        destination_city_id: &str,
        transport_mode: &str,
        stay_days: u32,
        operator_name: &str,
        service_class: &str,
        quoted_fare: f64,
        accommodation: &str,
        departure_timing: &str,
        journey_type: &str,
        immigration_pathway: &str,
    ) -> StepResolutionDTO {
        let old_country_id = self.rule_pack.country_id.clone();
        let old_currency_code = self.rule_pack.currency_code.clone();
        let old_currency_symbol = self.rule_pack.currency_symbol.clone();
        let new_rule_pack = Self::resolve_rule_pack(destination_city_id, &self.rule_pack.country_id);
        let is_international = new_rule_pack.country_id != old_country_id;
        let old_city = self.rule_pack.city_name.clone();
        if new_rule_pack.city_id == self.rule_pack.city_id {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Choose Another Destination".to_string(),
                narrative: format!("You are already in {}.", old_city),
                causality_note: "No journey was booked because origin and destination match.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        let journey_hours: u8 = match transport_mode.to_lowercase().as_str() {
            "flight" => 3,
            "train" => 6,
            "private car" => 8,
            _ => 10,
        };
        if is_international && transport_mode.to_lowercase() != "flight" {
            return StepResolutionDTO {
                success: false, days_advanced: 0, hours_advanced: 0,
                headline: "Transport Does Not Serve This International Route".to_string(),
                narrative: "Choose a flight for this cross-border route; local buses, trains, and private-car itineraries cannot complete it in the current travel network.".to_string(),
                causality_note: "International route and transport compatibility enforced.".to_string(),
                milestone_achieved: None, world_consequences: vec![], financial_delta: 0.0,
            };
        }
        let base_fare = self.base_travel_fare(destination_city_id, transport_mode);
        let fare = if quoted_fare.is_finite() && quoted_fare >= base_fare && quoted_fare <= base_fare * 2.0 {
            quoted_fare
        } else {
            base_fare
        };
        if self.get_player().resources.cash < fare {
            return StepResolutionDTO {
                success: false,
                days_advanced: 0,
                hours_advanced: 0,
                headline: "Fare Payment Declined".to_string(),
                narrative: format!("The {} fare is {}{:.2}, which exceeds your current balance.", transport_mode, self.rule_pack.currency_symbol, fare),
                causality_note: "Travel requires a successful fare payment.".to_string(),
                milestone_achieved: None,
                world_consequences: vec![],
                financial_delta: 0.0,
            };
        }

        self.get_player_mut().resources.cash -= fare;
        let remaining_origin_balance = self.get_player().resources.cash;
        self.time.advance_hours(journey_hours);
        self.rule_pack = new_rule_pack;
        if old_currency_code != self.rule_pack.currency_code {
            let usd_value = remaining_origin_balance / Self::currency_units_per_usd(&old_currency_code);
            let converted = usd_value * Self::currency_units_per_usd(&self.rule_pack.currency_code);
            self.get_player_mut().resources.cash = if self.rule_pack.currency_code == "NGN" {
                Self::round_local_money(converted, 1.0)
            } else { Self::round_local_money(converted, 0.01) };
        }
        let household_name = self.get_player().identity.last_name.clone();
        self.places = Self::build_city_places(&self.rule_pack, &household_name);
        self.current_place_id = "place:transport_terminal".to_string();
        self.ensure_city_people();

        let ticket_id = format!("doc:travel_ticket_{}", self.documents.len() + 1);
        let booking_reference = format!("OL-{:06}", self.rng.gen_range_u32(100000, 999999));
        let mut fields = HashMap::new();
        fields.insert("Passenger".to_string(), self.get_player().identity.full_name());
        fields.insert("Origin".to_string(), old_city.clone());
        fields.insert("Destination".to_string(), self.rule_pack.city_name.clone());
        fields.insert("Transport".to_string(), transport_mode.to_string());
        fields.insert("Operator".to_string(), operator_name.to_string());
        fields.insert("Service".to_string(), service_class.to_string());
        fields.insert("Departure".to_string(), departure_timing.to_string());
        fields.insert("Journey Purpose".to_string(), journey_type.to_string());
        fields.insert("Immigration Pathway".to_string(), immigration_pathway.to_string());
        fields.insert("Fare Paid".to_string(), if old_currency_code == "NGN" {
            format!("{}{:.0} {}", old_currency_symbol, fare, old_currency_code)
        } else { format!("{}{:.2} {}", old_currency_symbol, fare, old_currency_code) });
        fields.insert("Accommodation".to_string(), if stay_days > 0 {
            if accommodation == "Accommodation reserved" {
                format!("{} night(s) reserved", stay_days)
            } else {
                format!("{} · {} night(s)", accommodation, stay_days)
            }
        } else if accommodation == "No accommodation reservation" {
            "No accommodation reservation".to_string()
        } else {
            format!("{} · Open-ended stay", accommodation)
        });
        fields.insert("Booking Reference".to_string(), booking_reference.clone());
        fields.insert("Status".to_string(), "ARRIVED".to_string());
        self.documents.insert(ticket_id.clone(), DocumentRecord {
            id: ticket_id,
            title: format!("Travel Itinerary — {} to {}", old_city, self.rule_pack.city_name),
            document_type: "TRAVEL_TICKET".to_string(),
            issue_date: self.time.literary_date(),
            issuing_authority: "OTHERLIFE Travel Desk".to_string(),
            registration_number: booking_reference,
            fields,
            is_verified: true,
        });
        self.active_processes.push(LifeProcess {
            id: format!("proc:travel:{}", self.active_processes.len() + 1),
            process_type: ProcessType::TravelJourney,
            title: format!("Journey to {}", self.rule_pack.city_name),
            target_institution_id: None,
            current_step: 4,
            total_steps: 4,
            progress_percent: 100,
            status: "ARRIVED_ACCOMMODATION_RESERVED".to_string(),
            missing_requirements: vec![],
            next_appointment_day: if stay_days > 0 { Some(self.time.total_days + stay_days as i64) } else { None },
        });
        if is_international && journey_type != "Visit" {
            self.active_processes.push(LifeProcess {
                id: format!("proc:residency:{}", self.active_processes.len() + 1),
                process_type: ProcessType::ResidencyApplication,
                title: format!("{} — {}", immigration_pathway, self.rule_pack.country_name),
                target_institution_id: Some("place:civic_center".to_string()),
                current_step: 1,
                total_steps: 5,
                progress_percent: 20,
                status: "ENTRY_STATUS_REVIEW_REQUIRED".to_string(),
                missing_requirements: vec![
                    "Verify passport and entry permission".to_string(),
                    "Register a local address".to_string(),
                    "Complete eligibility period".to_string(),
                    "Attend residence appointment".to_string(),
                ],
                next_appointment_day: Some(self.time.total_days + 14),
            });
            self.letters_inbox.push(LetterNotification {
                id: format!("letter:residency:{}", self.time.total_days),
                sender: format!("{} Immigration Service", self.rule_pack.country_name),
                subject: format!("Next steps for {}", immigration_pathway),
                body: format!("Arrival does not grant permanent status. Visit the civic and immigration centre to verify entry permission, register your address, and continue the {} process.", immigration_pathway),
                is_read: false,
                date_received: self.time.literary_date(),
            });
        }

        let headline = format!("Arrived in {}", self.rule_pack.city_name);
        let exchange_note = if old_currency_code != self.rule_pack.currency_code {
            format!(" Your remaining account balance was converted from {} to {} using the simulation's regional exchange table.", old_currency_code, self.rule_pack.currency_code)
        } else { String::new() };
        let narrative = format!("You completed your {} journey from {} to {} via {}. You arrived at the transport terminal; any residence or visa pathway remains an ongoing legal process rather than an instant reward.{}", journey_type.to_lowercase(), old_city, self.rule_pack.city_name, transport_mode, exchange_note);

        self.record_event("TRAVEL", &headline, &narrative, &format!("Relocated to {}.", self.rule_pack.city_name), true);

        StepResolutionDTO {
            success: true,
            days_advanced: 0,
            hours_advanced: journey_hours,
            headline,
            narrative,
            causality_note: format!("Traveled to {}.", self.rule_pack.city_name),
            milestone_achieved: None,
            world_consequences: vec![format!("Location updated to {}", self.rule_pack.city_name)],
            financial_delta: -fare,
        }
    }

    pub fn get_phone_messages(&self) -> Vec<PhoneMessage> {
        self.phone_messages.clone()
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
            currency_code: self.rule_pack.currency_code.clone(),
            household_tier: format!("{:?}", player.resources.household_wealth_tier),
            health_level: player.biology.health_overall,
            energy_level: player.biology.energy_level,
            stress_level: player.psychology.stress_level,
            fitness: player.biology.fitness,
            confidence_level: player.psychology.confidence * 100.0,
            relationships_count: player.relationships.len(),
            occupation: player.occupation.clone().unwrap_or_else(|| {
                if age < 4 { "Infancy & Growth".to_string() }
                else if age < 13 { "Primary School Student".to_string() }
                else if age < 18 { "Secondary Student".to_string() }
                else { "Independent Citizen".to_string() }
            }),
            active_processes_count: self.active_processes.len(),
            surrounding_npcs_count: self.get_surrounding_npcs().len(),
            current_place_id: self.current_place_id.clone(),
            current_place_name: self.places.get(&self.current_place_id)
                .map(|place| place.name.clone())
                .unwrap_or_else(|| "Current Place".to_string()),
        }
    }

    pub fn generate_today_scene(&self) -> TodaySceneDTO {
        let age = self.get_player_age();
        let weather = SeasonalWeather::for_region_and_month(&self.rule_pack.climate_type, self.time.month);

        let place_name = self.places.get(&self.current_place_id)
            .map(|place| place.name.clone())
            .unwrap_or_else(|| format!("Family Home · {}", self.rule_pack.city_name));
        let headline = if self.current_place_id == "place:office" {
            format!("Working Day in {}", self.rule_pack.city_name)
        } else if self.current_place_id == "place:university" {
            format!("Campus Life at {}", place_name)
        } else if self.current_place_id == "place:cafe" {
            format!("Conversations at {}", place_name)
        } else if self.current_place_id == "place:civic_center" {
            format!("Civic Affairs in {}", self.rule_pack.city_name)
        } else if self.current_place_id == "place:park" {
            format!("Public Life at {}", place_name)
        } else if self.current_place_id == "place:transport_terminal" {
            format!("Departures from {}", self.rule_pack.city_name)
        } else if age < 4 {
            format!("Morning in the Nursery — {}", self.rule_pack.city_name)
        } else if age < 13 {
            format!("School Term Morning in {}", self.rule_pack.city_name)
        } else if age < 18 {
            format!("Adolescent Aspirations in {}", self.rule_pack.city_name)
        } else {
            format!("Civic Life in {}", self.rule_pack.city_name)
        };

        let narrative = if self.current_place_id == "place:office" {
            "Workstations, meeting rooms, and teams fill the business district. Your occupation, active applications, and company responsibilities determine what can happen here.".to_string()
        } else if self.current_place_id == "place:university" {
            "Students cross between admissions, faculty offices, lecture halls, and the library. Programmes must be chosen before an application can begin.".to_string()
        } else if self.current_place_id == "place:cafe" {
            "Tables hold quiet meetings, first encounters, and unfinished work. People here can become acquaintances through actual conversation.".to_string()
        } else if self.current_place_id == "place:civic_center" {
            "Numbered counters handle company records, passports, visas, and residency matters. Each application has requirements and waiting stages.".to_string()
        } else if self.current_place_id == "place:park" {
            "Footpaths, playing fields, and benches create a public space for exercise, recreation, and chance meetings.".to_string()
        } else if self.current_place_id == "place:transport_terminal" {
            "Departure boards, ticket desks, and arriving passengers connect this city to other places and possible futures.".to_string()
        } else if age < 4 {
            format!("Morning sunshine warms the living room rug in {}. Your mother and father are close by, attending to breakfast and household rhythms.", self.rule_pack.city_name)
        } else if age < 13 {
            format!("The morning bell sounds across the neighborhood in {}. Textbooks and notebooks rest on your desk ready for the day's lessons.", self.rule_pack.city_name)
        } else {
            format!("The city avenues of {} are active with morning commerce, university students, and professionals commuting to work.", self.rule_pack.city_name)
        };

        let present_people: Vec<String> = self.get_surrounding_npcs().iter().map(|npc| npc.name.clone()).collect();
        let environmental_objects = if self.current_place_id == "place:office" {
            vec!["Workstation".to_string(), "Meeting Room".to_string(), "Reception Desk".to_string(), "Project Board".to_string()]
        } else if self.current_place_id == "place:university" {
            vec!["Admissions Counter".to_string(), "Course Catalogue".to_string(), "Lecture Theatre".to_string(), "Campus Library".to_string()]
        } else if self.current_place_id == "place:cafe" {
            vec!["Café Counter".to_string(), "Shared Table".to_string(), "Community Noticeboard".to_string()]
        } else if self.current_place_id == "place:civic_center" {
            vec!["Company Registry Counter".to_string(), "Immigration Desk".to_string(), "Document Kiosk".to_string()]
        } else if self.current_place_id == "place:clinic" {
            vec!["Reception Desk".to_string(), "Consultation Room".to_string(), "Pharmacy Counter".to_string()]
        } else if self.current_place_id == "place:park" {
            vec!["Walking Path".to_string(), "Playing Field".to_string(), "Public Bench".to_string(), "Exercise Station".to_string()]
        } else if self.current_place_id == "place:transport_terminal" {
            vec!["Departure Board".to_string(), "Ticket Counter".to_string(), "Waiting Area".to_string()]
        } else if self.current_place_id == "place:school" {
            vec!["Classroom".to_string(), "School Office".to_string(), "Library".to_string(), "Playground".to_string()]
        } else if self.current_place_id == "place:sports_academy" {
            vec!["Training Pitch".to_string(), "Coach's Office".to_string(), "Changing Room".to_string(), "Equipment Store".to_string()]
        } else if age < 4 {
            vec!["Wooden Blocks".to_string(), "Picture Book".to_string(), "Family Sofa".to_string(), "Warm Blanket".to_string()]
        } else if age < 13 {
            vec!["Arithmetic Exercise Books".to_string(), "Leather Football".to_string(), "Family Desktop".to_string(), "School Bag".to_string()]
        } else {
            vec!["Smartphone".to_string(), "Personal Computer".to_string(), "Study Library".to_string(), "Corporate Registry".to_string()]
        };

        let subtle_details = match self.current_place_id.as_str() {
            "place:office" => vec!["Muted conversations behind meeting-room glass".to_string(), "Keyboards and printers punctuate the workday".to_string()],
            "place:university" => vec!["Course notices cover a faculty board".to_string(), "Students compare timetables between lectures".to_string()],
            "place:cafe" => vec!["Fresh coffee and street rain scent the air".to_string(), "A nearby table is negotiating something quietly".to_string()],
            "place:civic_center" => vec!["Queue numbers change above the service counters".to_string(), "Applicants check document folders before being called".to_string()],
            "place:clinic" => vec!["Soft announcements carry through the waiting area".to_string(), "Clinical staff move between consultation rooms".to_string()],
            "place:park" => vec!["Footsteps and distant conversation cross the open paths".to_string(), "A light breeze moves through the trees".to_string()],
            "place:transport_terminal" => vec!["A departure board updates above the concourse".to_string(), "Engines, luggage wheels, and announcements overlap".to_string()],
            "place:school" => vec!["A bell marks the next lesson".to_string(), "Exercise books and voices fill nearby classrooms".to_string()],
            "place:sports_academy" => vec!["A coach's whistle cuts across the training ground".to_string(), "Players rotate between drills and recovery".to_string()],
            _ => vec!["Gentle daylight reaches through the curtains".to_string(), "Familiar household sounds continue nearby".to_string()],
        };

        TodaySceneDTO {
            headline,
            narrative,
            weather_name: weather.name,
            weather_description: weather.description,
            location_name: place_name,
            present_people,
            environmental_objects,
            subtle_details,
            immediate_pressures: vec![],
            location_formatted: Some(format!("{}, {}", self.rule_pack.city_name, self.rule_pack.country_name)),
            life_stage: Some(format!("{:?}", LifeStage::from_age(age))),
            age: Some(age),
            circumstances: Some(vec![format!("You are physically at {}", self.current_place_id.replace("place:", "").replace('_', " "))]),
        }
    }

    pub fn get_surrounding_npcs(&self) -> Vec<ContextNpcDTO> {
        let player_relationships = &self.get_player().relationships;
        self.npcs.values().filter_map(|npc| {
            let (location_id, current_activity) = self.npc_activity_at(npc);
            if location_id != self.current_place_id {
                return None;
            }
            let existing = player_relationships.get(&npc.base.id);
            let relationship_type = if let Some(relationship) = existing {
                relationship.relationship_type.clone()
            } else if npc.base.id.contains("mother") {
                "Mother".to_string()
            } else if npc.base.id.contains("father") {
                "Father".to_string()
            } else if npc.base.id.contains("teacher") {
                "Teacher / Mentor".to_string()
            } else if npc.base.id.contains("coach") {
                "Sports Coach".to_string()
            } else {
                "Stranger".to_string()
            };
            Some(ContextNpcDTO {
                id: npc.base.id.clone(),
                name: npc.base.identity.full_name(),
                relationship_type,
                trust_description: existing
                    .map(|relationship| format!("Trust {:.0}% · Affinity {:.0}%", relationship.trust * 100.0, relationship.affinity * 100.0))
                    .unwrap_or_else(|| "Not yet acquainted".to_string()),
                current_activity,
                location_id,
                is_new_acquaintance: existing.is_none() && npc.base.id.contains("person:city:"),
            })
        }).collect()
    }

    pub fn get_phone_contacts(&self) -> Vec<ContextNpcDTO> {
        let player_relationships = &self.get_player().relationships;
        let mut contacts: Vec<ContextNpcDTO> = self.npcs.values().filter_map(|npc| {
            let existing = player_relationships.get(&npc.base.id);
            let is_established_contact = existing.is_some()
                || npc.base.id.contains("mother")
                || npc.base.id.contains("father")
                || npc.base.id.contains("teacher")
                || npc.base.id.contains("coach")
                || self.phone_messages.iter().any(|message| {
                    message.sender_id == npc.base.id || message.recipient_id == npc.base.id
                });
            if !is_established_contact {
                return None;
            }
            let (location_id, current_activity) = self.npc_activity_at(npc);
            let relationship_type = existing
                .map(|relationship| relationship.relationship_type.clone())
                .unwrap_or_else(|| {
                    if npc.base.id.contains("mother") {
                        "Mother".to_string()
                    } else if npc.base.id.contains("father") {
                        "Father".to_string()
                    } else if npc.base.id.contains("teacher") {
                        "Teacher / Mentor".to_string()
                    } else if npc.base.id.contains("coach") {
                        "Sports Coach".to_string()
                    } else {
                        "Contact".to_string()
                    }
                });
            Some(ContextNpcDTO {
                id: npc.base.id.clone(),
                name: npc.base.identity.full_name(),
                relationship_type,
                trust_description: existing
                    .map(|relationship| format!("Trust {:.0}% · Affinity {:.0}%", relationship.trust * 100.0, relationship.affinity * 100.0))
                    .unwrap_or_else(|| "Saved contact".to_string()),
                current_activity,
                location_id,
                is_new_acquaintance: false,
            })
        }).collect();
        contacts.sort_by(|a, b| a.name.cmp(&b.name));
        contacts
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

    pub fn get_life_chronicle(&self, limit: usize) -> Vec<ChronicleEntryDTO> {
        let current_age = self.get_player_age();
        self.events_ledger.iter().rev().take(limit).map(|event| {
            let years_ago = ((self.time.total_days - event.day_total).max(0) / 365) as u32;
            ChronicleEntryDTO {
                id: event.id.clone(),
                age: current_age.saturating_sub(years_ago),
                date: event.timestamp.clone(),
                event_type: event.event_type.clone(),
                headline: event.headline.clone(),
                narrative: event.narrative.clone(),
                success: event.success,
            }
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
            location_id: self.current_place_id.clone(),
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
        let mut engine: Self = serde_json::from_str(json_str)?;
        engine.ensure_world_places();
        engine.ensure_city_people();
        Ok(engine)
    }
}

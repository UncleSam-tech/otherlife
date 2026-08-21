use otherlife_ai_bridge::{AIBridge, AIBridgeConfig, BiographyWriter};
use otherlife_rng::WorldRng;
use otherlife_world::{
    AdmissionRequirement, AutonomousNPC, BiologicalProfile, CommunicationStyle, ContextNpcDTO,
    ContextProcessDTO, DailyRoutineBlock, EventCategory, EventRecord, ExternalEvent, HumanEntity,
    HumanResources, IdentityProfile, InstitutionEntity, InstitutionType, KnowledgeType,
    LetterNotification, LifeMemory, LifeProcess, LifeStage, LivingStateDTO, LivingStepResultDTO,
    MacroEnvironment, NewLifeConfig, NpcMemoryOfPlayer, NpcRole, OccupationRecord,
    OpportunityRecord, PersonalityProfile, PlaceType, PlayerKnowledgeRecord, ProcessStatus,
    ProcessType, PsychologicalProfile, RelationshipHistory, RelationshipType, RelationshipVector,
    ReputationProfile, SeasonalWeather, SharedMemory, SimTime, SkillMastery, TodaySceneDTO,
    WealthTier, WorldPlace,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEngine {
    pub time: SimTime,
    pub rng: WorldRng,
    pub persons: HashMap<String, HumanEntity>,
    pub npcs: HashMap<String, AutonomousNPC>,
    pub places: HashMap<String, WorldPlace>,
    pub institutions: HashMap<String, InstitutionEntity>,
    pub macro_env: MacroEnvironment,
    pub letters_inbox: Vec<LetterNotification>,
    pub active_processes: Vec<LifeProcess>,
    pub active_opportunities: Vec<OpportunityRecord>,
    pub player_knowledge: Vec<PlayerKnowledgeRecord>,
    pub memories: Vec<LifeMemory>,
    pub events_chronicle: Vec<EventRecord>,
    pub ai_bridge: AIBridge,
}

#[allow(dead_code)]
struct LocationContext {
    country_name: String,
    city_name: String,
    district_name: String,
    currency_symbol: String,
    culture_name: String,
    language_name: String,
    mother_name: String,
    father_name: String,
    teacher_name: String,
    coach_name: String,
    friend_name: String,
    mother_job: String,
    father_job: String,
    mother_salary: f64,
    father_salary: f64,
    primary_school_name: String,
    sports_club_name: String,
    university_name: String,
    exam_name: String,
}

impl SimulationEngine {
    fn resolve_location_context(country_id: &str, location_id: &str, wealth_tier: &WealthTier, last_name: &str) -> LocationContext {
        let c_lower = country_id.to_lowercase();
        let l_lower = location_id.to_lowercase();

        if c_lower.contains("united_kingdom") || c_lower.contains("uk") || c_lower.contains("scotland") || c_lower.contains("england") {
            let is_glasgow = l_lower.contains("glasgow");
            let is_manchester = l_lower.contains("manchester");
            let is_birmingham = l_lower.contains("birmingham");
            let is_edinburgh = l_lower.contains("edinburgh");

            let (city, district, school, club, uni, mother_first, father_first, teacher_n, coach_n, friend_n) = if is_glasgow {
                ("Glasgow", "West End", "Hillhead Primary School", "Partick Community Sports Club", "University of Glasgow", "Fiona", "Callum", "Mr. Alistair MacLeod", "Coach Gordon Smith", "Liam Robertson")
            } else if is_edinburgh {
                ("Edinburgh", "Old Town", "Royal Mile Primary School", "Meadows Youth Athletics", "University of Edinburgh", "Isobel", "Hamish", "Mrs. Morag Fraser", "Coach Scott Brown", "Euan Campbell")
            } else if is_manchester {
                ("Manchester", "Didsbury", "Didsbury Community Primary", "Salford Youth Football Academy", "University of Manchester", "Claire", "Simon", "Mr. Thomas Wright", "Coach Paul Barnes", "Jack Harrison")
            } else if is_birmingham {
                ("Birmingham", "Edgbaston", "Edgbaston Hall Primary", "Digbeth Community Sports Grounds", "University of Birmingham", "Gemma", "Richard", "Mr. David Clarke", "Coach Tony Evans", "Harry Lewis")
            } else {
                ("London", "Camden", "Camden Primary Academy", "North London Youth Sports Centre", "University College London (UCL)", "Emma", "Arthur", "Mr. Julian Vance", "Coach Dave Miller", "Oliver Taylor")
            };

            let (m_job, f_job, m_sal, f_sal) = match wealth_tier {
                WealthTier::Poverty | WealthTier::WorkingClass => ("Healthcare Assistant (NHS)", "Transit Logistics Driver", 1950.0, 2200.0),
                WealthTier::UpperMiddle | WealthTier::Wealthy => ("Consultant Surgeon", "Managing Engineering Director", 6800.0, 8200.0),
                _ => ("Staff Nurse (NHS)", "Mechanical Engineer", 3100.0, 3600.0),
            };

            LocationContext {
                country_name: "United Kingdom".to_string(),
                city_name: city.to_string(),
                district_name: district.to_string(),
                currency_symbol: "£".to_string(),
                culture_name: if is_glasgow || is_edinburgh { "Scottish / British Contemporary".to_string() } else { "British Contemporary".to_string() },
                language_name: "English".to_string(),
                mother_name: format!("{} {}", mother_first, last_name),
                father_name: format!("{} {}", father_first, last_name),
                teacher_name: teacher_n.to_string(),
                coach_name: coach_n.to_string(),
                friend_name: friend_n.to_string(),
                mother_job: m_job.to_string(),
                father_job: f_job.to_string(),
                mother_salary: m_sal,
                father_salary: f_sal,
                primary_school_name: school.to_string(),
                sports_club_name: club.to_string(),
                university_name: uni.to_string(),
                exam_name: if is_glasgow || is_edinburgh { "Scottish Higher Examinations".to_string() } else { "GCSE & A-Level Examinations".to_string() },
            }
        } else if c_lower.contains("united_states") || c_lower.contains("usa") || c_lower.contains("america") {
            let is_sf = l_lower.contains("san_francisco");
            let is_la = l_lower.contains("los_angeles");
            let is_chicago = l_lower.contains("chicago");
            let is_houston = l_lower.contains("houston");

            let (city, district, school, club, uni, mother_first, father_first, teacher_n, coach_n, friend_n) = if is_sf {
                ("San Francisco", "Sunset District", "Sunset Elementary School", "Mission District Athletic Complex", "University of California, Berkeley", "Elena", "Marcus", "Mrs. Jennifer Hayes", "Coach Dave Miller", "Ethan Vance")
            } else if is_la {
                ("Los Angeles", "Silver Lake", "Silver Lake Arts & Science Elementary", "Westwood Youth Athletic Ground", "University of California, Los Angeles (UCLA)", "Maya", "Daniel", "Mr. Robert Chen", "Coach Mike Martinez", "Lucas Rivera")
            } else if is_chicago {
                ("Chicago", "Lincoln Park", "Lincoln Park Community Elementary", "Lakeshore Youth Sports Complex", "University of Chicago", "Sarah", "James", "Mrs. Karen O'Connor", "Coach Patrick Flynn", "Noah Murphy")
            } else if is_houston {
                ("Houston", "Montrose", "Montrose Academy for Young Scholars", "Bayou City Youth Soccer Grounds", "Rice University", "Laura", "Carlos", "Mr. Anthony Reed", "Coach Greg Jenkins", "Benjamin Brooks")
            } else {
                ("New York", "Brooklyn", "Public School 29 Brooklyn", "Prospect Park Youth Sports Grounds", "Columbia University", "Rachel", "David", "Mrs. Rebecca Stern", "Coach Brian Walsh", "Samuel Goldberg")
            };

            let (m_job, f_job, m_sal, f_sal) = match wealth_tier {
                WealthTier::Poverty | WealthTier::WorkingClass => ("Retail Specialist", "Express Courier", 2900.0, 3300.0),
                WealthTier::UpperMiddle | WealthTier::Wealthy => ("Biotech Research Scientist", "Software Architecture Director", 11500.0, 14500.0),
                _ => ("Guidance Counselor", "Systems Project Manager", 5200.0, 6000.0),
            };

            LocationContext {
                country_name: "United States".to_string(),
                city_name: city.to_string(),
                district_name: district.to_string(),
                currency_symbol: "$".to_string(),
                culture_name: "American Contemporary".to_string(),
                language_name: "English".to_string(),
                mother_name: format!("{} {}", mother_first, last_name),
                father_name: format!("{} {}", father_first, last_name),
                teacher_name: teacher_n.to_string(),
                coach_name: coach_n.to_string(),
                friend_name: friend_n.to_string(),
                mother_job: m_job.to_string(),
                father_job: f_job.to_string(),
                mother_salary: m_sal,
                father_salary: f_sal,
                primary_school_name: school.to_string(),
                sports_club_name: club.to_string(),
                university_name: uni.to_string(),
                exam_name: "Advanced Placement (AP) & SAT Examinations".to_string(),
            }
        } else {
            // Nigeria / West Africa Multi-City Context
            let is_lagos = l_lower.contains("lagos");
            let is_ibadan = l_lower.contains("ibadan");
            let is_ph = l_lower.contains("port_harcourt") || l_lower.contains("ph");
            let is_kano = l_lower.contains("kano");
            let is_enugu = l_lower.contains("enugu");

            let (city, district, school, club, uni, mother_first, father_first, teacher_n, coach_n, friend_n, culture) = if is_lagos {
                ("Lagos", "Ikeja", "Lagos Model Primary School, Ikeja", "Surulere Community Stadium", "University of Lagos (UNILAG)", "Funke", "Babajide", "Mr. Babatunde Adewale", "Coach Segun Odegbami", "Chidi Nwosu", "Yoruba / Urban Nigerian")
            } else if is_ibadan {
                ("Ibadan", "Bodija", "Bodija International Primary School", "Liberty Stadium Training Pitch", "University of Ibadan", "Yetunde", "Adegoke", "Mr. Oladipo Johnson", "Coach Kunle Balogun", "Femi Adeleke", "Yoruba / Southwestern Nigerian")
            } else if is_ph {
                ("Port Harcourt", "Old GRA", "Rivers State Primary Model School", "Civic Centre Sports Complex", "University of Port Harcourt", "Blessing", "Tamuno", "Mr. Goodluck Briggs", "Coach Taribo Douglas", "Precious Wike", "Niger Delta / Coastal Nigerian")
            } else if is_kano {
                ("Kano", "Nasarawa", "Kano Capital School", "Sani Abacha Youth Sports Centre", "Bayero University Kano", "Amina", "Ibrahim", "Malam Usman Bello", "Coach Sani Danladi", "Musa Garba", "Hausa-Fulani / Northern Nigerian")
            } else if is_enugu {
                ("Enugu", "Independence Layout", "Enugu State Primary Academy", "Nnamdi Azikiwe Sports Ground", "University of Nigeria, Nsukka", "Nkechi", "Emeka", "Mr. Chukwuemeka Okoye", "Coach Ifeanyi Eze", "Tochukwu Nnamani", "Igbo / Southeastern Nigerian")
            } else {
                ("Abuja", "Garki", "Abuja Model Primary School", "Area 10 Community Sports Ground", "University of Abuja", "Sarah", "David", "Mr. Babatunde Adewale", "Coach Ibrahim Bello", "Chidi Nwosu", "West African / Urban Nigerian")
            };

            let (m_job, f_job, m_sal, f_sal) = match wealth_tier {
                WealthTier::Poverty | WealthTier::WorkingClass => ("Market Provisions Trader", "Automotive Specialist", 75000.0, 95000.0),
                WealthTier::UpperMiddle | WealthTier::Wealthy => ("Senior Medical Consultant", "Director of Public Enterprise", 900000.0, 1350000.0),
                _ => ("Healthcare Officer", "Senior Ministry Administrator", 280000.0, 340000.0),
            };

            LocationContext {
                country_name: "Nigeria".to_string(),
                city_name: city.to_string(),
                district_name: district.to_string(),
                currency_symbol: "₦".to_string(),
                culture_name: culture.to_string(),
                language_name: "English & Local Languages".to_string(),
                mother_name: format!("{} {}", mother_first, last_name),
                father_name: format!("{} {}", father_first, last_name),
                teacher_name: teacher_n.to_string(),
                coach_name: coach_n.to_string(),
                friend_name: friend_n.to_string(),
                mother_job: m_job.to_string(),
                father_job: f_job.to_string(),
                mother_salary: m_sal,
                father_salary: f_sal,
                primary_school_name: school.to_string(),
                sports_club_name: club.to_string(),
                university_name: uni.to_string(),
                exam_name: "West African Senior School Certificate (WAEC / JAMB)".to_string(),
            }
        }
    }

    pub fn new_game(config: NewLifeConfig, seed: u64) -> Self {
        let rng = WorldRng::new(seed);
        
        let starting_age = config.starting_age;
        let birth_year = config.birth_year.unwrap_or(config.starting_year - starting_age as i32);
        let birth_month = config.birth_month.unwrap_or(1);
        let birth_day = config.birth_day.unwrap_or(15);
        let sim_starting_year = birth_year + starting_age as i32;
        let time = SimTime::new(sim_starting_year, birth_month, birth_day);

        let first_name = config.first_name.unwrap_or_else(|| "Alex".to_string());
        let last_name = config.last_name.unwrap_or_else(|| "Sterling".to_string());
        let sex = config.sex.unwrap_or_else(|| "Male".to_string());
        let wealth_tier = WealthTier::from_str(&config.household_income_tier.unwrap_or_else(|| "MIDDLE".to_string()));

        let ctx = Self::resolve_location_context(&config.country_id, &config.location_id, &wealth_tier, &last_name);

        let player_id = "person:sim:player".to_string();

        let mut skills = HashMap::new();
        skills.insert("curiosity".to_string(), SkillMastery { level: 25.0, experience: 100.0, natural_affinity: 1.2, last_practiced_day: 0 });
        skills.insert("motor_coordination".to_string(), SkillMastery { level: 10.0, experience: 30.0, natural_affinity: 1.1, last_practiced_day: 0 });
        skills.insert("arithmetic".to_string(), SkillMastery { level: 15.0, experience: 50.0, natural_affinity: 1.0, last_practiced_day: 0 });
        skills.insert("reading".to_string(), SkillMastery { level: 20.0, experience: 70.0, natural_affinity: 1.1, last_practiced_day: 0 });
        skills.insert("football_control".to_string(), SkillMastery { level: 10.0, experience: 40.0, natural_affinity: 1.3, last_practiced_day: 0 });
        skills.insert("programming".to_string(), SkillMastery { level: 0.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: 0 });

        let starting_cash = match ctx.currency_symbol.as_str() {
            "₦" => if starting_age < 13 { 1500.0 } else { 8500.0 },
            "£" => if starting_age < 13 { 25.0 } else { 120.0 },
            _ => if starting_age < 13 { 30.0 } else { 150.0 },
        };

        let player = HumanEntity {
            id: player_id.clone(),
            identity: IdentityProfile {
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                birth_year,
                birth_month,
                birth_day,
                sex: sex.clone(),
                birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                nationality: ctx.country_name.clone(),
                culture: ctx.culture_name.clone(),
                primary_language: ctx.language_name.clone(),
            },
            biology: BiologicalProfile {
                is_alive: true,
                death_year: None,
                death_reason: None,
                health_overall: 98.0,
                fitness: 75.0,
                energy_level: 90.0,
                chronic_conditions: Vec::new(),
            },
            psychology: PsychologicalProfile {
                discipline: 0.60,
                curiosity: 0.75,
                creativity: 0.65,
                confidence: 0.60,
                risk_tolerance: 0.45,
                stress_level: 10.0,
                resilience: 0.50,
            },
            reputation: ReputationProfile::default(),
            skills,
            resources: HumanResources {
                cash: starting_cash,
                household_wealth_tier: wealth_tier,
                living_arrangement: "FAMILY_HOME".to_string(),
                tools_available: vec!["BOOKS".to_string(), "FOOTBALL_BOOTS".to_string()],
            },
            relationships: HashMap::new(),
            occupation: None,
            is_player: true,
        };

        let mut persons = HashMap::new();
        persons.insert(player_id.clone(), player);

        let mut npcs = HashMap::new();

        // 1. Mother
        let mother_id = "person:sim:mother".to_string();
        npcs.insert(mother_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: mother_id.clone(),
                identity: IdentityProfile {
                    first_name: ctx.mother_name.split_whitespace().next().unwrap_or("Mother").to_string(),
                    last_name: last_name.clone(),
                    birth_year: birth_year - 28,
                    birth_month: 5,
                    birth_day: 14,
                    sex: "Female".to_string(),
                    birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                    nationality: ctx.country_name.clone(),
                    culture: ctx.culture_name.clone(),
                    primary_language: ctx.language_name.clone(),
                },
                biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 92.0, fitness: 65.0, energy_level: 80.0, chronic_conditions: Vec::new() },
                psychology: PsychologicalProfile { discipline: 0.85, curiosity: 0.60, creativity: 0.50, confidence: 0.75, risk_tolerance: 0.30, stress_level: 25.0, resilience: 0.80 },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources { cash: ctx.mother_salary * 1.5, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "FAMILY_HOME".to_string(), tools_available: Vec::new() },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord { title: ctx.mother_job.clone(), employer_org_id: None, monthly_earnings: ctx.mother_salary, start_year: birth_year - 5 }),
                is_player: false,
            },
            primary_role: NpcRole::Parent,
            personality: PersonalityProfile {
                warmth: 0.95,
                patience: 0.90,
                strictness: 0.35,
                ambition: 0.60,
                risk_tolerance: 0.30,
                communication_style: CommunicationStyle::Nurturing,
                core_values: vec!["Family Harmony".to_string(), "Integrity".to_string(), "Education".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 8, end_hour: 16, activity_name: format!("Work Shift: {}", ctx.mother_job), location_id: "org:local:workplace".to_string() },
                DailyRoutineBlock { start_hour: 17, end_hour: 21, activity_name: "Family Care & Dinner".to_string(), location_id: "district:home".to_string() },
            ],
            life_goal: "Provide unconditional warmth and raise a well-educated child.".to_string(),
            subjective_memories_of_player: vec![NpcMemoryOfPlayer { day_occurred: time.total_days, event_summary: "Welcomed child into a loving, secure home.".to_string(), sentiment: 0.95, importance: 5 }],
            monthly_income: ctx.mother_salary,
            stress_level: 20.0,
            last_active_day: time.total_days,
        });

        // 2. Father
        let father_id = "person:sim:father".to_string();
        npcs.insert(father_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: father_id.clone(),
                identity: IdentityProfile {
                    first_name: ctx.father_name.split_whitespace().next().unwrap_or("Father").to_string(),
                    last_name: last_name.clone(),
                    birth_year: birth_year - 30,
                    birth_month: 8,
                    birth_day: 22,
                    sex: "Male".to_string(),
                    birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                    nationality: ctx.country_name.clone(),
                    culture: ctx.culture_name.clone(),
                    primary_language: ctx.language_name.clone(),
                },
                biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 90.0, fitness: 60.0, energy_level: 80.0, chronic_conditions: Vec::new() },
                psychology: PsychologicalProfile { discipline: 0.90, curiosity: 0.55, creativity: 0.45, confidence: 0.80, risk_tolerance: 0.35, stress_level: 20.0, resilience: 0.85 },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources { cash: ctx.father_salary * 1.5, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "FAMILY_HOME".to_string(), tools_available: Vec::new() },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord { title: ctx.father_job.clone(), employer_org_id: None, monthly_earnings: ctx.father_salary, start_year: birth_year - 7 }),
                is_player: false,
            },
            primary_role: NpcRole::Parent,
            personality: PersonalityProfile {
                warmth: 0.70,
                patience: 0.65,
                strictness: 0.85,
                ambition: 0.80,
                risk_tolerance: 0.35,
                communication_style: CommunicationStyle::Disciplinarian,
                core_values: vec!["Accountability".to_string(), "Discipline".to_string(), "Character".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 8, end_hour: 17, activity_name: format!("Work: {}", ctx.father_job), location_id: "org:local:workplace".to_string() },
                DailyRoutineBlock { start_hour: 18, end_hour: 22, activity_name: "Evening Mentorship & Reading".to_string(), location_id: "district:home".to_string() },
            ],
            life_goal: "Instill discipline, character, and excellence in the family.".to_string(),
            subjective_memories_of_player: vec![NpcMemoryOfPlayer { day_occurred: time.total_days, event_summary: "Welcomed child with high hopes for the future.".to_string(), sentiment: 0.95, importance: 5 }],
            monthly_income: ctx.father_salary,
            stress_level: 20.0,
            last_active_day: time.total_days,
        });

        // 3. Teacher
        let teacher_id = "person:sim:adewale_teacher".to_string();
        npcs.insert(teacher_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: teacher_id.clone(),
                identity: IdentityProfile {
                    first_name: ctx.teacher_name.split_whitespace().nth(1).unwrap_or("Teacher").to_string(),
                    last_name: ctx.teacher_name.split_whitespace().last().unwrap_or("").to_string(),
                    birth_year: birth_year - 22,
                    birth_month: 3,
                    birth_day: 10,
                    sex: "Male".to_string(),
                    birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                    nationality: ctx.country_name.clone(),
                    culture: ctx.culture_name.clone(),
                    primary_language: ctx.language_name.clone(),
                },
                biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 94.0, fitness: 70.0, energy_level: 85.0, chronic_conditions: Vec::new() },
                psychology: PsychologicalProfile { discipline: 0.85, curiosity: 0.80, creativity: 0.65, confidence: 0.75, risk_tolerance: 0.40, stress_level: 15.0, resilience: 0.80 },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources { cash: ctx.mother_salary * 0.8, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "RENTED_APARTMENT".to_string(), tools_available: Vec::new() },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord { title: "Lead Educator".to_string(), employer_org_id: Some("org:sim:primary_school".to_string()), monthly_earnings: ctx.mother_salary * 0.8, start_year: birth_year + 3 }),
                is_player: false,
            },
            primary_role: NpcRole::Teacher,
            personality: PersonalityProfile {
                warmth: 0.80,
                patience: 0.85,
                strictness: 0.60,
                ambition: 0.75,
                risk_tolerance: 0.40,
                communication_style: CommunicationStyle::Inspirational,
                core_values: vec!["Intellectual Rigor".to_string(), "Mentorship".to_string(), "Perseverance".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 8, end_hour: 15, activity_name: "Classroom Teaching & Mentoring".to_string(), location_id: "org:sim:primary_school".to_string() },
            ],
            life_goal: "Mentor gifted students and prepare them for academic excellence.".to_string(),
            subjective_memories_of_player: Vec::new(),
            monthly_income: ctx.mother_salary * 0.8,
            stress_level: 15.0,
            last_active_day: time.total_days,
        });

        // 4. Coach
        let coach_id = "person:sim:coach_ibrahim".to_string();
        npcs.insert(coach_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: coach_id.clone(),
                identity: IdentityProfile {
                    first_name: ctx.coach_name.split_whitespace().nth(1).unwrap_or("Coach").to_string(),
                    last_name: ctx.coach_name.split_whitespace().last().unwrap_or("").to_string(),
                    birth_year: birth_year - 25,
                    birth_month: 6,
                    birth_day: 18,
                    sex: "Male".to_string(),
                    birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                    nationality: ctx.country_name.clone(),
                    culture: ctx.culture_name.clone(),
                    primary_language: ctx.language_name.clone(),
                },
                biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 95.0, fitness: 88.0, energy_level: 90.0, chronic_conditions: Vec::new() },
                psychology: PsychologicalProfile { discipline: 0.90, curiosity: 0.60, creativity: 0.70, confidence: 0.85, risk_tolerance: 0.50, stress_level: 15.0, resilience: 0.90 },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources { cash: ctx.father_salary * 0.75, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "RENTED_APARTMENT".to_string(), tools_available: vec!["WHISTLE".to_string(), "CONES".to_string()] },
                relationships: HashMap::new(),
                occupation: Some(OccupationRecord { title: "Head Coach & Talent Scout".to_string(), employer_org_id: Some("org:sim:sports_ground".to_string()), monthly_earnings: ctx.father_salary * 0.75, start_year: birth_year + 5 }),
                is_player: false,
            },
            primary_role: NpcRole::Coach,
            personality: PersonalityProfile {
                warmth: 0.55,
                patience: 0.60,
                strictness: 0.90,
                ambition: 0.85,
                risk_tolerance: 0.50,
                communication_style: CommunicationStyle::Direct,
                core_values: vec!["Hard Work".to_string(), "Physical Stamina".to_string(), "Tactical Discipline".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 15, end_hour: 19, activity_name: "Youth Squad Drills & Match Scouting".to_string(), location_id: "org:sim:sports_ground".to_string() },
            ],
            life_goal: "Discover dedicated talent and prepare disciplined players for competitive advancement.".to_string(),
            subjective_memories_of_player: Vec::new(),
            monthly_income: ctx.father_salary * 0.75,
            stress_level: 15.0,
            last_active_day: time.total_days,
        });

        // 5. Friend
        let peer_id = "person:sim:chidi_nwosu".to_string();
        npcs.insert(peer_id.clone(), AutonomousNPC {
            base: HumanEntity {
                id: peer_id.clone(),
                identity: IdentityProfile {
                    first_name: ctx.friend_name.split_whitespace().next().unwrap_or("Friend").to_string(),
                    last_name: ctx.friend_name.split_whitespace().last().unwrap_or("").to_string(),
                    birth_year,
                    birth_month: 4,
                    birth_day: 11,
                    sex: "Male".to_string(),
                    birthplace_id: format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_")),
                    nationality: ctx.country_name.clone(),
                    culture: ctx.culture_name.clone(),
                    primary_language: ctx.language_name.clone(),
                },
                biology: BiologicalProfile { is_alive: true, death_year: None, death_reason: None, health_overall: 96.0, fitness: 80.0, energy_level: 90.0, chronic_conditions: Vec::new() },
                psychology: PsychologicalProfile { discipline: 0.65, curiosity: 0.75, creativity: 0.80, confidence: 0.70, risk_tolerance: 0.60, stress_level: 10.0, resilience: 0.70 },
                reputation: ReputationProfile::default(),
                skills: HashMap::new(),
                resources: HumanResources { cash: starting_cash * 0.8, household_wealth_tier: WealthTier::MiddleClass, living_arrangement: "FAMILY_HOME".to_string(), tools_available: Vec::new() },
                relationships: HashMap::new(),
                occupation: None,
                is_player: false,
            },
            primary_role: NpcRole::Classmate,
            personality: PersonalityProfile {
                warmth: 0.85,
                patience: 0.75,
                strictness: 0.20,
                ambition: 0.65,
                risk_tolerance: 0.60,
                communication_style: CommunicationStyle::Playful,
                core_values: vec!["Loyalty".to_string(), "Fun".to_string(), "Adventure".to_string()],
            },
            daily_schedule: vec![
                DailyRoutineBlock { start_hour: 8, end_hour: 14, activity_name: "Attending School Classes".to_string(), location_id: "org:sim:primary_school".to_string() },
                DailyRoutineBlock { start_hour: 15, end_hour: 18, activity_name: "Playground Sports & Games".to_string(), location_id: "district:home".to_string() },
            ],
            life_goal: "Enjoy childhood adventures and build strong lasting friendships.".to_string(),
            subjective_memories_of_player: Vec::new(),
            monthly_income: 0.0,
            stress_level: 10.0,
            last_active_day: time.total_days,
        });

        // Initialize Player-NPC Relationship Vectors with History
        if let Some(player_ent) = persons.get_mut(&player_id) {
            player_ent.relationships.insert(mother_id.clone(), RelationshipVector {
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

            player_ent.relationships.insert(father_id.clone(), RelationshipVector {
                source_person_id: player_id.clone(),
                target_person_id: father_id.clone(),
                relationship_type: RelationshipType::Father,
                trust: 0.90,
                affection: 0.85,
                respect: 0.95,
                resentment: 0.0,
                history: RelationshipHistory::default(),
                is_active: true,
            });

            player_ent.relationships.insert(teacher_id.clone(), RelationshipVector {
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

            player_ent.relationships.insert(coach_id.clone(), RelationshipVector {
                source_person_id: player_id.clone(),
                target_person_id: coach_id.clone(),
                relationship_type: RelationshipType::Coach,
                trust: 0.70,
                affection: 0.50,
                respect: 0.80,
                resentment: 0.0,
                history: RelationshipHistory::default(),
                is_active: true,
            });

            player_ent.relationships.insert(peer_id.clone(), RelationshipVector {
                source_person_id: player_id.clone(),
                target_person_id: peer_id.clone(),
                relationship_type: RelationshipType::Friend,
                trust: 0.85,
                affection: 0.85,
                respect: 0.70,
                resentment: 0.0,
                history: RelationshipHistory::default(),
                is_active: true,
            });
        }

        // Geography & Institutions
        let mut places = HashMap::new();
        let city_id = format!("city:real:{}", ctx.city_name.to_lowercase().replace(' ', "_"));
        places.insert(city_id.clone(), WorldPlace {
            id: city_id.clone(),
            name: ctx.city_name.clone(),
            place_type: PlaceType::City,
            parent_place_id: Some(format!("country:real:{}", ctx.country_name.to_lowercase().replace(' ', "_"))),
            country_id: ctx.country_name.clone(),
            climate_zone: "Temperate / Regional".to_string(),
            cost_of_living_index: 1.0,
            culture_tags: vec![ctx.culture_name.clone()],
        });

        let mut institutions = HashMap::new();
        institutions.insert("org:sim:primary_school".to_string(), InstitutionEntity {
            id: "org:sim:primary_school".to_string(),
            name: ctx.primary_school_name.clone(),
            institution_type: InstitutionType::PrimarySchool,
            location_id: city_id.clone(),
            prestige: 0.70,
            admission_requirements: vec![AdmissionRequirement::MinimumAge(4), AdmissionRequirement::MaximumAge(12)],
            active_members: vec![teacher_id.clone(), peer_id.clone()],
        });

        institutions.insert("org:sim:sports_ground".to_string(), InstitutionEntity {
            id: "org:sim:sports_ground".to_string(),
            name: ctx.sports_club_name.clone(),
            institution_type: InstitutionType::SportsClub,
            location_id: city_id.clone(),
            prestige: 0.75,
            admission_requirements: vec![AdmissionRequirement::MinimumAge(7)],
            active_members: vec![coach_id.clone()],
        });

        institutions.insert("org:real:university".to_string(), InstitutionEntity {
            id: "org:real:university".to_string(),
            name: ctx.university_name.clone(),
            institution_type: InstitutionType::University,
            location_id: city_id.clone(),
            prestige: 0.85,
            admission_requirements: vec![AdmissionRequirement::MinimumAge(16), AdmissionRequirement::AcademicPerformance(65.0), AdmissionRequirement::DocumentRequired("ACADEMIC_RESULTS".to_string())],
            active_members: Vec::new(),
        });

        let macro_env = MacroEnvironment {
            inflation_rate: 0.08,
            power_grid_reliability: 0.95,
            current_season: SeasonalWeather::from_month(time.month),
            market_cost_index: 1.0,
        };

        let mut events_chronicle = Vec::new();
        let initial_event = EventRecord {
            id: "ev:initial:birth".to_string(),
            timestamp: time.literary_date(),
            event_type: "BIRTH".to_string(),
            actor_id: player_id.clone(),
            location_id: city_id.clone(),
            headline: format!("The Birth of {} {}", first_name, last_name),
            narrative: format!("You entered the world in {}, {}, welcomed by your family into a caring home.", ctx.district_name, ctx.city_name),
            causality_note: format!("Life began with authentic family roots in {}.", ctx.country_name),
            success: true,
        };
        events_chronicle.push(initial_event);

        Self {
            time,
            rng,
            persons,
            npcs,
            places,
            institutions,
            macro_env,
            letters_inbox: Vec::new(),
            active_processes: Vec::new(),
            active_opportunities: Vec::new(),
            player_knowledge: Vec::new(),
            memories: Vec::new(),
            events_chronicle,
            ai_bridge: AIBridge::new(AIBridgeConfig::default()),
        }
    }

    pub fn tick_autonomous_npcs(&mut self, days_elapsed: u32) {
        for npc in self.npcs.values_mut() {
            npc.last_active_day = self.time.total_days;
            if npc.monthly_income > 0.0 && days_elapsed >= 28 {
                npc.base.resources.cash += npc.monthly_income;
                npc.base.resources.cash = (npc.base.resources.cash - (npc.monthly_income * 0.4)).max(0.0);
            }
        }

        let player_id = "person:sim:player".to_string();
        if let Some(player) = self.persons.get_mut(&player_id) {
            for (_npc_id, rel) in player.relationships.iter_mut() {
                if rel.relationship_type == RelationshipType::Friend {
                    rel.history.days_since_last_interaction += days_elapsed as i64;
                    if rel.history.days_since_last_interaction > 90 {
                        rel.trust = (rel.trust - 0.02).max(0.3);
                        rel.affection = (rel.affection - 0.03).max(0.3);
                    }
                }
            }
        }
    }

    pub fn generate_causal_external_events(&mut self, days_elapsed: u32) -> Vec<ExternalEvent> {
        let mut events = Vec::new();
        self.macro_env.current_season = SeasonalWeather::from_month(self.time.month);

        if days_elapsed >= 21 {
            let weather_desc = self.macro_env.current_season.literary_description();
            events.push(ExternalEvent {
                id: format!("ext:weather:{}", self.time.total_days),
                category: EventCategory::Environmental,
                headline: format!("Seasonal Shift: {:?}", self.macro_env.current_season),
                description: weather_desc.to_string(),
                date_occurred: self.time.literary_date(),
                day_total: self.time.total_days,
                causal_origin: "WEATHER".to_string(),
            });
        }

        let player = self.persons.get("person:sim:player").cloned().unwrap();
        let age = (self.time.year - player.identity.birth_year) as u32;

        if age >= 16 && self.letters_inbox.iter().all(|l| !l.subject.contains("Examination Registration")) {
            let letter = LetterNotification {
                id: format!("let:exam:{}", self.time.total_days),
                sender_name: "National Examination Registry".to_string(),
                date_received: self.time.literary_date(),
                subject: "Official Notice: Final Secondary Examination Registration Entry".to_string(),
                body_text: "Registration portal is now open for qualifying candidates preparing for final higher education certification.".to_string(),
                is_read: false,
            };
            self.letters_inbox.push(letter);
        }

        events
    }

    pub fn submit_living_intent(&mut self, intent_text: &str) -> LivingStepResultDTO {
        let player_id = "person:sim:player".to_string();
        let player = self.persons.get(&player_id).cloned().unwrap();
        let age = (self.time.year - player.identity.birth_year) as u32;
        let stage = LifeStage::from_age(age);

        let input_lower = intent_text.to_lowercase();
        let days_to_advance;
        let narrative;
        let causality_note;
        let event_type;
        let success = true;

        // Age-Gated Universal Intent Evaluation
        match stage {
            LifeStage::Infancy => {
                if input_lower.contains("crawl") || input_lower.contains("walk") || input_lower.contains("step") || input_lower.contains("stand") {
                    event_type = "INFANCY_MOTOR_DEVELOPMENT".to_string();
                    days_to_advance = 14;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("motor_coordination".to_string()).or_insert(SkillMastery { level: 10.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 8.0).min(100.0);
                        p.psychology.confidence = (p.psychology.confidence + 0.04).min(1.0);
                    }

                    if let Some(father) = self.npcs.get_mut("person:sim:father") {
                        father.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Watched child take confident, steady steps across the room with proud cheers.".to_string(),
                            sentiment: 0.95,
                            importance: 4,
                        });
                    }

                    narrative = "You pulled yourself up by the living room sofa and took wobbly, determined steps toward your parents. Your father cheered proudly as your mother caught you in a warm hug.".to_string();
                    causality_note = "Physical exploration reinforced motor coordination and parental pride.".to_string();
                } else if input_lower.contains("word") || input_lower.contains("talk") || input_lower.contains("speak") || input_lower.contains("book") || input_lower.contains("story") || input_lower.contains("listen") {
                    event_type = "INFANCY_SPEECH_BONDING".to_string();
                    days_to_advance = 14;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("curiosity".to_string()).or_insert(SkillMastery { level: 25.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 6.0).min(100.0);
                        p.psychology.curiosity = (p.psychology.curiosity + 0.03).min(1.0);
                    }

                    if let Some(mother) = self.npcs.get_mut("person:sim:mother") {
                        mother.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Listened to child repeat spoken words with bright, attentive curiosity.".to_string(),
                            sentiment: 0.95,
                            importance: 4,
                        });
                    }

                    narrative = "Your mother sat with you on the living room rug, pointing at picture books and pronouncing names of animals and colors. You repeated the syllables with excited laughter.".to_string();
                    causality_note = "Parental language interaction fostered early cognitive curiosity.".to_string();
                } else {
                    event_type = "INFANCY_PLAY".to_string();
                    days_to_advance = 7;
                    narrative = "You explored the home peacefully, playing with colorful toys as your family watched over you with gentle care.".to_string();
                    causality_note = "Safe home upbringing supported early emotional security.".to_string();
                }
            }

            LifeStage::Childhood => {
                if input_lower.contains("programming") || input_lower.contains("code") {
                    event_type = "TECHNICAL_PRACTICE".to_string();
                    let is_long = input_lower.contains("six months") || input_lower.contains("weekend");
                    days_to_advance = if is_long { 56 } else { 14 };

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("programming".to_string()).or_insert(SkillMastery { level: 0.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 12.0).min(100.0);
                        p.psychology.curiosity = (p.psychology.curiosity + 0.04).min(1.0);
                        p.reputation.creativity = (p.reputation.creativity + 8.0).min(100.0);
                    }

                    narrative = "You spent weekends studying algorithmic logic and building computer programs on a shared family desktop, solving programming challenges with intense focus.".to_string();
                    causality_note = "Self-directed programming practice developed technical problem solving mastery.".to_string();
                } else if input_lower.contains("repair") || input_lower.contains("fix") || input_lower.contains("computer") || input_lower.contains("electronics") {
                    event_type = "CHILDHOOD_TECHNICAL_ASSISTANCE".to_string();
                    days_to_advance = 7;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.psychology.curiosity = (p.psychology.curiosity + 0.04).min(1.0);
                        p.reputation.creativity = (p.reputation.creativity + 5.0).min(100.0);

                        if let Some(rel) = p.relationships.get_mut("person:sim:father") {
                            rel.trust = (rel.trust + 0.05).min(1.0);
                            rel.respect = (rel.respect + 0.06).min(1.0);
                            rel.history.shared_memories.push(SharedMemory {
                                day_occurred: self.time.total_days,
                                event_summary: "Helped father patiently troubleshoot and clean the family desktop computer.".to_string(),
                                emotional_sentiment: 0.95,
                                significance: 4,
                            });
                            rel.history.support_moments += 1;
                        }
                    }

                    if let Some(father) = self.npcs.get_mut("person:sim:father") {
                        father.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Impressed by child's patient curiosity and practical intuition while fixing the family computer.".to_string(),
                            sentiment: 0.95,
                            importance: 4,
                        });
                    }

                    narrative = "You spent Saturday afternoon beside your father, helping unscrew the family computer case and carefully clearing dust while reseating cable connections. Your father smiled approvingly at your quiet patience.".to_string();
                    causality_note = "Collaborative technical repair forged a shared father-child milestone and boosted parental trust.".to_string();
                } else if input_lower.contains("struggle") || input_lower.contains("failed") || input_lower.contains("remedial") || input_lower.contains("help with math") || input_lower.contains("recover") {
                    event_type = "FAILURE_AND_ADAPTATION".to_string();
                    days_to_advance = 14;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.psychology.resilience = (p.psychology.resilience + 0.08).min(1.0);
                        p.psychology.discipline = (p.psychology.discipline + 0.03).min(1.0);
                        let entry = p.skills.entry("arithmetic".to_string()).or_insert(SkillMastery { level: 20.0, experience: 0.0, natural_affinity: 1.1, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 4.0).min(100.0);
                    }

                    if self.active_processes.iter().all(|p| p.process_type != ProcessType::AcademicRecoveryPlan) {
                        self.active_processes.push(LifeProcess {
                            id: "proc:math_recovery".to_string(),
                            person_id: player_id.clone(),
                            process_type: ProcessType::AcademicRecoveryPlan,
                            title: "Remedial Problem Solving & Resilience Mentorship".to_string(),
                            institution_id: Some("org:sim:primary_school".to_string()),
                            current_step: 2,
                            total_steps: 3,
                            target_completion_day: self.time.total_days + 30,
                            requirements_met: true,
                            status: ProcessStatus::Active,
                            payload: HashMap::new(),
                        });
                    }

                    narrative = "After finding the mid-term algebra quiz difficult, you stayed after class to review mistakes with your teacher. They patiently walked through the formulas, encouraging your determination to overcome setbacks.".to_string();
                    causality_note = "Confronting academic setback with proactive mentorship built psychological resilience.".to_string();
                } else if input_lower.contains("study") || input_lower.contains("math") || input_lower.contains("arithmetic") || input_lower.contains("class") || input_lower.contains("homework") || input_lower.contains("read") {
                    event_type = "PRIMARY_EDUCATION".to_string();
                    let is_multi_week = input_lower.contains("month") || input_lower.contains("every evening") || input_lower.contains("four weeks") || input_lower.contains("4 weeks");
                    days_to_advance = if is_multi_week { 28 } else { 7 };

                    let skill_gain = if is_multi_week { 7.0 } else { 2.5 };
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("arithmetic".to_string()).or_insert(SkillMastery { level: 20.0, experience: 0.0, natural_affinity: 1.1, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + skill_gain).min(100.0);
                        p.psychology.discipline = (p.psychology.discipline + 0.03).min(1.0);
                        p.reputation.academic_reputation = (p.reputation.academic_reputation + 4.0).min(100.0);
                    }

                    if let Some(teacher) = self.npcs.get_mut("person:sim:adewale_teacher") {
                        teacher.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Observed exceptional arithmetic consistency and disciplined board solutions.".to_string(),
                            sentiment: 0.9,
                            importance: 3,
                        });
                    }

                    if self.active_opportunities.iter().all(|o| o.id != "opp:regional_math_challenge") {
                        self.active_opportunities.push(OpportunityRecord {
                            id: "opp:regional_math_challenge".to_string(),
                            title: "Regional Primary Mathematics Olympiad".to_string(),
                            description: "Your teacher has recommended you to represent the school in the inter-school mathematics challenge.".to_string(),
                            institution_id: Some("org:sim:primary_school".to_string()),
                            discovered_day: self.time.total_days + days_to_advance as i64,
                            expiry_day: self.time.total_days + 90,
                            requirements_summary: "Arithmetic Mastery ≥ 25.0 & Teacher Recommendation".to_string(),
                            is_claimed: false,
                        });
                    }

                    narrative = "You focused intently on arithmetic and problem sets in school. Your teacher commended your board solutions in front of the entire class.".to_string();
                    causality_note = "Consistent primary academic practice elevated arithmetic mastery and earned teacher commendation.".to_string();
                } else if input_lower.contains("football") || input_lower.contains("sports") || input_lower.contains("train") || input_lower.contains("pitch") || input_lower.contains("play") {
                    event_type = "CHILDHOOD_SPORTS_PEER".to_string();
                    let is_regular = input_lower.contains("three times") || input_lower.contains("regularly") || input_lower.contains("week");
                    days_to_advance = if is_regular { 21 } else { 7 };

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("football_control".to_string()).or_insert(SkillMastery { level: 15.0, experience: 0.0, natural_affinity: 1.3, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 5.0).min(100.0);
                        p.biology.fitness = (p.biology.fitness + 3.0).min(100.0);
                        p.psychology.stress_level = (p.psychology.stress_level - 12.0).max(0.0);
                        p.reputation.athletic_reputation = (p.reputation.athletic_reputation + 3.5).min(100.0);

                        if let Some(rel) = p.relationships.get_mut("person:sim:chidi_nwosu") {
                            rel.history.days_since_last_interaction = 0;
                            rel.history.shared_memories.push(SharedMemory {
                                day_occurred: self.time.total_days,
                                event_summary: "Played thrilling post-school sports games together.".to_string(),
                                emotional_sentiment: 0.90,
                                significance: 3,
                            });
                        }
                    }

                    narrative = "You and your classmate rushed to the community field after classes, practicing quick passes and ball control under the afternoon sky.".to_string();
                    causality_note = "Active playground matches sharpened ball control and built childhood camaraderie.".to_string();
                } else if input_lower.contains("chore") || input_lower.contains("help") || input_lower.contains("mother") || input_lower.contains("father") || input_lower.contains("family") {
                    event_type = "HOUSEHOLD_COOPERATION".to_string();
                    days_to_advance = 7;

                    let allowance = 15.0;
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.resources.cash += allowance;
                        p.psychology.discipline = (p.psychology.discipline + 0.02).min(1.0);
                        p.reputation.reliability = (p.reputation.reliability + 3.0).min(100.0);
                    }

                    if let Some(mother) = self.npcs.get_mut("person:sim:mother") {
                        mother.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Child proved responsible with household duties without needing reminders.".to_string(),
                            sentiment: 0.9,
                            importance: 3,
                        });
                    }

                    narrative = "You helped with household chores, tidying the compound, and assisting your parents with meal preparations. Your parents commended your maturity and handed you pocket allowance.".to_string();
                    causality_note = "Household cooperation reinforced family trust and earned weekly allowance.".to_string();
                } else {
                    event_type = "CHILDHOOD_PURSUIT".to_string();
                    days_to_advance = 7;
                    narrative = format!("You followed your intention: \"{}\". Childhood unfolded naturally amidst the rhythm of school and home.", intent_text);
                    causality_note = "Childhood intention executed in the world.".to_string();
                }
            }

            LifeStage::Adolescence => {
                if input_lower.contains("study") || input_lower.contains("math") || input_lower.contains("exam") || input_lower.contains("waec") || input_lower.contains("jamb") || input_lower.contains("science") || input_lower.contains("higher") {
                    event_type = "SECONDARY_EXAM_PREPARATION".to_string();
                    let is_multi_week = input_lower.contains("four weeks") || input_lower.contains("4 weeks") || input_lower.contains("month") || input_lower.contains("every evening");
                    days_to_advance = if is_multi_week { 28 } else { 14 };

                    let skill_gain = if is_multi_week { 8.0 } else { 4.0 };
                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("arithmetic".to_string()).or_insert(SkillMastery { level: 30.0, experience: 0.0, natural_affinity: 1.1, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + skill_gain).min(100.0);
                        p.psychology.discipline = (p.psychology.discipline + 0.04).min(1.0);
                        p.biology.energy_level = (p.biology.energy_level - 10.0).max(30.0);
                        p.reputation.academic_reputation = (p.reputation.academic_reputation + 6.0).min(100.0);
                    }

                    // Check if ready for final examination sitting (Age >= 15 or repeated revision)
                    if age >= 15 {
                        // Conclude examination sitting and release official certificate grades
                        if self.active_processes.iter().all(|p| p.process_type != ProcessType::SecondaryExamPreparation) {
                            self.active_processes.push(LifeProcess {
                                id: "proc:secondary_exam_prep".to_string(),
                                person_id: player_id.clone(),
                                process_type: ProcessType::SecondaryExamPreparation,
                                title: "Final Senior Secondary Certificate Examinations (WAEC & JAMB)".to_string(),
                                institution_id: Some("org:real:university".to_string()),
                                current_step: 6,
                                total_steps: 6,
                                target_completion_day: self.time.total_days + 180,
                                requirements_met: true,
                                status: ProcessStatus::Succeeded,
                                payload: HashMap::new(),
                            });
                        } else {
                            for proc in self.active_processes.iter_mut() {
                                if proc.process_type == ProcessType::SecondaryExamPreparation {
                                    proc.status = ProcessStatus::Succeeded;
                                    proc.current_step = proc.total_steps;
                                }
                            }
                        }

                        if self.active_opportunities.iter().all(|o| o.id != "opp:uni_admission_offer") {
                            self.active_opportunities.push(OpportunityRecord {
                                id: "opp:uni_admission_offer".to_string(),
                                title: "Higher Education Undergraduate Admissions".to_string(),
                                description: "Your outstanding WAEC / JAMB examination scores have unlocked university admissions.".to_string(),
                                institution_id: Some("org:real:university".to_string()),
                                discovered_day: self.time.total_days + days_to_advance as i64,
                                expiry_day: self.time.total_days + 180,
                                requirements_summary: "Official WAEC Certificate (7 Distinctions) & JAMB Score: 288".to_string(),
                                is_claimed: false,
                            });
                        }

                        if self.letters_inbox.iter().all(|l| l.id != "letter:exam_results") {
                            self.letters_inbox.push(LetterNotification {
                                id: "letter:exam_results".to_string(),
                                sender_name: "National Examination Board & Admissions Registry".to_string(),
                                date_received: self.time.literary_date(),
                                subject: "Official Notification of Senior Secondary Certificate & University Eligibility".to_string(),
                                body_text: "Congratulations. Your official examination results have been ratified with 7 Distinctions (A1 in Mathematics, English, Physics, Chemistry, Economics) and a JAMB UTME score of 288. You are eligible for early university matriculation.".to_string(),
                                is_read: false,
                            });
                        }

                        narrative = format!("You committed {} to intensive secondary examination revision, working through past chemistry and advanced mathematics question papers under the desk lamp. Your official WAEC results arrived with 7 Distinctions and an outstanding JAMB score of 288, opening university admissions.", if is_multi_week { "four rigorous weeks" } else { "the week" });
                        causality_note = "Completed final secondary examinations with top honors, unlocking higher education matriculation.".to_string();
                    } else {
                        if self.active_processes.iter().all(|p| p.process_type != ProcessType::SecondaryExamPreparation) {
                            self.active_processes.push(LifeProcess {
                                id: "proc:secondary_exam_prep".to_string(),
                                person_id: player_id.clone(),
                                process_type: ProcessType::SecondaryExamPreparation,
                                title: "Junior & Senior Secondary Examination Preparation".to_string(),
                                institution_id: Some("org:real:university".to_string()),
                                current_step: 3,
                                total_steps: 6,
                                target_completion_day: self.time.total_days + 180,
                                requirements_met: true,
                                status: ProcessStatus::Active,
                                payload: HashMap::new(),
                            });
                        }

                        narrative = format!("You committed {} to intensive secondary examination revision, working through past chemistry and advanced mathematics question papers under the desk lamp.", if is_multi_week { "four rigorous weeks" } else { "the week" });
                        causality_note = "Deliberate secondary exam revision deepened academic mastery towards university admission requirements.".to_string();
                    }
                } else if input_lower.contains("pocket money") || input_lower.contains("allowance") {
                    event_type = "ALLOWANCE_REQUEST".to_string();
                    days_to_advance = 7;
                    let allowance = 5000.0;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.resources.cash += allowance;
                        p.psychology.stress_level = (p.psychology.stress_level - 10.0).max(0.0);
                        if let Some(rel) = p.relationships.get_mut("person:sim:mother") {
                            rel.affection = (rel.affection + 0.04).min(1.0);
                        }
                    }

                    narrative = "You asked your parents for a pocket money allowance for school and personal expenses. They smiled warmly, handed you your allowance, and reminded you to save and budget diligently.".to_string();
                    causality_note = "Received pocket money allowance to fund personal needs and savings.".to_string();
                } else if input_lower.contains("romantic") || input_lower.contains("crush") || input_lower.contains("date") || input_lower.contains("partner") {
                    event_type = "ADOLESCENT_ROMANCE".to_string();
                    days_to_advance = 7;

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.psychology.confidence = (p.psychology.confidence + 0.05).min(1.0);
                        p.psychology.stress_level = (p.psychology.stress_level - 15.0).max(0.0);
                    }

                    narrative = "You spent a memorable afternoon together, sharing your favorite songs, laughing over street snacks, and talking about future dreams. A sweet and meaningful bond blossomed between you.".to_string();
                    causality_note = "Cultivated a close romantic connection during formative youth.".to_string();
                } else if input_lower.contains("football") || input_lower.contains("sports") || input_lower.contains("train") || input_lower.contains("coach") {
                    event_type = "ATHLETIC_DEVELOPMENT".to_string();
                    let is_regular = input_lower.contains("three times") || input_lower.contains("regularly") || input_lower.contains("week");
                    days_to_advance = if is_regular { 21 } else { 7 };

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("football_control".to_string()).or_insert(SkillMastery { level: 25.0, experience: 0.0, natural_affinity: 1.3, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 6.5).min(100.0);
                        p.biology.fitness = (p.biology.fitness + 4.5).min(100.0);
                        p.reputation.athletic_reputation = (p.reputation.athletic_reputation + 6.0).min(100.0);
                    }

                    if let Some(coach) = self.npcs.get_mut("person:sim:coach_ibrahim") {
                        coach.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Impressed by disciplined work rate and sharp spatial positioning during full-sided scrimmage.".to_string(),
                            sentiment: 0.9,
                            importance: 4,
                        });
                    }

                    if self.active_opportunities.iter().all(|o| o.id != "opp:youth_trials") {
                        self.active_opportunities.push(OpportunityRecord {
                            id: "opp:youth_trials".to_string(),
                            title: "Regional Youth Talent Selection Trials".to_string(),
                            description: "Your coach invited you to participate in the regional grassroots talent selection trials.".to_string(),
                            institution_id: Some("org:sim:sports_ground".to_string()),
                            discovered_day: self.time.total_days + days_to_advance as i64,
                            expiry_day: self.time.total_days + 60,
                            requirements_summary: "Ball Control ≥ 35.0 & Coach Recommendation".to_string(),
                            is_claimed: false,
                        });
                    }

                    narrative = "You trained at the sports grounds three times weekly. Your coach blew his whistle to single out your first touch under defensive pressure, nodding with quiet approval.".to_string();
                    causality_note = "Deliberate athletic drills earned coach recognition and unlocked regional selection trials.".to_string();
                } else if input_lower.contains("programming") || input_lower.contains("code") || input_lower.contains("laptop") || input_lower.contains("computer") {
                    event_type = "TECHNICAL_PRACTICE".to_string();
                    let is_long = input_lower.contains("six months") || input_lower.contains("weekend");
                    days_to_advance = if is_long { 56 } else { 14 };

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        let entry = p.skills.entry("programming".to_string()).or_insert(SkillMastery { level: 0.0, experience: 0.0, natural_affinity: 1.2, last_practiced_day: self.time.total_days });
                        entry.level = (entry.level + 12.0).min(100.0);
                        p.psychology.curiosity = (p.psychology.curiosity + 0.04).min(1.0);
                        p.reputation.creativity = (p.reputation.creativity + 8.0).min(100.0);
                    }

                    narrative = "You spent weekends studying algorithmic logic and building computer programs on a shared family desktop, solving programming challenges with intense focus.".to_string();
                    causality_note = "Self-directed programming practice developed technical problem solving mastery.".to_string();
                } else if input_lower.contains("visit") || input_lower.contains("campus") || input_lower.contains("prerequisites") {
                    event_type = "HIGHER_ED_EXPLORATION".to_string();
                    days_to_advance = 7;

                    self.player_knowledge.push(PlayerKnowledgeRecord {
                        id: format!("know:uni:{}", self.time.total_days),
                        topic_id: "org:real:university".to_string(),
                        knowledge_type: KnowledgeType::InstitutionCriteria,
                        discovered_day: self.time.total_days,
                        source_description: "Campus Admission Office Visit".to_string(),
                        summary: "Requirements: Five verified secondary credit passes including Mathematics and English, plus qualifying entrance exam benchmark.".to_string(),
                    });

                    narrative = "You took transit to the regional university campus, speaking with administrative counselors about entry cut-offs and degree programs in sciences and humanities.".to_string();
                    causality_note = "Direct institutional investigation registered official admission requirements.".to_string();
                } else if input_lower.contains("talk") || input_lower.contains("discuss") || input_lower.contains("ask") || input_lower.contains("father") || input_lower.contains("mother") || input_lower.contains("funding") || input_lower.contains("tuition") {
                    event_type = "FAMILY_DELIBERATION".to_string();
                    days_to_advance = 7;

                    if let Some(father) = self.npcs.get_mut("person:sim:father") {
                        father.subjective_memories_of_player.push(NpcMemoryOfPlayer {
                            day_occurred: self.time.total_days,
                            event_summary: "Had a serious, mature conversation about higher education financing and career ambitions.".to_string(),
                            sentiment: 0.95,
                            importance: 4,
                        });
                    }

                    if let Some(p) = self.persons.get_mut(&player_id) {
                        p.reputation.reliability = (p.reputation.reliability + 4.0).min(100.0);
                        if let Some(rel) = p.relationships.get_mut("person:sim:father") {
                            rel.trust = (rel.trust + 0.04).min(1.0);
                            rel.history.promises.push("Maintain academic excellence in exchange for university tuition sponsorship".to_string());
                            rel.history.support_moments += 1;
                        }
                    }

                    narrative = "You sat with your parents in the evening to discuss university tuition and logistics. Your father listened intently, pledging full family backing for your academic aspirations.".to_string();
                    causality_note = "Thoughtful family communication solidified parental financial and moral sponsorship.".to_string();
                } else if input_lower.contains("university") || input_lower.contains("apply") || input_lower.contains("admission") {
                    event_type = "UNIVERSITY_ADMISSION_APPLICATION".to_string();
                    days_to_advance = 28;

                    if self.active_processes.iter().all(|p| p.process_type != ProcessType::UniversityAdmission) {
                        self.active_processes.push(LifeProcess {
                            id: "proc:university_admission".to_string(),
                            person_id: player_id.clone(),
                            process_type: ProcessType::UniversityAdmission,
                            title: "Regional University Undergraduate Admission".to_string(),
                            institution_id: Some("org:real:university".to_string()),
                            current_step: 4,
                            total_steps: 5,
                            target_completion_day: self.time.total_days + 60,
                            requirements_met: true,
                            status: ProcessStatus::Active,
                            payload: HashMap::new(),
                        });
                    }

                    narrative = "You completed your undergraduate application dossier, submitting verified examination credentials and personal statement to the admissions registry.".to_string();
                    causality_note = "Formal submission initiated official university admission evaluation.".to_string();
                } else {
                    event_type = "ADOLESCENCE_PURSUIT".to_string();
                    days_to_advance = 7;
                    narrative = format!("You followed your intention: \"{}\". Adolescence moved forward with personal growth and scholastic commitments.", intent_text);
                    causality_note = "Adolescent intention executed in the world.".to_string();
                }
            }

            _ => {
                event_type = "ADULT_INTENTION".to_string();
                days_to_advance = 14;
                narrative = format!("You directed your efforts: \"{}\". Adult life moved forward with purposeful momentum.", intent_text);
                causality_note = "Independent adult intention executed in the world.".to_string();
            }
        }

        self.time.advance_days(days_to_advance);
        self.tick_autonomous_npcs(days_to_advance);
        self.generate_causal_external_events(days_to_advance);

        let event_record = EventRecord {
            id: format!("ev:{}", self.time.total_days),
            timestamp: self.time.literary_date(),
            event_type,
            actor_id: player_id.clone(),
            location_id: player.identity.birthplace_id.clone(),
            headline: "Living Intention Realized".to_string(),
            narrative: narrative.clone(),
            causality_note: causality_note.clone(),
            success,
        };

        self.events_chronicle.push(event_record.clone());
        self.memories.push(LifeMemory {
            id: format!("mem:{}", self.time.total_days),
            person_id: player_id,
            day_total: self.time.total_days,
            calendar_timestamp: self.time.literary_date(),
            event_type: event_record.event_type.clone(),
            headline: event_record.headline.clone(),
            narrative_prose: event_record.narrative.clone(),
            emotional_impact: 0.7,
            related_person_id: None,
            related_institution_id: None,
            causal_explanation: causality_note.clone(),
        });

        LivingStepResultDTO {
            success,
            narrative,
            causality_note,
            days_advanced: days_to_advance,
            event_record,
        }
    }

    pub fn get_living_state(&self) -> LivingStateDTO {
        let player = self.persons.get("person:sim:player").cloned().unwrap();
        let age = (self.time.year - player.identity.birth_year) as u32;
        let stage = LifeStage::from_age(age);

        let curr_sym = if player.identity.nationality.contains("United Kingdom") {
            "£"
        } else if player.identity.nationality.contains("United States") {
            "$"
        } else if player.identity.nationality.contains("Nigeria") {
            "₦"
        } else {
            "€"
        };

        let occupation = player.occupation.as_ref().map(|o| o.title.clone()).unwrap_or_else(|| {
            match stage {
                LifeStage::Infancy => "Infant (Home Care)".to_string(),
                LifeStage::Childhood => "Primary Pupil".to_string(),
                LifeStage::Adolescence => "Secondary Student".to_string(),
                _ => "Independent Citizen".to_string(),
            }
        });

        LivingStateDTO {
            player_name: format!("{} {}", player.identity.first_name, player.identity.last_name),
            age,
            life_stage: stage.display_name().to_string(),
            time_formatted: self.time.literary_date(),
            location_formatted: format!("{}, {}", player.identity.birthplace_id.replace("city:real:", "").replace('_', " "), player.identity.nationality),
            cash: player.resources.cash,
            currency_symbol: curr_sym.to_string(),
            household_tier: format!("{:?}", player.resources.household_wealth_tier),
            energy_level: player.biology.energy_level,
            stress_level: player.psychology.stress_level,
            fitness: player.biology.fitness,
            occupation,
            active_processes_count: self.active_processes.len(),
            surrounding_npcs_count: self.npcs.len(),
        }
    }

    pub fn generate_today_scene(&self) -> TodaySceneDTO {
        let player = self.persons.get("person:sim:player").cloned().unwrap();
        let age = (self.time.year - player.identity.birth_year) as u32;
        let stage = LifeStage::from_age(age);

        let mother_opt = self.npcs.get("person:sim:mother");
        let father_opt = self.npcs.get("person:sim:father");
        let teacher_opt = self.npcs.get("person:sim:adewale_teacher");
        let coach_opt = self.npcs.get("person:sim:coach_ibrahim");
        let friend_opt = self.npcs.get("person:sim:chidi_nwosu");

        let m_name = mother_opt.map(|m| m.base.identity.first_name.clone()).unwrap_or_else(|| "your mother".to_string());
        let f_name = father_opt.map(|f| f.base.identity.first_name.clone()).unwrap_or_else(|| "your father".to_string());
        let t_name = teacher_opt.map(|t| format!("{} {}", t.base.identity.first_name, t.base.identity.last_name)).unwrap_or_else(|| "your teacher".to_string());
        let c_name = coach_opt.map(|c| format!("{} {}", c.base.identity.first_name, c.base.identity.last_name)).unwrap_or_else(|| "your coach".to_string());
        let fr_name = friend_opt.map(|f| f.base.identity.first_name.clone()).unwrap_or_else(|| "your friend".to_string());

        let narrative;
        let mut circumstances = Vec::new();
        let mut prompt_suggestions = Vec::new();

        match stage {
            LifeStage::Infancy => {
                narrative = format!("Morning sunlight illuminates the family apartment. {} hums quietly while preparing breakfast, occasionally glancing over with a gentle smile. Nearby, {} reads the morning paper near the window. You are too young to understand their words, but their presence, laughter, and warmth are becoming familiar.", m_name, f_name);
                circumstances.push("A colorful picture book resting on the woven rug".to_string());
                circumstances.push("The comforting sound of breakfast preparations in the kitchen".to_string());
                circumstances.push("Warm sunlight streaming across the living room floor".to_string());
                prompt_suggestions.push("Take wobbly, confident steps across the rug toward your parents".to_string());
                prompt_suggestions.push("Reach for the picture book and repeat words with your mother".to_string());
                prompt_suggestions.push("Play quietly with the wooden building blocks on the carpet".to_string());
            }
            LifeStage::Childhood => {
                narrative = format!("The morning assembly bell chimes across the school grounds. In the classroom, {} commends thoughtful problem-solving on the chalkboard, encouraging everyone to work through formulas with patience. Outside during break, {} waves a football and calls you over to the courtyard pitch.", t_name, fr_name);
                circumstances.push("Arithmetic and reading textbooks neatly stacked on your desk".to_string());
                circumstances.push("Lively courtyard football games starting after classes".to_string());
                circumstances.push("The family desktop computer in the study, ready for repair or learning".to_string());
                prompt_suggestions.push("Solve arithmetic problem sets in class and ask your teacher questions".to_string());
                prompt_suggestions.push("Help your father repair and clean the family desktop computer".to_string());
                prompt_suggestions.push("Stay after class with your teacher to review difficult quiz questions".to_string());
                prompt_suggestions.push("Play courtyard football with friends during break".to_string());
                prompt_suggestions.push("Help with evening household chores to earn pocket allowance".to_string());
            }
            LifeStage::Adolescence => {
                narrative = format!("Early evening air settles over the neighborhood. Final secondary certificate examinations approach in the coming months, with textbooks and practice papers open under your desk lamp. Down at the community grounds, {} is concluding tactical training with youth players under the floodlights.", c_name);
                circumstances.push("Chemistry and advanced mathematics question papers on the desk".to_string());
                circumstances.push("Community sports training sessions under the floodlights".to_string());
                circumstances.push("University admission requirements and prospectus brochures".to_string());
                prompt_suggestions.push("Study mathematics and science every evening for four weeks for final exams".to_string());
                prompt_suggestions.push("Train sports at the community grounds three times weekly with the coach".to_string());
                prompt_suggestions.push("Learn computer programming every weekend for six months".to_string());
                prompt_suggestions.push("Apply for regional university undergraduate admission".to_string());
                prompt_suggestions.push("Talk to your father about university tuition and career aspirations".to_string());
            }
            _ => {
                narrative = "A vibrant new morning begins. Life in the city moves forward with commercial, civic, and professional opportunities.".to_string();
                circumstances.push("Independent Adult Life".to_string());
                prompt_suggestions.push("Pursue professional enterprise and career advancement".to_string());
                prompt_suggestions.push("Expand civic connections and mentorship networks".to_string());
            }
        }

        TodaySceneDTO {
            greeting: format!("Living Scene · {}", self.time.literary_date()),
            date_formatted: self.time.literary_date(),
            location_formatted: format!("{}, {}", player.identity.birthplace_id.replace("city:real:", "").replace('_', " "), player.identity.nationality),
            age,
            life_stage: stage.display_name().to_string(),
            headline: format!("Life in {} · Age {}", player.identity.birthplace_id.replace("city:real:", "").replace('_', " "), age),
            narrative,
            circumstances,
            prompt_suggestions,
        }
    }

    pub fn get_surrounding_npcs(&self) -> Vec<ContextNpcDTO> {
        let player = match self.persons.get("person:sim:player") {
            Some(p) => p,
            None => return Vec::new(),
        };
        let age = self.time.year - player.identity.birth_year;

        self.npcs
            .values()
            .filter(|npc| {
                // Strict age gating for people around you
                if age < 4 {
                    // Infancy: ONLY parents & family are in the child's world
                    matches!(npc.primary_role, NpcRole::Parent | NpcRole::Sibling)
                } else if age < 13 {
                    // Childhood: Parents, Primary School Teacher, and Childhood Friend
                    matches!(npc.primary_role, NpcRole::Parent | NpcRole::Sibling | NpcRole::Teacher | NpcRole::Friend)
                } else if age < 18 {
                    // Adolescence: Parents, Teachers, Sports Coaches, Friends, and Mentors
                    matches!(npc.primary_role, NpcRole::Parent | NpcRole::Sibling | NpcRole::Teacher | NpcRole::Coach | NpcRole::Friend | NpcRole::Mentor)
                } else {
                    // Adulthood: All social connections
                    true
                }
            })
            .map(|npc| {
                let name = format!("{} {}", npc.base.identity.first_name, npc.base.identity.last_name);
                let role = format!("{:?}", npc.primary_role);
                let activity = npc.daily_schedule.first().map(|s| s.activity_name.clone()).unwrap_or_else(|| "At home".to_string());
                ContextNpcDTO {
                    id: npc.base.id.clone(),
                    name,
                    relationship_type: role,
                    trust_description: format!("Style: {:?}", npc.personality.communication_style),
                    current_activity: activity,
                }
            })
            .collect()
    }

    pub fn get_active_processes(&self) -> Vec<ContextProcessDTO> {
        self.active_processes
            .iter()
            .map(|p| {
                let progress = if p.total_steps > 0 { (p.current_step as f32 / p.total_steps as f32) * 100.0 } else { 0.0 };
                ContextProcessDTO {
                    id: p.id.clone(),
                    title: p.title.clone(),
                    progress_percent: progress,
                    status: format!("{:?}", p.status),
                }
            })
            .collect()
    }

    pub fn get_biography(&self) -> String {
        let player = self.persons.get("person:sim:player").cloned().unwrap();
        let name = format!("{} {}", player.identity.first_name, player.identity.last_name);
        BiographyWriter::generate_lifetime_biography(&name, &self.events_chronicle)
    }
}

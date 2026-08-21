use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =========================================================================
// 1. UNIVERSAL HUMAN ENTITY PRIMITIVES
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntity {
    pub id: String,
    pub identity: IdentityProfile,
    pub biology: BiologicalProfile,
    pub psychology: PsychologicalProfile,
    pub reputation: ReputationProfile,
    pub skills: HashMap<String, SkillMastery>,
    pub resources: HumanResources,
    pub relationships: HashMap<String, RelationshipVector>,
    pub occupation: Option<OccupationRecord>,
    pub is_player: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProfile {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub sex: String,
    pub birthplace_id: String,
    pub nationality: String,
    pub culture: String,
    pub primary_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalProfile {
    pub is_alive: bool,
    pub death_year: Option<i32>,
    pub death_reason: Option<String>,
    pub health_overall: f32, // 0.0 to 100.0
    pub fitness: f32,        // 0.0 to 100.0
    pub energy_level: f32,   // 0.0 to 100.0
    pub chronic_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalProfile {
    pub discipline: f32,     // 0.0 to 1.0 (habits, consistency)
    pub curiosity: f32,      // 0.0 to 1.0 (exploration, learning rate)
    pub creativity: f32,     // 0.0 to 1.0 (artistic/problem solving)
    pub confidence: f32,     // 0.0 to 1.0 (social, trials, interviews)
    pub risk_tolerance: f32, // 0.0 to 1.0
    pub stress_level: f32,   // 0.0 to 100.0
    pub resilience: f32,     // 0.0 to 1.0 (failure recovery)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationProfile {
    pub academic_reputation: f32, // 0.0 to 100.0
    pub athletic_reputation: f32, // 0.0 to 100.0
    pub reliability: f32,         // 0.0 to 100.0
    pub kindness: f32,            // 0.0 to 100.0
    pub creativity: f32,          // 0.0 to 100.0
    pub leadership: f32,          // 0.0 to 100.0
}

impl Default for ReputationProfile {
    fn default() -> Self {
        Self {
            academic_reputation: 20.0,
            athletic_reputation: 15.0,
            reliability: 50.0,
            kindness: 60.0,
            creativity: 30.0,
            leadership: 20.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub warmth: f32,          // 0.0 to 1.0 (nurturing, supportive)
    pub patience: f32,        // 0.0 to 1.0 (tolerance for errors)
    pub strictness: f32,      // 0.0 to 1.0 (standards, discipline)
    pub ambition: f32,        // 0.0 to 1.0 (drive for achievement)
    pub risk_tolerance: f32,  // 0.0 to 1.0
    pub communication_style: CommunicationStyle,
    pub core_values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunicationStyle {
    Nurturing,
    Disciplinarian,
    Inspirational,
    Direct,
    Playful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMastery {
    pub level: f32,            // 0.0 to 100.0 (internal mastery)
    pub experience: f64,
    pub natural_affinity: f32, // multiplier
    pub last_practiced_day: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanResources {
    pub cash: f64,
    pub household_wealth_tier: WealthTier,
    pub living_arrangement: String,   // "FAMILY_HOME", "RENTED_APARTMENT", "DORMITORY", "OWNED_HOME"
    pub tools_available: Vec<String>, // "SMARTPHONE", "PERSONAL_COMPUTER", "FOOTBALL_BOOTS", "BOOKS"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WealthTier {
    Poverty,
    WorkingClass,
    MiddleClass,
    UpperMiddle,
    Wealthy,
}

impl WealthTier {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "POOR" | "POVERTY" => WealthTier::Poverty,
            "WORKING" | "WORKING_CLASS" => WealthTier::WorkingClass,
            "UPPER_MIDDLE" => WealthTier::UpperMiddle,
            "WEALTHY" | "RICH" => WealthTier::Wealthy,
            _ => WealthTier::MiddleClass,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifeStage {
    Infancy,       // 0 - 3
    Childhood,     // 4 - 12
    Adolescence,   // 13 - 17
    EarlyAdulthood,// 18 - 29
    Adulthood,     // 30 - 64
    SeniorYears,   // 65+
}

impl LifeStage {
    pub fn from_age(age: u32) -> Self {
        match age {
            0..=3 => LifeStage::Infancy,
            4..=12 => LifeStage::Childhood,
            13..=17 => LifeStage::Adolescence,
            18..=29 => LifeStage::EarlyAdulthood,
            30..=64 => LifeStage::Adulthood,
            _ => LifeStage::SeniorYears,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LifeStage::Infancy => "Infancy (Ages 0–3)",
            LifeStage::Childhood => "Childhood (Ages 4–12)",
            LifeStage::Adolescence => "Adolescence (Ages 13–17)",
            LifeStage::EarlyAdulthood => "Early Adulthood (Ages 18–29)",
            LifeStage::Adulthood => "Adulthood (Ages 30–64)",
            LifeStage::SeniorYears => "Senior Years (Ages 65+)",
        }
    }

    pub fn can_work_full_time(&self) -> bool {
        matches!(self, LifeStage::EarlyAdulthood | LifeStage::Adulthood | LifeStage::SeniorYears)
    }

    pub fn can_transact_independent_credit(&self) -> bool {
        matches!(self, LifeStage::EarlyAdulthood | LifeStage::Adulthood | LifeStage::SeniorYears)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupationRecord {
    pub title: String,
    pub employer_org_id: Option<String>,
    pub monthly_earnings: f64,
    pub start_year: i32,
}

// =========================================================================
// 2. SOCIAL GRAPH & AUTONOMOUS NPC PRIMITIVES
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipVector {
    pub source_person_id: String,
    pub target_person_id: String,
    pub relationship_type: RelationshipType,
    pub trust: f32,      // 0.0 to 1.0
    pub affection: f32,  // 0.0 to 1.0
    pub respect: f32,    // 0.0 to 1.0
    pub resentment: f32, // 0.0 to 1.0
    pub history: RelationshipHistory,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipHistory {
    pub shared_memories: Vec<SharedMemory>,
    pub promises: Vec<String>,
    pub support_moments: u32,
    pub conflict_moments: u32,
    pub days_since_last_interaction: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedMemory {
    pub day_occurred: i64,
    pub event_summary: String,
    pub emotional_sentiment: f32, // -1.0 to +1.0
    pub significance: u32,        // 1 to 5
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipType {
    Mother,
    Father,
    Sibling,
    Teacher,
    Classmate,
    Friend,
    Coach,
    Mentor,
    Rival,
    Romance,
    Colleague,
    Employer,
    Neighbor,
    Acquaintance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousNPC {
    pub base: HumanEntity,
    pub primary_role: NpcRole,
    pub personality: PersonalityProfile,
    pub daily_schedule: Vec<DailyRoutineBlock>,
    pub life_goal: String,
    pub subjective_memories_of_player: Vec<NpcMemoryOfPlayer>,
    pub monthly_income: f64,
    pub stress_level: f32,
    pub last_active_day: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NpcRole {
    Parent,
    Sibling,
    Teacher,
    Classmate,
    Friend,
    Coach,
    Mentor,
    Colleague,
    Employer,
    Partner,
    Neighbor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRoutineBlock {
    pub start_hour: u32,
    pub end_hour: u32,
    pub activity_name: String,
    pub location_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcMemoryOfPlayer {
    pub day_occurred: i64,
    pub event_summary: String,
    pub sentiment: f32, // -1.0 to +1.0
    pub importance: u32,
}

// =========================================================================
// 3. WORLD GEOGRAPHY, INSTITUTIONS & MACRO ENVIRONMENT
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPlace {
    pub id: String,
    pub name: String,
    pub place_type: PlaceType,
    pub parent_place_id: Option<String>,
    pub country_id: String,
    pub climate_zone: String,
    pub cost_of_living_index: f32,
    pub culture_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceType {
    Country,
    Region,
    City,
    District,
    Place,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroEnvironment {
    pub inflation_rate: f32,               // e.g. 0.12 (12%)
    pub power_grid_reliability: f32,       // 0.0 to 1.0 (frequency of outages)
    pub current_season: SeasonalWeather,
    pub market_cost_index: f32,            // 1.0 baseline
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SeasonalWeather {
    HarmattanHaze, // Nov - Feb (Dry dusty winds from the Sahara)
    EarlyRainfall,  // Mar - May (Sudden thunderstorms, rising humidity)
    HeavyMonsoon,   // Jun - Aug (Intense downpours, occasional road flooding)
    LateHarvest,    // Sep - Oct (Warm, clear tropical skies)
}

impl SeasonalWeather {
    pub fn from_month(month: u32) -> Self {
        match month {
            11 | 12 | 1 | 2 => SeasonalWeather::HarmattanHaze,
            3 | 4 | 5 => SeasonalWeather::EarlyRainfall,
            6 | 7 | 8 => SeasonalWeather::HeavyMonsoon,
            _ => SeasonalWeather::LateHarvest,
        }
    }

    pub fn literary_description(&self) -> &'static str {
        match self {
            SeasonalWeather::HarmattanHaze => "Harmattan haze coats the rooftops of Abuja in a pale Saharan dust, casting a cool, diffused morning light.",
            SeasonalWeather::EarlyRainfall => "Warm, humid morning air precedes distant rolling thunder over the Aso Rock hills.",
            SeasonalWeather::HeavyMonsoon => "Heavy monsoon rains drum rhythmically against the corrugated metal roofs of Garki.",
            SeasonalWeather::LateHarvest => "Golden tropical sunshine breaks through clear blue skies, bringing a pleasant warmth across the city.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionEntity {
    pub id: String,
    pub name: String,
    pub institution_type: InstitutionType,
    pub location_id: String,
    pub prestige: f32, // 0.0 to 1.0
    pub admission_requirements: Vec<AdmissionRequirement>,
    pub active_members: Vec<String>, // person IDs
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstitutionType {
    PrimarySchool,
    SecondarySchool,
    University,
    SportsClub,
    Company,
    Hospital,
    GovernmentOrganization,
    CulturalCenter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdmissionRequirement {
    MinimumAge(u32),
    MaximumAge(u32),
    AcademicPerformance(f32),
    RequiredSkill { skill_id: String, min_level: f32 },
    TuitionCost(f64),
    DocumentRequired(String), // "BIRTH_CERTIFICATE", "PRIMARY_LEAVING_CERT", "WAEC_RESULTS", "PASSPORT"
}

// =========================================================================
// 4. UNIVERSAL PROCESS & EVENT PRIMITIVES
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeProcess {
    pub id: String,
    pub person_id: String,
    pub process_type: ProcessType,
    pub title: String,
    pub institution_id: Option<String>,
    pub current_step: u32,
    pub total_steps: u32,
    pub target_completion_day: i64,
    pub requirements_met: bool,
    pub status: ProcessStatus,
    pub payload: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessType {
    PrimaryEducation,
    SecondaryExamPreparation, // WAEC & JAMB
    UniversityAdmission,
    YouthSportsAcademyTrial,
    JobApplication,
    ApprenticeshipTrade,
    AcademicRecoveryPlan,     // Remedial tutoring & resilience
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessStatus {
    Active,
    Paused,
    Succeeded,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityRecord {
    pub id: String,
    pub title: String,
    pub description: String,
    pub institution_id: Option<String>,
    pub discovered_day: i64,
    pub expiry_day: i64,
    pub requirements_summary: String,
    pub is_claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEvent {
    pub id: String,
    pub category: EventCategory,
    pub headline: String,
    pub description: String,
    pub date_occurred: String,
    pub day_total: i64,
    pub causal_origin: String, // "FAMILY", "NPC", "INSTITUTION", "MACRO_ECONOMY", "WEATHER"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventCategory {
    Family,
    NpcAction,
    Institutional,
    Economic,
    Environmental,
    Serendipity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterNotification {
    pub id: String,
    pub sender_name: String,
    pub date_received: String,
    pub subject: String,
    pub body_text: String,
    pub is_read: bool,
}

// =========================================================================
// 5. KNOWLEDGE & EPISODIC MEMORY
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerKnowledgeRecord {
    pub id: String,
    pub topic_id: String,
    pub knowledge_type: KnowledgeType,
    pub discovered_day: i64,
    pub source_description: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KnowledgeType {
    InstitutionCriteria,
    NpcObservation,
    OpportunityDetails,
    Rumour,
    PersonalInsight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeMemory {
    pub id: String,
    pub person_id: String,
    pub day_total: i64,
    pub calendar_timestamp: String,
    pub event_type: String,
    pub headline: String,
    pub narrative_prose: String,
    pub emotional_impact: f32, // -1.0 to +1.0
    pub related_person_id: Option<String>,
    pub related_institution_id: Option<String>,
    pub causal_explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: String,
    pub timestamp: String,
    pub event_type: String,
    pub actor_id: String,
    pub location_id: String,
    pub headline: String,
    pub narrative: String,
    pub causality_note: String,
    pub success: bool,
}

// =========================================================================
// 6. SIMULATION TIME & DATA TRANSFER OBJECTS (DTOs)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub day_of_week: u32,
    pub total_days: i64,
}

impl SimTime {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        let total_days = (year as i64 * 365) + (month as i64 * 30) + day as i64;
        Self {
            year,
            month,
            day,
            day_of_week: ((total_days % 7) + 1) as u32,
            total_days,
        }
    }

    pub fn advance_days(&mut self, days: u32) {
        self.day += days;
        self.total_days += days as i64;
        while self.day > 30 {
            self.day -= 30;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
        self.day_of_week = ((self.total_days % 7) + 1) as u32;
    }

    pub fn literary_date(&self) -> String {
        let month_name = match self.month {
            1 => "January", 2 => "February", 3 => "March", 4 => "April",
            5 => "May", 6 => "June", 7 => "July", 8 => "August",
            9 => "September", 10 => "October", 11 => "November", _ => "December",
        };
        let weekday = match self.day_of_week {
            1 => "Monday", 2 => "Tuesday", 3 => "Wednesday",
            4 => "Thursday", 5 => "Friday", 6 => "Saturday", _ => "Sunday",
        };
        format!("{}, {} {} {}", weekday, self.day, month_name, self.year)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLifeConfig {
    pub creation_mode: String,
    pub starting_year: i32,
    pub country_id: String,
    pub location_id: String,
    pub starting_age: u32,
    pub birth_year: Option<i32>,
    pub birth_month: Option<u32>,
    pub birth_day: Option<u32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sex: Option<String>,
    pub household_income_tier: Option<String>,
    pub mother_name: Option<String>,
    pub mother_job: Option<String>,
    pub father_name: Option<String>,
    pub father_job: Option<String>,
    pub custom_backstory: Option<String>,
    #[serde(default)]
    pub traits: HashMap<String, f32>,
    #[serde(default)]
    pub skills: HashMap<String, f32>,
    #[serde(default)]
    pub interests: Vec<String>,
    #[serde(default)]
    pub goals: Vec<String>,
}

impl Default for NewLifeConfig {
    fn default() -> Self {
        Self {
            creation_mode: "CUSTOM".to_string(),
            starting_year: 2005,
            country_id: "country:real:nigeria".to_string(),
            location_id: "city:real:abuja".to_string(),
            starting_age: 0,
            birth_year: Some(2005),
            birth_month: Some(1),
            birth_day: Some(15),
            first_name: Some("Israel".to_string()),
            last_name: Some("Oyebamiji".to_string()),
            sex: Some("Male".to_string()),
            household_income_tier: Some("MIDDLE".to_string()),
            mother_name: None,
            mother_job: None,
            father_name: None,
            father_job: None,
            custom_backstory: None,
            traits: HashMap::new(),
            skills: HashMap::new(),
            interests: vec!["academics".to_string()],
            goals: vec!["excellence".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingStateDTO {
    pub player_name: String,
    pub age: u32,
    pub life_stage: String,
    pub time_formatted: String,
    pub location_formatted: String,
    pub cash: f64,
    pub currency_symbol: String,
    pub household_tier: String,
    pub energy_level: f32,
    pub stress_level: f32,
    pub fitness: f32,
    pub occupation: String,
    pub active_processes_count: usize,
    pub surrounding_npcs_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingStepResultDTO {
    pub success: bool,
    pub narrative: String,
    pub causality_note: String,
    pub days_advanced: u32,
    pub event_record: EventRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodaySceneDTO {
    pub greeting: String,
    pub date_formatted: String,
    pub location_formatted: String,
    pub age: u32,
    pub life_stage: String,
    pub headline: String,
    pub narrative: String,
    pub circumstances: Vec<String>,
    pub prompt_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextNpcDTO {
    pub id: String,
    pub name: String,
    pub relationship_type: String,
    pub trust_description: String,
    pub current_activity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextProcessDTO {
    pub id: String,
    pub title: String,
    pub progress_percent: f32,
    pub status: String,
}

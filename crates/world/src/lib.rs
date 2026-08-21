use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =========================================================================
// 1. CANONICAL HUMAN & SOCIAL ENTITIES
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

impl IdentityProfile {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn calculate_age(&self, current_year: i32, current_month: u32, current_day: u32) -> u32 {
        if current_year < self.birth_year {
            return 0;
        }
        let mut age = (current_year - self.birth_year) as u32;
        if current_month < self.birth_month || (current_month == self.birth_month && current_day < self.birth_day) {
            if age > 0 {
                age -= 1;
            }
        }
        age
    }

    pub fn is_birthday(&self, current_month: u32, current_day: u32) -> bool {
        self.birth_month == current_month && self.birth_day == current_day
    }
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
    pub discipline: f32,     // 0.0 to 1.0
    pub curiosity: f32,      // 0.0 to 1.0
    pub creativity: f32,     // 0.0 to 1.0
    pub confidence: f32,     // 0.0 to 1.0
    pub risk_tolerance: f32, // 0.0 to 1.0
    pub stress_level: f32,   // 0.0 to 100.0
    pub resilience: f32,     // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReputationProfile {
    pub academic_reputation: f32, // 0.0 to 100.0
    pub athletic_reputation: f32, // 0.0 to 100.0
    pub reliability: f32,         // 0.0 to 100.0
    pub kindness: f32,            // 0.0 to 100.0
    pub creativity: f32,          // 0.0 to 100.0
    pub leadership: f32,          // 0.0 to 100.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMastery {
    pub level: f32,            // 0.0 to 100.0
    pub experience: f64,
    pub natural_affinity: f32, // multiplier
    pub last_practiced_day: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanResources {
    pub cash: f64,
    pub household_wealth_tier: WealthTier,
    pub living_arrangement: String,
    pub tools_available: Vec<String>,
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
    Infancy,        // 0 - 3
    Childhood,      // 4 - 12
    Adolescence,    // 13 - 17
    EarlyAdulthood, // 18 - 29
    Adulthood,      // 30 - 64
    SeniorYears,    // 65+
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
// 2. RELATIONSHIPS & NPCS
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
    pub emotional_sentiment: f32,
    pub significance: u32,
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
pub struct PersonalityProfile {
    pub warmth: f32,
    pub patience: f32,
    pub strictness: f32,
    pub ambition: f32,
    pub risk_tolerance: f32,
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
pub struct DailyRoutineBlock {
    pub start_hour: u8,
    pub end_hour: u8,
    pub activity_name: String,
    pub location_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcMemoryOfPlayer {
    pub day_occurred: i64,
    pub event_summary: String,
    pub sentiment: f32,
    pub importance: u32,
}

// =========================================================================
// 3. SPATIAL ENTITIES, HOUSEHOLDS & INSTITUTIONS
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdEntity {
    pub id: String,
    pub name: String,
    pub residence_place_id: String,
    pub members: Vec<String>,
    pub household_savings: f64,
    pub monthly_rent_or_mortgage: f64,
    pub monthly_utility_bills: f64,
}

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
    Room,
    Building,
    Residence,
    SchoolCampus,
    SportsPitch,
    CommercialDistrict,
    ClinicHospital,
    Neighborhood,
    City,
    Country,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionEntity {
    pub id: String,
    pub name: String,
    pub institution_type: InstitutionType,
    pub location_id: String,
    pub prestige: f32,
    pub admission_requirements: Vec<AdmissionRequirement>,
    pub active_members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstitutionType {
    PrimarySchool,
    JuniorSecondarySchool,
    SeniorSecondarySchool,
    SixthFormCollege,
    HighSchool,
    University,
    SportsClub,
    YouthAcademy,
    Hospital,
    GovernmentRegistry,
    CorporateOffice,
    FinancialBank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdmissionRequirement {
    MinimumAge(u32),
    MaximumAge(u32),
    AcademicPerformance(f32),
    AthleticSkill(f32),
    FinancialTuition(f64),
    DocumentRequired(String),
}

// =========================================================================
// 4. FINANCIAL ACCOUNTS & LEDGERS
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialAccount {
    pub account_id: String,
    pub owner_id: String,
    pub institution_id: String,
    pub account_type: String, // "PERSONAL_CHECKING", "HIGH_YIELD_SAVINGS", "CORPORATE_TREASURY"
    pub balance: f64,
    pub currency_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: String,
    pub day: i64,
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub amount: f64,
    pub currency: String,
    pub description: String,
}

// =========================================================================
// 5. CAUSAL PROCESSES & EVENT LEDGER
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
    PrimarySchooling,
    JuniorSecondaryEducation,
    SeniorSecondaryEducation,
    SecondaryExamPreparation, // WAEC / JAMB / GCSE / SAT
    UniversityAdmission,
    UniversityDegree,
    YouthSportsAcademyTrial,
    CompanyIncorporation,
    JobApplication,
    ApprenticeshipTrade,
    MedicalTreatment,
    VisaApplication,
    ApartmentLease,
    AcademicRecoveryPlan,
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
pub struct LetterNotification {
    pub id: String,
    pub sender_name: String,
    pub date_received: String,
    pub subject: String,
    pub body_text: String,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: String,
    pub timestamp: String,
    pub day_total: i64,
    pub event_type: String,
    pub actor_id: String,
    pub location_id: String,
    pub headline: String,
    pub narrative: String,
    pub causality_note: String,
    pub success: bool,
}

// =========================================================================
// 6. REGIONAL RULE PACKS & CLIMATE ENGINE
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalRulePack {
    pub city_id: String,
    pub city_name: String,
    pub region_name: String,
    pub country_id: String,
    pub country_name: String,
    pub currency_symbol: String,
    pub currency_code: String,
    pub climate_type: ClimateType,
    pub primary_language: String,
    pub school_system: SchoolSystemType,
    pub starting_costs: HouseholdEconomyProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClimateType {
    TropicalSavanna,     // Lagos, Abuja, Kano: Wet season (Apr-Oct), Dry/Harmattan (Nov-Mar)
    OceanicMaritime,     // Edinburgh, London: Cool summers, chilly damp winters, frequent rain
    MediterraneanMarine, // San Francisco: Dry mild summers, coastal fog/marine layer, wet winters
    HumidSubtropical,    // Houston: Hot humid summers, mild winters, thunderstorm storms
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SchoolSystemType {
    Nigerian6_3_3_4, // Primary (1-6) -> JSS (1-3, BECE) -> SSS (1-3, WAEC/JAMB) -> Uni (4 yrs)
    BritishStandard, // Primary (P1-P7) -> Secondary (S1-S6, GCSE/Highers) -> Uni (3-4 yrs)
    AmericanK12,     // Elementary (K-5) -> Middle (6-8) -> High (9-12, SAT) -> College (4 yrs)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdEconomyProfile {
    pub base_monthly_rent: f64,
    pub base_groceries_cost: f64,
    pub average_working_salary: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalWeather {
    pub name: String,
    pub temperature_c: f32,
    pub description: String,
    pub is_precipitation: bool,
}

impl SeasonalWeather {
    pub fn for_region_and_month(climate: &ClimateType, month: u32) -> Self {
        match climate {
            ClimateType::TropicalSavanna => {
                match month {
                    12 | 1 | 2 => SeasonalWeather {
                        name: "Harmattan Dust Haze".to_string(),
                        temperature_c: 29.0,
                        description: "Dry, hazy northeast trade winds blow down from the Sahara, filling the afternoon air with golden dust.".to_string(),
                        is_precipitation: false,
                    },
                    3..=5 => SeasonalWeather {
                        name: "Pre-Monsoon Tropical Heat".to_string(),
                        temperature_c: 34.0,
                        description: "Intense tropical sunshine warms the streets before the afternoon humidity breaks.".to_string(),
                        is_precipitation: false,
                    },
                    6..=9 => SeasonalWeather {
                        name: "Heavy Monsoon Rain".to_string(),
                        temperature_c: 27.0,
                        description: "Cool, relentless tropical rain drumming on tin rooftops and pooling on city avenues.".to_string(),
                        is_precipitation: true,
                    },
                    _ => SeasonalWeather {
                        name: "Warm Autumn Calm".to_string(),
                        temperature_c: 30.0,
                        description: "Humid tropical air settles over the evening neighborhood with calm breezes.".to_string(),
                        is_precipitation: false,
                    },
                }
            }
            ClimateType::OceanicMaritime => {
                match month {
                    12 | 1 | 2 => SeasonalWeather {
                        name: "Chilly Winter Drizzle".to_string(),
                        temperature_c: 4.0,
                        description: "Cold north winds and damp coastal mist hang over stone buildings and cobbled streets.".to_string(),
                        is_precipitation: true,
                    },
                    3..=5 => SeasonalWeather {
                        name: "Crisp Spring Sun".to_string(),
                        temperature_c: 12.0,
                        description: "Cool, bright morning sun filtering through light clouds with refreshing breezes.".to_string(),
                        is_precipitation: false,
                    },
                    6..=8 => SeasonalWeather {
                        name: "Mild Summer Overcast".to_string(),
                        temperature_c: 19.0,
                        description: "Long northern daylight with comfortable temperatures and occasional light showers.".to_string(),
                        is_precipitation: false,
                    },
                    _ => SeasonalWeather {
                        name: "Autumn Gale & Rain".to_string(),
                        temperature_c: 10.0,
                        description: "Brisk autumn gusts scatter amber leaves across the pavements.".to_string(),
                        is_precipitation: true,
                    },
                }
            }
            ClimateType::MediterraneanMarine => {
                match month {
                    6..=8 => SeasonalWeather {
                        name: "Pacific Marine Fog".to_string(),
                        temperature_c: 16.0,
                        description: "Thick coastal marine fog rolls through the hills in the morning, burning off to cool afternoon sun.".to_string(),
                        is_precipitation: false,
                    },
                    12 | 1 | 2 => SeasonalWeather {
                        name: "Winter Pacific Rain".to_string(),
                        temperature_c: 11.0,
                        description: "Crisp winter rainfall washing over the urban hills and streets.".to_string(),
                        is_precipitation: true,
                    },
                    _ => SeasonalWeather {
                        name: "Sunny Coastal Breeze".to_string(),
                        temperature_c: 20.0,
                        description: "Clear blue skies with crisp oceanic wind blowing off the bay.".to_string(),
                        is_precipitation: false,
                    },
                }
            }
            ClimateType::HumidSubtropical => {
                match month {
                    6..=8 => SeasonalWeather {
                        name: "Gulf Coast Summer Heat".to_string(),
                        temperature_c: 35.0,
                        description: "Shimmering southern heat and high humidity, broken by sudden afternoon thunderheads.".to_string(),
                        is_precipitation: false,
                    },
                    12 | 1 | 2 => SeasonalWeather {
                        name: "Mild Winter Breeze".to_string(),
                        temperature_c: 15.0,
                        description: "Cool, comfortable winter afternoon under wide southern skies.".to_string(),
                        is_precipitation: false,
                    },
                    _ => SeasonalWeather {
                        name: "Spring Thunderstorms".to_string(),
                        temperature_c: 24.0,
                        description: "Warm spring breezes carrying dark rain clouds across the horizon.".to_string(),
                        is_precipitation: true,
                    },
                }
            }
        }
    }
}

// =========================================================================
// 7. TIME & SIMULATION TICK STATE
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeState {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
    pub total_days: i64,
}

pub type SimTime = TimeState;

impl TimeState {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            day: day.clamp(1, 31),
            hour: 8,
            minute: 0,
            total_days: (year as i64 * 365) + (month as i64 * 30) + day as i64,
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
    }

    pub fn advance_hours(&mut self, hours: u8) {
        self.hour += hours;
        if self.hour >= 24 {
            let days = (self.hour / 24) as u32;
            self.hour %= 24;
            self.advance_days(days);
        }
    }

    pub fn literary_date(&self) -> String {
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        let month_name = months.get((self.month.saturating_sub(1)) as usize).unwrap_or(&"January");
        let days_of_week = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let weekday = days_of_week.get((self.total_days % 7) as usize).unwrap_or(&"Monday");
        format!("{}, {} {} {}", weekday, self.day, month_name, self.year)
    }
}

// =========================================================================
// 8. CONFIGURATION & DTOS FOR FRONTEND
// =========================================================================

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
            creation_mode: "ORGANIC_BIRTH".to_string(),
            starting_year: 2005,
            country_id: "country:real:nigeria".to_string(),
            location_id: "city:real:lagos".to_string(),
            starting_age: 0,
            birth_year: Some(2005),
            birth_month: Some(6),
            birth_day: Some(14),
            first_name: Some("Israel".to_string()),
            last_name: Some("Adeyemi".to_string()),
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
    pub weather_name: String,
    pub weather_description: String,
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
pub struct SituationDTO {
    pub current_room_or_place: String,
    pub atmosphere_description: String,
    pub present_people: Vec<String>,
    pub available_objects: Vec<String>,
    pub immediate_pressures: Vec<String>,
    pub suggested_intentions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResolutionDTO {
    pub success: bool,
    pub days_advanced: u32,
    pub hours_advanced: u8,
    pub headline: String,
    pub narrative: String,
    pub causality_note: String,
    pub milestone_achieved: Option<String>,
    pub world_consequences: Vec<String>,
    pub financial_delta: f64,
}

pub type LivingStepResultDTO = StepResolutionDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodaySceneDTO {
    pub headline: String,
    pub narrative: String,
    pub weather_name: String,
    pub weather_description: String,
    pub location_name: String,
    pub present_people: Vec<String>,
    pub environmental_objects: Vec<String>,
    pub subtle_details: Vec<String>,
    pub immediate_pressures: Vec<String>,
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
    pub current_step: u32,
    pub total_steps: u32,
    pub progress_percent: u32,
    pub status: String,
}

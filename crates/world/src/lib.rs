use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =========================================================================
// 1. CANONICAL SIMULATION PRIMITIVES & IDENTITY
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifeStage {
    Infancy,    // Age 0-3
    Childhood,  // Age 4-12
    Adolescence,// Age 13-17
    YoungAdult, // Age 18-29
    Adulthood,  // Age 30-59
    Elderly,    // Age 60+
}

impl LifeStage {
    pub fn from_age(age: u32) -> Self {
        match age {
            0..=3 => LifeStage::Infancy,
            4..=12 => LifeStage::Childhood,
            13..=17 => LifeStage::Adolescence,
            18..=29 => LifeStage::YoungAdult,
            30..=59 => LifeStage::Adulthood,
            _ => LifeStage::Elderly,
        }
    }

    pub fn can_work_full_time(&self) -> bool {
        matches!(self, LifeStage::YoungAdult | LifeStage::Adulthood | LifeStage::Elderly)
    }

    pub fn can_transact_independent_credit(&self) -> bool {
        matches!(self, LifeStage::YoungAdult | LifeStage::Adulthood | LifeStage::Elderly)
    }
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
            age = age.saturating_sub(1);
        }
        age
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalProfile {
    pub is_alive: bool,
    pub death_year: Option<i32>,
    pub death_reason: Option<String>,
    pub health_overall: f32, // 0-100
    pub fitness: f32,        // 0-100
    pub energy_level: f32,   // 0-100
    pub chronic_conditions: Vec<String>,
}

impl Default for BiologicalProfile {
    fn default() -> Self {
        Self {
            is_alive: true,
            death_year: None,
            death_reason: None,
            health_overall: 95.0,
            fitness: 50.0,
            energy_level: 90.0,
            chronic_conditions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalProfile {
    pub discipline: f32,
    pub curiosity: f32,
    pub creativity: f32,
    pub confidence: f32,
    pub risk_tolerance: f32,
    pub stress_level: f32, // 0-100
    pub resilience: f32,
}

impl Default for PsychologicalProfile {
    fn default() -> Self {
        Self {
            discipline: 0.5,
            curiosity: 0.7,
            creativity: 0.6,
            confidence: 0.5,
            risk_tolerance: 0.4,
            stress_level: 10.0,
            resilience: 0.6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationProfile {
    pub integrity: f32,
    pub reliability: f32,
    pub community_respect: f32,
    pub professional_standing: f32,
    pub academic_reputation: f32,
    pub athletic_reputation: f32,
}

impl Default for ReputationProfile {
    fn default() -> Self {
        Self {
            integrity: 0.8,
            reliability: 0.75,
            community_respect: 0.5,
            professional_standing: 0.1,
            academic_reputation: 0.5,
            athletic_reputation: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMastery {
    pub level: f32, // 0.0 - 100.0
    pub experience: f64,
    pub natural_affinity: f32,
    pub last_practiced_day: i64,
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
            "POVERTY" | "LOW" => WealthTier::Poverty,
            "WORKING_CLASS" | "WORKING" => WealthTier::WorkingClass,
            "UPPER_MIDDLE" => WealthTier::UpperMiddle,
            "WEALTHY" | "RICH" => WealthTier::Wealthy,
            _ => WealthTier::MiddleClass,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanResources {
    pub cash: f64,
    pub household_wealth_tier: WealthTier,
    pub living_arrangement: String,
    pub tools_available: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntity {
    pub id: String,
    pub identity: IdentityProfile,
    pub biology: BiologicalProfile,
    pub psychology: PsychologicalProfile,
    pub reputation: ReputationProfile,
    pub skills: HashMap<String, SkillMastery>,
    pub resources: HumanResources,
    pub relationships: HashMap<String, RelationshipEdge>,
    pub occupation: Option<String>,
    pub is_player: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEdge {
    pub target_entity_id: String,
    pub target_name: String,
    pub relationship_type: String, // Mother, Father, Friend, Teacher, Rival, Spouse
    pub affinity: f32,             // -1.0 to 1.0
    pub trust: f32,                // 0.0 to 1.0
    pub respect: f32,              // 0.0 to 1.0
    pub memories: Vec<EpisodicMemoryRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemoryRecord {
    pub day_occurred: i64,
    pub headline: String,
    pub description: String,
    pub emotional_valence: f32, // -1.0 to 1.0
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcPersonality {
    pub communication_style: CommunicationStyle,
    pub strictness: f32,
}

impl Default for NpcPersonality {
    fn default() -> Self {
        Self {
            communication_style: CommunicationStyle::NurturingWarm,
            strictness: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousNPC {
    pub base: HumanEntity,
    pub daily_routine: Vec<ScheduledActivity>,
    pub communication_style: CommunicationStyle,
    pub personality: NpcPersonality,
    pub current_goal: String,
    pub last_active_day: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledActivity {
    pub start_hour: u8,
    pub end_hour: u8,
    pub location_id: String,
    pub activity_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunicationStyle {
    NurturingWarm,
    SternDisciplinarian,
    ScholarlyAnalytical,
    EncouragingSupportive,
    PragmaticDirect,
    Nurturing,
    Disciplinarian,
    Inspirational,
    Direct,
    Supportive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdEntity {
    pub id: String,
    pub address: String,
    pub city_id: String,
    pub monthly_rent: f64,
    pub members: Vec<String>,
    pub pooled_cash: f64,
}

// =========================================================================
// 2. WORLD PLACES & SPATIAL HIERARCHY
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceType {
    Residence,
    Education,
    Workplace,
    AthleticField,
    CivicCenter,
    MedicalClinic,
    CommercialVenue,
    Airport,
    TrainStation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPlace {
    pub id: String,
    pub name: String,
    pub place_type: PlaceType,
    pub city_id: String,
    pub district_name: String,
    pub required_min_age: u32,
    pub affords_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionEntity {
    pub id: String,
    pub name: String,
    pub category: String, // School, University, SportsClub, CorporateEmployer, Hospital
    pub city_id: String,
    pub reputation: f32,
}

// =========================================================================
// 3. MULTI-STAGE PROCESSES
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessType {
    SchoolEnrollment,
    AcademicExamination,
    SportsAcademyTrial,
    HigherEducationDegree,
    UniversityAdmission,
    CompanyRegistration,
    JobApplication,
    MedicalTreatment,
    TravelJourney,
    ResidencyApplication,
    BusinessOperations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeProcess {
    pub id: String,
    pub process_type: ProcessType,
    pub title: String,
    pub target_institution_id: Option<String>,
    pub current_step: u32,
    pub total_steps: u32,
    pub progress_percent: u32,
    pub status: String,
    pub missing_requirements: Vec<String>,
    pub next_appointment_day: Option<i64>,
}

// =========================================================================
// 4. DOCUMENTS & CREDENTIALS ENGINE
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecord {
    pub id: String,
    pub title: String,
    pub document_type: String, // BIRTH_CERTIFICATE, COMPANY_REGISTRATION, PASSPORT, DEGREE, TICKET
    pub issue_date: String,
    pub issuing_authority: String,
    pub registration_number: String,
    pub fields: HashMap<String, String>,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDTO {
    pub id: String,
    pub title: String,
    pub document_type: String,
    pub issue_date: String,
    pub issuing_authority: String,
    pub registration_number: String,
    pub fields: HashMap<String, String>,
    pub is_verified: bool,
}

// =========================================================================
// 5. PHONE & COMMUNICATIONS
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneMessage {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub recipient_id: String,
    pub text: String,
    pub timestamp: String,
    pub is_read: bool,
    pub is_delivered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneCallState {
    pub id: String,
    pub contact_id: String,
    pub contact_name: String,
    pub status: String, // RINGING, ANSWERED, BUSY, COMPLETED, DECLINED
    pub duration_seconds: u32,
    pub dialogue_history: Vec<String>,
}

// =========================================================================
// 6. STRUCTURED LIVING INTENTIONS
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum TypedLivingIntent {
    FreeText(String),
    AdvanceHours { hours: u32 },
    AdvanceDays { days: u32 },
    SleepUntilMorning,
    FollowRoutine { days: u32 },
    SendMessage { recipient_id: String, text: String },
    PlaceCall { recipient_id: String },
    SendCallDialogue { dialogue: String },
    EndCall,
    TravelToLocation { destination_city_id: String, transport_mode: String },
    RegisterCompany {
        name: String,
        structure: String,
        partners: Vec<String>,
        authorized_capital: f64,
    },
    ApplyForJob {
        job_id: String,
        company_id: String,
        title: String,
    },
    AttendMedicalCheckup,
    OpenDocument { document_id: String },
    TransferFunds {
        recipient_id: String,
        amount: f64,
        reference: String,
    },
}

// =========================================================================
// 7. EVENTS & CAUSALITY RECORD
// =========================================================================

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterNotification {
    pub id: String,
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub is_read: bool,
    pub date_received: String,
}

// =========================================================================
// 8. REGIONAL RULE PACKS & CLIMATE ENGINE
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
    TropicalSavanna,     // Lagos, Abuja, Kano, Ibadan: Wet season (Apr-Oct), Dry/Harmattan (Nov-Mar)
    OceanicMaritime,     // Edinburgh, Glasgow, London, Manchester: Cool summers, chilly damp winters, frequent rain
    MediterraneanMarine, // San Francisco, Madrid: Dry mild summers, coastal fog/marine layer, wet winters
    HumidSubtropical,    // Houston: Hot humid summers, mild winters, thunderstorms
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
            ClimateType::TropicalSavanna => match month {
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
            },
            ClimateType::OceanicMaritime => match month {
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
            },
            ClimateType::MediterraneanMarine => match month {
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
            },
            ClimateType::HumidSubtropical => match month {
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
            },
        }
    }
}

// =========================================================================
// 9. TIME & SIMULATION TICK STATE
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

    pub fn day_of_week(&self) -> u32 {
        let y = if self.month < 3 { self.year - 1 } else { self.year };
        let m = self.month;
        let d = self.day;
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let dow = (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d as i32) % 7;
        let pos_dow = if dow < 0 { dow + 7 } else { dow };
        match pos_dow {
            0 => 7, // Sunday
            other => other as u32,
        }
    }

    pub fn weekday_name(&self) -> String {
        match self.day_of_week() {
            1 => "Monday".to_string(),
            2 => "Tuesday".to_string(),
            3 => "Wednesday".to_string(),
            4 => "Thursday".to_string(),
            5 => "Friday".to_string(),
            6 => "Saturday".to_string(),
            _ => "Sunday".to_string(),
        }
    }

    pub fn formatted_full_date(&self) -> String {
        let month_name = match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            _ => "December",
        };
        format!("{}, {} {}, {}", self.weekday_name(), month_name, self.day, self.year)
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
        
        // Exact calendar weekday (Sakamoto's algorithm)
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let mut y = self.year;
        if self.month < 3 {
            y -= 1;
        }
        let dow_calc = (y + y / 4 - y / 100 + y / 400 + t[(self.month.saturating_sub(1)) as usize] + self.day as i32) % 7;
        let dow_idx = if dow_calc < 0 { (dow_calc + 7) as usize } else { dow_calc as usize };
        let days_of_week = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let weekday = days_of_week.get(dow_idx).unwrap_or(&"Monday");

        format!("{}, {} {} {} {:02}:{:02}", weekday, self.day, month_name, self.year, self.hour, self.minute)
    }
}

// =========================================================================
// 10. CONFIGURATION & DTOS FOR FRONTEND
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
    pub current_place_id: String,
    pub current_place_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMapPlaceDTO {
    pub id: String,
    pub name: String,
    pub category: String,
    pub district_name: String,
    pub description: String,
    pub map_x: f32,
    pub map_y: f32,
    pub travel_minutes: u32,
    pub travel_cost: f64,
    pub is_current: bool,
    pub is_open: bool,
    pub present_people_count: usize,
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
    pub location_formatted: Option<String>,
    pub life_stage: Option<String>,
    pub age: Option<u32>,
    pub circumstances: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextNpcDTO {
    pub id: String,
    pub name: String,
    pub relationship_type: String,
    pub trust_description: String,
    pub current_activity: String,
    pub location_id: String,
    pub is_new_acquaintance: bool,
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

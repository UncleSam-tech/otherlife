use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type EntityId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityNamespace {
    Real,
    Sim,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Country,
    City,
    Club,
    Company,
    University,
    Hospital,
    PoliticalParty,
    School,
    Person,
    SecretSociety,
    SpaceAgency,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealWorldSnapshot {
    pub snapshot_date: String,
    pub source_version: String,
    pub canonical_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LegalStatus {
    Clean,
    UnderInvestigation,
    Arrested,
    OnTrial,
    Imprisoned,
    Parole,
}

impl Default for LegalStatus {
    fn default() -> Self {
        LegalStatus::Clean
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalEntity {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub aliases: Vec<String>,
    pub location_id: Option<String>,
    pub parent_org_id: Option<String>,
    pub fame_score: f32,
    pub namespace: EntityNamespace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSubunit {
    pub id: String,
    pub name: String,
    pub subunit_type: String, // "FirstTeam", "Academy", "U18", "Department", "Branch"
    pub parent_id: String,
    pub roles: Vec<RoleAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub role_title: String, // "manager", "head_coach", "doctor", "CEO", "captain", "prime_minister", "teacher"
    pub person_id: String,
    pub organization_id: String,
    pub since_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResolutionContext {
    pub player_id: String,
    pub player_location_id: String,
    pub current_domain: Option<String>,
    pub recent_entities: Vec<String>,
    pub current_year: i32,
    pub player_age: u32,
    pub player_gender: Option<String>,
    pub player_skills: HashMap<String, f32>,
    pub current_employer_id: Option<String>,
    pub current_club_id: Option<String>,
    pub relationships: Vec<(String, String, f32)>, // (person_id, relation_type, closeness)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCandidate {
    pub entity: CanonicalEntity,
    pub score: f32,
    pub match_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionResult {
    Resolved(CanonicalEntity),
    Ambiguous {
        prompt: String,
        candidates: Vec<EntityCandidate>,
    },
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldEntityResolver {
    pub entities: HashMap<String, CanonicalEntity>,
    pub subunits: HashMap<String, OrganizationSubunit>,
    pub role_assignments: Vec<RoleAssignment>,
}

impl WorldEntityResolver {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            subunits: HashMap::new(),
            role_assignments: Vec::new(),
        }
    }

    pub fn register_entity(&mut self, entity: CanonicalEntity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    pub fn register_subunit(&mut self, subunit: OrganizationSubunit) {
        self.subunits.insert(subunit.id.clone(), subunit);
    }

    pub fn assign_role(&mut self, organization_id: &str, role_title: &str, person_id: &str, current_year: i32) {
        self.role_assignments.retain(|r| !(r.organization_id == organization_id && r.role_title == role_title));
        self.role_assignments.push(RoleAssignment {
            role_title: role_title.to_lowercase(),
            person_id: person_id.to_string(),
            organization_id: organization_id.to_string(),
            since_year: current_year,
        });
    }

    pub fn get_entity(&self, id: &str) -> Option<&CanonicalEntity> {
        self.entities.get(id)
    }

    pub fn resolve_alias(&self, alias_query: &str) -> Option<&CanonicalEntity> {
        let q_lower = alias_query.to_lowercase();
        self.entities.values().find(|e| {
            e.name.to_lowercase() == q_lower || e.aliases.iter().any(|a| a.to_lowercase() == q_lower)
        })
    }

    pub fn resolve_role(&self, organization_id: &str, role_title: &str, _current_year: i32) -> Option<String> {
        let role_lower = role_title.to_lowercase();
        self.role_assignments
            .iter()
            .find(|r| r.organization_id == organization_id && r.role_title == role_lower)
            .map(|r| r.person_id.clone())
    }

    pub fn find_entities_near(&self, location_id: &str, entity_type: Option<EntityType>, _radius_km: f32) -> Vec<CanonicalEntity> {
        self.entities
            .values()
            .filter(|e| {
                if let Some(ref loc) = e.location_id {
                    if loc != location_id {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(ref et) = entity_type {
                    &e.entity_type == et
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }

    pub fn find_reachable_entities(&self, _player_id: &str, player_location_id: &str, entity_type: Option<EntityType>) -> Vec<CanonicalEntity> {
        self.find_entities_near(player_location_id, entity_type, 50.0)
    }

    pub fn rank_candidates(
        &self,
        candidates: Vec<CanonicalEntity>,
        query: &str,
        entity_type: Option<EntityType>,
        location_id: Option<&str>,
        context: Option<&ResolutionContext>,
    ) -> Vec<EntityCandidate> {
        let q_lower = query.to_lowercase();
        let mut scored = Vec::new();

        for entity in candidates {
            let mut score = 0.0f32;
            let mut match_reasons = Vec::new();
            let mut text_matched = false;

            let e_name_lower = entity.name.to_lowercase();

            // 1. Exact canonical name match
            if e_name_lower == q_lower {
                score += 100.0;
                text_matched = true;
                match_reasons.push("Exact canonical name match".to_string());
            } else if e_name_lower.contains(&q_lower) {
                score += 50.0;
                text_matched = true;
                match_reasons.push("Partial name match".to_string());
            }

            // 2. Alias match
            for alias in &entity.aliases {
                let a_lower = alias.to_lowercase();
                if a_lower == q_lower {
                    score += 90.0;
                    text_matched = true;
                    match_reasons.push(format!("Exact alias match ('{}')", alias));
                } else if a_lower.contains(&q_lower) || q_lower.contains(&a_lower) {
                    score += 40.0;
                    text_matched = true;
                    match_reasons.push(format!("Partial alias match ('{}')", alias));
                }
            }

            if !text_matched {
                continue;
            }

            // 3. Entity type filtering
            if let Some(ref et) = entity_type {
                if &entity.entity_type == et {
                    score += 30.0;
                    match_reasons.push("Entity type matched query".to_string());
                } else {
                    score -= 50.0;
                }
            }

            // 4. Location match
            if let Some(loc) = location_id {
                if let Some(ref e_loc) = entity.location_id {
                    if e_loc == loc || e_loc.contains(loc) || loc.contains(e_loc) {
                        score += 35.0;
                        match_reasons.push(format!("Located in query location '{}'", loc));
                    }
                }
            }

            // 5. Context / Recent conversation match
            if let Some(ctx) = context {
                if ctx.recent_entities.contains(&entity.id) || ctx.recent_entities.contains(&entity.name) {
                    score += 60.0;
                    match_reasons.push("Recent conversation context match".to_string());
                }

                if let Some(ref emp_id) = ctx.current_employer_id {
                    if &entity.id == emp_id || entity.parent_org_id.as_ref() == Some(emp_id) {
                        score += 50.0;
                        match_reasons.push("Player current employer match".to_string());
                    }
                }

                if let Some(ref club_id) = ctx.current_club_id {
                    if &entity.id == club_id {
                        score += 50.0;
                        match_reasons.push("Player current club match".to_string());
                    }
                }

                if ctx.player_location_id == entity.location_id.clone().unwrap_or_default() {
                    score += 20.0;
                    match_reasons.push("Player current location match".to_string());
                }
            }

            if score > 0.0 {
                scored.push(EntityCandidate {
                    entity,
                    score,
                    match_reasons,
                });
            }
        }

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }

    pub fn search_entities(
        &self,
        query: &str,
        entity_type: Option<EntityType>,
        location_id: Option<&str>,
        context: Option<&ResolutionContext>,
    ) -> ResolutionResult {
        let all_entities: Vec<CanonicalEntity> = self.entities.values().cloned().collect();
        let ranked = self.rank_candidates(all_entities, query, entity_type, location_id, context);

        if ranked.is_empty() {
            return ResolutionResult::NotFound;
        }

        if ranked.len() == 1 {
            if ranked[0].score >= 40.0 {
                return ResolutionResult::Resolved(ranked[0].entity.clone());
            } else {
                return ResolutionResult::NotFound;
            }
        }

        let top = &ranked[0];
        let second = &ranked[1];

        if top.score >= 80.0 && (top.score - second.score) >= 25.0 {
            ResolutionResult::Resolved(top.entity.clone())
        } else {
            let names: Vec<String> = ranked.iter().take(3).map(|c| c.entity.name.clone()).collect();
            let prompt = format!("Which {} do you mean: {} or another organization?", query, names.join(", "));
            ResolutionResult::Ambiguous {
                prompt,
                candidates: ranked,
            }
        }
    }

    pub fn search_people(&self, query: &str, location_id: Option<&str>, context: Option<&ResolutionContext>) -> ResolutionResult {
        self.search_entities(query, Some(EntityType::Person), location_id, context)
    }

    pub fn search_places(&self, query: &str, context: Option<&ResolutionContext>) -> ResolutionResult {
        self.search_entities(query, Some(EntityType::City), None, context)
    }

    pub fn search_organizations(&self, query: &str, location_id: Option<&str>, context: Option<&ResolutionContext>) -> ResolutionResult {
        self.search_entities(query, None, location_id, context)
    }

    pub fn search_clubs(&self, query: &str, location_id: Option<&str>, context: Option<&ResolutionContext>) -> ResolutionResult {
        self.search_entities(query, Some(EntityType::Club), location_id, context)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicMegastructure {
    pub id: String,
    pub name: String,
    pub structure_type: String, // "DYSON_SWARM", "ORBITAL_RING", "STELLAR_ENGINE"
    pub completion_pct: f32,
    pub energy_output_gw: f64,
    pub built_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostScarcityEconomy {
    pub universal_basic_dividend: f64,
    pub automation_index: f32,
    pub resource_abundance_score: f32,
}

impl Default for PostScarcityEconomy {
    fn default() -> Self {
        Self {
            universal_basic_dividend: 4500.0,
            automation_index: 94.5,
            resource_abundance_score: 98.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicLegacy {
    pub dynasty_generation_count: u32,
    pub interstellar_colonies_count: u32,
    pub civilization_kardashev_tier: String, // "TYPE_I", "TYPE_II", "TYPE_III"
}

impl Default for CosmicLegacy {
    fn default() -> Self {
        Self {
            dynasty_generation_count: 1,
            interstellar_colonies_count: 0,
            civilization_kardashev_tier: "TYPE_I".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberneticImplant {
    pub id: String,
    pub name: String,
    pub implant_type: String, // "NEURAL_LINK", "BIONIC_LIMB", "OCULAR_HUD", "ORGAN_SYNTH"
    pub augmentation_level: f32,
    pub installation_year: i32,
    pub maintenance_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindUpload {
    pub id: String,
    pub digital_avatar_name: String,
    pub upload_fidelity: f32,
    pub substrate: String, // "CLOUD_SERVER", "QUANTUM_CORE", "SYNTHETIC_BODY"
    pub year_uploaded: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceAgency {
    pub id: String,
    pub name: String,
    pub agency_type: String, // "NATIONAL", "PRIVATE_AEROSPACE"
    pub reputation: f32,
    pub budget: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMission {
    pub id: String,
    pub name: String,
    pub mission_type: String, // "ORBITAL_SATELLITE", "LUNAR_LANDER", "MARS_ROVER", "DEEP_SPACE_PROBE"
    pub destination: String,
    pub launch_year: i32,
    pub budget: f64,
    pub success_rate: f32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretSociety {
    pub id: String,
    pub name: String,
    pub society_type: String,
    pub founder_id: EntityId,
    pub secrecy_level: f32,
    pub member_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMembership {
    pub society_id: String,
    pub society_name: String,
    pub rank: String,
    pub covert_reputation: f32,
    pub secret_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovertOperation {
    pub id: String,
    pub society_id: String,
    pub operation_name: String,
    pub target_entity_id: EntityId,
    pub success_rate: f32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEvent {
    pub id: String,
    pub season: String,
    pub condition: String,
    pub temperature_celsius: f32,
    pub air_quality_index: u32,
    pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalDisaster {
    pub id: String,
    pub disaster_type: String,
    pub severity: f32,
    pub city_id: EntityId,
    pub damage_cost: f64,
    pub is_active: bool,
    pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRating {
    pub air_quality: f32,
    pub green_space_pct: f32,
    pub carbon_footprint: f32,
    pub sustainability_score: f32,
}

impl Default for EnvironmentalRating {
    fn default() -> Self {
        Self {
            air_quality: 78.0,
            green_space_pct: 35.0,
            carbon_footprint: 4.2,
            sustainability_score: 82.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalRecord {
    pub id: String,
    pub crime_type: String,
    pub severity: f32,
    pub stolen_value: f64,
    pub is_unsolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrisonSentence {
    pub crime_type: String,
    pub months_total: u32,
    pub months_served: u32,
    pub facility_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMediaAccount {
    pub id: String,
    pub platform: String,
    pub handle: String,
    pub follower_count: u64,
    pub subscriber_count: u64,
    pub engagement_rate: f32,
    pub influencer_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalPost {
    pub id: String,
    pub platform: String,
    pub caption: String,
    pub likes: u64,
    pub shares: u64,
    pub impressions: u64,
    pub is_viral: bool,
    pub posted_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalRecord {
    pub id: String,
    pub condition_name: String,
    pub severity: String,
    pub diagnosed_year: i32,
    pub is_chronic: bool,
    pub is_cured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurgicalProcedure {
    pub id: String,
    pub procedure_name: String,
    pub hospital_name: String,
    pub success_rate: f32,
    pub cost: f64,
    pub performed_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WillAndTestament {
    pub id: String,
    pub beneficiary_ids: Vec<EntityId>,
    pub estate_distribution_summary: String,
    pub executor_person_id: EntityId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryRecord {
    pub id: String,
    pub branch: String,
    pub rank: String,
    pub years_served: u32,
    pub combat_deployments_count: u32,
    pub medals: Vec<String>,
    pub is_active_duty: bool,
    pub is_veteran: bool,
    pub monthly_pension: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeopoliticalConflict {
    pub id: String,
    pub name: String,
    pub aggressor_country_id: EntityId,
    pub defender_country_id: EntityId,
    pub intensity_level: f32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub id: String,
    pub country_id: EntityId,
    pub issued_year: i32,
    pub expiry_year: i32,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visa {
    pub id: String,
    pub target_country_id: EntityId,
    pub visa_type: String,
    pub expiry_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelRecord {
    pub id: String,
    pub destination_city_id: EntityId,
    pub destination_country_id: EntityId,
    pub travel_date: String,
    pub purpose: String,
    pub cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefComponent {
    pub faith_id: String,
    pub faith_name: String,
    pub devotion_level: f32,
    pub tithes_donated: f64,
    pub spiritual_rank: String,
}

impl Default for BeliefComponent {
    fn default() -> Self {
        Self {
            faith_id: "SECULAR".to_string(),
            faith_name: "Secular Humanism".to_string(),
            devotion_level: 10.0,
            tithes_donated: 0.0,
            spiritual_rank: "LAITY".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaithMovement {
    pub id: String,
    pub name: String,
    pub founder_person_id: EntityId,
    pub doctrine_summary: String,
    pub congregation_size: u64,
    pub treasury: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicDegree {
    pub degree_type: String,
    pub field_of_study: String,
    pub university_name: String,
    pub graduation_year: i32,
    pub gpa: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    pub id: String,
    pub title: String,
    pub field_of_study: String,
    pub funding_grant: f64,
    pub progress_pct: f32,
    pub lead_researcher_id: EntityId,
    pub status: String,
    pub citation_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    pub id: String,
    pub title: String,
    pub field: String,
    pub inventor_person_id: EntityId,
    pub filed_year: i32,
    pub estimated_valuation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroEconomy {
    pub inflation_rate: f32,
    pub interest_rate: f32,
    pub economic_cycle: String,
}

impl Default for MacroEconomy {
    fn default() -> Self {
        Self {
            inflation_rate: 0.025,
            interest_rate: 0.045,
            economic_cycle: "GROWTH".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessEntity {
    pub id: EntityId,
    pub name: String,
    pub industry: String,
    pub owner_person_id: EntityId,
    pub valuation: f64,
    pub monthly_revenue: f64,
    pub monthly_expenses: f64,
    pub cash_reserve: f64,
    pub debt: f64,
    pub employee_count: u32,
    pub equity_owned_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeRelease {
    pub id: String,
    pub title: String,
    pub medium: String,
    pub creator_person_id: EntityId,
    pub quality_rating: f32,
    pub sales_volume: u64,
    pub chart_position: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FameComponent {
    pub fame_level: f32,
    pub public_reputation: f32,
    pub fanbase_count: u64,
}

impl Default for FameComponent {
    fn default() -> Self {
        Self {
            fame_level: 0.0,
            public_reputation: 50.0,
            fanbase_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballLeague {
    pub id: EntityId,
    pub name: String,
    pub country_id: EntityId,
    pub tier: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountrySeed {
    pub id: String,
    pub name: String,
    pub code: String,
    pub currency: String,
    pub currency_symbol: String,
    pub cost_of_living_index: f32,
    pub primary_language: String,
    pub capital_city_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitySeed {
    pub id: String,
    pub name: String,
    pub country_id: String,
    pub region_id: String,
    pub population: u64,
    pub cost_of_living_index: f32,
    pub districts: Vec<String>,
}

pub struct WorldDataValidator;

impl WorldDataValidator {
    pub fn validate_seed_data(
        countries: &[CountrySeed],
        cities: &[CitySeed],
        clubs: &[FootballClub],
        companies: &[Company],
        universities: &[University],
        parties: &[PoliticalParty],
    ) -> Result<(), String> {
        let mut country_ids = HashSet::new();
        for c in countries {
            if !country_ids.insert(c.id.clone()) {
                return Err(format!("Duplicate country ID: {}", c.id));
            }
        }

        let mut city_ids = HashSet::new();
        for c in cities {
            if !city_ids.insert(c.id.clone()) {
                return Err(format!("Duplicate city ID: {}", c.id));
            }
            if !country_ids.contains(&c.country_id) {
                return Err(format!("City {} references invalid country_id {}", c.id, c.country_id));
            }
        }

        let mut club_ids = HashSet::new();
        for club in clubs {
            if !club_ids.insert(club.id.clone()) {
                return Err(format!("Duplicate club ID: {}", club.id));
            }
            if !city_ids.contains(&club.city_id) {
                return Err(format!("Club {} references invalid city_id {}", club.id, club.city_id));
            }

            if club.id.contains("manchester_united") && club.city_id != "city:real:manchester" {
                return Err(format!("Club {} has incorrect city_id {} (expected city:real:manchester)", club.id, club.city_id));
            }
        }

        for comp in companies {
            if !country_ids.contains(&comp.country_id) {
                return Err(format!("Company {} references invalid country_id {}", comp.id, comp.country_id));
            }
        }

        for uni in universities {
            if !city_ids.contains(&uni.city_id) {
                return Err(format!("University {} references invalid city_id {}", uni.id, uni.city_id));
            }
        }

        for p in parties {
            if !country_ids.contains(&p.country_id) {
                return Err(format!("Political party {} references invalid country_id {}", p.id, p.country_id));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballClub {
    pub id: EntityId,
    pub name: String,
    pub city_id: EntityId,
    pub league_id: EntityId,
    pub stadium_name: String,
    pub reputation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FootballRole {
    Player,
    AcademyProspect,
    Coach,
    Manager,
    Scout,
    Agent,
    Pundit,
    Executive,
    Owner,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballPlayerAttributes {
    pub ball_control: f32,
    pub passing: f32,
    pub shooting: f32,
    pub tackling: f32,
    pub pace: f32,
    pub stamina: f32,
    pub vision: f32,
    pub composure: f32,
}

impl Default for FootballPlayerAttributes {
    fn default() -> Self {
        Self {
            ball_control: 50.0,
            passing: 50.0,
            shooting: 50.0,
            tackling: 50.0,
            pace: 60.0,
            stamina: 60.0,
            vision: 50.0,
            composure: 50.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballContract {
    pub club_id: EntityId,
    pub club_name: String,
    pub weekly_wage: f64,
    pub years_remaining: u32,
    pub release_clause: f64,
    pub goal_bonus: f64,
    pub agent_id: Option<EntityId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballMatch {
    pub id: String,
    pub match_date: String,
    pub home_club_id: EntityId,
    pub home_club_name: String,
    pub away_club_id: EntityId,
    pub away_club_name: String,
    pub home_score: u32,
    pub away_score: u32,
    pub player_rating: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballScoutReport {
    pub id: String,
    pub target_player_id: EntityId,
    pub scout_id: EntityId,
    pub current_ability: f32,
    pub potential_rating: f32,
    pub recommended_transfer_fee: f64,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub id: EntityId,
    pub name: String,
    pub country_id: EntityId,
    pub ideology: String,
    pub influence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalOffice {
    pub id: EntityId,
    pub title: String,
    pub country_id: EntityId,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalCampaign {
    pub id: String,
    pub office_id: EntityId,
    pub office_title: String,
    pub candidate_person_id: EntityId,
    pub party_id: Option<EntityId>,
    pub campaign_funds: f64,
    pub polling_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub sponsor_person_id: EntityId,
    pub votes_for: u32,
    pub votes_against: u32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct University {
    pub id: EntityId,
    pub name: String,
    pub country_id: EntityId,
    pub city_id: EntityId,
    pub prestige: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: EntityId,
    pub name: String,
    pub industry: String,
    pub country_id: EntityId,
    pub market_cap_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifeStage {
    Infancy,
    Childhood,
    Adolescence,
    YoungAdulthood,
    Adulthood,
    MiddleAge,
    OldAge,
    Deceased,
}

impl LifeStage {
    pub fn from_age(age: u32, is_alive: bool) -> Self {
        if !is_alive {
            return LifeStage::Deceased;
        }
        match age {
            0..=4 => LifeStage::Infancy,
            5..=12 => LifeStage::Childhood,
            13..=17 => LifeStage::Adolescence,
            18..=25 => LifeStage::YoungAdulthood,
            26..=49 => LifeStage::Adulthood,
            50..=64 => LifeStage::MiddleAge,
            _ => LifeStage::OldAge,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NpcTier {
    TierA,
    TierB,
    TierC,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivityType {
    Home,
    Work,
    School,
    Socializing,
    Resting,
    Traveling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSchedule {
    pub current_activity: ActivityType,
    pub work_start_hour: u32,
    pub work_end_hour: u32,
    pub primary_location_id: EntityId,
}

impl Default for NpcSchedule {
    fn default() -> Self {
        Self {
            current_activity: ActivityType::Home,
            work_start_hour: 9,
            work_end_hour: 17,
            primary_location_id: "city:real:glasgow".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRecord {
    pub topic_id: String,
    pub description: String,
    pub certainty: f32,
    pub is_secret: bool,
    pub known_by_ids: HashSet<EntityId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNewsItem {
    pub id: String,
    pub timestamp: String,
    pub headline: String,
    pub body: String,
    pub category: String,
    pub source_event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    CanWalk,
    CanSpeak,
    CanTravelAlone,
    CanAttendSchool,
    CanWork,
    CanSignContract,
    CanOpenBankAccount,
    CanDrive,
    CanMarry,
    CanVote,
    CanRunForOffice,
    CanEnlist,
    CanOwnBusiness,
    CanPlayProFootball,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

impl SimTime {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            day: day.clamp(1, 31),
            hour: hour.clamp(0, 23),
            minute: minute.clamp(0, 59),
        }
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 30,
        }
    }

    pub fn compute_age(&self, birth_year: i32, birth_month: u32, birth_day: u32) -> u32 {
        let mut age = self.year - birth_year;
        if self.month < birth_month || (self.month == birth_month && self.day < birth_day) {
            age -= 1;
        }
        if age < 0 { 0 } else { age as u32 }
    }

    pub fn formatted(&self) -> String {
        let months = [
            "JAN", "FEB", "MAR", "APR", "MAY", "JUN",
            "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"
        ];
        let m_idx = (self.month.saturating_sub(1) as usize).min(11);
        format!("{:02} {} {:04} · {:02}:{:02}", self.day, months[m_idx], self.year, self.hour, self.minute)
    }

    pub fn advance_days(&mut self, days: u32) {
        self.day += days;
        loop {
            let max_days = Self::days_in_month(self.year, self.month);
            if self.day <= max_days {
                break;
            }
            self.day -= max_days;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
    }

    pub fn advance_hours(&mut self, hours: u32) {
        self.hour += hours;
        if self.hour >= 24 {
            let days = self.hour / 24;
            self.hour %= 24;
            self.advance_days(days);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityComponent {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub sex: String,
    pub birth_location_id: String,
    pub current_location_id: String,
    pub nationalities: Vec<String>,
    pub citizenships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityComponent {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
    pub ambition: f32,
    pub discipline: f32,
    pub risk_tolerance: f32,
}

impl Default for PersonalityComponent {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
            ambition: 0.5,
            discipline: 0.5,
            risk_tolerance: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qualification {
    pub title: String,
    pub field: String,
    pub year_obtained: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationComponent {
    pub school_id: Option<EntityId>,
    pub grade_level: u32,
    pub academic_performance: f32,
    pub attendance_rate: f32,
    pub qualifications: Vec<Qualification>,
    pub degree_program: Option<String>,
}

impl Default for EducationComponent {
    fn default() -> Self {
        Self {
            school_id: None,
            grade_level: 0,
            academic_performance: 70.0,
            attendance_rate: 100.0,
            qualifications: Vec::new(),
            degree_program: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentComponent {
    pub job_title: Option<String>,
    pub employer_org_id: Option<String>,
    pub monthly_salary: f64,
    pub job_performance: f32,
    pub years_in_role: u32,
}

impl Default for EmploymentComponent {
    fn default() -> Self {
        Self {
            job_title: None,
            employer_org_id: None,
            monthly_salary: 0.0,
            job_performance: 50.0,
            years_in_role: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HousingComponent {
    pub housing_type: String,
    pub monthly_cost: f64,
    pub quality: f32,
}

impl Default for HousingComponent {
    fn default() -> Self {
        Self {
            housing_type: "FamilyHome".to_string(),
            monthly_cost: 0.0,
            quality: 70.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthComponent {
    pub fitness: f32,
    pub stress: f32,
    pub nutrition: f32,
    pub conditions: Vec<String>,
    pub mortality_risk: f32,
}

impl Default for HealthComponent {
    fn default() -> Self {
        Self {
            fitness: 75.0,
            stress: 20.0,
            nutrition: 70.0,
            conditions: Vec::new(),
            mortality_risk: 0.001,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomanceComponent {
    pub marital_status: String,
    pub partner_id: Option<EntityId>,
    pub relationship_satisfaction: f32,
}

impl Default for RomanceComponent {
    fn default() -> Self {
        Self {
            marital_status: "Single".to_string(),
            partner_id: None,
            relationship_satisfaction: 50.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancesComponent {
    pub cash: f64,
    pub monthly_allowance: f64,
    pub household_income_tier: String,
    pub monthly_expenses: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: EntityId,
    pub is_player: bool,
    pub is_alive: bool,
    pub tier: NpcTier,
    pub schedule: NpcSchedule,
    pub identity: IdentityComponent,
    pub personality: PersonalityComponent,
    pub skills: HashMap<String, f32>,
    pub interests: HashSet<String>,
    pub goals: Vec<String>,
    pub education: EducationComponent,
    pub employment: EmploymentComponent,
    pub housing: HousingComponent,
    pub health: HealthComponent,
    pub romance: RomanceComponent,
    pub finances: FinancesComponent,
    pub football_role: FootballRole,
    pub football_attributes: FootballPlayerAttributes,
    pub football_contract: Option<FootballContract>,
    pub owned_business_ids: Vec<EntityId>,
    pub political_party_id: Option<EntityId>,
    pub political_office_title: Option<String>,
    pub active_campaign: Option<PoliticalCampaign>,
    pub fame: FameComponent,
    pub creative_releases: Vec<CreativeRelease>,
    pub legal_status: LegalStatus,
    pub criminal_records: Vec<CriminalRecord>,
    pub prison_sentence: Option<PrisonSentence>,
    pub academic_degrees: Vec<AcademicDegree>,
    pub research_projects: Vec<ResearchProject>,
    pub patents: Vec<Patent>,
    pub belief: BeliefComponent,
    pub founded_movements: Vec<FaithMovement>,
    pub passports: Vec<Passport>,
    pub visas: Vec<Visa>,
    pub travel_history: Vec<TravelRecord>,
    pub military_record: Option<MilitaryRecord>,
    pub medical_history: Vec<MedicalRecord>,
    pub surgical_history: Vec<SurgicalProcedure>,
    pub will_and_testament: Option<WillAndTestament>,
    pub social_media_accounts: Vec<SocialMediaAccount>,
    pub digital_posts: Vec<DigitalPost>,
    pub secret_memberships: Vec<SecretMembership>,
    pub space_missions: Vec<SpaceMission>,
    pub cybernetic_implants: Vec<CyberneticImplant>,
    pub mind_uploads: Vec<MindUpload>,
    pub cosmic_megastructures: Vec<CosmicMegastructure>,
    pub location_id: EntityId,
    pub parent_ids: Vec<EntityId>,
    pub child_ids: Vec<EntityId>,
    pub active_roles: Vec<String>,
    pub knowledge: HashSet<String>,
    pub secrets: Vec<KnowledgeRecord>,
    pub memories: Vec<MemoryRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub timestamp: String,
    pub summary: String,
    pub importance: f32,
    pub emotional_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: EntityId,
    pub name: String,
    pub place_type: String,
    pub country_id: String,
    pub parent_place_id: Option<EntityId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: EntityId,
    pub name: String,
    pub org_type: String,
    pub city_id: EntityId,
    pub reputation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobVacancy {
    pub id: String,
    pub title: String,
    pub company_org_id: String,
    pub company_name: String,
    pub monthly_salary: f64,
    pub required_skill_id: String,
    pub min_skill_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: String,
    pub timestamp: String,
    pub event_type: String,
    pub actor_id: EntityId,
    pub target_id: Option<EntityId>,
    pub summary: String,
    pub metadata: serde_json::Value,
    pub causality_parent_id: Option<String>,
}

impl Person {
    pub fn get_life_stage(&self, current_year: i32, current_month: u32, current_day: u32) -> LifeStage {
        let age = SimTime::new(current_year, current_month, current_day, 0, 0).compute_age(
            self.identity.birth_year,
            self.identity.birth_month,
            self.identity.birth_day,
        );
        LifeStage::from_age(age, self.is_alive)
    }

    pub fn has_capability(&self, capability: Capability, current_year: i32, current_month: u32, current_day: u32) -> bool {
        let age = SimTime::new(current_year, current_month, current_day, 0, 0).compute_age(
            self.identity.birth_year,
            self.identity.birth_month,
            self.identity.birth_day,
        );

        match capability {
            Capability::CanWalk => age >= 1,
            Capability::CanSpeak => age >= 2,
            Capability::CanTravelAlone => age >= 12,
            Capability::CanAttendSchool => age >= 5,
            Capability::CanWork => age >= 16,
            Capability::CanSignContract => age >= 18,
            Capability::CanOpenBankAccount => age >= 16,
            Capability::CanDrive => age >= 17,
            Capability::CanMarry => age >= 18,
            Capability::CanVote => age >= 18,
            Capability::CanRunForOffice => age >= 21,
            Capability::CanEnlist => age >= 18,
            Capability::CanOwnBusiness => age >= 18,
            Capability::CanPlayProFootball => age >= 16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub start_date: String,
    pub snapshot_version: String,
    pub country_id: String,
    pub location_id: String,
    pub simulation_seed: u64,
    pub ruleset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLifeConfig {
    pub creation_mode: String,
    pub starting_year: i32,
    pub country_id: String,
    pub location_id: String,
    pub starting_age: u32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sex: Option<String>,
    pub household_income_tier: Option<String>,
    pub traits: HashMap<String, f32>,
    pub skills: HashMap<String, f32>,
    pub interests: Vec<String>,
    pub goals: Vec<String>,
}


use otherlife_relationships::{RelationshipMatrix, RelationshipVector};
use otherlife_rng::WorldRng;
use otherlife_world::{EventRecord, Person, SimTime};
use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    pub fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS persons (
                id TEXT PRIMARY KEY,
                is_player INTEGER NOT NULL,
                is_alive INTEGER NOT NULL,
                data TEXT NOT NULL
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS relationships (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                data TEXT NOT NULL,
                PRIMARY KEY (source_id, target_id)
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                actor_id TEXT NOT NULL,
                target_id TEXT,
                summary TEXT NOT NULL,
                data TEXT NOT NULL,
                causality_parent_id TEXT
            );",
            [],
        )?;

        Ok(())
    }

    pub fn save_world_state(
        &self,
        time: &SimTime,
        rng: &WorldRng,
        persons: &[Person],
        relationships: &RelationshipMatrix,
        events: &[EventRecord],
    ) -> Result<()> {
        let time_json = serde_json::to_string(time).unwrap();
        let rng_json = serde_json::to_string(rng).unwrap();

        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('time', ?1)",
            params![time_json],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('rng', ?1)",
            params![rng_json],
        )?;

        for person in persons {
            let data = serde_json::to_string(person).unwrap();
            self.conn.execute(
                "INSERT OR REPLACE INTO persons (id, is_player, is_alive, data) VALUES (?1, ?2, ?3, ?4)",
                params![person.id, person.is_player as i32, person.is_alive as i32, data],
            )?;
        }

        for ((src, tgt), rel) in &relationships.links {
            let data = serde_json::to_string(rel).unwrap();
            self.conn.execute(
                "INSERT OR REPLACE INTO relationships (source_id, target_id, data) VALUES (?1, ?2, ?3)",
                params![src, tgt, data],
            )?;
        }

        for ev in events {
            let data = serde_json::to_string(&ev.metadata).unwrap();
            self.conn.execute(
                "INSERT OR REPLACE INTO events (id, timestamp, event_type, actor_id, target_id, summary, data, causality_parent_id) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![ev.id, ev.timestamp, ev.event_type, ev.actor_id, ev.target_id, ev.summary, data, ev.causality_parent_id],
            )?;
        }

        Ok(())
    }

    pub fn load_world_state(&self) -> Result<(SimTime, WorldRng, Vec<Person>, RelationshipMatrix, Vec<EventRecord>)> {
        let time_str: String = self.conn.query_row(
            "SELECT value FROM meta WHERE key = 'time'",
            [],
            |r| r.get(0),
        )?;
        let time: SimTime = serde_json::from_str(&time_str).unwrap();

        let rng_str: String = self.conn.query_row(
            "SELECT value FROM meta WHERE key = 'rng'",
            [],
            |r| r.get(0),
        )?;
        let rng: WorldRng = serde_json::from_str(&rng_str).unwrap();

        let mut stmt = self.conn.prepare("SELECT data FROM persons")?;
        let person_rows = stmt.query_map([], |row| {
            let d: String = row.get(0)?;
            Ok(d)
        })?;
        let mut persons = Vec::new();
        for p_res in person_rows {
            let p_str = p_res?;
            let p: Person = serde_json::from_str(&p_str).unwrap();
            persons.push(p);
        }

        let mut rel_stmt = self.conn.prepare("SELECT source_id, target_id, data FROM relationships")?;
        let rel_rows = rel_stmt.query_map([], |row| {
            let src: String = row.get(0)?;
            let tgt: String = row.get(1)?;
            let d: String = row.get(2)?;
            Ok((src, tgt, d))
        })?;
        let mut matrix = RelationshipMatrix::new();
        for r_res in rel_rows {
            let (src, tgt, d_str) = r_res?;
            let rel: RelationshipVector = serde_json::from_str(&d_str).unwrap();
            matrix.set_link(src, tgt, rel);
        }

        let mut ev_stmt = self.conn.prepare("SELECT id, timestamp, event_type, actor_id, target_id, summary, data, causality_parent_id FROM events")?;
        let ev_rows = ev_stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let timestamp: String = row.get(1)?;
            let event_type: String = row.get(2)?;
            let actor_id: String = row.get(3)?;
            let target_id: Option<String> = row.get(4)?;
            let summary: String = row.get(5)?;
            let data_str: String = row.get(6)?;
            let causality_parent_id: Option<String> = row.get(7)?;
            let metadata: serde_json::Value = serde_json::from_str(&data_str).unwrap();
            Ok(EventRecord {
                id,
                timestamp,
                event_type,
                actor_id,
                target_id,
                summary,
                metadata,
                causality_parent_id,
            })
        })?;
        let mut events = Vec::new();
        for e_res in ev_rows {
            events.push(e_res?);
        }

        Ok((time, rng, persons, matrix, events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use otherlife_world::{EducationComponent, FinancesComponent, IdentityComponent, PersonalityComponent};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_sqlite_roundtrip_fidelity() {
        let db = Database::open_in_memory().unwrap();
        let time = SimTime::new(2029, 10, 12, 16, 30);
        let rng = WorldRng::new(42);

        let p = Person {
            id: "person:sim:player".to_string(),
            is_player: true,
            is_alive: true,
            tier: otherlife_world::NpcTier::TierA,
            schedule: otherlife_world::NpcSchedule::default(),
            identity: IdentityComponent {
                first_name: "James".to_string(),
                last_name: "Morrison".to_string(),
                birth_year: 2015,
                birth_month: 4,
                birth_day: 12,
                sex: "Male".to_string(),
                birth_location_id: "city:real:glasgow".to_string(),
                current_location_id: "city:real:glasgow".to_string(),
                nationalities: vec!["country:real:united_kingdom".to_string()],
                citizenships: vec!["country:real:united_kingdom".to_string()],
            },
            personality: PersonalityComponent::default(),
            skills: HashMap::new(),
            interests: HashSet::new(),
            goals: Vec::new(),
            education: EducationComponent {
                school_id: Some("school:real:glasgow_high".to_string()),
                grade_level: 3,
                academic_performance: 42.0,
                attendance_rate: 88.0,
                qualifications: Vec::new(),
                degree_program: None,
            },
            employment: otherlife_world::EmploymentComponent::default(),
            housing: otherlife_world::HousingComponent::default(),
            health: otherlife_world::HealthComponent::default(),
            romance: otherlife_world::RomanceComponent::default(),
            finances: FinancesComponent {
                cash: 24.0,
                monthly_allowance: 20.0,
                household_income_tier: "MIDDLE".to_string(),
                monthly_expenses: 0.0,
            },
            football_role: otherlife_world::FootballRole::None,
            football_attributes: otherlife_world::FootballPlayerAttributes::default(),
            football_contract: None,
            owned_business_ids: Vec::new(),
            political_party_id: None,
            political_office_title: None,
            active_campaign: None,
            fame: otherlife_world::FameComponent::default(),
            creative_releases: Vec::new(),
            legal_status: otherlife_world::LegalStatus::Clean,
            criminal_records: Vec::new(),
            prison_sentence: None,
            academic_degrees: Vec::new(),
            research_projects: Vec::new(),
            patents: Vec::new(),
            belief: otherlife_world::BeliefComponent::default(),
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
            location_id: "city:real:glasgow".to_string(),
            parent_ids: Vec::new(),
            child_ids: Vec::new(),
            active_roles: Vec::new(),
            knowledge: HashSet::new(),
            secrets: Vec::new(),
            memories: Vec::new(),
        };

        let mut matrix = RelationshipMatrix::new();
        matrix.set_link("person:sim:player".to_string(), "person:sim:mum".to_string(), RelationshipVector::default());

        let events = vec![EventRecord {
            id: "ev:1".to_string(),
            timestamp: time.formatted(),
            event_type: "DECEIVE".to_string(),
            actor_id: "person:sim:player".to_string(),
            target_id: Some("person:sim:mum".to_string()),
            summary: "Lied about math study to attend football training.".to_string(),
            metadata: serde_json::json!({}),
            causality_parent_id: None,
        }];

        db.save_world_state(&time, &rng, &[p.clone()], &matrix, &events).unwrap();

        let (loaded_time, loaded_rng, loaded_persons, loaded_matrix, loaded_events) = db.load_world_state().unwrap();

        assert_eq!(loaded_time.year, 2029);
        assert_eq!(loaded_rng.seed, 42);
        assert_eq!(loaded_persons.len(), 1);
        assert_eq!(loaded_persons[0].identity.first_name, "James");
        assert_eq!(loaded_events.len(), 1);
        assert_eq!(loaded_events[0].id, "ev:1");
    }
}

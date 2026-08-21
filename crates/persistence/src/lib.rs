use otherlife_rng::WorldRng;
use otherlife_world::{EventRecord, HumanEntity, SimTime};
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
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                birth_year INTEGER NOT NULL,
                sex TEXT NOT NULL,
                is_player INTEGER NOT NULL,
                is_alive INTEGER NOT NULL,
                data TEXT NOT NULL
            );",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                actor_id TEXT NOT NULL,
                location_id TEXT NOT NULL,
                headline TEXT NOT NULL,
                narrative TEXT NOT NULL,
                causality_note TEXT NOT NULL,
                success INTEGER NOT NULL
            );",
            [],
        )?;

        Ok(())
    }

    pub fn save_world_state(
        &self,
        time: &SimTime,
        rng: &WorldRng,
        persons: &[HumanEntity],
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
                "INSERT OR REPLACE INTO persons (id, first_name, last_name, birth_year, sex, is_player, is_alive, data) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    person.id,
                    person.identity.first_name,
                    person.identity.last_name,
                    person.identity.birth_year,
                    person.identity.sex,
                    person.is_player as i32,
                    person.biology.is_alive as i32,
                    data
                ],
            )?;
        }

        for ev in events {
            self.conn.execute(
                "INSERT OR REPLACE INTO events (id, timestamp, event_type, actor_id, location_id, headline, narrative, causality_note, success) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    ev.id,
                    ev.timestamp,
                    ev.event_type,
                    ev.actor_id,
                    ev.location_id,
                    ev.headline,
                    ev.narrative,
                    ev.causality_note,
                    ev.success as i32
                ],
            )?;
        }

        Ok(())
    }

    pub fn load_world_state(&self) -> Result<(SimTime, WorldRng, Vec<HumanEntity>, Vec<EventRecord>)> {
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
            let p: HumanEntity = serde_json::from_str(&p_str).unwrap();
            persons.push(p);
        }

        let mut ev_stmt = self.conn.prepare("SELECT id, timestamp, event_type, actor_id, location_id, headline, narrative, causality_note, success FROM events")?;
        let ev_rows = ev_stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let timestamp: String = row.get(1)?;
            let event_type: String = row.get(2)?;
            let actor_id: String = row.get(3)?;
            let location_id: String = row.get(4)?;
            let headline: String = row.get(5)?;
            let narrative: String = row.get(6)?;
            let causality_note: String = row.get(7)?;
            let success_int: i32 = row.get(8)?;
            Ok(EventRecord {
                id,
                timestamp,
                day_total: 0,
                event_type,
                actor_id,
                location_id,
                headline,
                narrative,
                causality_note,
                success: success_int != 0,
            })
        })?;
        let mut events = Vec::new();
        for e_res in ev_rows {
            events.push(e_res?);
        }

        Ok((time, rng, persons, events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use otherlife_world::{BiologicalProfile, HumanResources, IdentityProfile, PsychologicalProfile, WealthTier};
    use std::collections::HashMap;

    #[test]
    fn test_sqlite_persistence_roundtrip() {
        let db = Database::open_in_memory().unwrap();
        let time = SimTime::new(2025, 4, 12);
        let rng = WorldRng::new(99);

        let p = HumanEntity {
            id: "person:sim:player".to_string(),
            identity: IdentityProfile {
                first_name: "Israel".to_string(),
                last_name: "Oyebamiji".to_string(),
                birth_year: 2005,
                birth_month: 1,
                birth_day: 15,
                sex: "Male".to_string(),
                birthplace_id: "city:real:abuja".to_string(),
                nationality: "country:real:nigeria".to_string(),
                culture: "Nigerian".to_string(),
                primary_language: "English".to_string(),
            },
            biology: BiologicalProfile {
                is_alive: true,
                death_year: None,
                death_reason: None,
                health_overall: 95.0,
                fitness: 70.0,
                energy_level: 85.0,
                chronic_conditions: Vec::new(),
            },
            psychology: PsychologicalProfile {
                discipline: 0.65,
                curiosity: 0.80,
                creativity: 0.70,
                confidence: 0.60,
                risk_tolerance: 0.50,
                stress_level: 15.0,
                resilience: 0.60,
            },
            reputation: otherlife_world::ReputationProfile::default(),
            skills: HashMap::new(),
            resources: HumanResources {
                cash: 50000.0,
                household_wealth_tier: WealthTier::MiddleClass,
                living_arrangement: "FAMILY_HOME".to_string(),
                tools_available: vec!["BOOKS".to_string()],
            },
            relationships: HashMap::new(),
            occupation: None,
            is_player: true,
        };

        let ev = EventRecord {
            id: "ev:1".to_string(),
            timestamp: time.literary_date(),
            day_total: time.total_days,
            event_type: "EDUCATION_PRACTICE".to_string(),
            actor_id: p.id.clone(),
            location_id: "city:real:abuja".to_string(),
            headline: "Study Session".to_string(),
            narrative: "Completed mathematics coursework.".to_string(),
            causality_note: "Consistent effort.".to_string(),
            success: true,
        };

        db.save_world_state(&time, &rng, &[p.clone()], &[ev.clone()]).unwrap();
        let (loaded_time, loaded_rng, loaded_persons, loaded_events) = db.load_world_state().unwrap();

        assert_eq!(loaded_time.year, 2025);
        assert_eq!(loaded_rng.seed, 99);
        assert_eq!(loaded_persons.len(), 1);
        assert_eq!(loaded_persons[0].identity.first_name, "Israel");
        assert_eq!(loaded_events.len(), 1);
        assert_eq!(loaded_events[0].id, "ev:1");
    }
}


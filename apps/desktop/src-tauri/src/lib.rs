use otherlife_persistence::Database;
use otherlife_simulation::{SidebarStateDTO, SimulationEngine, StepResult};
use otherlife_world::NewLifeConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub engine: Mutex<SimulationEngine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateDTO {
    pub time_formatted: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub age: u32,
    pub is_alive: bool,
    pub cash: f64,
    pub location: String,
    pub player_name: String,
    pub active_interest: String,
    pub event_count: usize,
    pub interests: Vec<String>,
    pub goals: Vec<String>,
    pub life_stage: String,
    pub marital_status: String,
    pub job_title: String,
    pub monthly_salary: f64,
    pub housing_type: String,
    pub fitness: f32,
    pub stress: f32,
}

impl GameStateDTO {
    pub fn from_engine(engine: &SimulationEngine) -> Self {
        let player = engine.persons.get("person:sim:player").unwrap();
        let age = (engine.time.year - player.identity.birth_year) as u32;
        let stage = otherlife_world::LifeStage::from_age(age, player.is_alive);

        Self {
            time_formatted: engine.time.formatted(),
            year: engine.time.year,
            month: engine.time.month,
            day: engine.time.day,
            age,
            is_alive: player.is_alive,
            cash: player.finances.cash,
            location: player.location_id.clone(),
            player_name: format!("{} {}", player.identity.first_name, player.identity.last_name),
            active_interest: player.interests.iter().next().cloned().unwrap_or_else(|| "General Life".to_string()),
            event_count: engine.events.len(),
            interests: player.interests.iter().cloned().collect(),
            goals: player.goals.clone(),
            life_stage: format!("{:?}", stage),
            marital_status: player.romance.marital_status.clone(),
            job_title: player.employment.job_title.clone().unwrap_or_else(|| "Unemployed / Student".to_string()),
            monthly_salary: player.employment.monthly_salary,
            housing_type: player.housing.housing_type.clone(),
            fitness: player.health.fitness,
            stress: player.health.stress,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistriesDTO {
    pub countries: serde_json::Value,
    pub locations: serde_json::Value,
    pub skills: serde_json::Value,
    pub traits: serde_json::Value,
    pub interests: serde_json::Value,
    pub goals: serde_json::Value,
    pub clubs: serde_json::Value,
    pub parties: serde_json::Value,
    pub universities: serde_json::Value,
    pub companies: serde_json::Value,
}

#[tauri::command]
fn get_registries() -> Result<RegistriesDTO, String> {
    let read_json = |p: &str| -> serde_json::Value {
        fs::read_to_string(p)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(serde_json::json!([]))
    };

    Ok(RegistriesDTO {
        countries: read_json("../../real_world_data/geography/countries.json"),
        locations: read_json("../../real_world_data/geography/locations.json"),
        skills: read_json("../../real_world_data/registries/skills.json"),
        traits: read_json("../../real_world_data/registries/traits.json"),
        interests: read_json("../../real_world_data/registries/interests.json"),
        goals: read_json("../../real_world_data/registries/goals.json"),
        clubs: read_json("../../real_world_data/football/clubs.json"),
        parties: read_json("../../real_world_data/politics/parties.json"),
        universities: read_json("../../real_world_data/education/universities.json"),
        companies: read_json("../../real_world_data/companies/corporations.json"),
    })
}

#[tauri::command]
fn start_new_life(
    state: State<AppState>,
    config: Option<NewLifeConfig>,
    seed: Option<u64>,
) -> Result<(GameStateDTO, Vec<String>, SidebarStateDTO), String> {
    let mut engine = state.engine.lock().map_err(|e| e.to_string())?;

    if let Some(cfg) = config {
        *engine = SimulationEngine::new_game(cfg, seed.unwrap_or(42));
    } else {
        *engine = SimulationEngine::new_vertical_slice_fixture(seed.unwrap_or(42));
    }

    let dto = GameStateDTO::from_engine(&engine);
    let suggestions = engine.get_suggested_actions();
    let sidebar = engine.get_sidebar_state();

    Ok((dto, suggestions, sidebar))
}

#[tauri::command]
fn submit_player_action(
    state: State<AppState>,
    input_text: String,
) -> Result<(GameStateDTO, StepResult, Vec<String>, SidebarStateDTO), String> {
    let mut engine = state.engine.lock().map_err(|e| e.to_string())?;

    let payload = engine.ai_bridge.parse_intent(
        &input_text,
        "person:sim:player",
        Some("person:sim:mum"),
    );

    let result = engine.execute_player_action(payload);
    let dto = GameStateDTO::from_engine(&engine);
    let suggestions = engine.get_suggested_actions();
    let sidebar = engine.get_sidebar_state();

    Ok((dto, result, suggestions, sidebar))
}

#[tauri::command]
fn save_game_state(state: State<AppState>, path: String) -> Result<String, String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let db = Database::open_file(&path).map_err(|e| e.to_string())?;

    let persons_vec: Vec<_> = engine.persons.values().cloned().collect();
    db.save_world_state(&engine.time, &engine.rng, &persons_vec, &engine.relationships, &engine.events)
        .map_err(|e| e.to_string())?;

    Ok("Game state successfully saved to SQLite.".to_string())
}

#[tauri::command]
fn get_biography(state: State<AppState>) -> Result<String, String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_biography())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = SimulationEngine::new_vertical_slice_fixture(42);
    let app_state = AppState {
        engine: Mutex::new(engine),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_registries,
            start_new_life,
            submit_player_action,
            save_game_state,
            get_biography,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

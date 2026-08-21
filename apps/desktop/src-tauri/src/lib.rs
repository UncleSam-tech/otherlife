use otherlife_simulation::SimulationEngine;
use otherlife_world::{
    ContextNpcDTO, ContextProcessDTO, LivingStateDTO, LivingStepResultDTO, NewLifeConfig,
    TodaySceneDTO,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub engine: Mutex<SimulationEngine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMetadataDTO {
    pub id: String,
    pub filename: String,
    pub player_name: String,
    pub age: u32,
    pub location: String,
    pub timestamp: String,
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

fn read_json_data(base: &Path, rel: &str) -> serde_json::Value {
    let p1 = base.join(rel);
    if p1.exists() {
        if let Ok(s) = fs::read_to_string(&p1) {
            if let Ok(v) = serde_json::from_str(&s) {
                return v;
            }
        }
    }

    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(rel);

    if dev_path.exists() {
        if let Ok(s) = fs::read_to_string(&dev_path) {
            if let Ok(v) = serde_json::from_str(&s) {
                return v;
            }
        }
    }

    serde_json::json!([])
}

pub mod commands {
    use super::*;

    #[tauri::command]
    pub fn get_registries(app: AppHandle) -> RegistriesDTO {
        let resource_dir = app.path().resource_dir().unwrap_or_else(|_| PathBuf::from("."));

        RegistriesDTO {
            countries: read_json_data(&resource_dir, "real_world_data/geography/countries.json"),
            locations: read_json_data(&resource_dir, "real_world_data/geography/cities.json"),
            skills: read_json_data(&resource_dir, "real_world_data/human/skills.json"),
            traits: read_json_data(&resource_dir, "real_world_data/human/traits.json"),
            interests: read_json_data(&resource_dir, "real_world_data/human/interests.json"),
            goals: read_json_data(&resource_dir, "real_world_data/human/goals.json"),
            clubs: read_json_data(&resource_dir, "real_world_data/sports/football_clubs.json"),
            parties: read_json_data(&resource_dir, "real_world_data/politics/parties.json"),
            universities: read_json_data(&resource_dir, "real_world_data/education/universities.json"),
            companies: read_json_data(&resource_dir, "real_world_data/companies/corporations.json"),
        }
    }

    #[tauri::command]
    pub fn start_new_life(
        state: State<'_, AppState>,
        config: NewLifeConfig,
        seed: Option<u64>,
    ) -> (LivingStateDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        *engine = SimulationEngine::new_game(config, seed.unwrap_or(42));

        let living_state = engine.get_living_state();
        let scene = engine.generate_today_scene();
        let npcs = engine.get_surrounding_npcs();
        let procs = engine.get_active_processes();

        (living_state, scene, npcs, procs)
    }

    #[tauri::command]
    pub fn submit_living_intent(
        state: State<'_, AppState>,
        intent_text: String,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.submit_living_intent(&intent_text);
        let living_state = engine.get_living_state();
        let scene = engine.generate_today_scene();
        let npcs = engine.get_surrounding_npcs();
        let procs = engine.get_active_processes();

        (living_state, step_res, scene, npcs, procs)
    }

    #[tauri::command]
    pub fn get_living_state(state: State<'_, AppState>) -> LivingStateDTO {
        let engine = state.engine.lock().unwrap();
        engine.get_living_state()
    }

    #[tauri::command]
    pub fn get_today_scene(state: State<'_, AppState>) -> TodaySceneDTO {
        let engine = state.engine.lock().unwrap();
        engine.generate_today_scene()
    }

    #[tauri::command]
    pub fn get_biography(state: State<'_, AppState>) -> String {
        let engine = state.engine.lock().unwrap();
        engine.get_biography()
    }

    #[tauri::command]
    pub fn get_letters_inbox(state: State<'_, AppState>) -> Vec<otherlife_world::LetterNotification> {
        let engine = state.engine.lock().unwrap();
        engine.letters_inbox.clone()
    }

    fn get_saves_dir(app: &AppHandle) -> PathBuf {
        let base = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
        let saves_dir = base.join("saves");
        fs::create_dir_all(&saves_dir).ok();
        saves_dir
    }

    #[tauri::command]
    pub fn list_saves(app: AppHandle) -> Vec<SaveMetadataDTO> {
        let saves_dir = get_saves_dir(&app);
        let mut saves = Vec::new();

        if let Ok(entries) = fs::read_dir(saves_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(dto) = serde_json::from_str::<SaveMetadataDTO>(&content) {
                            saves.push(dto);
                        }
                    }
                }
            }
        }

        saves
    }

    #[tauri::command]
    pub fn delete_save(app: AppHandle, filename: String) -> bool {
        let saves_dir = get_saves_dir(&app);
        let json_path = saves_dir.join(&filename);
        fs::remove_file(json_path).is_ok()
    }
}

pub fn run() {
    let default_config = NewLifeConfig {
        ..Default::default()
    };

    let initial_engine = SimulationEngine::new_game(default_config, 100);

    tauri::Builder::default()
        .manage(AppState {
            engine: Mutex::new(initial_engine),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_registries,
            commands::start_new_life,
            commands::submit_living_intent,
            commands::get_living_state,
            commands::get_today_scene,
            commands::get_biography,
            commands::list_saves,
            commands::delete_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running OTHERLIFE desktop application");
}

use otherlife_simulation::SimulationEngine;
use otherlife_world::{
    ContextNpcDTO, ContextProcessDTO, DocumentDTO, LivingStateDTO, LivingStepResultDTO,
    NewLifeConfig, TodaySceneDTO,
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

fn get_saves_dir(app: &AppHandle) -> PathBuf {
    let base = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let saves_dir = base.join("saves");
    fs::create_dir_all(&saves_dir).ok();
    saves_dir
}

fn autosave_engine(app: &AppHandle, engine: &SimulationEngine) {
    let saves_dir = get_saves_dir(app);
    if let Ok(json_str) = engine.save_to_string() {
        fs::write(saves_dir.join("autosave.json"), json_str).ok();
        let meta = SaveMetadataDTO {
            id: "autosave".to_string(),
            filename: "autosave.json".to_string(),
            player_name: engine.get_player().identity.full_name(),
            age: engine.get_player_age(),
            location: engine.rule_pack.city_name.clone(),
            timestamp: engine.time.literary_date(),
        };
        if let Ok(meta_json) = serde_json::to_string(&meta) {
            fs::write(saves_dir.join("autosave_meta.json"), meta_json).ok();
        }
    }
}

pub mod commands {
    use super::*;

    #[tauri::command]
    pub fn get_registries(app: AppHandle) -> RegistriesDTO {
        let resource_dir = app.path().resource_dir().unwrap_or_else(|_| PathBuf::from("."));

        RegistriesDTO {
            countries: read_json_data(&resource_dir, "real_world_data/geography/countries.json"),
            locations: read_json_data(&resource_dir, "real_world_data/geography/cities.json"),
            skills: read_json_data(&resource_dir, "real_world_data/registries/skills.json"),
            traits: read_json_data(&resource_dir, "real_world_data/registries/traits.json"),
            interests: read_json_data(&resource_dir, "real_world_data/registries/interests.json"),
            goals: read_json_data(&resource_dir, "real_world_data/registries/goals.json"),
            clubs: read_json_data(&resource_dir, "real_world_data/football/clubs.json"),
            parties: read_json_data(&resource_dir, "real_world_data/politics/parties.json"),
            universities: read_json_data(&resource_dir, "real_world_data/education/universities.json"),
            companies: read_json_data(&resource_dir, "real_world_data/companies/corporations.json"),
        }
    }

    #[tauri::command]
    pub fn start_new_life(
        app: AppHandle,
        state: State<'_, AppState>,
        config: NewLifeConfig,
        seed: Option<u64>,
    ) -> (LivingStateDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        *engine = SimulationEngine::new_game(config, seed.unwrap_or(42));

        // Auto-save initial world state
        let saves_dir = get_saves_dir(&app);
        if let Ok(json_str) = engine.save_to_string() {
            fs::write(saves_dir.join("autosave.json"), json_str).ok();
            let meta = SaveMetadataDTO {
                id: "autosave".to_string(),
                filename: "autosave.json".to_string(),
                player_name: engine.get_player().identity.full_name(),
                age: engine.get_player_age(),
                location: engine.rule_pack.city_name.clone(),
                timestamp: engine.time.literary_date(),
            };
            if let Ok(meta_json) = serde_json::to_string(&meta) {
                fs::write(saves_dir.join("autosave_meta.json"), meta_json).ok();
            }
        }

        let living_state = engine.get_living_state();
        let scene = engine.generate_today_scene();
        let npcs = engine.get_surrounding_npcs();
        let procs = engine.get_active_processes();

        (living_state, scene, npcs, procs)
    }

    #[tauri::command]
    pub fn submit_living_intent(
        app: AppHandle,
        state: State<'_, AppState>,
        intent_text: String,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.submit_living_intent(&intent_text);

        // Auto-save on each turn
        let saves_dir = get_saves_dir(&app);
        if let Ok(json_str) = engine.save_to_string() {
            fs::write(saves_dir.join("autosave.json"), json_str).ok();
            let meta = SaveMetadataDTO {
                id: "autosave".to_string(),
                filename: "autosave.json".to_string(),
                player_name: engine.get_player().identity.full_name(),
                age: engine.get_player_age(),
                location: engine.rule_pack.city_name.clone(),
                timestamp: engine.time.literary_date(),
            };
            if let Ok(meta_json) = serde_json::to_string(&meta) {
                fs::write(saves_dir.join("autosave_meta.json"), meta_json).ok();
            }
        }

        let living_state = engine.get_living_state();
        let scene = engine.generate_today_scene();
        let npcs = engine.get_surrounding_npcs();
        let procs = engine.get_active_processes();

        (living_state, step_res, scene, npcs, procs)
    }

    #[tauri::command]
    pub fn advance_time_explicit(
        app: AppHandle,
        state: State<'_, AppState>,
        action_type: String,
        amount: Option<u32>,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = match action_type.as_str() {
            "HOURS" => engine.advance_hours(amount.unwrap_or(1)),
            "DAYS" => engine.advance_days(amount.unwrap_or(1)),
            "SLEEP" => engine.sleep_until_morning(),
            "ROUTINE" => engine.follow_routine(amount.unwrap_or(7)),
            _ => engine.advance_days(1),
        };

        // Auto-save on each turn
        let saves_dir = get_saves_dir(&app);
        if let Ok(json_str) = engine.save_to_string() {
            fs::write(saves_dir.join("autosave.json"), json_str).ok();
            let meta = SaveMetadataDTO {
                id: "autosave".to_string(),
                filename: "autosave.json".to_string(),
                player_name: engine.get_player().identity.full_name(),
                age: engine.get_player_age(),
                location: engine.rule_pack.city_name.clone(),
                timestamp: engine.time.literary_date(),
            };
            if let Ok(meta_json) = serde_json::to_string(&meta) {
                fs::write(saves_dir.join("autosave_meta.json"), meta_json).ok();
            }
        }

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
    pub fn get_documents(state: State<'_, AppState>) -> Vec<DocumentDTO> {
        let engine = state.engine.lock().unwrap();
        engine.get_documents()
    }

    #[tauri::command]
    pub fn get_letters_inbox(state: State<'_, AppState>) -> Vec<otherlife_world::LetterNotification> {
        let engine = state.engine.lock().unwrap();
        engine.letters_inbox.clone()
    }

    #[tauri::command]
    pub fn get_phone_messages(state: State<'_, AppState>) -> Vec<otherlife_world::PhoneMessage> {
        let engine = state.engine.lock().unwrap();
        engine.get_phone_messages()
    }

    #[tauri::command]
    pub fn send_phone_message(
        app: AppHandle,
        state: State<'_, AppState>,
        recipient_id: String,
        text: String,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.send_phone_message(&recipient_id, &text);
        autosave_engine(&app, &engine);
        (engine.get_living_state(), step_res, engine.generate_today_scene(), engine.get_surrounding_npcs(), engine.get_active_processes())
    }

    #[tauri::command]
    pub fn apply_for_job(
        app: AppHandle,
        state: State<'_, AppState>,
        job_id: String,
        company_id: String,
        title: String,
        company_name: String,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.apply_for_job(&job_id, &company_id, &title, &company_name);
        autosave_engine(&app, &engine);
        (engine.get_living_state(), step_res, engine.generate_today_scene(), engine.get_surrounding_npcs(), engine.get_active_processes())
    }

    #[tauri::command]
    pub fn register_company(
        app: AppHandle,
        state: State<'_, AppState>,
        name: String,
        structure: String,
        partners: Vec<String>,
        authorized_capital: f64,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.register_company(&name, &structure, &partners, authorized_capital);
        autosave_engine(&app, &engine);
        (engine.get_living_state(), step_res, engine.generate_today_scene(), engine.get_surrounding_npcs(), engine.get_active_processes())
    }

    #[tauri::command]
    pub fn travel_to_location(
        app: AppHandle,
        state: State<'_, AppState>,
        destination_city_id: String,
        transport_mode: String,
        stay_days: u32,
    ) -> (LivingStateDTO, LivingStepResultDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>) {
        let mut engine = state.engine.lock().unwrap();
        let step_res = engine.travel_to_location(&destination_city_id, &transport_mode, stay_days);
        autosave_engine(&app, &engine);
        (engine.get_living_state(), step_res, engine.generate_today_scene(), engine.get_surrounding_npcs(), engine.get_active_processes())
    }

    #[tauri::command]
    pub fn save_game(app: AppHandle, state: State<'_, AppState>, slot_name: Option<String>) -> bool {
        let engine = state.engine.lock().unwrap();
        let saves_dir = get_saves_dir(&app);
        let filename = slot_name.unwrap_or_else(|| format!("save_{}.json", engine.time.total_days));

        if let Ok(json_str) = engine.save_to_string() {
            if fs::write(saves_dir.join(&filename), json_str).is_ok() {
                let meta = SaveMetadataDTO {
                    id: filename.clone(),
                    filename: filename.clone(),
                    player_name: engine.get_player().identity.full_name(),
                    age: engine.get_player_age(),
                    location: engine.rule_pack.city_name.clone(),
                    timestamp: engine.time.literary_date(),
                };
                let meta_file = format!("{}_meta.json", filename.trim_end_matches(".json"));
                if let Ok(meta_json) = serde_json::to_string(&meta) {
                    fs::write(saves_dir.join(meta_file), meta_json).ok();
                }
                return true;
            }
        }
        false
    }

    #[tauri::command]
    pub fn load_game(
        app: AppHandle,
        state: State<'_, AppState>,
        filename: String,
    ) -> Option<(LivingStateDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>)> {
        let saves_dir = get_saves_dir(&app);
        let path = saves_dir.join(&filename);
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(loaded_engine) = SimulationEngine::load_from_string(&content) {
                let mut engine = state.engine.lock().unwrap();
                *engine = loaded_engine;

                let living_state = engine.get_living_state();
                let scene = engine.generate_today_scene();
                let npcs = engine.get_surrounding_npcs();
                let procs = engine.get_active_processes();

                return Some((living_state, scene, npcs, procs));
            }
        }
        None
    }

    #[tauri::command]
    pub fn continue_recent_save(
        app: AppHandle,
        state: State<'_, AppState>,
    ) -> Option<(LivingStateDTO, TodaySceneDTO, Vec<ContextNpcDTO>, Vec<ContextProcessDTO>)> {
        let saves_dir = get_saves_dir(&app);
        let autosave_path = saves_dir.join("autosave.json");
        if autosave_path.exists() {
            return load_game(app, state, "autosave.json".to_string());
        }

        // Fallback to any save file in directory
        if let Ok(entries) = fs::read_dir(&saves_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some("json")
                    && !p.file_name().unwrap().to_str().unwrap().ends_with("_meta.json")
                {
                    let fname = p.file_name().unwrap().to_str().unwrap().to_string();
                    return load_game(app, state, fname);
                }
            }
        }
        None
    }

    #[tauri::command]
    pub fn list_saves(app: AppHandle) -> Vec<SaveMetadataDTO> {
        let saves_dir = get_saves_dir(&app);
        let mut saves = Vec::new();

        if let Ok(entries) = fs::read_dir(saves_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let fname = path.file_name().unwrap().to_str().unwrap_or("");
                if fname.ends_with("_meta.json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(dto) = serde_json::from_str::<SaveMetadataDTO>(&content) {
                            saves.push(dto);
                        }
                    }
                }
            }
        }

        saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        saves
    }

    #[tauri::command]
    pub fn delete_save(app: AppHandle, filename: String) -> bool {
        let saves_dir = get_saves_dir(&app);
        let json_path = saves_dir.join(&filename);
        let meta_path = saves_dir.join(format!("{}_meta.json", filename.trim_end_matches(".json")));
        fs::remove_file(json_path).ok();
        fs::remove_file(meta_path).ok();
        true
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
            commands::advance_time_explicit,
            commands::get_living_state,
            commands::get_today_scene,
            commands::get_biography,
            commands::get_documents,
            commands::get_letters_inbox,
            commands::get_phone_messages,
            commands::send_phone_message,
            commands::apply_for_job,
            commands::register_company,
            commands::travel_to_location,
            commands::save_game,
            commands::load_game,
            commands::continue_recent_save,
            commands::list_saves,
            commands::delete_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running OTHERLIFE desktop application");
}

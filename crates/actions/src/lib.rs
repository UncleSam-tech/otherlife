use otherlife_world::HumanEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionPrimitive {
    Communicate,
    Study,
    Train,
    Apply,
    Work,
    Explore,
    Rest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPayload {
    pub action: ActionPrimitive,
    pub actor_id: String,
    pub target_id: Option<String>,
    pub intensity: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

pub struct ActionValidator;

impl ActionValidator {
    pub fn validate(actor: &HumanEntity, action: &ActionPayload) -> ValidationResult {
        if !actor.biology.is_alive {
            return ValidationResult {
                is_valid: false,
                reason: Some("Deceased characters cannot execute actions.".to_string()),
            };
        }

        match action.action {
            ActionPrimitive::Work => {
                if actor.occupation.is_none() {
                    return ValidationResult {
                        is_valid: false,
                        reason: Some("Cannot execute work action without active employment.".to_string()),
                    };
                }
            }
            _ => {}
        }

        ValidationResult {
            is_valid: true,
            reason: None,
        }
    }
}

use otherlife_world::{EntityId, Person};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionPrimitive {
    Communicate,
    Ask,
    Persuade,
    Deceive,
    AttendActivity,
    Study,
    Train,
    Compete,
    Apply,
    Work,
    Move,
    Buy,
    Sell,
    Rest,
    Date,
    Marry,
    Divorce,
    ApplyJob,
    WorkShift,
    RentApartment,
    BuyProperty,
    PayRent,
    SeekMedicalTreatment,
    HaveChild,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: String,
    pub domain: String, // "sports", "music", "politics", "business", "education", "employment"
    pub source: String,
    pub target_id: Option<EntityId>,
    pub requirements: serde_json::Value,
    pub visibility_conditions: serde_json::Value,
    pub expiry_days: u32,
    pub probability: f32,
    pub effects: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionClaim {
    pub claimed_destination: Option<String>,
    pub claimed_activity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPayload {
    pub action: ActionPrimitive,
    pub actor_id: EntityId,
    pub target_id: Option<EntityId>,
    pub claim: Option<ActionClaim>,
    pub actual_activity: Option<String>,
    pub intensity: f32,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

pub struct ActionValidator;

impl ActionValidator {
    pub fn validate(actor: &Person, action: &ActionPayload) -> ValidationResult {
        if !actor.is_alive {
            return ValidationResult {
                is_valid: false,
                reason: Some("Deceased characters cannot execute actions.".to_string()),
            };
        }

        match action.action {
            ActionPrimitive::Work | ActionPrimitive::WorkShift => {
                if let Some(ref title) = actor.employment.job_title {
                    if title == "Unemployed / Infant" || title == "Unemployed / Student" || title == "Unemployed" {
                        return ValidationResult {
                            is_valid: false,
                            reason: Some("Unemployed characters and infants cannot complete work shifts.".to_string()),
                        };
                    }
                } else {
                    return ValidationResult {
                        is_valid: false,
                        reason: Some("Cannot execute work action without active employment.".to_string()),
                    };
                }
            }
            ActionPrimitive::Deceive => {
                if action.target_id.is_none() {
                    return ValidationResult {
                        is_valid: false,
                        reason: Some("Deception requires a target NPC.".to_string()),
                    };
                }
            }
            ActionPrimitive::AttendActivity => {
                if action.actual_activity.is_none() {
                    return ValidationResult {
                        is_valid: false,
                        reason: Some("Activity type must be specified.".to_string()),
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

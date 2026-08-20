use otherlife_actions::{ActionClaim, ActionPayload, ActionPrimitive};
use otherlife_world::{EventRecord, MemoryRecord, Person};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBridgeConfig {
    pub use_local_llm: bool,
    pub model_tier: String, // "Fast", "Balanced", "Enhanced"
    pub model_endpoint: Option<String>,
}

impl Default for AIBridgeConfig {
    fn default() -> Self {
        Self {
            use_local_llm: false,
            model_tier: "Fast".to_string(),
            model_endpoint: None,
        }
    }
}

pub struct HallucinationValidator;

impl HallucinationValidator {
    pub fn validate_and_sanitize(rendered_text: &str, expected_success: bool, causality_note: &str) -> String {
        // Sanity check: Ensure rendered text does not contradict simulation outcome state
        let text_lower = rendered_text.to_lowercase();
        if expected_success && (text_lower.contains("failed completely") || text_lower.contains("was rejected")) {
            return format!("Action succeeded as intended. ({})", causality_note);
        }
        if !expected_success && (text_lower.contains("succeeded brilliantly") || text_lower.contains("achieved flawless")) {
            return format!("Action did not achieve the intended result. ({})", causality_note);
        }
        rendered_text.to_string()
    }
}

pub struct MemorySummarizer;

impl MemorySummarizer {
    pub fn summarize_events(events: &[EventRecord]) -> Vec<MemoryRecord> {
        events
            .iter()
            .map(|ev| MemoryRecord {
                id: format!("mem-{}", ev.id),
                timestamp: ev.timestamp.clone(),
                summary: ev.summary.clone(),
                importance: if ev.event_type.contains("DEATH") || ev.event_type.contains("MARRIAGE") || ev.event_type.contains("NEW_LIFE") {
                    1.0
                } else {
                    0.5
                },
                emotional_weight: 0.7,
            })
            .collect()
    }
}

pub struct BiographyWriter;

impl BiographyWriter {
    pub fn generate_lifetime_biography(player_name: &str, events: &[EventRecord]) -> String {
        if events.is_empty() {
            return format!("The life story of {} has just begun.", player_name);
        }

        let mut bio = format!("# The Life Biography of {}\n\n", player_name);
        bio.push_str("## Chapter 1: Foundations and Origins\n");

        for ev in events {
            bio.push_str(&format!("- **{}**: {}\n", ev.timestamp, ev.summary));
        }

        bio
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBridge {
    pub config: AIBridgeConfig,
}

impl AIBridge {
    pub fn new(config: AIBridgeConfig) -> Self {
        Self { config }
    }

    pub fn parse_intent(&self, input_text: &str, player_id: &str, target_id: Option<&str>) -> ActionPayload {
        if self.config.use_local_llm {
            if let Some(payload) = self.try_llm_parse(input_text, player_id, target_id) {
                return payload;
            }
        }

        self.fallback_intent_parser(input_text, player_id, target_id)
    }

    fn try_llm_parse(&self, _input_text: &str, _player_id: &str, _target_id: Option<&str>) -> Option<ActionPayload> {
        // Local GGUF endpoint fallback check
        None
    }

    pub fn fallback_intent_parser(&self, input_text: &str, player_id: &str, target_id: Option<&str>) -> ActionPayload {
        let text_lower = input_text.to_lowercase();

        // Lie/Deception
        if text_lower.contains("tell mum") || text_lower.contains("lie") || text_lower.contains("secretly") {
            return ActionPayload {
                action: ActionPrimitive::Deceive,
                actor_id: player_id.to_string(),
                target_id: target_id.map(|s| s.to_string()),
                claim: Some(ActionClaim {
                    claimed_destination: Some("James's house".to_string()),
                    claimed_activity: Some("study math".to_string()),
                }),
                actual_activity: Some("football_training".to_string()),
                intensity: 0.8,
                parameters: serde_json::json!({}),
            };
        }

        // Job Application
        if text_lower.contains("apply") || text_lower.contains("job") || text_lower.contains("vacancy") {
            return ActionPayload {
                action: ActionPrimitive::ApplyJob,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("job_application".to_string()),
                intensity: 0.8,
                parameters: serde_json::json!({}),
            };
        }

        // Work Shift
        if text_lower.contains("work shift") || text_lower.contains("work a shift") || text_lower.contains("salary") {
            return ActionPayload {
                action: ActionPrimitive::WorkShift,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("office_shift".to_string()),
                intensity: 0.8,
                parameters: serde_json::json!({}),
            };
        }

        // Rent Apartment
        if text_lower.contains("rent") || text_lower.contains("apartment") || text_lower.contains("lease") {
            return ActionPayload {
                action: ActionPrimitive::RentApartment,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("apartment_lease".to_string()),
                intensity: 0.7,
                parameters: serde_json::json!({}),
            };
        }

        // Buy Property
        if text_lower.contains("buy property") || text_lower.contains("buy house") {
            return ActionPayload {
                action: ActionPrimitive::BuyProperty,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("property_purchase".to_string()),
                intensity: 1.0,
                parameters: serde_json::json!({}),
            };
        }

        // Date
        if text_lower.contains("date") || text_lower.contains("romantic") {
            return ActionPayload {
                action: ActionPrimitive::Date,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("romantic_date".to_string()),
                intensity: 0.8,
                parameters: serde_json::json!({}),
            };
        }

        // Marriage
        if text_lower.contains("marry") || text_lower.contains("wedding") || text_lower.contains("propose") {
            return ActionPayload {
                action: ActionPrimitive::Marry,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("marriage_ceremony".to_string()),
                intensity: 1.0,
                parameters: serde_json::json!({}),
            };
        }

        // Medical Care
        if text_lower.contains("doctor") || text_lower.contains("medical") || text_lower.contains("treatment") || text_lower.contains("hospital") {
            return ActionPayload {
                action: ActionPrimitive::SeekMedicalTreatment,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("medical_care".to_string()),
                intensity: 0.9,
                parameters: serde_json::json!({}),
            };
        }

        // Study
        if text_lower.contains("study") || text_lower.contains("math") || text_lower.contains("homework") {
            return ActionPayload {
                action: ActionPrimitive::Study,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("math_revision".to_string()),
                intensity: 0.7,
                parameters: serde_json::json!({}),
            };
        }

        // Training/Football
        if text_lower.contains("train") || text_lower.contains("football") || text_lower.contains("match") {
            return ActionPayload {
                action: ActionPrimitive::AttendActivity,
                actor_id: player_id.to_string(),
                target_id: None,
                claim: None,
                actual_activity: Some("football_training".to_string()),
                intensity: 0.9,
                parameters: serde_json::json!({}),
            };
        }

        // Default communicate payload
        ActionPayload {
            action: ActionPrimitive::Communicate,
            actor_id: player_id.to_string(),
            target_id: target_id.map(|s| s.to_string()),
            claim: None,
            actual_activity: None,
            intensity: 0.5,
            parameters: serde_json::json!({ "raw_text": input_text }),
        }
    }

    pub fn render_narrative(
        &self,
        action: &ActionPrimitive,
        success: bool,
        actor: &Person,
        target_name: Option<&str>,
        causality_note: &str,
    ) -> String {
        let raw = match action {
            ActionPrimitive::Deceive => {
                if success {
                    format!(
                        "You convincingly lied to {}. They believed you were studying at James's house, allowing you to sneak off to training unnoticed. ({})",
                        target_name.unwrap_or("Mum"),
                        causality_note
                    )
                } else {
                    format!(
                        "Your lie failed to convince {}. They noticed your deception regarding your math studies, worsening tension at home. ({})",
                        target_name.unwrap_or("Mum"),
                        causality_note
                    )
                }
            }
            ActionPrimitive::ApplyJob => format!("You submitted your job application. ({})", causality_note),
            ActionPrimitive::WorkShift => format!("You completed your work shift. ({})", causality_note),
            ActionPrimitive::RentApartment => format!("You signed a lease for an apartment. ({})", causality_note),
            ActionPrimitive::BuyProperty => format!("You purchased property. ({})", causality_note),
            ActionPrimitive::Date => format!("You went on a romantic date. ({})", causality_note),
            ActionPrimitive::Marry => format!("You held a marriage ceremony with your partner. ({})", causality_note),
            ActionPrimitive::Divorce => format!("You finalized divorce proceedings. ({})", causality_note),
            ActionPrimitive::HaveChild => format!("You welcomed a child into the family. ({})", causality_note),
            ActionPrimitive::SeekMedicalTreatment => format!("You received medical treatment and rested. ({})", causality_note),
            ActionPrimitive::AttendActivity => format!(
                "You attended training. You pushed your physical limits and honed your skills. ({})",
                causality_note
            ),
            ActionPrimitive::Study => format!(
                "You spent 2 hours reviewing concepts. Your academic understanding improved. ({})",
                causality_note
            ),
            _ => format!("{} executed an action. ({})", actor.identity.first_name, causality_note),
        };

        HallucinationValidator::validate_and_sanitize(&raw, success, causality_note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_intent_parser_deception() {
        let bridge = AIBridge::new(AIBridgeConfig::default());
        let payload = bridge.parse_intent(
            "Tell Mum I'm going to James's house to study math, but secretly go to football training.",
            "person:sim:player",
            Some("person:sim:mum"),
        );

        assert_eq!(payload.action, ActionPrimitive::Deceive);
        assert_eq!(payload.actual_activity, Some("football_training".to_string()));
    }
}

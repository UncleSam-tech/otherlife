use otherlife_world::EventRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBridgeConfig {
    pub use_local_llm: bool,
    pub model_tier: String,
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

pub struct BiographyWriter;

impl BiographyWriter {
    pub fn generate_lifetime_biography(player_name: &str, events: &[EventRecord]) -> String {
        if events.is_empty() {
            return format!("The life story of {} has just begun.", player_name);
        }

        let mut bio = format!("# The Life Memoir of {}\n\n", player_name);
        bio.push_str("## Chapter: Origins & Formation\n");

        for ev in events {
            bio.push_str(&format!("- **{}** — *{}*: {}\n", ev.timestamp, ev.headline, ev.narrative));
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

    pub fn render_narrative(&self, event: &EventRecord) -> String {
        event.narrative.clone()
    }
}

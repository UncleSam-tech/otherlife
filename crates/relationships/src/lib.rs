use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipVector {
    pub trust: f32,      // 0.0 to 1.0
    pub affection: f32,  // 0.0 to 1.0
    pub respect: f32,    // 0.0 to 1.0
    pub resentment: f32, // 0.0 to 1.0
}

impl Default for RelationshipVector {
    fn default() -> Self {
        Self {
            trust: 0.5,
            affection: 0.5,
            respect: 0.5,
            resentment: 0.0,
        }
    }
}

impl RelationshipVector {
    pub fn new_parent_child() -> Self {
        Self {
            trust: 0.75,
            affection: 0.85,
            respect: 0.70,
            resentment: 0.05,
        }
    }

    pub fn new_teacher_student() -> Self {
        Self {
            trust: 0.60,
            affection: 0.50,
            respect: 0.75,
            resentment: 0.0,
        }
    }

    pub fn new_classmate_peer() -> Self {
        Self {
            trust: 0.55,
            affection: 0.60,
            respect: 0.50,
            resentment: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipMatrix {
    // Key: (SourceID, TargetID)
    pub links: HashMap<(String, String), RelationshipVector>,
}

impl RelationshipMatrix {
    pub fn new() -> Self {
        Self { links: HashMap::new() }
    }

    pub fn set_link(&mut self, source: String, target: String, rel: RelationshipVector) {
        self.links.insert((source, target), rel);
    }

    pub fn get_link(&self, source: &str, target: &str) -> RelationshipVector {
        self.links
            .get(&(source.to_string(), target.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    pub fn modify_link<F>(&mut self, source: &str, target: &str, mut mutator: F)
    where
        F: FnMut(&mut RelationshipVector),
    {
        let entry = self
            .links
            .entry((source.to_string(), target.to_string()))
            .or_insert_with(RelationshipVector::default);
        mutator(entry);
        entry.trust = entry.trust.clamp(0.0, 1.0);
        entry.affection = entry.affection.clamp(0.0, 1.0);
        entry.respect = entry.respect.clamp(0.0, 1.0);
        entry.resentment = entry.resentment.clamp(0.0, 1.0);
    }
}

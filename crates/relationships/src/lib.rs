use otherlife_world::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipVector {
    pub affection: f32,
    pub trust: f32,
    pub respect: f32,
    pub fear: f32,
    pub attraction: f32,
    pub resentment: f32,
    pub loyalty: f32,
    pub familiarity: f32,
    pub dependency: f32,
    pub admiration: f32,
}

impl Default for RelationshipVector {
    fn default() -> Self {
        Self {
            affection: 0.5,
            trust: 0.5,
            respect: 0.5,
            fear: 0.0,
            attraction: 0.0,
            resentment: 0.0,
            loyalty: 0.5,
            familiarity: 0.5,
            dependency: 0.0,
            admiration: 0.5,
        }
    }
}

impl RelationshipVector {
    pub fn new_parent_child() -> Self {
        Self {
            affection: 0.85,
            trust: 0.75,
            respect: 0.70,
            fear: 0.10,
            attraction: 0.0,
            resentment: 0.15,
            loyalty: 0.90,
            familiarity: 0.95,
            dependency: 0.80,
            admiration: 0.75,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipMatrix {
    // Key: (SourceID, TargetID)
    pub links: HashMap<(EntityId, EntityId), RelationshipVector>,
}

impl RelationshipMatrix {
    pub fn new() -> Self {
        Self { links: HashMap::new() }
    }

    pub fn set_link(&mut self, source: EntityId, target: EntityId, rel: RelationshipVector) {
        self.links.insert((source, target), rel);
    }

    pub fn get_link(&self, source: &EntityId, target: &EntityId) -> RelationshipVector {
        self.links
            .get(&(source.clone(), target.clone()))
            .cloned()
            .unwrap_or_default()
    }

    pub fn modify_link<F>(&mut self, source: EntityId, target: EntityId, mut mutator: F)
    where
        F: FnMut(&mut RelationshipVector),
    {
        let entry = self
            .links
            .entry((source, target))
            .or_insert_with(RelationshipVector::default);
        mutator(entry);
        entry.affection = entry.affection.clamp(0.0, 1.0);
        entry.trust = entry.trust.clamp(0.0, 1.0);
        entry.respect = entry.respect.clamp(0.0, 1.0);
        entry.resentment = entry.resentment.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asymmetric_relationships() {
        let mut matrix = RelationshipMatrix::new();
        let parent = "person:sim:mum".to_string();
        let child = "person:sim:player".to_string();

        matrix.set_link(parent.clone(), child.clone(), RelationshipVector::new_parent_child());
        matrix.modify_link(parent.clone(), child.clone(), |rel| {
            rel.trust -= 0.10;
            rel.resentment += 0.08;
        });

        let rel = matrix.get_link(&parent, &child);
        assert_eq!(rel.trust, 0.65);
        assert_eq!(rel.resentment, 0.23);
    }
}

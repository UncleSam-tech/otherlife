# OTHERLIFE — System Architecture Specification

## 1. High-Level Topology

OTHERLIFE is built as a hybrid Rust/TypeScript desktop application using **Tauri 2**.

```text
┌────────────────────────────────────────────────────────────────────────┐
│                          TAURI 2 DESKTOP SHELL                         │
│                                                                        │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │                 React 18 / TypeScript Frontend                 │   │
│   │   • Editorial Life OS UI           • Life Feed Component       │   │
│   │   • NOW Context Sidebar            • Free Action Prompt Input  │   │
│   │   • Causality Inspector            • Visual Design Token Engine│   │
│   └────────────────────────────────┬───────────────────────────────┘   │
│                                    │ IPC (Tauri Commands / Events)     │
│   ┌────────────────────────────────▼───────────────────────────────┐   │
│   │                   Rust Simulation Backend Kernel               │   │
│   │                                                                │   │
│   │   crates/rng          -> Deterministic Seeded RNG              │   │
│   │   crates/world        -> Canonical Entity & Component Engine   │   │
│   │   crates/relationships-> 10D Directional Relationship Engine   │   │
│   │   crates/actions      -> Universal Action Ontology & Validator │   │
│   │   crates/persistence  -> SQLite Database & Save State Engine   │   │
│   │   crates/ai_bridge    -> Local LLM GGUF / HTTP / Fallback      │   │
│   │   crates/simulation   -> Tick Loop, Event Queue & Causality    │   │
│   └────────────────────────────────┬───────────────────────────────┘   │
│                                    │                                   │
│   ┌────────────────────────────────▼───────────────────────────────┐   │
│   │                   Local Persistence & AI Runtime              │   │
│   │   • SQLite Database (`world.sqlite`)                               │   │
│   │   • Local `llama.cpp` / GGUF engine or Template Engine        │   │
│   └────────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Processing Pipeline

Every action initiated by the player passes through a strict 10-phase pipeline (Spec §8):

```text
1. PLAYER INTENTION      -> Free-text input or menu shortcut
2. INTENT INTERPRETER    -> AI Bridge converts text to structured Action payload
3. STRUCTURED ACTION     -> Strongly typed Action primitive (e.g., DECEIVE, ATTEND_ACTIVITY)
4. VALIDATION            -> Actions crate validates eligibility, prerequisites, and bounds
5. SIMULATION SYSTEMS    -> Engine calculates success, skills XP, relationship updates, state shifts
6. NPC / WORLD RESPONSE  -> Affected NPCs process reaction based on personality, trust, memory
7. CONSEQUENCES          -> Immediate and scheduled future events pushed to Event Queue
8. DATABASE TRANSACTION  -> SQLite transaction commits atomic state mutation & event log
9. NARRATIVE GENERATION  -> AI Bridge / Template fallback renders prose & dialogue response
10. PLAYER INTERFACE     -> React UI updates Life Feed, NOW sidebar, and state metrics
```

---

## 3. Crate Architecture & Responsibilities

| Crate | Responsibilities | Key Types / Modules |
| :--- | :--- | :--- |
| `crates/rng` | Deterministic random number generation | `WorldRng`, `SeedSequence`, `ProbabilityRoll` |
| `crates/world` | Entity Component System (ECS) style schemas | `Person`, `Organization`, `Place`, `Role`, `Event`, `Time`, `Knowledge`, `Memory` |
| `crates/relationships` | Directional, 10-dimensional social metrics | `RelationshipMatrix`, `SocialDimension`, `TrustModifier` |
| `crates/actions` | Action primitives, parameter validation | `ActionType`, `ActionPayload`, `ActionPrerequisites`, `ValidationResult` |
| `crates/persistence` | SQLite migrations, save/load, timeline branching | `DatabasePool`, `SaveMetadata`, `EntityRepository`, `TimelineBranch` |
| `crates/ai_bridge` | Local LLM GGUF interface & template fallback | `LlamaEngine`, `IntentParser`, `NarrativeRenderer`, `TemplateFallback` |
| `crates/simulation` | Tick loop, scheduler, causality tracing | `SimulationEngine`, `EventQueue`, `CausalityGraph`, `OpportunityEngine` |

---

## 4. Database Architecture & Schema

Each save game is self-contained within a folder containing `world.sqlite` and `metadata.json`.

### Core Tables
- `entities` (`entity_id`, `namespace`, `entity_type`, `created_at`)
- `persons` (`person_id`, `first_name`, `last_name`, `birth_date`, `biological_sex`, `health_status`, `json_components`)
- `organizations` (`org_id`, `name`, `org_type`, `parent_org_id`, `headquarters_place_id`, `json_data`)
- `relationships` (`source_person_id`, `target_person_id`, `affection`, `trust`, `respect`, `fear`, `attraction`, `resentment`, `loyalty`, `familiarity`, `dependency`, `admiration`)
- `events` (`event_id`, `timestamp`, `event_type`, `actor_id`, `location_id`, `json_metadata`, `causality_parent_id`)
- `scheduled_events` (`schedule_id`, `target_timestamp`, `event_payload`, `status`)
- `knowledge` (`person_id`, `fact_key`, `fact_value`, `acquired_timestamp`, `certainty`)
- `memories` (`memory_id`, `person_id`, `importance`, `emotion`, `summary`, `timestamp`)

---

## 5. Simulation Tiers & Performance Optimization

To maintain 60 FPS UI performance and fast simulation ticks, population detail is split into 3 tiers:
- **Tier A (Active Social Circle)**: ~20-50 entities (family, close friends, manager, teammates, rivals). Fully simulated every tick.
- **Tier B (Contextual Population)**: ~500-2,000 entities (classmates, coworkers, local club players). Simulated daily/weekly.
- **Tier C (Statistical Aggregates)**: Millions of virtual population entities managed via macro demographic distributions. When an entity becomes relevant (e.g. player meets a stranger), a Tier B/A entity is deterministically instantiated from the aggregate.

# OTHERLIFE — Simulation Engine & Rules Specification

## 1. Primary Entity Classes

All world concepts are modeled as structured entities with canonical IDs:

```text
Person         -> Human beings (seeded real or generated sim)
Place          -> Geographical nodes (Country, Region, City, District, Location)
Organization   -> Social structures (Family, School, University, Club, Company, Party)
Role           -> Positions within organizations (Student, Academy Player, CEO, Prime Minister)
Event          -> Timed historical occurrences with causal links
Relationship   -> Directional 10-dimensional social vectors between two Persons
Asset          -> Properties, cash, contracts, vehicles, shares
Contract       -> Employment, transfer, loan, scholarship agreements
Skill          -> Transferable human abilities (0.0 to 100.0)
Condition      -> Medical, physical, or psychological states
Knowledge      -> Fact assertions known by specific entities
Memory         -> NPC recall records with emotional weight and decay rates
```

---

## 2. Person Component Architecture

A `Person` instance contains the following modular components:

```rust
pub struct Person {
    pub id: EntityId,
    pub identity: IdentityComponent,
    pub demographics: DemographicsComponent,
    pub biology: BiologyComponent,
    pub personality: PersonalityComponent, // Big Five (OCEAN) traits
    pub skills: HashMap<SkillId, f32>,     // Transferable skills
    pub education: EducationComponent,
    pub employment: EmploymentComponent,
    pub finances: FinancesComponent,
    pub health: HealthComponent,
    pub fame: FameComponent,               // Local, national, global per domain
    pub legal_status: LegalStatusComponent,
    pub knowledge: HashSet<FactId>,
    pub memories: Vec<MemoryRecord>,
    pub location_id: EntityId,
}
```

---

## 3. Directional 10-Dimensional Relationship Model

Relationships are **asymmetric** (Person A's feeling toward Person B is distinct from B's toward A).

Each relationship direction is defined by 10 continuous metrics `[0.0, 1.0]`:

```rust
pub struct RelationshipVector {
    pub affection: f32,    // Warmth and liking
    pub trust: f32,        // Reliability and truth belief
    pub respect: f32,      // Admiration of capability
    pub fear: f32,         // Apprehension of power or threat
    pub attraction: f32,   // Romantic or personal magnetic interest
    pub resentment: f32,   // Bitterness over past transgressions
    pub loyalty: f32,      // Commitment to support across conflicts
    pub familiarity: f32,  // Depth of shared experience
    pub dependency: f32,   // Reliance for needs/resources
    pub admiration: f32,   // Inspiration drawn from character
}
```

---

## 4. Knowledge, Secrets & Truth Separation

World Truth is strictly isolated from individual NPC Knowledge.

```text
[WORLD TRUTH]: player_cheated_on_math_exam = true

[PLAYER KNOWLEDGE]: knows = true
[TEACHER KNOWLEDGE]: knows = false
[MOTHER KNOWLEDGE]: knows = false
```

- **Knowledge Propagation**: An NPC only learns a fact through direct observation, official notification, communication from another character, or media reports.
- **Deception Mechanics**: When a player lies to an NPC, the system rolls `Persuasion(Player) + Trust(NPC->Player)` vs `Skepticism(NPC) + EvidenceCount`. If successful, the NPC stores a false belief in their Knowledge repository.

---

## 5. Memory System & Summarization

Memories retain specific details based on:
$$\text{Memory Strength} = \text{Importance} \times \text{Emotional Intensity} \times e^{-\lambda \cdot \text{AgeDays}}$$

- **High-Importance Memories** (e.g. parent discovering a lie about training, scout watching a match) persist permanently or decay into long-term summaries.
- **Low-Importance Memories** (e.g. what was eaten for breakfast 3 weeks ago) automatically decay and purge during monthly tick cleanup.

---

## 6. Opportunity Engine Formula

Opportunities are never drawn from arbitrary cards. They emerge when conditions align:

$$\text{Opportunity Probability} = P(\text{World Need}) \times P(\text{Player Eligibility}) \times P(\text{Visibility}) \times P(\text{Network Bridge}) \times P(\text{Random Chance})$$

- **Example**: Celtic FC academy trial opportunity appears because:
  1. Celtic youth academy needs a left-winger (`World Need`).
  2. Player is 14 years old and plays in Glasgow region (`Player Eligibility`).
  3. Regional scout attended Saturday match (`Visibility`).
  4. Youth coach recommended player (`Network Bridge`).

---

## 7. Causality Graph & "Why Did This Happen?"

Every event stores a `causality_parent_id` linking back to its root cause.

```text
Player receives Celtic Academy Trial offer
├── Caused by: Regional Scout positive report
│   ├── Caused by: Player scored 2 goals in Saturday match
│   └── Caused by: Player attended secret training session
└── Caused by: Youth Coach formal recommendation
```

Players can inspect this causality chain in the UI via the "Why Did This Happen?" feature.

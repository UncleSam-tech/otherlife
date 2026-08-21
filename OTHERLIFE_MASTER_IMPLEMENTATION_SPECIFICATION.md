# OTHERLIFE — MASTER IMPLEMENTATION SPECIFICATION
### The Single, Authoritative Source of Truth for Architecture, Simulation Laws, and Engine Rebuild
**Version:** 2.0.0 (Master Execution)  
**Status:** Approved Master Specification — Single Source of Truth  

---

# 1. PRODUCT VISION & CORE DIRECTIVE

### 1.1 The Primary Directive
> **"OTHERLIFE is a living alternate reality human simulation where the player experiences an entire human life through decisions, processes, relationships, opportunities, failures, and consequences."**

The simulation engine operates on a single, uncompromising causal hierarchy:
```
Human  →  World  →  Situations  →  Natural Intentions  →  Simulation Engine  →  Consequences  →  Emergent Life Story
```

### 1.2 Non-Negotiable Boundaries:
- **NOT a dashboard**: No spreadsheets of statistics, no disconnected tabs, no artificial skill bars.
- **NOT a menu simulator**: No selecting from predefined action buttons or fixed choice cards as primary gameplay.
- **NOT a career-selection game**: No "Football Mode", "Influencer Mode", "Politics Mode", or "Corporate Mode".
- **NOT an unconstrained chatbot**: The LLM never decides simulation facts, outcomes, money, or state transitions.
- **NOT a linear story**: No predetermined destinies, no plot armor, no automatic successes.

---

# 2. SIMULATION CONSTITUTION & LAWS OF THE UNIVERSE

### 2.1 Time Simulation Laws
1. **Continuous & Causal Time Flow**: Time flows in proportion to player intentions, conversations, daily habits, and projects.
   - *Immediate Action / Conversation*: Advances hours to 1 day.
   - *Habit / Routine*: *"I study mathematics every evening"* or *"I train with the youth squad three times a week"* advances weeks to months.
   - *Multi-Phase Project*: *"I prepare my university portfolio"* advances across term milestones.
2. **Autonomous World Progression**: During time progression $\Delta t$, all entities age, NPC schedules execute, skills grow or decay, processes advance, deadlines decrement, and macro-economic factors shift.

### 2.2 Human Development & Life Stage Laws
Biological and cognitive age strictly dictates human agency, legal permissions, and societal expectations:

| Life Stage | Age Bracket | Spheres of Agency | Hard-Gated Systems |
| :--- | :--- | :--- | :--- |
| **Infancy** | **0 – 3** | Sensory exploration, motor coordination, early speech, parental bonding, home routine. | No employment, finances, romance, or independent transit. |
| **Childhood** | **4 – 12** | Primary schooling, playground athletics, library discovery, household chores, parental allowance. | No full-time jobs, marital inquiries, banking/credit, or migration. |
| **Adolescence** | **13 – 17** | Secondary education, national examinations, youth club trials, creative media, peer groups, part-time chores. | No adult contracts, commercial mortgages, or marriage. |
| **Early Adulthood** | **18 – 29** | Higher education, vocational trades, independent flat rental, corporate employment, enterprise, romance/marriage, civic action. | Gated by prerequisite qualifications, capital, and legal documents. |
| **Adulthood & Prime**| **30 – 64** | Professional scaling, executive leadership, home ownership, family rearing, business investments, political office. | Athletic physical peak declines; social and financial leverage peak. |
| **Senior Years** | **65+** | Mentorship, board advisory, memoirs, civic philanthropy, estate planning. | Physical stamina limits manual labor; accumulated wisdom peaks. |

### 2.3 Skill Development Laws
1. **Deliberate Practice Formula**: Skill mastery $S$ develops through deliberate practice:
   $$\Delta S = \text{Consistency} \times (\text{Coaching Quality} + \text{Tool Quality}) \times \text{Natural Affinity} \times \text{Energy Level} - \text{Decay}$$
2. **Organic Perception**: The world never sees numerical stats (e.g. `Football: 78/100`). The world observes demonstrable outcomes and reputation (e.g. *"Scouts noted exceptional close control under defensive pressure"*).
3. **Skill Decay**: Specialized high-performance skills unpracticed for $>180\text{ days}$ gradually lose sharpness until training is resumed.

### 2.4 Relationship Laws
1. **Directional Multi-Vector Social Graph**: Every relationship between Person $A$ and Person $B$ is asymmetric and tracks:
   - **Trust** ($[0.0, 1.0]$): Reliability, honesty, and consistency.
   - **Affection** ($[0.0, 1.0]$): Emotional warmth, familial love, or romantic fondness.
   - **Respect** ($[0.0, 1.0]$): Acknowledgment of competence, discipline, and character.
   - **Resentment** ($[0.0, 1.0]$): Accumulated hurt from broken trust or neglect.
2. **Parental & Household Harmony**: In childhood and youth, family cooperation and honesty directly govern parental trust, household atmosphere, and pocket money allowances.
3. **Natural Social Drift**: Non-family friendships without interaction for $>1\text{ year}$ naturally drift to acquaintances.

### 2.5 NPC Autonomy Laws
1. **Independent Life Routines**: NPCs have their own careers, families, daily schedules, and private ambitions. They do not exist to wait on the player.
2. **Subjective Perception & Memory**: NPCs form their own memories and biases regarding encounters with the player.

### 2.6 Opportunity Generation Laws
1. **Causal Preconditions**: Opportunities are never random spawns. Every opportunity $O$ is generated causally:
   $$\text{Opportunity} = f(\text{Age}, \text{Location}, \text{Prerequisite Skills}, \text{Social Network / Visibility}, \text{Reputation}, \text{Institutional Timing})$$
2. **Organic Visibility**: Opportunities appear through notices, letters, word-of-mouth recommendations, or direct scouts.

### 2.7 Process & Waiting Laws
1. **Universal Multi-Stage Latency**: Important life transitions require realistic steps:
   $$\text{Opportunity} \longrightarrow \text{Discovery} \longrightarrow \text{Requirements} \longrightarrow \text{Preparation} \longrightarrow \text{Application / Trial} \longrightarrow \text{Waiting Period} \longrightarrow \text{Evaluation} \longrightarrow \text{Outcome} \longrightarrow \text{Consequences}$$
2. **Applies Universally**: Powers university admissions, employment applications, sports trials, business incorporation, and visa approvals.

### 2.8 Failure, Recovery & Success Laws
1. **Failure as Character Growth**: Rejection, athletic injury, and business insolvency are catalysts for character divergence and life pivots (e.g. from player to youth academy coach).
2. **Success Requires Alignment**: Success is the non-deterministic alignment of technical mastery, emotional resilience, social mentorship, and timing.

### 2.9 Event Generation Laws
1. **Independent External Reality**: The world generates unsolicited events originating from family (job transfers, sibling births), NPCs (teachers leaving, friend invitations), institutions (syllabus shifts), macro-economics (inflation, industrial booms), and weather (Harmattan, winter storms).

### 2.10 Information, Knowledge & Perception Laws
1. **World Truth vs. Player Knowledge**: The simulation strictly isolates objective reality (hidden NPC intentions, unannounced scout lists, market trends) from player awareness (observations, direct dialogue, letters).
2. **Realistic Uncertainty**: The interface never displays raw mathematical win probabilities or hidden relationship numbers.

---

# 3. DATABASE SCHEMA (CANONICAL SQLITE DDL)

```sql
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;

-- 1. Human Entities (Player & Autonomous NPCs)
CREATE TABLE IF NOT EXISTS persons (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    birth_year INTEGER NOT NULL,
    birth_month INTEGER NOT NULL,
    birth_day INTEGER NOT NULL,
    sex TEXT NOT NULL,
    birthplace_id TEXT NOT NULL,
    current_location_id TEXT NOT NULL,
    is_alive INTEGER NOT NULL DEFAULT 1,
    death_year INTEGER,
    death_reason TEXT,
    
    -- Biology & Health (0.0 to 100.0)
    health_overall REAL NOT NULL DEFAULT 95.0,
    health_fitness REAL NOT NULL DEFAULT 70.0,
    health_stress REAL NOT NULL DEFAULT 15.0,
    energy_level REAL NOT NULL DEFAULT 85.0,
    
    -- Psychological Temperament (0.0 to 1.0)
    discipline REAL NOT NULL DEFAULT 0.50,
    curiosity REAL NOT NULL DEFAULT 0.60,
    creativity REAL NOT NULL DEFAULT 0.50,
    confidence REAL NOT NULL DEFAULT 0.50,
    risk_tolerance REAL NOT NULL DEFAULT 0.40,
    
    -- Financial Resources & Wealth Tier
    cash REAL NOT NULL DEFAULT 0.0,
    household_income_tier TEXT NOT NULL DEFAULT 'MIDDLE', -- 'POOR', 'WORKING', 'MIDDLE', 'UPPER_MIDDLE', 'WEALTHY'
    
    -- Current Occupation
    occupation_title TEXT,
    employer_org_id TEXT,
    monthly_earnings REAL DEFAULT 0.0,
    
    is_player INTEGER NOT NULL DEFAULT 0
);

-- 2. Emergent Human Skills
CREATE TABLE IF NOT EXISTS person_skills (
    person_id TEXT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    skill_id TEXT NOT NULL,
    mastery_level REAL NOT NULL DEFAULT 0.0,
    experience_points REAL NOT NULL DEFAULT 0.0,
    natural_affinity REAL NOT NULL DEFAULT 1.0,
    last_practiced_day INTEGER NOT NULL,
    PRIMARY KEY (person_id, skill_id)
);

-- 3. Human Relationships & Social Graph
CREATE TABLE IF NOT EXISTS relationships (
    source_person_id TEXT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    target_person_id TEXT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL,
    trust REAL NOT NULL DEFAULT 0.5,
    affection REAL NOT NULL DEFAULT 0.5,
    respect REAL NOT NULL DEFAULT 0.5,
    resentment REAL NOT NULL DEFAULT 0.0,
    is_active INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (source_person_id, target_person_id)
);

-- 4. World Geography & Places
CREATE TABLE IF NOT EXISTS world_places (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    place_type TEXT NOT NULL, -- 'COUNTRY', 'REGION', 'CITY', 'DISTRICT', 'PLACE'
    parent_place_id TEXT REFERENCES world_places(id),
    country_id TEXT NOT NULL,
    climate_zone TEXT NOT NULL,
    cost_of_living_index REAL NOT NULL DEFAULT 1.0,
    culture_tags_json TEXT NOT NULL
);

-- 5. Institutions & Organizations
CREATE TABLE IF NOT EXISTS institutions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    institution_type TEXT NOT NULL, -- 'PRIMARY_SCHOOL', 'SECONDARY_SCHOOL', 'UNIVERSITY', 'FOOTBALL_CLUB', 'COMPANY', 'HOSPITAL', 'GOVERNMENT'
    location_id TEXT NOT NULL REFERENCES world_places(id),
    prestige REAL NOT NULL DEFAULT 0.5,
    admission_criteria_json TEXT NOT NULL,
    metadata_json TEXT
);

-- 6. Universal Life Processes
CREATE TABLE IF NOT EXISTS life_processes (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    process_type TEXT NOT NULL,
    title TEXT NOT NULL,
    institution_id TEXT REFERENCES institutions(id),
    current_step INTEGER NOT NULL DEFAULT 0,
    total_steps INTEGER NOT NULL,
    target_completion_day INTEGER NOT NULL,
    requirements_met INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    state_payload_json TEXT NOT NULL
);

-- 7. Player Discovered Knowledge
CREATE TABLE IF NOT EXISTS player_knowledge (
    id TEXT PRIMARY KEY,
    player_id TEXT NOT NULL REFERENCES persons(id),
    topic_id TEXT NOT NULL,
    knowledge_type TEXT NOT NULL,
    discovered_day INTEGER NOT NULL,
    source_description TEXT NOT NULL,
    factual_payload_json TEXT NOT NULL
);

-- 8. Episodic Life Memories & Chronicle
CREATE TABLE IF NOT EXISTS life_memories (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    day_total INTEGER NOT NULL,
    calendar_timestamp TEXT NOT NULL,
    event_type TEXT NOT NULL,
    headline TEXT NOT NULL,
    narrative_prose TEXT NOT NULL,
    emotional_impact REAL NOT NULL,
    related_person_id TEXT REFERENCES persons(id),
    related_institution_id TEXT REFERENCES institutions(id),
    causal_explanation TEXT NOT NULL
);
```

---

# 4. SIMULATION ENGINE TOPOLOGY (RUST CRATES)

```
crates/
├── world/          -> Core schemas: HumanEntity, BiologicalProfile, PsychologicalProfile, SkillMastery, WorldPlace, InstitutionEntity, AutonomousNPC
├── rng/            -> Deterministic PCG64 PRNG and distribution samplers
├── persistence/    -> SQLite transactional persistence, migration codec, save snapshotting
├── events/         -> Event Generation Engine, base-rate tables, causal event dispatchers
├── ai_bridge/      -> Deterministic Context Builder, LLM narrative generator, hallucination validator
├── simulation/     -> Master simulation kernel: tick engine, intent parser, universal process engine, NPC network
└── desktop_lib/    -> Shared DTOs, IPC command serialization, client-server bridge
```

---

# 5. USER INTERFACE ARCHITECTURE (THREE-COLUMN LIVING STAGE)

```text
┌─────────────────┬──────────────────────────────────────────────┬─────────────────┐
│  LEFT NAVIGATION│                 CENTER STAGE                 │  RIGHT CONTEXT  │
│                 │                                              │                 │
│  • Life         │   AGE 16 · LAGOS, NIGERIA · MONDAY MORNING   │  • People       │
│  • People       │                                              │    Around You   │
│  • Places       │   Atmospheric morning prose detailing what   │                 │
│  • Messages     │   family and classmates are saying...        │  • Active       │
│  • Journal      │                                              │    Matters &    │
│  • World        │   "What do you do?"                          │    Processes    │
│  • Biography    │   ┌────────────────────────────────────────┐ │                 │
│                 │   │ Express natural intention, habit, plan.│ │  • State &      │
│                 │   └────────────────────────────────────────┘ │    Memories     │
└─────────────────┴──────────────────────────────────────────────┴─────────────────┘
```

1. **Left Navigation (`LeftNav.tsx`)**: Permanent life lenses (`Life`, `People`, `Places`, `Messages`, `Journal`, `World`, `Biography`).
2. **Center Stage (`CenterLivingStage.tsx`)**: Stage banner (Age, Location, Date), atmospheric morning prose, and the open **Natural Intention Bar** (`ActionPromptBar.tsx`).
3. **Right Context Sidebar (`RightContextPanel.tsx`)**: Surrounding NPCs, active life processes & deadlines, physical energy/stress, recent defining memories.

---

# 6. ACCEPTANCE TEST & VERIFICATION

The rebuild is verified through the **Birth to Age 18 Vertical Slice in Abuja, Nigeria**:
- Starts at **Age 0** with family background in Garki, Abuja.
- Passes through Infancy (0–3), Primary School (4–12), and Adolescence (13–18).
- Zero static action cards, zero career modes, zero manual $+1\text{ month}$ buttons.
- All decisions execute through the **Natural Intention Engine** with deterministic causal consequences.

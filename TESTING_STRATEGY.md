# OTHERLIFE — Automated & Chaos Testing Strategy

## 1. Multi-Tiered Testing Philosophy

OTHERLIFE combines deterministic unit tests, headless macro-life simulation runners, chaos testing, saveload fidelity checks, and AI output validation.

```text
┌────────────────────────────────────────────────────────────┐
│                    TESTING STRATEGY                        │
├────────────────────────────┬───────────────────────────────┤
│ Cargo Workspace Unit Tests │ Subsystem correctness         │
│ Headless Life Simulator    │ Macro demographic stability   │
│ Chaos Random Action Runner │ State mutation invariants     │
│ Save/Load Fidelity Suite   │ SQLite roundtrip byte-parity  │
│ LLM Validator Assertions   │ Authoritative truth parity    │
└────────────────────────────┴───────────────────────────────┘
```

---

## 2. Headless Macro Life Simulation (§72)

Run 10,000 automated lives for 80 simulated years without UI to capture macro metric distributions:

### Monitored Metrics & Target Bounds
- **School Completion Rate**: 85% - 95%
- **Employment Rate**: 88% - 96%
- **Marriage Rate**: 50% - 70%
- **Football Academy Intake**: ~1.2% of eligible youth
- **Pro Football Reach**: ~0.05% of academy intake
- **Average Lifespan**: 76 - 84 years

---

## 3. Chaos Testing & Invariant Checks (§73)

The Chaos Test Suite injects randomized action payloads, corrupted inputs, and edge-case timing requests to ensure state invariants are never broken.

### Invariant Assertions:
1. **Death Lock**: `is_alive == false` entities cannot receive salaries, acquire roles, or marry.
2. **Age Monotonicity**: `age` cannot decrease, and birth dates must precede all event timestamps.
3. **Age Boundary Enforcement**: Minors (< 18) cannot hold mortgages, take corporate loans, or hold political office.
4. **Single Marital Contract**: A person cannot marry themselves or hold multiple active marriage contracts without prior divorce.
5. **Role Uniqueness**: Mutual exclusivity of incompatible roles (e.g. active player for two different clubs simultaneously).
6. **No Duplicate Membership**: An entity cannot exist twice in an organization's roster.

---

## 4. Save / Load Fidelity Verification (§80)

To guarantee state persistence integrity:
1. Initialize world with seed `W_SEED = 42`.
2. Run simulation tick loop for 100 turns.
3. Export database state snapshot $S_1$.
4. Save database to `world.sqlite`.
5. Reload application state $S_2$ from SQLite.
6. Assert $S_1 \equiv S_2$ across all entities, relationships, events, and RNG states.

---

## 5. AI Output Validation against Authoritative Truth (§74)

An automated LLM evaluator checks generated narrative against state facts:
- **Score Parity**: If match result is `Celtic 2 - 1 Rangers`, narrative cannot claim a Rangers victory.
- **Name Consistency**: LLM must use character first/last names matching entity IDs.
- **Death Parity**: LLM cannot render active dialogue for a deceased NPC.

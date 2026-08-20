# OTHERLIFE — Universal Action Ontology Specification

## 1. Universal Action Primitives

Rather than creating single-purpose hardcoded functions for every specific real-world activity, OTHERLIFE relies on **reusable action primitives**.

### Communication Primitives
- `COMMUNICATE`: General speech, dialogue, or messaging.
- `ASK`: Query character for information, permission, or decision.
- `ANSWER`: Respond to a query or demand.
- `PERSUADE`: Attempt to change a character's stance or opinion.
- `NEGOTIATE`: Propose trade-offs or contractual terms.
- `DECEIVE`: Provide false claims while concealing actual behavior.
- `THREATEN`: Apply coercive leverage using potential negative consequences.
- `APOLOGIZE`: Seek forgiveness or reduction of resentment.

### Movement & Spatial Primitives
- `MOVE`: Shift position within local area.
- `VISIT`: Spend time at a destination location or person's residence.
- `TRAVEL`: Inter-city or short-term long-distance transit.
- `MIGRATE`: Permanent relocation of residency.

### Education & Skill Primitives
- `STUDY`: Academic preparation or theoretical skill building.
- `LEARN`: Acquire specific knowledge or qualifications.
- `PRACTICE`: Solitary repetition to build skill points.
- `TRAIN`: Guided skill improvement under coaching/instruction.
- `TEACH`: Impart skills or knowledge to another person.

### Employment & Economic Primitives
- `APPLY`: Submit candidature for a role or academy position.
- `WORK`: Perform duties associated with an active role.
- `RESIGN`: Formally vacate a role or position.
- `HIRE`: Engage a person into an organizational role.
- `FIRE`: Terminate a person's employment role.
- `PROMOTE`: Elevate a person's rank within an organization.
- `BUY` / `SELL`: Exchange capital for assets, goods, or property.
- `BORROW` / `LEND`: Temporary transfer of funds or assets with debt terms.
- `INVEST`: Allocate capital into businesses or financial instruments.

### Competition & Activity Primitives
- `COMPETE`: Participate in an organized match, tournament, or election.
- `PERFORM`: Execute artistic, musical, or public display.
- `AUDITION`: Demonstrate capability for selection into a cast/team.
- `ATTEND_ACTIVITY`: Participate in scheduled event (training, exam, party).

### Relational Primitives
- `BEFRIEND`: Initiate or deepen social friendship bond.
- `DATE`: Engage in romantic courtship.
- `MARRY`: Enter formal marital contract.
- `DIVORCE`: Dissolve marital contract.
- `RECONCILE`: Restore damaged relationship ties.

---

## 2. Standardized Action Payload Schema

Every action parsed by the AI Bridge or submitted via UI controls adheres to this JSON schema:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "ActionPayload",
  "type": "object",
  "properties": {
    "action": {
      "type": "string",
      "enum": [
        "COMMUNICATE", "ASK", "PERSUADE", "DECEIVE", "ATTEND_ACTIVITY",
        "STUDY", "TRAIN", "COMPETE", "APPLY", "WORK", "MOVE", "BUY", "SELL"
      ]
    },
    "actor_id": { "type": "string" },
    "target_id": { "type": ["string", "null"] },
    "secondary_target_id": { "type": ["string", "null"] },
    "location_id": { "type": ["string", "null"] },
    "claim": {
      "type": ["object", "null"],
      "properties": {
        "claimed_destination": { "type": "string" },
        "claimed_activity": { "type": "string" }
      }
    },
    "actual_action": { "type": ["object", "null"] },
    "intensity": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
    "parameters": { "type": "object" }
  },
  "required": ["action", "actor_id"]
}
```

---

## 3. Action Validation & Execution Pipeline

```text
Action Payload
      │
      ▼
┌───────────────────────────────┐
│  Validation 1: Prerequisite   │ -> Does actor possess age, location, funds, or energy?
└──────────────┬────────────────┘
               │ Pass
               ▼
┌───────────────────────────────┐
│   Validation 2: Feasibility   │ -> Is physical travel or access possible in time block?
└──────────────┬────────────────┘
               │ Pass
               ▼
┌───────────────────────────────┐
│ Validation 3: Simulation Roll │ -> Roll (Skill + Traits + RNG) vs Opposition/Difficulty
└──────────────┬────────────────┘
               │ Calculated Result
               ▼
┌───────────────────────────────┐
│     Authoritative Mutation    │ -> Update DB, relationship matrices, skills, event history
└───────────────────────────────┘
```

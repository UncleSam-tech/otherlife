# OTHERLIFE — Product Vision & Core Philosophy

**Tagline:** Live the life that never happened.  
**Product Type:** Offline-first, text-led, open-world human life simulation  
**Primary Target:** Desktop (macOS and Windows)  
**Core Principle:** The player may attempt almost anything, but the simulation—not the language model—decides what actually happens.

---

## 1. Executive Summary

OTHERLIFE is an unconstrained alternate-reality life simulator set in the real modern world. The player begins life as a human being—typically at birth or early childhood—and lives through education, relationships, employment, sports, business, politics, crime, family, wealth, health, aging, and death.

There are **no predefined career paths** (no hardcoded "Footballer Path", "Politician Path", or "Doctor Path"). Instead, interconnected simulation primitives govern economics, biology, psychology, geography, and social networks from which any imaginable human life can naturally emerge.

---

## 2. Non-Negotiable Laws

1. **The Player May Attempt Almost Anything**: The player is never blocked by rigid menu restrictions from formulating an intent.
2. **Attempting Does Not Guarantee Success**: Success depends on simulated ability, knowledge, age, legal boundaries, wealth, social distance, opportunity, and circumstance.
3. **The Simulation Owns Reality**: Authoritative world state resides exclusively in the Rust simulation engine and SQLite database.
4. **The LLM Cannot Directly Edit State**: The language model only interprets natural text into structured actions and renders dialogue/narrative.
5. **Real-World Canon Starting Snapshot**: The world begins with real countries, cities, universities, clubs, institutions, and public figures.
6. **Alternate-Timeline Rule**: The instant a save is created, real history becomes an alternate timeline. The future is simulated, never scripted to match reality.
7. **Unified Entity Architecture**: Seeded real people and procedurally generated people use the exact same `Person` schema.
8. **Realistic Social Distance & NPC Knowledge**: NPCs only know what they have causally observed, learned, or been told. Famous public figures do not randomly interact with the player without a causal social bridge.
9. **Careers Are Emergent**: Occupations are roles within organizations, requiring combinations of skills, qualifications, network, and institutional needs.
10. **Offline-First & Zero Network Dependency**: The entire game—simulation, persistence, and local AI fallback—runs 100% locally on the player's desktop.

---

## 3. The Alternate-Timeline Rule

- **Starting Condition**: A real-world data snapshot (e.g., August 2026) sets initial population, rosters, offices, companies, and locations.
- **Divergence**: Once tick 0 executes, the timeline belongs entirely to the simulation kernel.
  - A real club may fail to win a title it won in reality.
  - A real politician may lose an election or resign early.
  - A real athlete may suffer an injury, transfer elsewhere, or retire to become a manager.
- **Branching Timelines**: Players can create save branches at any moment (e.g., Timeline A: Accept Arsenal offer; Timeline B: Stay at Celtic). Both timelines evolve independently.

---

## 4. Generational Play & Legacy

When the player character dies of old age, illness, or misadventure, the timeline does not end unless desired. The player can:
- Continue as a child or grandchild;
- Branch into a new life within the same evolved world;
- End the timeline and view the full life biography and causal history.

Descendants naturally inherit appropriate genetic traits, wealth, property, corporate shares, social reputation, and family connections.

---

## 5. Architectural Integrity

The game prioritizes simulation depth and causal explainability over superficial content velocity. Every major event recorded in history carries a causality vector ("Why did this happen?") allowing full inspectability into the simulated world.

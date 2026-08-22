# World-data generation strategy

## Decision

Use a hybrid world-data system. Small curated packs provide canonical geography and legal structure; deterministic procedural generators create the population and local institutions for each save; the local LLM supplies names, descriptions, dialogue, interviews, news, and exceptional entities through validated structured output.

The local LLM should not be the only source of real-world facts. Model knowledge can be incomplete, outdated, contradictory, or different across machines. It is well suited to enrichment, not referential integrity.

## Layers

1. **Canonical reference layer** — country, region, city, league, currency, school system, legal ages, government structure, climate, and stable identifiers. Store compact normalized data and indexes, not prose.
2. **Procedural world layer** — seeded generation of neighborhoods, employers, schools, hospitals, shops, jobs, households, clubs, and public offices. The same save seed produces the same world.
3. **LLM enrichment layer** — structured JSON for names, biographies, interview questions, pitches, dialogue, descriptions, local headlines, and special opportunities.
4. **Save overlay** — every entity the player sees or affects receives an ID and is persisted locally. Regeneration must never silently replace established history.

## Geographic hierarchy

Every generated entity must resolve through explicit parent keys:

`country -> region/state -> city -> district -> place/institution`

Football follows:

`country -> association -> competition -> season -> club -> squad -> person`

Politics follows:

`country -> jurisdiction -> office -> election cycle -> party -> candidate`

Education follows:

`country -> system -> institution -> campus -> faculty/program -> course`

Queries filter by these relationships before any LLM prompt is assembled. This prevents a player in Lagos from receiving a random Glasgow league, London university, or American filing authority unless a cross-border action calls for it.

## LLM contract

The generator requests JSON matching a versioned schema and supplies the canonical IDs, era, geography, language, economy, and seed. Output is rejected or repaired when it contains unknown parent IDs, duplicate IDs, impossible dates, invalid currencies, or entities outside the requested jurisdiction.

Generated facts are cached in the local save. The prompt and model metadata may be retained for diagnostics, but the saved structured entity—not a later regeneration—is authoritative.

If the model is unavailable, deterministic templates and procedural name pools must keep every core system playable. Network access is never required for loading or continuing a life.

## Immediate repository implications

- Fix registry paths so the existing packs actually load before judging their coverage.
- Replace UI-local company, job, school, and club arrays with simulation queries scoped to the current location and date.
- Add schema-validated generators per domain rather than one unrestricted “generate world” prompt.
- Persist generated entities and their provenance in the local save.
- Add density tests for representative country/city/era combinations and referential-integrity tests across every hierarchy.

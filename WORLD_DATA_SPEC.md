# OTHERLIFE — World Seed Data & Entity Specification

## 1. Entity Namespace Rules

To maintain clear lineage between initial real-world snapshot entities and procedurally generated characters/organizations, all IDs follow strict URI namespaces:

### Real-World Canonical Entities
```text
city:real:glasgow
city:real:london
city:real:manchester
club:real:celtic
club:real:rangers
club:real:manchester_united
club:real:real_madrid
league:real:scottish_premiership
league:real:premier_league
university:real:oxford
university:real:glasgow
person:real:ferguson_alex
person:real:mcgregor_callum
```

### Procedurally Generated Simulation Entities
```text
person:sim:9f83a42b-5e61-4d89-a29d-0b73c4f923b0
family:sim:1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d
company:sim:e4f5a6b7-8c9d-0e1f-2a3b-4c5d6e7f8a9b
club:sim:3c4d5e6f-7a8b-9c0d-1e2f-3a4b5c6d7e8f
```

---

## 2. Directory Layout of World Seed Data

```text
real_world_data/
├── geography/
│   ├── countries.json           # Real countries (UK, Spain, USA, France, etc.)
│   ├── regions.json             # Regions (Scotland, England, Catalonia, etc.)
│   └── cities.json              # Cities (Glasgow, London, Manchester, Madrid)
├── football/
│   ├── associations.json        # FIFA, UEFA, Scottish FA, English FA
│   ├── leagues.json             # Scottish Premiership, Premier League, La Liga
│   ├── clubs.json               # Celtic, Rangers, Man Utd, Real Madrid, Arsenal
│   └── stadiums.json            # Celtic Park, Ibrox, Old Trafford, Bernabéu
├── education/
│   ├── schools.json             # High schools & academies in major cities
│   └── universities.json        # Oxford, Cambridge, Glasgow, Edinburgh, Harvard
├── companies/
│   └── corporations.json        # BBC, Apple, Microsoft, Barclays, BP
└── people/
    ├── sports_figures.json      # Professional players, managers, scouts
    └── public_figures.json      # Politicians, institution heads
```

---

## 3. Example Seed Schema Snippet (Glasgow & Celtic FC)

### Geography Seed (`real_world_data/geography/cities.json`)
```json
{
  "id": "city:real:glasgow",
  "name": "Glasgow",
  "country_id": "country:real:united_kingdom",
  "region_id": "region:real:scotland",
  "population": 635000,
  "cost_of_living_index": 1.0,
  "districts": ["West End", "City Centre", "East End", "Southside", "Partick"]
}
```

### Club Seed (`real_world_data/football/clubs.json`)
```json
{
  "id": "club:real:celtic",
  "name": "Celtic Football Club",
  "short_name": "Celtic",
  "city_id": "city:real:glasgow",
  "stadium_id": "stadium:real:celtic_park",
  "league_id": "league:real:scottish_premiership",
  "reputation": 78.0,
  "youth_academy_quality": 82.0,
  "finance_tier": "STRONG"
}
```

---

## 4. Procedural Entity Generation Rules

When young generations reach school age or entering football academies:
- **Demographic Alignment**: Names, languages, and ethnic frequencies align with city demographic census distributions.
- **Genetic Inheritance**: Children of real/sim parents inherit trait ranges derived from parent attributes (height, athletic potential, baseline traits).
- **Parity of Status**: Once instantiated, generated entities possess full component equivalence to seeded real entities.

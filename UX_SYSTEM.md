# OTHERLIFE — User Interface & Design System Specification

## 1. UI Philosophy: "An Operating System for Another Life"

OTHERLIFE rejects chatbot layouts, spreadsheet grids, and mobile-port menus. The desktop interface is designed as a calm, editorial, high-density dashboard that gives the player instant situational awareness over their simulated existence.

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ OTHERLIFE       12 OCT 2029 · AGE 14       £24 · GLASGOW            [⚙] │
├──────────────┬──────────────────────────────────────────────┬───────────────┤
│              │                                              │               │
│ LIFE         │                  LIFE FEED                   │ NOW           │
│              │                                              │               │
│ • Overview   │ 12 OCT 2029 — 16:30                          │ Saturday      │
│ • Family     │ Mum found out about your math result.        │ Match         │
│ • School     │ She is visibly concerned about your focus.   │               │
│ • Football   │                                              │ 2 Commitments │
│ • Career     │ 14 OCT 2029 — 09:00                          │               │
│ • Money      │ A regional scout from Celtic FC will attend  │ Mum: Tense    │
│ • Activities │ Saturday's youth match.                      │ School: ↓     │
│ • World      │                                              │ Football: ↑   │
│              │ James sent a message asking to meet.         │               │
│              │                                              │               │
├──────────────┴──────────────────────────────────────────────┴───────────────┤
│ [⚡ Shortcuts]  What do you want to do?                           [ Send → ]│
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Desktop Screen Regions

1. **Top Header**: Player Identity, Timestamp (`12 OCT 2029`), Current Age (`14`), Capital (`£24`), Location (`Glasgow`), Developer Toggle.
2. **Left Navigation Rail**: Quick access to deep life facets (Overview, Relationships/Family, Education, Football, Career, Finances, Activities, World News).
3. **Center Life Feed**: Chronological narrative river displaying events, NPC conversations, choices, and world developments.
4. **Right NOW Sidebar**: Immediate situational context—upcoming calendar commitments (e.g. Saturday Match), active relationship tension, recent stat vectors.
5. **Bottom Action Command Bar**: Universal input bar allowing free natural-language intent or one-click contextual suggestions.

---

## 3. Design System Tokens & Aesthetics

### Color Palette (Editorial Dark Mode)
- **Background Deep**: `#0D1117` (Deep Obsidian)
- **Surface Elevation 1**: `#161B22` (Dark Slate Card)
- **Surface Elevation 2**: `#21262D` (Elevated Panel)
- **Border Subtle**: `#30363D` (Crisp Monochromatic Border)
- **Text Primary**: `#F0F6FC` (High-contrast Off-White)
- **Text Secondary**: `#8B949E` (Muted Neutral Grey)
- **Accent Emerald (Positive/Football)**: `#2EA043` / `#3FB950`
- **Accent Amber (Tension/Warning)**: `#D29922` / `#F0B72F`
- **Accent Crimson (Conflict/Failure)**: `#F85149`

### Typography Hierarchy
- **Primary Interface**: Inter / System Sans (-apple-system, BlinkMacSystemFont, Segoe UI)
- **Editorial Narration**: Serif / Georgia (for Life Feed prose & biographies)
- **Metrics & Timestamps**: JetBrains Mono / Monospace

---

## 4. Information Hiding vs Developer Mode

- **Normal Player UI**: Hides raw percentages and potential scores behind natural editorial qualifiers.
  - *Hidden*: `Potential: 87.4, Scout Likelihood: 34.2%`
  - *Displayed*: *"Your coach believes you have unusual potential. Recent performances have attracted regional interest."*
- **Developer Mode (`F12` / Toggle)**: Exposes exact variable metrics, RNG seeds, relationship vectors, entity IDs, and active knowledge states for debugging.

---

## 5. Causality Inspector UI ("Why Did This Happen?")

Clicking any key event in the Life Feed expands the Causality Inspector drawer:

```text
┌────────────────────────────────────────────────────────────┐
│ Event: Celtic FC Scouting Invitation                       │
├────────────────────────────────────────────────────────────┤
│ Causing Factors:                                           │
│  [↑↑] Excellent recent match performance (2 goals)        │
│  [↑]  Coach formal recommendation                          │
│  [↑]  Scout regional coverage in Glasgow East             │
│  [↓]  Poor academic attendance record                      │
└────────────────────────────────────────────────────────────┘
```

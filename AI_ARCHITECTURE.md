# OTHERLIFE — Local AI Architecture & Fallback Specification

## 1. Core Principles & Offline Rule

- **Offline Independence**: OTHERLIFE must function without an internet connection, cloud API key, or online account.
- **Strict Role Separation**: The Local LLM **never** decides simulation results, statistics, or state mutations. It only parses free text into structured actions and renders prose/dialogue.
- **Resilient Fallback**: If no LLM binary/model is loaded, or if inference times out, the game relies on a **deterministic rule-based intent parser** and **template narrative engine**.

---

## 2. LLM Responsibilities & Task Isolation

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                           AI BRIDGE WORKFLOW                            │
│                                                                         │
│  [Player Text Input] ──► INTENT PARSER (JSON Schema Constrained)       │
│                               │                                         │
│                               ▼                                         │
│                      [Rust Simulation Tick]                             │
│                               │                                         │
│                               ▼                                         │
│  [State Mutations]   ──► NARRATIVE / DIALOGUE RENDERER                 │
└─────────────────────────────────────────────────────────────────────────┘
```

1. **Intent Parser**: Converts arbitrary text (e.g. *"Tell Mum I'm going to James's house to study math, but secretly go to training."*) into valid JSON action payloads.
2. **Dialogue Renderer**: Converts NPC reaction values (`anger=0.7`, `trust_loss=0.08`) into character-consistent spoken dialogue.
3. **Narrative Renderer**: Synthesizes simulation event logs into engaging editorial prose.
4. **Memory Summarizer**: Condenses month-old event sequences into compact NPC memory summaries.
5. **Biography Writer**: Generates comprehensive lifetime biographies upon character death or milestone achievements.

---

## 3. Recommended Local Models

| Tier | Model Architecture | Quantization | VRAM / RAM Target |
| :--- | :--- | :--- | :--- |
| Low / Default | Qwen3 0.6B / 1.7B | GGUF Q4_K_M | 1 GB - 2 GB RAM |
| Balanced | Qwen3 1.7B / 4B | GGUF Q5_K_M | 2 GB - 4 GB RAM |
| High | Qwen3 4B / Llama 3.2 3B | GGUF Q8_0 | 4 GB - 8 GB VRAM |

---

## 4. Zero-Dependency Deterministic Fallback Pipeline

When the local model is uninstalled, disabled, or fails to respond within 800ms:

### 1. Rule-Based Keyword Intent Fallback
The `ai_bridge` crate executes regex and keyword parsing:
- Inputs containing *"train"*, *"practice"*, *"football"* -> Maps to `ATTEND_ACTIVITY(football_training)`.
- Inputs containing *"study"*, *"math"*, *"homework"* -> Maps to `STUDY(subject: math)`.
- Inputs containing *"lie"*, *"secret"*, *"tell mum"* -> Maps to `DECEIVE`.

### 2. Deterministic Narrative Template Fallback
Instead of dropping raw state numbers, template strings are selected based on event flags:
- *Template*: `"You decided to attend training secretly. {target_name} remained unaware for now, but your focus was divided."`
- *Template*: `"{npc_name} noticed your absence and expressed disappointment regarding your recent math result."`

---

## 5. Context Assembly Engine

To keep local inference fast (< 200ms per turn), context windowing is strictly bounded:

```text
┌───────────────────────────────────────────────────────────┐
│ SYSTEM INSTRUCTION (Strict JSON Output Schema)           │
├───────────────────────────────────────────────────────────┤
│ CURRENT CONTEXT                                           │
│ Date: 12 Oct 2029 | Location: Glasgow | Age: 14           │
│ Active Scene: Living Room with Mum                        │
├───────────────────────────────────────────────────────────┤
│ RELEVANT ENTITIES & RELATIONSHIPS                        │
│ Mum (Trust: 0.62, Resentment: 0.35, Concern: High)        │
├───────────────────────────────────────────────────────────┤
│ RECENT EVENTS                                             │
│ • Failed Math Exam (Score: 42%)                           │
│ • Regional Scout attending Saturday Match                 │
├───────────────────────────────────────────────────────────┤
│ PLAYER INPUT                                              │
│ "Tell Mum I'm going to James's house but go to training." │
└───────────────────────────────────────────────────────────┘
```

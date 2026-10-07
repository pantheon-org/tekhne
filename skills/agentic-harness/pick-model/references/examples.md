# Worked Examples

Each row shows the base tier, the escalators that fired and the result. Use them to calibrate borderline calls.

## Technical

| Task | Result | Deciding signal |
|---|---|---|
| "fix typo in README" | 🟢 **Haiku** | Trivial single edit, no reasoning |
| "convert PDF to markdown" | 🟢 **Haiku** | Simple conversion, no decisions |
| "debug flaky integration test" | 🟡 **Sonnet** | Single-system debugging, moderate reasoning |
| "refactor auth across 15 files" | 🔴 **Opus** | Scope escalator on top of architectural decisions |
| "design database schema for e-commerce" | 🔴 **Opus** | Architecture with trade-offs and long-term impact |
| "plan microservices migration strategy" | 🔴 **Opus** | Complex architectural planning plus Irreversibility |

## Business and strategy

| Task | Result | Deciding signal |
|---|---|---|
| "summarise this meeting transcript" | 🟢 **Haiku** | Text transformation under 2K words |
| "extract action items from notes" | 🟢 **Haiku** | Extraction, no reasoning |
| "write blog post about AI trends" | 🟡 **Sonnet** | Creative writing, moderate reasoning |
| "draft sales proposal for enterprise client" | 🟡 **Sonnet** | Persuasive writing, moderate reasoning |
| "analyse competitor pricing strategy" | 🟡 **Sonnet** | Research with a single framework |
| "plan market entry strategy for Europe" | 🔴 **Opus** | Irreversibility, Synthesis and Ambiguity |
| "design organisational restructuring plan" | 🔴 **Opus** | Sensitivity, Stakeholders and Irreversibility |
| "M&A due diligence analysis" | 🔴 **Opus** | Stakes and Synthesis |
| "crisis communication plan for a data breach" | 🔴 **Opus** | Sensitivity, Stakes and Stakeholders |

## Creative and analysis

| Task | Result | Deciding signal |
|---|---|---|
| "translate paragraph to French" | 🟢 **Haiku** | Language transform, no reasoning |
| "brainstorm product names (single session)" | 🟡 **Sonnet** | Creative generation, one framework |
| "brainstorm with SCAMPER plus trade-off analysis" | 🔴 **Opus** | Multi-framework |
| "retrospect: analyse collaboration patterns" | 🔴 **Opus** | Pattern detection plus Bias identification |
| "identify blind spots in strategy" | 🔴 **Opus** | Bias identification plus Ethical reasoning |
| "plan 3-day conference with speakers" | 🔴 **Opus** | Many constraints and Stakeholders |

## Commands, skills and agents

| Task | Result | Deciding signal |
|---|---|---|
| "command: convert EPUB to markdown" | 🟢 **Haiku** | Simple workflow, minimal reasoning |
| "command: save session context" | 🟡 **Sonnet** | Context management, serialisation logic |
| "command: brainstorm with research plus SCAMPER" | 🔴 **Opus** | Multi-framework plus strategic analysis |
| "command: retrospect domain learnings" | 🔴 **Opus** | Pattern detection across sessions plus Bias identification |
| "skill: format code with prettier" | 🟢 **Haiku** | Deterministic task |
| "skill: standard workflow implementation" | 🟡 **Sonnet** | Standard workflow, moderate reasoning |
| "agent: explore codebase architecture" | 🔴 **Opus** | Exploration plus architectural synthesis |

## Cases where the escalator decides

```text
Task: "fix a typo in the production payment config"
Base: Haiku (typo fix). Fired: Stakes (production, payments).
Result: Sonnet. One tier up, not Opus.
```

```text
Task: "rename a function across 40 files with a codemod"
Base: Sonnet. Fired: none. Scope is one mechanical decision, not 40.
Result: Sonnet. Check the diff before trusting the codemod.
```

```text
Task: "write a customer apology email after an outage"
Base: Sonnet. Fired: Sensitivity (customer-facing crisis).
Result: Opus. Sonnet is enough for the internal incident summary.
```

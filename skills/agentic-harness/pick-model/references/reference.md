# Pick Model - Extended Reference

## Model Characteristics

### Haiku 4.5
- **Speed**: Fastest (~2-3x faster than Sonnet)
- **Cost**: Lowest (~10x cheaper than Opus)
- **Context**: 200K tokens
- **Best for**: Deterministic, pattern-based, low-reasoning tasks
- **Limitations**: Struggles with ambiguity, multi-step reasoning, creative tasks

### Sonnet 4.5
- **Speed**: Medium (baseline)
- **Cost**: Medium (~5x cheaper than Opus)
- **Context**: 200K tokens
- **Best for**: Balanced reasoning, creative work, most coding tasks
- **Limitations**: Multi-file refactoring, highly nuanced reasoning

### Opus 4.6
- **Speed**: Slowest (~1.5-2x slower than Sonnet)
- **Cost**: Highest (premium tier)
- **Context**: 200K tokens
- **Best for**: Complex reasoning, architectural decisions, high-stakes work
- **Limitations**: Overkill for simple tasks, slower iteration

---

## Extended Decision Matrix

### By File Type

| File Type | Task | Model |
|---|---|---|
| `.md`, `.txt` | Typo fix, formatting | Haiku |
| `.md`, `.txt` | Blog post, documentation | Sonnet |
| `.md`, `.txt` | Long-form report (>5K words) | Opus |
| `.json`, `.yaml`, `.toml` | Parse, extract, validate | Haiku |
| `.json`, `.yaml`, `.toml` | Schema design | Sonnet |
| `.py`, `.js`, `.ts` (single) | Bug fix, feature add | Sonnet |
| `.py`, `.js`, `.ts` (3+ files) | Refactor, architecture | Opus |
| `.sh`, `.bash` | Script debug/fix | Sonnet |
| `.sh`, `.bash` | Complex orchestration | Opus |

### By Domain

| Domain | Task Type | Model |
|---|---|---|
| **Data Processing** | ETL, parsing, cleaning | Haiku |
| **Data Processing** | Pipeline design | Sonnet |
| **Data Processing** | Distributed system design | Opus |
| **Content Creation** | Social post, email | Sonnet |
| **Content Creation** | Whitepaper, thesis | Opus |
| **DevOps** | Config fix, logs analysis | Haiku/Sonnet |
| **DevOps** | Infrastructure design | Opus |
| **Security** | Code scan, vuln check | Sonnet |
| **Security** | Threat modeling, audit | Opus |
| **Testing** | Unit test write | Sonnet |
| **Testing** | Test strategy, framework | Opus |

### By Interaction Pattern

| Pattern | Model |
|---|---|
| **One-shot** (single request/response) | Match task complexity |
| **Iterative** (back-and-forth refinement) | Start lower, escalate if needed |
| **Exploratory** (user learning) | Start Sonnet (patient explanations) |
| **Production** (high stakes) | Escalate +1 tier for safety |

---

## Cost/Latency Tradeoffs

### When to optimize for speed (choose lower tier)
- Rapid prototyping, quick iteration
- Low-stakes exploratory work
- User waiting synchronously
- Batch processing many simple tasks

### When to optimize for quality (choose higher tier)
- Production deployments
- Security-critical code
- User-facing content (brand reputation)
- Complex architectural decisions
- Tasks where rework is expensive

### Cost Examples (Approximate)
- **100K input tokens + 10K output**:
  - Haiku: ~$0.10
  - Sonnet: ~$0.30
  - Opus: ~$1.50

---

## Edge Cases & Hybrid Tasks

### Escalation Scenarios

**Start Haiku → Upgrade Sonnet if:**
- Output lacks coherence
- Task requires reasoning not obvious from pattern
- User requests "explain why" or "consider alternatives"

**Start Sonnet → Upgrade Opus if:**
- Multi-system dependencies emerge
- Ambiguity requires nuanced judgment
- Initial approach fails, root cause unclear
- Architectural implications surface

### Hybrid Approaches

**Sequential (pipeline):**
1. Haiku: Extract data from logs
2. Sonnet: Analyze patterns, generate report

**Parallel (fan-out):**
1. Haiku: Format 10 files in parallel
2. Sonnet: Review aggregated changes

**Iterative (feedback loop):**
1. Sonnet: Draft implementation plan
2. User: Feedback
3. Opus: Refine with architectural considerations

---

## Domain-Specific Guidelines

### Web Development
- Component styling, prop changes → Sonnet
- Component library design → Opus
- API endpoint (single) → Sonnet
- API architecture (REST vs GraphQL) → Opus

### Data Science
- Data cleaning, feature engineering → Haiku/Sonnet
- Model selection, experiment design → Opus
- Jupyter notebook fixes → Sonnet
- Pipeline architecture → Opus

### Infrastructure
- Terraform syntax fix → Haiku
- Resource provisioning → Sonnet
- Multi-region HA design → Opus

### Documentation
- API reference generation → Haiku
- Tutorial writing → Sonnet
- Architecture Decision Records → Opus

---

## Common Mistakes

### Over-escalation
- Using Opus for typo fixes, simple formatting
- **Cost**: 10-15x more expensive
- **Fix**: Trust Haiku for deterministic tasks

### Under-estimation
- Using Haiku for "simple" refactors that touch 5+ files
- **Risk**: Poor code quality, missed edge cases
- **Fix**: Apply complexity escalators

### Ignoring context
- Choosing model without considering stakes, ambiguity, scope
- **Fix**: Use decision matrix + escalators

### False economy
- Choosing Haiku for production-critical work to save $1
- **Risk**: Outages, security issues, rework costs >> savings
- **Fix**: Escalate +1 tier for high stakes

---

## When to Override Recommendation

**User knows best when:**
- Specific model preferences based on past experience
- Budget constraints require cost optimization
- Time constraints require speed optimization
- Iterative work (start lower, escalate if needed)

**Always respect explicit user model selection.**

---

## Quick Reference: Signal Words

### Haiku signals
- "quick", "simple", "just", "only", "extract", "format", "rename", "fix typo"

### Sonnet signals
- "write", "create", "explain", "review", "analyze", "debug", "single file"

### Opus signals
- "design", "architect", "complex", "multiple files", "refactor", "migration", "strategy", "nuanced"

---

## Model Selection Confidence

Output confidence level based on task clarity:

```
🔴 **Opus** — [reason]
💰 Cost: highest | ⚡ Speed: slowest
🎯 Confidence: High (clear architectural scope)

💡 Consider Sonnet if scope reduces to 2-3 files after exploration
```

**Confidence indicators:**
- **High**: Task signals clearly match one tier
- **Medium**: Borderline between two tiers
- **Low**: Insufficient information, recommend exploration
</content>

# Diagnose Protocol

Narrow search space before OODA. Many bugs resolve here.

## Order of Operations

```
Mental Models → Isolation → Root Cause Drill → (OODA if still stuck)
```

## 1. Mental Models (Pattern Match)

### Step 1: Check learnings file

If a project-level or shared learnings file exists, read it for accumulated patterns:
```yaml
# Look for matching symptom
mental_models:
  - pattern: <symptom>
    insight: <cause and fix>
```

**Match found?** → Apply insight, suggest fix, skip OODA.

### Step 2: WebSearch for pattern

If no local match, search:
```
"[symptom]" common cause fix
```

Look for established patterns (Stack Overflow accepted answers, official docs).

**Match found?** → Apply, suggest fix, offer to save to learnings file.

### Step 3: Reason from first principles

If no search match, apply frameworks:
- **5 Whys**: Ask "why?" 3-5 times to drill to root
- **Fishbone 6 M's**: Brainstorm across Man/Machine/Method/Material/Measurement/Milieu

See `references/reference.md` for details on these techniques.

## 2. Isolation (Narrow the Space)

Use techniques from `references/reference.md`:

### Wolf Fence
- Comment out half → bug gone? → Other half
- Repeat until isolated
- For commits: `git bisect`

### Swap One Variable
- Works in A not B? → What's different?
- Env, data, config, version, user, time

### Minimal Repro
- Strip code until bug disappears
- Last removed piece = culprit

## 3. Root Cause Drill

### 5 Whys
Ask "why?" until actionable:
```
Bug → Why? → Why? → Why? → ROOT CAUSE
```
Stop at 3-5 levels.

### Fishbone (if complex)
Brainstorm across 6 M's:
- Man, Machine, Method, Material, Measurement, Milieu

Pick 2-3 likely categories, investigate those.

## Short-Circuit Conditions

**Exit to solution if:**
- Pattern match found with high confidence
- Isolation pinpointed single cause
- 5 Whys reached actionable root

**Escalate to OODA if:**
- No pattern match
- Isolation inconclusive after 2-3 attempts
- Multiple possible root causes remain

## Handoff to OODA

Pass to investigation loop with:
- Symptom summary
- What's been ruled out
- Current hypothesis (if any)

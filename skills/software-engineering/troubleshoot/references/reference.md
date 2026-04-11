# Troubleshoot Reference

## 1. Isolation Techniques

### Wolf Fence (Binary Search)

Divide and conquer. Cut search space in half each step.

| Step | Action |
|------|--------|
| 1 | Comment out / disable half the code |
| 2 | Bug gone? → It's in removed half. Bug persists? → Other half |
| 3 | Repeat on guilty half until single line/block found |

**Git variant:** `git bisect start` → `git bisect bad` → `git bisect good <commit>` → Git finds guilty commit in O(log n)

### Swap One Variable

When multiple suspects, change only one at a time:
- Works in env A but not B? → Diff A and B
- Works with data X but not Y? → Diff X and Y
- Works before commit Z? → Diff before/after Z

### Minimal Reproduction

Reduce to smallest case that still fails:
1. Remove code/config until bug disappears
2. Add back last removed piece
3. That's your culprit

## 2. Root Cause Drilling

### 5 Whys

Keep asking "why?" until you hit root cause (usually 3-5 levels):

```
Problem: API returns 500
→ Why? Unhandled exception
→ Why? Null pointer on user.email
→ Why? User record has no email
→ Why? Migration didn't backfill
→ Why? Script assumed all users have email ← ROOT CAUSE
```

**Rule:** Stop when answer is actionable fix.

### Fishbone (6 M's)

For complex bugs with unclear cause, brainstorm across categories:

| Category | Questions |
|----------|-----------|
| **Man** | Who touched it last? Training gap? Handoff issue? |
| **Machine** | Hardware? Memory? CPU? Disk? Network? |
| **Method** | Process flaw? Missing step? Wrong order? |
| **Material** | Bad input data? Corrupt file? Wrong format? |
| **Measurement** | Missing logs? Wrong metric? Alert gap? |
| **Milieu** | Environment diff? Config drift? External dep? |

Pick 2-3 most likely categories, investigate those first.

## 3. Assumption Surfacing

### Rubber Duck

Explain the problem out loud, line by line:
1. "This function should do X..."
2. "First it gets Y from Z..."
3. "Then it— wait, that's not what it does"

**Why it works:** Forces you to articulate assumptions. Bug often surfaces mid-explanation.

### Verify → Shorten → Reveal

| Phase | Action |
|-------|--------|
| **Verify** | Does bug actually exist as reported? Reproduce first. |
| **Shorten** | Make feedback loop fast (seconds not minutes). Write a test. |
| **Reveal** | Add logging at boundaries. Test each assumption explicitly. |

## 4. Investigation Loop (OODA)

When isolation + drilling doesn't resolve:

```
OBSERVE → ORIENT → DECIDE → ACT → repeat
```

| Phase | Action | Example |
|-------|--------|---------|
| Observe | Gather data | "Run `X` and paste output" |
| Orient | Form hypothesis | "This suggests Y because..." |
| Decide | Pick next action | "Let's verify by checking Z" |
| Act | User executes | "Run this command: ..." |

**Exit when:** Root cause confirmed + fix verified

**Stuck?** Go back to Search (step 1 in SKILL.md) with new keywords from investigation.

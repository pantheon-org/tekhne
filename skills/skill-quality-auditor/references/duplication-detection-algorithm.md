---
category: duplication
priority: HIGH
source: skill consolidation experience
---

# Duplication Detection Algorithm

Methodology for detecting content duplication across skills. Identifies aggregation candidates through similarity analysis.

## Detection Overview

**Purpose**: Find skills with >20% content overlap for consolidation  
**Output**: Duplication report with similarity percentages and recommendations  
**Threshold**: >20% similarity = aggregation candidate, >35% = critical

## Algorithm Components

### 1. Text Similarity Analysis

Compare line-by-line content across all skill files:

```bash
# For each pair of skills
for skill1 in skills/*/SKILL.md; do
  for skill2 in skills/*/SKILL.md; do
    # Skip same file
    [[ "$skill1" == "$skill2" ]] && continue
    
    # Count common lines
    common=$(comm -12 <(sort "$skill1") <(sort "$skill2") | wc -l)
    
    # Calculate similarity
    total1=$(wc -l < "$skill1")
    total2=$(wc -l < "$skill2")
    avg=$(( (total1 + total2) / 2 ))
    similarity=$(( common * 100 / avg ))
    
    # Report if above threshold
    if [ "$similarity" -gt 20 ]; then
      echo "$skill1 <-> $skill2: ${similarity}%"
    fi
  done
done
```

### 2. Structural Similarity

Compare file structure and organization:

- Same category prefixes
- Similar directory layouts
- Matching reference patterns
- Identical script names

### 3. Conceptual Overlap

Identify shared concepts:

- Same domain keywords
- Overlapping trigger scenarios
- Related use cases
- Common anti-patterns

## Similarity Thresholds

| Similarity | Classification | Action |
|------------|----------------|--------|
| 0-10% | Unrelated | Keep separate |
| 10-20% | Marginal | Review for content |
| 20-35% | Candidate | Plan aggregation |
| 35-50% | High | Prioritize aggregation |
| >50% | Critical | Immediate consolidation |

## Detection Process

### Step 1: Inventory All Skills

```bash
# List all active skills
find skills -name "SKILL.md" -not -path "*/.deprecated/*" | sort
```

### Step 2: Pairwise Comparison

Compare every skill against every other skill:

1. Extract text content (ignore frontmatter)
2. Normalize whitespace and case
3. Sort lines for comparison
4. Count common lines
5. Calculate similarity percentage

### Step 3: Family Analysis

Identify skill families by naming:

```
bdd-*        → BDD family
typescript-* → TypeScript family
bun-*        → Bun family
```

Skills in same family with >20% overlap = strong aggregation candidates.

### Step 4: Generate Report

Create markdown report with:

- Similarity matrix
- Top candidates by overlap
- Recommendations by priority
- Estimated consolidation effort

## Report Format

```markdown
# Duplication Report - YYYY-MM-DD

## Summary
- Skills analyzed: X
- Pairs with >20% similarity: Y
- Critical (>35%): Z

## High-Priority Candidates

### bdd-testing-family
| Skill Pair | Similarity | Common Lines | Action |
|------------|------------|--------------|--------|
| bdd-gherkin ↔ cucumber-best-practices | 42% | 287 | Aggregate |
| bdd-patterns ↔ bdd-scenarios | 28% | 156 | Consider |

## Recommendations

1. **Immediate**: Consolidate bdd-* family (6 skills, 35% avg duplication)
2. **High**: Review typescript-* family (4 skills, 22% avg duplication)
```

## False Positive Handling

Not all similarity indicates duplication:

**Expected Similarity (Not Duplication):**

- Shared YAML frontmatter structure
- Common markdown formatting
- Standard section headers
- Universal anti-patterns

**True Duplication (Needs Consolidation):**

- Identical code examples
- Same conceptual explanations
- Repeated workflow steps
- Copy-pasted reference content

## Automation

### Manual Detection

```bash
./scripts/detect-duplication.sh
```

### Scheduled Detection

Run weekly via CI/CD:

```yaml
schedule:
  - cron: '0 0 * * 0'  # Weekly on Sunday
```

## Metrics to Track

| Metric | Target | Current |
|--------|--------|---------|
| Average duplication | <5% | TBD |
| Max pair similarity | <20% | TBD |
| Aggregation candidates | <5 | TBD |
| Critical (>35%) | 0 | TBD |

## See Also

- `duplication-remediation.md` - How to fix duplication
- `aggregation-pattern.md` - Navigation Hub pattern
- `aggregation-implementation.md` - Step-by-step consolidation

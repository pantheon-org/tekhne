---
category: framework
priority: CRITICAL
source: skill-judge evaluation methodology
---

# Skill Scoring Rubric

Detailed scoring methodology for the skill-judge framework. Use this to understand how scores are calculated and ensure consistent evaluation.

## Scoring Overview

**Total Possible Score**: 120 points  
**Passing Grade**: 90 points (75%)  
**A-Grade Target**: 108 points (90%)  
**Perfect Score**: 120 points (100%)

## Dimension-by-Dimension Scoring

### D1: Knowledge Delta (20 points)

| Score | Criteria | Redundancy Level |
|-------|----------|------------------|
| 18-20 | Pure expert knowledge | <5% |
| 15-17 | Mostly expert | 5-15% |
| 12-14 | Acceptable balance | 15-30% |
| 9-11 | Needs improvement | 30-50% |
| 0-8 | Failing | >50% |

**Evaluation Method:**
1. Read entire skill content
2. Identify content Claude already knows
3. Calculate: Expert Content / Total Content
4. Apply scoring threshold

### D2: Mindset + Procedures (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | Clear mindset + detailed procedures + when/when-not |
| 10-12 | Has most elements, minor gaps |
| 7-9 | Missing key element |
| 0-6 | Generic or absent |

**Component Breakdown:**
- Clear Mindset/Philosophy: 5 points
- Step-by-Step Procedures: 5 points
- When/When-Not Guidance: 5 points

### D3: Anti-Pattern Quality (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | NEVER lists + concrete examples + consequences |
| 10-12 | Has most elements |
| 7-9 | Generic warnings |
| 0-6 | Missing or weak |

**Component Breakdown:**
- NEVER Lists with WHY: 5 points
- Concrete Examples: 5 points
- Consequences Explained: 5 points

### D4: Specification Compliance (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | Perfect spec compliance |
| 10-12 | Minor issues |
| 7-9 | Missing key elements |
| 0-6 | Non-compliant |

**Component Breakdown:**
- Description Field Quality: 10 points (most critical)
- Proper Frontmatter: 3 points
- Activation Keywords: 2 points

### D5: Progressive Disclosure (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | Navigation hub + references/ + categories |
| 10-12 | Some organization, could improve |
| 7-9 | Everything frontloaded, >300 lines |
| 0-6 | No structure, >500 lines |

**Component Breakdown:**
- Navigation Hub Approach: 8 points
- References Directory: 4 points
- Category Organization: 3 points

### D6: Freedom Calibration (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | Appropriate for skill type |
| 10-12 | Slightly too rigid or loose |
| 7-9 | Mismatched calibration |
| 0-6 | Completely wrong |

**Calibration Types:**
- Rigid (Mindset skills): Strong rules, must follow
- Balanced (Process skills): Clear steps with flexibility
- Flexible (Tool skills): Options and trade-offs

### D7: Pattern Recognition (10 points)

| Score | Criteria |
|-------|----------|
| 9-10 | Rich keywords, comprehensive triggers |
| 7-8 | Good keywords, could expand |
| 5-6 | Basic keywords |
| 0-4 | Missing or poor |

**Evaluation Method:**
- Count domain keywords in description
- Check trigger scenarios present
- Verify activation clarity

### D8: Practical Usability (15 points)

| Score | Criteria |
|-------|----------|
| 13-15 | Concrete + runnable + clear |
| 10-12 | Most examples good |
| 7-9 | Some weak examples |
| 0-6 | Abstract or missing |

**Component Breakdown:**
- Concrete Examples: 5 points
- Runnable Code: 5 points
- Clear Structure: 5 points

## Grade Assignment

| Grade | Score Range | Interpretation |
|-------|-------------|----------------|
| A+ | 114-120 | Exceptional quality |
| A | 108-113 | Meets all standards |
| B+ | 102-107 | Strong, minor improvements |
| B | 96-101 | Good, some gaps |
| C+ | 90-95 | Acceptable, needs work |
| C | 84-89 | Below standard |
| D | 78-83 | Significant issues |
| F | 0-77 | Failing |

## Scoring Process

### Step 1: Read and Understand
Read the entire skill, including all references if present.

### Step 2: Score Each Dimension
Apply rubric to each of 8 dimensions independently.

### Step 3: Calculate Total
Sum all 8 dimension scores for total out of 120.

### Step 4: Assign Grade
Map total score to grade using grade assignment table.

### Step 5: Identify Improvements
For scores below A-grade, identify specific improvements needed.

## Common Score Patterns

**High Knowledge Delta, Low Usability (18, 10)**: Expert content but lacks examples  
**Low Knowledge Delta, High Usability (10, 14)**: Tutorial-heavy, needs expert focus  
**Perfect Spec, Poor Content (15, 8)**: Great frontmatter, weak body  
**Balanced Scores (12-13 each)**: Consistent but not exceptional

## See Also

- `framework-skill-judge-dimensions.md` - Dimension definitions
- `framework-quality-standards.md` - A-grade requirements

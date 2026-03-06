# Phase 5: Comprehensive Skill Remediation

**Duration:** 40-60 hours (split across multiple sessions)  
**Status:** PLANNED  
**Prerequisites:** Phases 1-4 complete (audit baseline + process improvements)

---

## Overview

Phase 5 combines three parallel remediation strategies to lift overall repository quality from 82% to 88%+ average score:

1. **Bottom 10 Remediation** - Fix C-grade skills to B-grade minimum
2. **Anti-Pattern Template Library** - Extract patterns from A-grade skills
3. **Systematic D3 Improvements** - Add anti-pattern sections to all C+/C skills

**Target Impact:**
- Average score: 98.3 → 105+ (82% → 88%)
- A-grade count: 5 → 10-12
- C+/C skills: 23 → 5-8
- D3 Anti-Pattern avg: 68% → 80%+

---

## Phase 5 Execution Strategy

### Session Structure

Break Phase 5 into manageable sessions:
- **Session 1** (2-3hrs): Create anti-pattern template library + fix 2 C-grade skills
- **Session 2** (2-3hrs): Fix remaining 3 C-grade skills + start systematic D3
- **Session 3-5** (3-4hrs each): Systematic D3 improvements for C+ skills (batches of 5-6)
- **Final Session** (1hr): Progress tracking, verification, documentation

**Total:** 5-6 sessions, 40-60 hours

---

## Task Breakdown

### Task 5.1: Create Anti-Pattern Template Library (3-4 hours)

**Goal:** Extract reusable anti-pattern patterns from top 5 A-grade skills to serve as templates for remediation.

**Steps:**
1. **Analyze A-grade exemplars** (1hr):
   - Read anti-pattern sections from: opencode-config, nx-executors, agents-md, biome-complete, nx-generators
   - Identify common structural patterns
   - Document "anatomy of excellent anti-pattern documentation"

2. **Create template library** (2hrs):
   - `templates/anti-pattern-minimal.md` - Minimum viable anti-pattern section (3 anti-patterns)
   - `templates/anti-pattern-comprehensive.md` - Full section with 5+ anti-patterns
   - `templates/anti-pattern-examples.md` - Extracted examples from A-grade skills
   - Include: structure (title, why, bad, good, impact), tone guidelines, length targets

3. **Validation** (1hr):
   - Apply minimal template to 1 C+ skill (test case)
   - Verify dimensional improvement (D3 score should increase)
   - Document learnings and refine templates

**Output Files:**
- `.context/templates/anti-pattern-minimal.md`
- `.context/templates/anti-pattern-comprehensive.md`
- `.context/templates/anti-pattern-examples.md`
- `.context/remediation/template-validation-results.md`

**Success Criteria:**
- [ ] 3 reusable templates created
- [ ] Templates validated on 1 test skill
- [ ] D3 score improved by 2-4 points on test skill
- [ ] Documented in progress tracker

---

### Task 5.2: Bottom 10 Skill Remediation (10-20 hours)

**Goal:** Lift all C-grade skills (score <90) to B-grade minimum (96/120).

**Priority Order (by score, lowest first):**

| Rank | Skill | Current | Target | Estimated Time | Critical Issues |
|------|-------|---------|--------|----------------|-----------------|
| 1 | github-copilot-models | 87/120 | 96/120 | 2-3hrs | D2, D3, D5 all weak |
| 2 | cfn-template-compare | 88/120 | 96/120 | 2-3hrs | D3 needs major work |
| 3 | journal-entry-creator | 88/120 | 96/120 | 1-2hrs | Thin content, weak patterns |
| 4 | create-context-file | 89/120 | 96/120 | 1-2hrs | D2, D3 below 60% |
| 5 | extending-nx-plugins | 89/120 | 96/120 | 2-3hrs | Progressive disclosure poor |

**Per-Skill Workflow:**

1. **Read current state** (15min):
   - Review `.context/audits/{skill}/latest/analysis.md`
   - Review `.context/audits/{skill}/latest/remediation-plan.md`
   - Identify top 3 dimensional weaknesses

2. **Apply targeted fixes** (1-2hrs):
   - Add anti-pattern section using minimal template (D3 fix)
   - Restructure with progressive disclosure (D5 fix)
   - Add mindset/procedure context where thin (D2 fix)
   - Improve examples and use cases (D7 fix if needed)

3. **Re-audit** (5min):
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh {skill-name} --json --store
   ```

4. **Validate improvement** (10min):
   - Compare old vs new audit scores
   - Verify target achieved (≥96/120)
   - Document changes and impact

5. **Update progress tracker** (5min):
   - Log completion in `.context/remediation/progress-tracker.md`
   - Note actual time vs estimate
   - Record lessons learned

**Output Files:**
- Updated skill SKILL.md files (5 total)
- New audit results in `.context/audits/{skill}/latest/`
- Progress entries in `.context/remediation/progress-tracker.md`

**Success Criteria:**
- [ ] All 5 C-grade skills reach ≥96/120 (B-grade)
- [ ] Average improvement: +7-9 points per skill
- [ ] D3 scores: all ≥10/15 (67%+)
- [ ] Documented in progress tracker

---

### Task 5.3: Systematic D3 Improvements (20-30 hours)

**Goal:** Add comprehensive anti-pattern sections to all C+/C skills (18 remaining after bottom 5 fixed) to address critical 68% D3 weakness.

**Target Skills (C+ grade, 90-95/120):**

Batch these into groups of 5-6 for manageable sessions:

**Batch A** (5-6hrs):
- conventional-commits (90/120)
- k8s-debug (91/120)
- azure-pipelines-validator (92/120)
- commanderjs (92/120)
- gitlab-ci-validator (92/120)
- ansible-generator (93/120)

**Batch B** (5-6hrs):
- snyk-security (93/120)
- cicd-pipeline-review (94/120)
- dotenvx-complete (94/120)
- github-actions-generator (94/120)
- gitlab-ci-generator (94/120)
- terraform-generator (94/120)

**Batch C** (5-6hrs):
- bun-development (95/120)
- github-actions-validator (95/120)
- jenkinsfile-generator (95/120)
- jenkinsfile-validator (95/120)
- mise-complete (95/120)
- typescript-advanced (95/120)

**Per-Batch Workflow:**

1. **Batch setup** (30min):
   - Review all remediation plans for batch
   - Identify common D3 patterns
   - Select appropriate anti-pattern template (minimal vs comprehensive)

2. **Apply template to each skill** (45-60min per skill):
   - Read current SKILL.md
   - Add anti-pattern section using template
   - Customize examples for skill domain
   - Ensure 3-5 anti-patterns documented
   - Include: why it fails, what to do instead, impact

3. **Batch re-audit** (10min):
   ```bash
   for skill in batch-list; do
     sh skills/skill-quality-auditor/scripts/evaluate.sh $skill --json --store
   done
   ```

4. **Batch validation** (30min):
   - Compare scores before/after for all skills in batch
   - Verify D3 improvements (target: +2-4 points each)
   - Check for any regressions in other dimensions
   - Document batch results

5. **Update progress tracker** (15min):
   - Log batch completion
   - Record average improvement
   - Note any challenges or learnings

**Output Files:**
- Updated SKILL.md files (18 total)
- New audit results for all remediated skills
- Progress entries per batch
- `.context/remediation/systematic-d3-results.md` - Summary of all improvements

**Success Criteria:**
- [ ] 18 C+ skills improved with anti-pattern sections
- [ ] D3 average: 68% → 75%+ across remediated skills
- [ ] 50% of C+ skills (9 skills) reach B-grade (96+)
- [ ] No regressions in other dimensions
- [ ] Documented in progress tracker

---

## Progress Tracking

### Master Progress Tracker

Create `.context/remediation/progress-tracker.md` with:

```markdown
# Phase 5 Remediation Progress Tracker

**Started:** [date]
**Target Completion:** [date]
**Status:** In Progress

## Overall Metrics

| Metric | Baseline (Phase 3) | Current | Target | Progress |
|--------|-------------------|---------|--------|----------|
| Average Score | 98.3/120 (82%) | [TBD] | 105/120 (88%) | 0% |
| A-grade Count | 5 | [TBD] | 10-12 | 0% |
| C+/C Count | 23 | [TBD] | 5-8 | 0% |
| D3 Average | 68% | [TBD] | 80%+ | 0% |

## Task Completion

- [ ] Task 5.1: Anti-Pattern Template Library (0/3 templates)
- [ ] Task 5.2: Bottom 10 Remediation (0/5 skills)
- [ ] Task 5.3: Systematic D3 - Batch A (0/6 skills)
- [ ] Task 5.3: Systematic D3 - Batch B (0/6 skills)
- [ ] Task 5.3: Systematic D3 - Batch C (0/6 skills)

## Skill-by-Skill Log

### github-copilot-models
- **Status:** Pending
- **Baseline:** 87/120 (C)
- **Target:** 96/120 (B)
- **Start:** [date]
- **Completed:** [date]
- **Final Score:** [TBD]
- **Improvement:** [+X points]
- **Time Spent:** [Xhrs]
- **Key Changes:** [list]

[Repeat for each skill]

## Lessons Learned

[Document challenges, successful patterns, time-saving techniques]
```

### Session Logs

For each remediation session, create:
- `.context/remediation/session-YYYY-MM-DD-N.md`
- Log: skills worked on, time spent, challenges, scores achieved
- Update master progress tracker at end of session

---

## Validation & Quality Gates

### After Each Skill Remediation

1. **Re-audit required:**
   ```bash
   sh skills/skill-quality-auditor/scripts/evaluate.sh {skill-name} --json --store
   ```

2. **Score verification:**
   - Must achieve target score (≥96 for C-grade, +2-4pts for C+)
   - No regressions in strong dimensions (D1, D4, D7, D8)
   - D3 improvement must be measurable (+2pts minimum)

3. **Quality checks:**
   - Anti-pattern section includes 3-5 patterns
   - Each pattern has: why/bad/good/impact structure
   - Examples are skill-specific, not generic
   - Tone matches existing skill voice

### After Each Batch

4. **Batch summary:**
   - Average improvement across batch
   - Success rate (% achieving target)
   - Time efficiency (actual vs estimated)
   - Pattern identification (common fixes)

5. **Progress tracker update:**
   - Update overall metrics
   - Mark completed skills
   - Adjust remaining estimates based on learnings

---

## Risk Management

### Identified Risks

1. **Remediation Fatigue**
   - **Risk:** 23 skills is large scope, may lose momentum
   - **Mitigation:** Break into 5-6 sessions, celebrate quick wins, prioritize bottom 10 first

2. **Score Regression**
   - **Risk:** Fixing D3 may hurt other dimensions if done carelessly
   - **Mitigation:** Review entire skill after changes, re-audit verifies no regressions

3. **Template Rigidity**
   - **Risk:** Templates too generic, don't fit all skill types
   - **Mitigation:** Create 2 template variants (minimal/comprehensive), customize per skill

4. **Time Overrun**
   - **Risk:** 40-60hr estimate may be optimistic
   - **Mitigation:** Track actual time per skill, adjust batch sizes, prioritize high-impact skills

### Early Warning Signs

Watch for:
- Skill remediation taking >2hrs (may need simpler approach)
- D3 improvements <2pts (template not effective)
- Other dimensions dropping (need more careful editing)
- Burnout/rushing (take breaks, smaller batches)

---

## Success Criteria (Phase 5 Complete)

### Quantitative Targets

- [ ] Average score: ≥105/120 (88%+)
- [ ] A-grade count: ≥10 skills
- [ ] C+/C count: ≤8 skills (65% reduction)
- [ ] D3 average: ≥80% (from 68%)
- [ ] Bottom 10: all ≥96/120 (B-grade)

### Qualitative Targets

- [ ] Anti-pattern template library created and validated
- [ ] All C+/C skills have comprehensive anti-pattern sections
- [ ] Progress tracker maintained throughout
- [ ] Lessons learned documented
- [ ] Process improvements identified for future remediation

### Deliverables

- [ ] 23 updated SKILL.md files (5 C-grade + 18 C+ grade)
- [ ] 3 anti-pattern templates
- [ ] Complete progress tracker with session logs
- [ ] Phase 5 completion summary document
- [ ] Updated repository average in README.md

---

## Estimated Timeline

**Session-based breakdown:**
- Session 1 (2-3hrs): Templates + 2 C-grade skills
- Session 2 (2-3hrs): 3 C-grade skills
- Session 3 (3-4hrs): Batch A (6 C+ skills)
- Session 4 (3-4hrs): Batch B (6 C+ skills)
- Session 5 (3-4hrs): Batch C (6 C+ skills)
- Session 6 (1hr): Verification, documentation, wrap-up

**Total:** 14-21 hours active work (within 40-60hr estimate including planning overhead)

---

## Next Steps

To begin Phase 5:

1. Create progress tracker: `.context/remediation/progress-tracker.md`
2. Start Session 1: Task 5.1 (anti-pattern templates)
3. Commit templates before starting skill remediation
4. Begin Task 5.2 with github-copilot-models (lowest score)

**Phase 5 Status:** Ready to begin after Phase 4 commit

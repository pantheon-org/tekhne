---
title: Prioritizing Requirements with MoSCoW
description:
  Skill for using the MoSCoW framework to categorize and prioritize requirements for effective project delivery
type: skill
category: Development
version: 1.0.0
tags:
  - prioritization
  - requirements
  - project-management
  - agile
  - scope-management
last_updated: 2026-01-26
---

# Skill: Prioritizing Requirements with MoSCoW

This skill guides you through using the MoSCoW framework to prioritize requirements, prevent scope creep, and deliver
maximum value within constraints.

## When to Use This Skill

Use this skill when you need to:

- Start a new project or feature
- Plan sprint backlogs
- Manage stakeholder expectations
- Make resource allocation decisions
- Prevent scope creep
- Define minimum viable product (MVP)
- Facing time, budget, or resource constraints

## Prerequisites

- List of all potential requirements/features
- Understanding of project goals and success criteria
- Knowledge of time, budget, and resource constraints
- Access to key stakeholders for decision-making

## Understanding MoSCoW

**MoSCoW** is a prioritization framework that categorizes requirements into four groups:

- **M**ust have
- **S**hould have
- **C**ould have
- **W**on't have (this time)

**Purpose:** Focus teams on what truly matters and prevent scope creep.

**Origin:** Created by Dai Clegg at Oracle, detailed in the Dynamic System Development Method (DSDM) handbook.

## Step-by-Step Guide

### Step 1: Gather All Requirements

Collect everything stakeholders want, regardless of priority.

**Sources to consult:**

- [ ] Stakeholders and executives
- [ ] End users and user research
- [ ] Product vision and strategy
- [ ] Technical team input
- [ ] Market analysis and competitive research
- [ ] Legal and compliance requirements
- [ ] Previous feedback and support tickets

**Capture format:**

- Write each requirement as a clear statement
- Include who requested it
- Note any dependencies
- Record initial reasoning or context

**Example requirements list:**

```
1. User authentication with email/password
2. Social media login (Google, Facebook)
3. Password strength validation
4. Remember me functionality
5. Two-factor authentication
6. Biometric authentication
7. Single sign-on with enterprise systems
8. Password recovery via email
9. Password recovery via SMS
10. Account lockout after failed attempts
```

### Step 2: Define Success Criteria

Before categorizing, align on what success looks like.

**Questions to answer:**

**Project Scope:**

- What defines a successful launch?
- What is the minimum viable product (MVP)?
- Who is the target audience for this release?

**Constraints:**

- What is the available timeline?
- What is the budget?
- What resources (people, technology) are available?

**Business Goals:**

- What problem are we solving?
- What value must we deliver?
- What differentiates us from competitors?

**Risk Tolerance:**

- What are we willing to defer?
- What can we build iteratively?
- What technical debt can we accept?

**Document these answers** before moving to Step 3.

### Step 3: Categorize Each Requirement

For each requirement, work through the decision tree to assign a category.

#### Decision Tree

For **each requirement**, ask these questions in order:

**Question 1: Is it legally or contractually required?**

- YES → **Must Have**
- NO → Continue

**Question 2: Will the product be useless without it?**

- YES → **Must Have**
- NO → Continue

**Question 3: Does core functionality break without it?**

- YES → **Must Have**
- NO → Continue

**Question 4: Is it critical for market entry or competition?**

- YES → **Must Have**
- NO → Continue

**Question 5: Does it add significant value?**

- YES → Continue
- NO → Skip to Question 8

**Question 6: Can we launch without it and add it later?**

- YES → Continue
- NO → **Must Have**

**Question 7: Would users notice and care if missing?**

- YES, significantly → **Should Have**
- YES, but not critical → **Could Have**
- NO → Skip to Question 8

**Question 8: Is it a nice-to-have enhancement?**

- YES → **Could Have**
- NO → Continue

**Question 9: Is it out of scope for this release?**

- YES → **Won't Have**
- NO → Review the questions again

#### Challenge Questions for "Must Haves"

When someone insists something is a Must Have, ask:

1. **"What happens if we launch without this?"**
   - If answer describes inconvenience, not failure → Likely Should Have

2. **"Can we provide a workaround or manual process temporarily?"**
   - If yes → Likely Should Have or Could Have

3. **"Is this required by law or regulation?"**
   - If no → Continue challenging

4. **"Would any users successfully use the product without this?"**
   - If yes → Likely Should Have

5. **"Can this wait for version 1.1?"**
   - If yes → Likely Should Have or Could Have

### Step 4: Understand Each Category

#### Must Have

**Definition:** Non-negotiable requirements without which the project fails.

**Characteristics:**

- Core functionality that defines the offering
- Bare minimum for viable product
- Legal or compliance requirements
- Critical for market entry
- Product is useless without these

**Typical allocation:** ~50-60% of total effort

**Examples:**

- User authentication for a secure app
- Payment processing for e-commerce
- Data encryption for healthcare (HIPAA)
- Basic CRUD operations for database app
- Core search for search engine

**Red flags:**

- Everything marked as Must Have → Challenge each rigorously
- Must Haves exceed 60% → Rethink scope or break into phases
- Must Haves are vague → Define more specifically

#### Should Have

**Definition:** Important but not critical for immediate success. Add significant value but can be delayed.

**Characteristics:**

- Adds significant value
- Can be delayed without breaking core
- Missing it requires adjustments or workarounds
- Important for user satisfaction
- Product works without them but may be less competitive

**Typical allocation:** ~20-30% of total effort

**Examples:**

- Third-party tool integrations
- Personalized user experience features
- Advanced reporting capabilities
- Multi-language support
- Export to multiple formats (CSV, PDF, Excel)
- Advanced search filters

**Red flags:**

- Should Haves stakeholders insist are Must Haves → Clarify true impact
- Too many Should Haves → May indicate scope creep
- Should Haves blocking other work → Might actually be Must Haves

#### Could Have

**Definition:** Nice-to-have features that improve experience but aren't essential.

**Characteristics:**

- Wish-list items
- First to be cut if resources constrained
- Enhances but doesn't define product
- Lower impact if omitted
- "Nice surprises" if delivered

**Typical allocation:** ~10-20% of total effort

**Examples:**

- Social media integration
- Advanced customization (themes, layouts)
- Gesture controls
- Animations and transitions
- Easter eggs or delightful micro-interactions
- Advanced analytics dashboard

**Red flags:**

- Could Haves taking too much time → Deprioritize or remove
- Could Haves blocking Must Haves → Remove from current scope
- Disagreement on Could vs. Should → Discuss business value

#### Won't Have (This Time)

**Definition:** Explicitly out of scope for the current timeframe.

**Characteristics:**

- Not valuable enough for current release
- Too resource-intensive to justify now
- May be considered for future releases
- Helps prevent scope creep
- Makes boundaries clear

**Purpose:**

- Managing stakeholder expectations
- Preventing scope creep
- Maintaining focus
- Documenting conscious decisions
- Providing "parking lot" for future ideas

**Examples:**

- Full smartphone replacement capabilities
- Advanced AI requiring extensive R&D
- Features requiring unavailable infrastructure
- Capabilities conflicting with current positioning
- Features for different market segments
- Low-demand third-party integrations

### Step 5: Validate and Balance

Review the categorization to ensure it's realistic and balanced.

#### Effort Distribution Check

**Target distribution:**

- Must Haves: ~50-60% of total effort
- Should Haves: ~20-30% of total effort
- Could Haves: ~10-20% of total effort
- Won't Haves: Explicitly documented

**If Must Haves exceed 60%, you must:**

1. Challenge each Must Have rigorously (use Step 3 questions)
2. Increase resources (more people, time, budget)
3. Extend timeline
4. Break into multiple releases/phases

#### Dependencies Check

- [ ] Review dependencies between items
- [ ] Ensure Must Haves don't depend on Should/Could Haves
- [ ] Identify Should Haves that enable Must Haves
- [ ] Document dependency chains

#### Reality Check

**Ask:**

- Can we actually deliver all Must Haves in the timeframe?
- Do we have skills/resources for the Must Haves?
- Are we being honest about what's truly required?
- Have we challenged enough items down from Must Have?

### Step 6: Reach Consensus

Ensure all key stakeholders agree before proceeding.

**Stakeholders to involve:**

- [ ] Product owner/manager
- [ ] Technical lead
- [ ] Key business stakeholders
- [ ] Development team representatives
- [ ] QA/testing representatives
- [ ] Executive sponsor (for major decisions)

**Consensus process:**

1. **Present the categorization**
   - Show the full MoSCoW breakdown
   - Present effort distribution
   - Highlight dependencies

2. **Review conflicts**
   - Where do stakeholders disagree?
   - What items are borderline?
   - What assumptions need validation?

3. **Resolve conflicts by:**
   - Discussing business value and user impact
   - Reviewing constraints (time, budget, resources)
   - Using data to support decisions
   - Applying the challenge questions
   - Voting if necessary (product owner breaks ties)

4. **Document decisions**
   - Record rationale for each category assignment
   - Note dissenting opinions
   - Capture assumptions made
   - Document what would change the decision

### Step 7: Document and Communicate

Create clear documentation that everyone can reference.

#### MoSCoW Table

Create a table like this:

| Must Have      | Should Have         | Could Have    | Won't Have     |
| -------------- | ------------------- | ------------- | -------------- |
| Critical req 1 | Important feature 1 | Enhancement 1 | Out of scope 1 |
| Critical req 2 | Important feature 2 | Enhancement 2 | Out of scope 2 |
| Critical req 3 | Important feature 3 | Enhancement 3 | Out of scope 3 |

#### Documentation to create:

**1. MoSCoW Summary Document**

```markdown
# [Project Name] - MoSCoW Prioritization

## Project Context

- **Timeline:** [dates]
- **Resources:** [team size, budget]
- **Goal:** [what success looks like]

## Must Have (60%)

1. [Requirement]
   - **Rationale:** [Why it's Must Have]
   - **Effort:** [estimate]
   - **Dependencies:** [list]

## Should Have (25%)

[Same format]

## Could Have (15%)

[Same format]

## Won't Have (This Release)

[Same format, note when it might be reconsidered]

## Assumptions

- [List key assumptions]

## Risks

- [List risks to delivery]
```

**2. Communication Plan**

- Share with all team members
- Post in visible location (wiki, project board)
- Reference in sprint planning
- Review in retrospectives

### Step 8: Review and Adjust

MoSCoW is not static—review and adjust as needed.

**When to review:**

- During backlog grooming (weekly/bi-weekly)
- At sprint planning (each sprint)
- When constraints change (budget, timeline, resources)
- When priorities shift (market changes, new information)
- When blockers emerge (technical challenges, dependencies)

**What can change:**

- Should Haves can become Must Haves (if critical)
- Must Haves can be split (MVP first, enhancements later)
- Could Haves can be moved to Won't Have (if time is short)
- Won't Haves can become Could Haves (if resources free up)

**What should NOT change without serious justification:**

- Adding new Must Haves mid-sprint
- Moving Won't Haves to Must Haves without removing others
- Expanding scope without adjusting timeline/resources

## Common Patterns by Project Type

### Pattern 1: New Product Launch (MVP)

**Goal:** Prove concept and get to market quickly

**Distribution:**

- Must Have: 60% - Core features that prove the concept
- Should Have: 25% - Features that improve adoption
- Could Have: 15% - Differentiators
- Won't Have: Everything else for v2

**Example: Task Management App**

| Must Have              | Should Have     | Could Have      | Won't Have           |
| ---------------------- | --------------- | --------------- | -------------------- |
| Create/view/edit tasks | Task categories | Color coding    | Team collaboration   |
| Mark tasks complete    | Due dates       | Task templates  | Time tracking        |
| Task list view         | Search tasks    | Task priorities | Calendar integration |
| User authentication    | Task notes      | Export to PDF   | Mobile app           |

### Pattern 2: Feature Enhancement

**Goal:** Improve existing functionality

**Distribution:**

- Must Have: 40% - Core enhancement
- Should Have: 35% - Related improvements
- Could Have: 25% - Nice additions
- Won't Have: Features requiring architectural changes

**Example: Adding Search Feature**

| Must Have       | Should Have       | Could Have      | Won't Have        |
| --------------- | ----------------- | --------------- | ----------------- |
| Keyword search  | Search filters    | Search history  | AI-powered search |
| Display results | Sort results      | Save searches   | Voice search      |
| Pagination      | Highlight matches | Recent searches | Natural language  |

### Pattern 3: Bug Fixes

**Distribution:**

- Must Have: Critical bugs affecting all users
- Should Have: High-priority bugs affecting many users
- Could Have: Low-priority bugs or cosmetic issues
- Won't Have: Won't-fix bugs or design limitations

### Pattern 4: Compliance Project

**Distribution:**

- Must Have: 90% - All legal/regulatory requirements
- Should Have: 10% - Nice-to-have compliance features
- Could Have: 0% - No room for extras
- Won't Have: Non-compliance features

## Real-World Example: E-commerce Checkout

**User Story:** Build a checkout flow for e-commerce platform

**Project Context:**

- Timeline: 6 weeks
- Team: 4 developers, 2 QA
- Goal: Enable customers to complete purchases

### Categorization Process

**1. Brainstormed Requirements (20 total)**

- Add items to cart
- Remove items from cart
- Update quantities
- Enter shipping address
- Address autocomplete
- Multiple shipping addresses
- Select payment method
- Store payment methods
- Credit card payment
- PayPal payment
- Apple Pay
- Google Pay
- Cryptocurrency payment
- Complete purchase
- View order summary
- Email confirmation
- SMS confirmation
- Voice confirmation
- SSL encryption
- Guest checkout
- One-click reorder
- Wishlist functionality
- Gift wrapping
- Gift messages
- Social shopping features
- Shipping tracking
- Delivery time estimates
- Same-day delivery
- Return/refund from checkout

**2. Applied Decision Tree**

**Must Have (9 items, ~60% effort):**

- Add items to cart
- Remove items from cart
- Update quantities
- Enter shipping address
- Select payment method (credit card minimum)
- Complete purchase
- View order summary
- SSL encryption
- Email confirmation

**Should Have (5 items, ~25% effort):**

- Address autocomplete
- Store payment methods
- PayPal payment
- Guest checkout
- Shipping tracking

**Could Have (6 items, ~15% effort):**

- Multiple shipping addresses
- Apple Pay / Google Pay
- SMS confirmation
- One-click reorder
- Gift messages
- Delivery time estimates

**Won't Have (This Release):**

- Cryptocurrency payment
- Voice confirmation
- Wishlist functionality (separate feature)
- Social shopping features
- Gift wrapping
- Same-day delivery
- Return/refund from checkout

**3. Resulting Prioritized Backlog**

Phase 1 (MVP - 6 weeks): All Must Haves Phase 2 (Enhancement - 3 weeks): Should Haves Phase 3 (Polish - 2 weeks): Could
Haves Future Consideration: Won't Haves

## Troubleshooting

### Issue: Everything is "Must Have"

**Symptoms:** 80%+ of requirements marked as Must Have

**Solution:**

1. Apply the challenge questions ruthlessly
2. Ask: "Can we launch with a workaround?"
3. Define MVP more narrowly
4. Consider phased releases
5. Get executive decision on scope vs. timeline

**Example conversation:**

```
Stakeholder: "We MUST have social login!"
You: "What happens if we launch without it?"
Stakeholder: "Users will have to create accounts manually."
You: "Can they still use the product?"
Stakeholder: "Yes, but..."
You: "Then it's a Should Have that improves UX, not a Must Have."
```

### Issue: Scope creep via "Should Haves"

**Symptoms:** Should Haves keep expanding, timeline slipping

**Solution:**

1. Set fixed percentage allocations (60/20/20)
2. Enforce timeboxing—when Must Haves are done, stop
3. Move overflow Should Haves to next iteration
4. Track velocity and adjust expectations

### Issue: Disagreement on categories

**Symptoms:** Stakeholders can't agree on priorities

**Solution:**

1. Return to success criteria—what defines success?
2. Use voting (each stakeholder gets X votes)
3. Product owner makes final call
4. Document dissenting opinions
5. Time-box the discussion

### Issue: Ignoring "Won't Haves"

**Symptoms:** Previously rejected features keep reappearing

**Solution:**

1. Document Won't Haves explicitly
2. Include rationale for exclusion
3. Reference Won't Haves when features reappear
4. Create a backlog for future consideration
5. Communicate Won't Haves to all stakeholders

### Issue: Static prioritization

**Symptoms:** MoSCoW never updated, doesn't reflect reality

**Solution:**

1. Schedule regular reviews (weekly grooming)
2. Adjust based on learnings and feedback
3. Re-prioritize when constraints change
4. Track what changes and why

## Quick Reference

### Decision Quick Guide

**Is it a Must Have?**

- [ ] Required by law/regulation?
- [ ] Product useless without it?
- [ ] Core functionality breaks without it?
- [ ] No workaround possible?

If all YES → Must Have If any NO → Continue to Should Have

**Is it a Should Have?**

- [ ] Adds significant value?
- [ ] Can be delayed without breaking core?
- [ ] Users would notice if missing?

If all YES → Should Have If any NO → Continue to Could Have

**Is it a Could Have?**

- [ ] Nice to have?
- [ ] Enhances but doesn't define?
- [ ] Can be easily delayed?

If all YES → Could Have If any NO → Won't Have

### Target Effort Distribution

```
Must Have:   50-60% ████████████░░░░░░░░
Should Have: 20-30% ████░░░░░░░░░░░░░░░░
Could Have:  10-20% ██░░░░░░░░░░░░░░░░░░
Won't Have:  0%     (Documented but not built)
```

### Integration with Agile

**Sprint Planning:**

1. All Must Haves in early sprints
2. Should Haves after Must Haves complete
3. Could Haves only if time permits
4. Won't Haves stay out of scope

**Acceptance Criteria:**

- Must Haves → Core acceptance criteria
- Should Haves → Optional criteria for current sprint
- Could Haves → Future enhancements
- Won't Haves → Explicitly out of scope

## Related Resources

- [MoSCoW Prioritization (Product Plan)](https://www.productplan.com/glossary/moscow-prioritization/)
- [Dynamic System Development Method (DSDM)](https://www.agilebusiness.org/)
- Complementary frameworks: Eisenhower Matrix, Kano Model

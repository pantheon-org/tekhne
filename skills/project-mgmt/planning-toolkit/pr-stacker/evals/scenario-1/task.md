# Scenario 1: Branch Naming Quality

## Context

A developer has agreed to split their branch into three groups. They suggest using
"wave" naming for the branches. The agent must redirect to descriptive naming.

## Agreed grouping

| PR | Commits | Concern |
|---|---|---|
| 1 | `a1b2c3`, `d4e5f6` | Infrastructure (CDK stack + IAM role) |
| 2 | `g7h8i9`, `j0k1l2`, `m3n4o5`, `p6q7r8`, `s9t0u1`, `v2w3x4` | Application logic + tests |
| 3 | `y5z6a7`, `z8a9b0` | Housekeeping |

## Developer's suggested branch names

- PR 1: `feature/PROJ-42-wave-1`
- PR 2: `feature/PROJ-42-wave-2`
- PR 3: `feature/PROJ-42-wave-3`

## Task

The developer is proposing these branch names. Evaluate them and either accept or
propose better alternatives with an explanation.

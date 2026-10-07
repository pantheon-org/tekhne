# Escalators

An escalator is a signal that the task needs more capability than its task type suggests. Move up exactly one tier when any escalator fires. Cap at Opus. Several escalators firing together raise confidence in the move; they do not stack into a second tier.

## Technical

| Signal | Fires when | Does not fire when |
|---|---|---|
| **Ambiguity** | Requirements are underspecified, or the task has several valid readings that lead to different work | The user gave exact inputs and a single expected output |
| **Scope** | The change touches 3 or more files, systems or components | One file changes many times, such as a mechanical rename within it |
| **Stakes** | Production systems, security, data loss or regulatory compliance are at risk | The work runs in a sandbox, a throwaway branch or a draft |
| **Novelty** | No established pattern exists, or the technology is new to the team | A well-documented pattern applies, even in an unfamiliar codebase |

## Business

| Signal | Fires when | Does not fire when |
|---|---|---|
| **Stakeholders** | Competing interests need balancing | Several readers receive the same message and nobody has a conflict |
| **Irreversibility** | The consequences are long-term, strategic or organisational | The output is a draft that someone reviews before any decision |
| **Sensitivity** | Layoffs, restructuring, executive messaging or crisis response are involved | Routine internal announcements |
| **Synthesis** | The work crosses domains, such as tech plus business plus legal | One domain with a vocabulary borrowed from another |

## Cognitive

| Signal | Fires when | Does not fire when |
|---|---|---|
| **Pattern detection** | The task analyses trends across many data points or sessions | A single table is summarised |
| **Bias identification** | The task must spot blind spots, cognitive biases or hidden assumptions | The user lists known assumptions to format |
| **Ethical reasoning** | Moral ambiguity, fairness or unintended consequences matter | Policy text is only reformatted |
| **Multi-framework** | Two or more analytical frameworks apply at the same time | One named framework is applied once |

## Borderline readings

- **Stakes outranks task type.** A typo fix in a production payment configuration is Haiku work by type and Sonnet work by stakes.
- **Scope counts independent units.** Fifteen files changed by one mechanical codemod are still one decision, so Scope may not fire. Fifteen files with distinct logic fire it.
- **Volume is not an escalator.** A long document needs the tier its reasoning demands. Length alone does not move it.
- **Cap at Opus.** Never recommend a tier above the top one, and do not add a second move because a fourth signal fired.
- **Name the signal.** Every escalated recommendation must state which signal moved it, so the user can disagree with a specific claim.

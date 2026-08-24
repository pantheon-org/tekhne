# Session-End Reflection Reference

Technique origins, rationale, and research backing for the two-question reflection.

## Technique Origin

This technique was described in a Reddit comment (r/ClaudeAI, 2025) by a user combining two question sources:

1. **Confidence audit** — Suggested by Claude itself when asked "how can we catch blind spots before wrapping up?"
2. **Blind-spot check** — Attributed to Sam Altman's decision-making framework

The user reported: *"I would say one out of four times one of the items is a huge deal and you're shocked that the AI even took action without understanding this first."*

## Why These Two Questions Work

They are complementary:

| Question | Direction | What it catches | Blind spot type |
|----------|-----------|-----------------|-----------------|
| "What are you least confident about?" | Agent → introspective | Under-investigated code paths, skipped verification, shallow searches | Commission (did something badly) |
| "What am I missing?" | Agent → user perspective | Assumptions not challenged, alternatives not explored, signals dropped | Omission (didn't consider something) |

## The ~1 in 4 Statistic

The claim that ~1 in 4 sessions surfaces a critical gap is consistent with known LLM failure patterns:

- **Overconfidence bias**: LLMs systematically overrate their own completeness (similar to the Dunning-Kruger effect in humans)
- **Recency bias**: The agent weights the last few actions heavily, forgetting earlier unresolved threads
- **Satisfaction of search**: Once the agent finds a plausible answer, it stops looking for better ones

The reflection bypasses all three by forcing an explicit confidence assessment before closure.

## Applying in Practice

| Scenario | Typical catch |
|----------|---------------|
| Code generation | "I assumed your codebase uses X pattern but only checked one file" |
| Debugging | "I never verified the input to function Y — the bug might be upstream" |
| Research | "I only considered approach A and B but not C, which a quick search would have found" |
| Configuration | "I set value Z based on a Stack Overflow answer without checking your actual version" |
| Architecture | "I didn't consider how this interacts with the auth layer" |

## Related Concepts

- **Premortem** (Klein, 2007): "Imagine the project has failed — what went wrong?" The confidence audit is a lightweight premortem focused on the agent's own work.
- **Lateral thinking** (De Bono): The blind-spot check forces a perspective shift from "what did I do" to "what am I not seeing."
- **Red teaming**: Having a second pass over work with an adversarial mindset. The agent red-teams its own output.

# Worked Examples

Three end-to-end framings. Each shows the signal that drove the classification, the check that guarded against a wrong route, and the handoff template used.

## 1. Degraded service: auto-classify, challenge, route

**Input:** "Our API endpoint /v2/payments has been returning 500 errors since the deploy at 14:30. It was working fine this morning."

1. The phrase "was working, now failing" with a time-bound trigger is explicit Degraded language. Confidence is above 80%, so Step 0 proposes Complicated, Degraded.
2. Adjacent Domain Challenge: could this be Complex, with an unpredictable failure mode? No. The system is known, the inputs are repeatable and the regression follows a deploy, so cause and effect are discoverable by analysis.
3. Route: `troubleshoot` → stabilise → re-frame, OpenSpec No. Fill `frame-problem-to-troubleshoot-llm.md`, then confirm.

**Wrong turns to avoid:** routing to `brainstorm` because the cause is unknown (an unknown cause is not an unknowable one), or classifying Complex because the word "errors" sounds alarming.

## 2. Solution-shaped request with expertise bias

**Input:** "We need a microservice for notifications. We have built three of these."

1. Restate the problem first: "Notifications are coupled to the order flow and block releases" is a need; "a microservice" is a solution. Ask the user to confirm the restatement.
2. Triangulate. T1=2 (the team has built these), T2=unordered (delivery behaviour varies heavily by customer segment), T3=entangled (preferences, channels and rate limits depend on each other).
3. Two of three tests say Complex. T1 alone cannot override them, so the verdict is Complex with a liminal note that some governing constraints exist.
4. No hypothesis exists yet, so route `brainstorm` → `probe` → `openspec-plan`, OpenSpec Yes. Fill `frame-problem-to-brainstorm-llm.md`.

**Wrong turn to avoid:** accepting Complicated because "we have done this three times". Three prior builds prove that expertise exists, not that this build is predictable.

## 3. Composite request

**Input:** "Improve developer experience, cut CI build times, fix auth service tech debt, and maybe move database."

1. Auto-classification confidence is low and the scope is broad. Write 2-3 clarifying sub-questions ("Which developers? What is the current build time and the target? What is driving the database question?") and ask them with T1 to T3 and Q-Scale in one call.
2. The answers disagree across the parts, so decompose:

```text
🧩 Composite problem, sub-parts in different domains:
├── Database move: Complex, no hypothesis → brainstorm → probe → openspec-plan
├── Developer experience: Complex, enabling constraints → probe
├── CI build times: Complicated, Degraded → troubleshoot
└── Auth tech debt: Complicated, Evolving → investigate
Proposed sequence: nothing blocks anything else, so Complex parts first
```

3. Confirm the decomposition with the user before routing, then invoke the first skill with the first sub-problem only.

**Sequencing rule:** a blocker goes first (for example a failing build that stops all other work), otherwise the Complex parts go first because probes take longest to return signal.

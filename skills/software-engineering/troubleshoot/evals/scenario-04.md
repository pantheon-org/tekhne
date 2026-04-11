# Scenario 04: Runtime Crash with Stack Trace

## User Prompt

"Our Node.js service crashed in production with this stack trace:

```
UnhandledPromiseRejectionWarning: TypeError: Cannot read properties of undefined (reading 'id')
    at UserService.getProfile (src/services/user.service.ts:84:32)
    at async ProfileController.show (src/controllers/profile.ts:21:18)
```"

## Expected Behavior

1. Agent performs a WebSearch: `"Cannot read properties of undefined" "reading 'id'" Node.js UserService fix`.
2. Agent scans for established patterns for undefined access in async service methods.
3. Agent invokes `AskUserQuestion` to qualify: Node.js version, framework, what changed recently (deploy, config, data).
4. Agent checks for empty answers after `AskUserQuestion` (AskUserQuestion guard).
5. Agent applies Diagnose protocol from `references/protocols/diagnose.md`: pattern match for null/undefined propagation, isolation.
6. Agent applies 5 Whys: crash → undefined `id` → `getProfile` received undefined user → why? → traces through async chain.
7. Agent enters OODA loop if diagnosis inconclusive: asks user to run specific checks (e.g., log the value before line 84, check what calls `getProfile`).
8. Agent identifies root cause (e.g., missing null check, wrong async resolution, race condition on user lookup).
9. Agent writes thinking artifact including: Symptoms, Hypotheses tested, OODA loops (if entered), Root cause, Resolution, Cynefin transition if applicable.
10. Agent offers Learn phase.

## Success Criteria

- **Search-first**: Agent searches before qualifying.
- **Stack trace parsed**: Agent identifies `UserService.getProfile:84` and `ProfileController.show:21` as the call chain.
- **5 Whys applied**: Agent drills at least 3 levels from symptom to root cause.
- **Thinking artifact content**: Artifact includes all required sections (Symptoms, Hypotheses tested, Root cause, Resolution).
- **OODA if needed**: Agent enters OODA only after diagnosis phase is exhausted.
- **Cynefin transition noted**: If problem was reclassified (e.g., simple → complicated), artifact records it.
- **Learn phase offered**: Agent asks user whether to save the learning.

## Failure Conditions

- Agent immediately suggests adding a null check at line 84 without investigating why the value is undefined.
- Agent skips the thinking artifact or writes it without the required sections.
- Agent does not check for empty `AskUserQuestion` responses.
- Agent enters OODA before attempting diagnosis with pattern matching and 5 Whys.
- Agent does not offer the Learn phase after resolution.
- Agent provides no Cynefin transition note despite problem complexity shifting during diagnosis.

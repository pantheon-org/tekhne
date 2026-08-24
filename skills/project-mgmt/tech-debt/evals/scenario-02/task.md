# Scenario 02: Misfiled Risk Should Be Redirected, Not Filed as Tech Debt

## User Prompt

"Our SES sending domain doesn't have DMARC configured, so mail to some
recipients might get silently rejected or spam-foldered. Nobody's raised it
formally. Can you just add it to the tech debt list so we don't forget?"

## Expected Behavior

1. Recognize this is NOT genuinely just cleanup -- it's a real risk with a
   cost if left unaddressed (silent delivery failure to real recipients),
   which is exactly the boundary Rule 3 and the When NOT to Use section
   describe.
2. Do NOT file it in `docs/TECH_DEBT.md` just because the user asked to.
3. Explain why: `docs/TECH_DEBT.md` has no append-only guard, so a real risk
   filed there can be silently lost the moment someone deletes the row
   thinking it's resolved.
4. Redirect the user to the `risk-register` skill / `docs/RISK_REGISTER.md`
   instead, and offer to file it there.
5. If the user insists on tech-debt anyway, still push back once with the
   reasoning above before complying (do not silently file it as asked).

## Success Criteria

- Agent identifies the DMARC/deliverability item as a risk, not cleanup.
- Agent declines to file it as a tech-debt row without pushback.
- Agent explains the append-only/no-audit-trail distinction as the reason.
- Agent offers or performs the redirect to `docs/RISK_REGISTER.md` /
  `risk-register` skill.

## Failure Conditions

- Agent files "SES/DMARC deliverability blocked" as a tech-debt row without
  any pushback or explanation, purely because the user asked.
- Agent files it in both docs redundantly without noting that's wrong.
- Agent refuses to help at all instead of redirecting to the correct doc.

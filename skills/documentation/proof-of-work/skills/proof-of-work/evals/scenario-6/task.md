# Scenario 6: Evidence for a Dated Journal Entry in a Project With Its Own Convention

## User Prompt

"I'm documenting today's checkout fix in `2026/10-October/01-Thursday/2026-10-01-checkout-fix.md`. Take a screenshot of the checkout page at https://staging.example.com/checkout with playwright-mcp now that the fix is deployed, and attach it as proof of work. Make sure the image is actually valid before you reference it."

## Repo state

A project that documents work as dated entries. Earlier entries keep their supporting files in a sibling directory named after the entry, such as `2026/09-September/30-Wednesday/2026-09-30-login-fix/assets/`. A `.context/evidence/` folder does not exist yet.

# Incident & Postmortem Context

Not a separate source, a **cross-cutting angle**. Incidents often motivate defensive code ("we added this check after the X outage"), so if the target looks defensive (null checks, retry logic, timeout handling, rate limiting, feature flags), specifically hunt for incident history across every available source:

- **Confluence / Notion**: search for postmortems mentioning the target file, feature, or error string
- **Jira / Linear**: look for tickets labeled `incident`, `sev-*`, `postmortem-action-item`, `reliability`
- **Slack**: search `#sev-*` and `#incident-*` channels around the dates the target code was added
- **Git**: commits with messages like "fix for incident", "add defensive check", "revert" followed by "re-apply with..." are strong signals
- **Infrastructure observability** (if connected): formal incident records with timelines; dashboards and monitors created as postmortem action items
- **Error tracking** (if connected): issues whose first-seen/last-seen window aligns with the target's PR/MR ship date; stack traces through the target
- **Product analytics warehouse** (if connected): product-analytics events that classify an error condition (client-reported failures, user-visible retry events, etc.) often spike during an incident window. A drop in that event count after the target PR/MR ships is circumstantial support that the target code resolved the user-visible symptom, even when other signal is noisy.

If you find an incident link, fetch the full postmortem. Postmortems typically have an "Action Items" section that ties directly to code changes. When multiple sources corroborate (an incident ID appears in a ticket, which appears in a doc, which appears in a chat thread that links to the target PR/MR), the evidence is especially strong.

Worth spending time on when the code's defensive character makes an incident-driven origin plausible. Skip it for code that doesn't look defensive.

In a regulated domain, a regulator-facing incident (a mandatory notification, a data-breach process) is its own strong signal — treat any postmortem tagged with a regulatory dimension as high-value evidence, and flag it explicitly in the synthesis rather than folding it into a generic "reliability" bucket.

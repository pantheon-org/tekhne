# Scenario 04: Produce a Complete MoSCoW Decision Log

## User Prompt

A team has completed a MoSCoW workshop for a mobile app release (v2.0) scheduled for 2026-05-01. The following decisions were reached during the workshop:

- **Push notifications** → Must (release fails without engagement mechanism; owner: @ali)
- **Offline mode** → Should (high value for field workers but app functions online; owner: @sam)
- **Biometric authentication** → Should (users can use password; owner: @ali)
- **Animated onboarding** → Could (nice to have, polish; owner: @design)
- **Apple Watch companion app** → Won't (out of scope for v2.0; revisit for v3.0 in 2026-Q4)
- **Landscape orientation support** → Won't (tablet only, deferred; revisit 2026-08-01)

Produce a complete, stakeholder-ready MoSCoW decision log saved to `decision-log.md`.

The log must include:

1. A header with the release name (v2.0), release date (2026-05-01), and the date the log was created (2026-03-16).
2. A table with columns: Requirement | Category | Business Rationale | Risk if Omitted | Owner | Review Date.
3. All six decisions populated with rationale and risk columns (not blank).
4. Won't items must have an explicit review date.
5. A summary line at the bottom stating Must count, Should count, Could count, Won't count.

## Expected Behavior

1. Include a header with release name (v2.0), release date (2026-05-01), and log creation date (2026-03-16)
2. Populate all six requirements in the table with their correct categories (push notifications Must, offline mode Should, biometric Should, animated onboarding Could, two Won'ts)
3. Provide non-empty business rationale and risk-if-omitted content for every row (not "N/A" or blank)
4. Both Won't items (Apple Watch, landscape) must have explicit review dates matching the provided dates (v3.0 / 2026-Q4 and 2026-08-01)
5. End with a summary line stating the count per tier (Must: 1, Should: 2, Could: 1, Won't: 2)

## Success Criteria

- **Header with release metadata**: `decision-log.md` contains a header with release name (v2.0), release date (2026-05-01), and log creation date (2026-03-16)
- **All six decisions present in table**: The table contains all six requirements with their correct categories (push notifications Must, offline mode Should, biometric Should, animated onboarding Could, two Won'ts)
- **Rationale and risk columns populated**: Every row has non-empty business rationale and risk-if-omitted content (not "N/A" or blank)
- **Won't items have review dates**: Both Won't items (Apple Watch, landscape) have explicit review dates matching the provided dates (v3.0 / 2026-Q4 and 2026-08-01)
- **Summary counts at bottom**: `decision-log.md` ends with a summary line stating the count per tier (Must: 1, Should: 2, Could: 1, Won't: 2)

## Failure Conditions

- Omits release metadata from the header or uses incorrect dates
- Omits one or more of the six requirements from the table
- Leaves rationale or risk cells blank or uses "N/A" as content
- Won't items lack review dates or use dates other than the provided ones
- Omits the summary line at the bottom of the document

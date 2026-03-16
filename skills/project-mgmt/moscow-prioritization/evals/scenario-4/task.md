# Scenario 4: Produce a Complete MoSCoW Decision Log

## Context

A team has completed a MoSCoW workshop for a mobile app release (v2.0) scheduled for
2026-05-01. The following decisions were reached during the workshop:

- **Push notifications** → Must (release fails without engagement mechanism; owner: @ali)
- **Offline mode** → Should (high value for field workers but app functions online; owner: @sam)
- **Biometric authentication** → Should (users can use password; owner: @ali)
- **Animated onboarding** → Could (nice to have, polish; owner: @design)
- **Apple Watch companion app** → Won't (out of scope for v2.0; revisit for v3.0 in 2026-Q4)
- **Landscape orientation support** → Won't (tablet only, deferred; revisit 2026-08-01)

## Your Task

Produce a complete, stakeholder-ready MoSCoW decision log saved to `decision-log.md`.

## Requirements

The log must include:

1. A header with the release name (v2.0), release date (2026-05-01), and the date
   the log was created (2026-03-16).
2. A table with columns: Requirement | Category | Business Rationale | Risk if Omitted | Owner | Review Date.
3. All six decisions populated with rationale and risk columns (not blank).
4. Won't items must have an explicit review date.
5. A summary line at the bottom stating Must count, Should count, Could count, Won't count.

## Output Spec

File: `decision-log.md`

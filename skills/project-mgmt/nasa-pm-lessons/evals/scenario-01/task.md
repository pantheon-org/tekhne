# Scenario 01: Root Cause Section of an Incident Journal Entry

## User Prompt

"I'm writing up the Root Cause section for the ServiceMix broker outage journal entry. We spent two days chasing three different theories before the KahaDB recovery lock was the actual cause. Is there
a lesson here worth noting?"

## Expected Behavior

1. Recognize this as a root-cause / incident-writeup trigger and load `references/design-engineering.md` only — no other category.
2. Select 1-3 aphorisms that genuinely fit the described situation (chasing multiple theories before landing on the real cause).
3. Attribute the citation to Jerry Madden / NASA LLIS #1956.
4. Apply the aphorism explicitly to the KahaDB scenario described, not as a floating, generic quote.
5. Keep the citation to a few lines — do not enumerate the design-engineering category.

## Success Criteria

- Only `references/design-engineering.md` is loaded; no other category file is referenced.
- 1-3 aphorisms are cited, not the full category list.
- The citation names Madden and/or LLIS #1956.
- The response ties the aphorism back to the specific incident (multiple theories, eventual root cause) rather than stating it generically.

## Failure Conditions

- More than one category file is loaded, or the wrong category is used (e.g. Working with Superiors).
- The full design-engineering list, or more than 3 aphorisms, is pasted into the response.
- No attribution to Madden or LLIS #1956.
- The aphorism is stated without being connected to the KahaDB/root-cause detail in the prompt.

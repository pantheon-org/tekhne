# Scenario 05: Write an Executive Risk Summary When Priorities Conflict

## User Prompt

Two critical issues have been escalated simultaneously. You must write a single executive briefing that addresses both. There is no clear single "most important" item — both carry significant risk.

**Issue A:** A data breach was detected in the EU region. Customer PII may be exposed. Legal and PR teams need to be notified. GDPR notification deadline is 72 hours from detection (detected 18 hours ago).

**Issue B:** A supply chain vulnerability (CVE-2024-3094 equivalent) was discovered in a core dependency. Exploitation could allow remote code execution. Patch is available but requires a 4-hour deployment window.

Write an executive briefing that:
1. Applies the conflict fallback rule (lead with risk and decision deadline).
2. Covers both issues without burying either.
3. Enables the executive to immediately understand what decisions are needed and by when.

## Expected Behavior

1. Write `Multiple critical issues — leading with risk and deadlines.` before writing the document
2. Present both the data breach and the security vulnerability prominently near the start without burying either after extensive background
3. Calculate the remaining GDPR notification time (54 hours at time of writing) and state it prominently
4. Explain CVE, PII, remote code execution, and supply chain vulnerability in plain language with acronyms defined on first use
5. Name the responsible team for both issues using `[Owner] must [action] by [deadline]` format

## Success Criteria

- **Conflicting priorities fallback stated explicitly**: Response writes the required prefix `Multiple critical issues — leading with risk and deadlines.` before writing the document.
- **Both issues visible near the opening**: Both the data breach and the security vulnerability appear prominently near the start — neither is buried after extensive background.
- **GDPR deadline calculated and surfaced**: The remaining GDPR notification time (54 hours at time of writing) is calculated and prominently stated.
- **Technical terms translated and acronyms defined**: CVE, PII, remote code execution, and supply chain vulnerability are explained in plain language with acronyms defined on first use.
- **Each issue has owner and deadline**: Both issues name the responsible team using `[Owner] must [action] by [deadline]` format.

## Failure Conditions

- Does not write `Multiple critical issues — leading with risk and deadlines.` before the document
- Either the data breach or the security vulnerability is buried after extensive background text
- Remaining GDPR notification time is not calculated or not prominently stated
- CVE, PII, remote code execution, or supply chain vulnerability are not explained in plain language
- Either issue is missing a named responsible team or a specific deadline

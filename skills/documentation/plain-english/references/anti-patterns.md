# Anti-Patterns Reference

Common mistakes when translating technical content to plain English, with examples and fixes.

## AP-1: Jargon Without Translation

**WHY**: Non-technical readers cannot act on unexplained specialist terms.
**Consequence**: Decision delay, confusion, and loss of trust.

| BAD | GOOD |
|-----|------|
| "The API returns 429 due to upstream throttling." | "The system is receiving too many requests; retry windows are being enforced." |
| "Refactor the monolith into microservices." | "Break the large application into smaller independent services to reduce release risk." |
| "The WAF and IDS flagged anomalous egress." | "The firewall and intrusion detection system flagged unusual outbound traffic." |

## AP-2: Assumed Technical Context

**WHY**: Stakeholders have varying domain familiarity across roles.
**Consequence**: Gaps in understanding lead to misaligned actions.

| BAD | GOOD |
|-----|------|
| "Use RBAC to restrict access." | "Restrict access so each person can only see data their role requires." |
| "The p99 latency is 820ms." | "Response time for 99% of requests is 820ms — well above our 200ms target." |
| "Scale horizontally with read replicas." | "Add additional database servers that handle read-only requests to reduce load on the primary database." |

## AP-3: Buried Key Decision

**WHY**: Time-constrained readers scan only the first section.
**Consequence**: Critical actions are missed or deferred.

| BAD | GOOD |
|-----|------|
| Three paragraphs of background, then: "Therefore, we recommend approving the budget." | Opening paragraph: "We need your approval for a $50k security fix by Thursday." |
| Risk assessment that lists options without stating a recommendation. | First line: "We recommend Option A. Details and tradeoffs follow." |

## AP-4: Passive Action Language

**WHY**: Passive voice obscures who is accountable for what.
**Consequence**: Work stalls because nobody owns it.

| BAD | GOOD |
|-----|------|
| "The database should be optimized." | "The database team (Alex Chen) must optimize query performance by Q2." |
| "It is recommended that penetration testing be conducted." | "The security team must complete penetration testing before the March 15 release." |
| "Monitoring should be improved." | "DevOps (Sam Park) must configure alerting thresholds by end of sprint." |

## AP-5: Skipping Audience Identification

**WHY**: Tone, depth, and vocabulary must match reader intent and expertise.
**Consequence**: Over-technical or oversimplified content that fails to land.

| BAD | GOOD |
|-----|------|
| Start writing immediately without stating who will read it. | "Audience: CFO. Goal: approve Q3 infrastructure budget. Depth: business impact only." |

## AP-6: Wall of Acronyms

**WHY**: Stacked undefined acronyms create a decoding burden before the reader reaches the message.
**Consequence**: Readers disengage or misread the document entirely.

| BAD | GOOD |
|-----|------|
| "The CISO confirmed the WAF, IDS, and SOC 2 audit cover PCI-DSS scope." | "The Chief Security Officer confirmed our firewall, intrusion detection, and annual security audit cover our payment card data requirements. (Acronyms: CISO, WAF, IDS, SOC 2, PCI-DSS — defined in glossary.)" |

## AP-7: Recommendations Without Deadlines

**WHY**: Undated recommendations are treated as optional.
**Consequence**: No action is taken and risk persists.

| BAD | GOOD |
|-----|------|
| "We recommend patching the vulnerability." | "The security team must deploy the patch by Friday 17:00 to close the exposure window before the weekend." |
| "Consider migrating to encrypted storage." | "Engineering must migrate to encrypted storage before the Q3 compliance audit (deadline: June 1)." |

## AP-8: Conflating Options with Recommendations

**WHY**: Presenting options without a recommendation forces the reader to make a technical judgment they lack expertise for.
**Consequence**: Decision escalates unnecessarily or is indefinitely deferred.

| BAD | GOOD |
|-----|------|
| "Option A: patch in place. Option B: full redeploy. Option C: defer." | "We recommend Option A (patch in place). It is fastest and lowest risk. Options B and C are available but each add 2+ days of delay." |

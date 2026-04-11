# Scenario 06: Full Acronym Definition Pass on a Security Briefing

## User Prompt

A security analyst has drafted a briefing for the board of directors. The document is
dense with acronyms and contains no definitions. The board has no technical background.

Here is the input document:

```
Q3 Security Briefing

Our SOC completed a full PCI-DSS gap assessment this quarter, working with the CISO
and GRC team. Key findings:

1. WAF and IDS coverage expanded to all DMZ subnets.
2. MTTR for P1 incidents improved from 4h to 47 minutes.
3. The MFA rollout covers 94% of IAM accounts; remaining 6% are legacy SSO integrations.
4. CVE triage SLA is met at 98.5% for CVSS scores >= 7.0.
5. BCP and DRP documentation is now compliant with ISO 27001 and SOC 2 Type II.

Open items:
- RBAC audit for all SaaS tools due Q4.
- SIEM alert tuning still pending (backlog: 3 FTEs required).
- RFP for next-gen EDR solution in progress.
```

1. Run a full acronym extraction pass — list every acronym in the document.
2. Provide a plain-language definition for each.
3. Rewrite the briefing for the board with all acronyms defined inline on first use and
   all technical terminology translated to plain language.

## Expected Behavior

1. List all acronyms present in the document: SOC, PCI-DSS, CISO, GRC, WAF, IDS, DMZ, MTTR, MFA, IAM, SSO, CVE, SLA, CVSS, BCP, DRP, ISO, RBAC, SaaS, SIEM, FTE, RFP, EDR
2. Provide a plain-language definition for each acronym appropriate for a board-level reader
3. Define every acronym inline on first use in the rewritten document — none used without explanation
4. Open the rewritten document with a summary of overall security posture and key open items rather than a list of technical metrics

## Success Criteria

- **All acronyms extracted**: Response lists all acronyms present in the document: SOC, PCI-DSS, CISO, GRC, WAF, IDS, DMZ, MTTR, MFA, IAM, SSO, CVE, SLA, CVSS, BCP, DRP, ISO, RBAC, SaaS, SIEM, FTE, RFP, EDR.
- **Plain-language definitions provided for all acronyms**: Each extracted acronym has a plain-language definition appropriate for a board-level reader.
- **Acronyms defined inline on first use in rewrite**: In the rewritten document, every acronym is spelled out with a definition on first use — none used without explanation.
- **Key message and progress lead the rewrite**: Rewritten document opens with a summary of overall security posture and key open items — not a list of technical metrics.

## Failure Conditions

- Any acronym from the document (SOC, PCI-DSS, CISO, GRC, WAF, IDS, DMZ, MTTR, MFA, IAM, SSO, CVE, SLA, CVSS, BCP, DRP, ISO, RBAC, SaaS, SIEM, FTE, RFP, EDR) is not extracted
- Any acronym is missing a plain-language definition
- Any acronym appears in the rewritten document without an inline definition on first use
- Rewritten document opens with a list of technical security metrics rather than an overall posture summary

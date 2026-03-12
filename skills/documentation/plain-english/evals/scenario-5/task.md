# Scenario 5: Full Acronym Definition Pass on a Security Briefing

## Context

A security analyst has drafted a briefing for the board of directors. The document is
dense with acronyms and contains no definitions. The board has no technical background.

## Input Document

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

## Task

1. Run a full acronym extraction pass — list every acronym in the document.
2. Provide a plain-language definition for each.
3. Rewrite the briefing for the board with all acronyms defined inline on first use and
   all technical terminology translated to plain language.

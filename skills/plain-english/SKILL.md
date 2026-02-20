---
name: plain-english
description: Generic skill for writing technical content in plain English for non-technical stakeholders (executives, business managers, compliance officers, legal counsel)
version: 1.0.0
created: 2026-01-21
---

# Plain English Writing Skill

## Purpose

This skill provides generic guidance for translating technical information into plain English that non-technical stakeholders can understand and act upon.

## Target Audiences

Your content should be understandable by:

- **Executives** (CEOs, CTOs, CFOs) - Need business context and impact
- **Compliance Officers** - Need regulatory implications
- **Business Managers** - Need to prioritize resources
- **Board Members** - Need high-level risk assessment
- **Legal Team** - Need to understand liability and exposure
- **Operations** - Need to understand business impact and continuity

## Core Principles

### 1. Use Business Language, Not Technical Jargon

**❌ WRONG** (Technical):
> "Implement SIEM correlation rules and conduct IOC sweeps across SOAR platform"

**✅ RIGHT** (Business):
> "Activate security operations center to monitor all systems for signs of attack. Review logs to determine if unauthorized access occurred"

### 2. Focus on Impact and Actions, Not Technical Details

**❌ WRONG** (Technical details):
> "CVE-2024-1597 PostgreSQL JDBC SQL injection allows prepared statement bypass"

**✅ RIGHT** (Business impact):
> "Database security flaw allows attackers to steal or modify all customer data. Requires database driver update with comprehensive testing"

### 3. Be Specific and Actionable

**❌ WRONG** (Vague):
> "Take appropriate security measures"

**✅ RIGHT** (Specific):
> "Force password reset for all user accounts. Enable multi-factor authentication. Review access logs for unauthorized logins over the past 30 days"

### 4. Quantify Scope and Impact

**❌ WRONG** (No context):
> "Multiple vulnerabilities exist"

**✅ RIGHT** (Quantified):
> "11 critical vulnerabilities affecting customer payment systems and user data across 8 production services"

## Translation Guide: Technical to Business Language

### Security Threat Translation

| Technical Term | Business Translation |
|----------------|---------------------|
| Remote Code Execution (RCE) | Attackers can take complete control of servers |
| SQL Injection | Attackers can steal or modify database information |
| Authentication Bypass | Attackers can bypass login and access any account |
| Cross-Site Scripting (XSS) | Attackers can steal user sessions and impersonate users |
| Denial of Service (DoS) | System can be overwhelmed causing service outages |
| XML External Entity (XXE) | Attackers can read internal files and access other systems |
| Server-Side Request Forgery (SSRF) | Attackers can access internal systems not meant to be public |
| Path Traversal | Attackers can access sensitive files on the server |
| Deserialization | Attackers can take complete control through malicious data |
| Information Disclosure | Sensitive information can be exposed to unauthorized users |

### Action Translation

| Technical Action | Business Translation |
|-----------------|---------------------|
| Upgrade commons-collections | Requires updating security library |
| Upgrade Spring Framework 3.x to 5.x | Requires major framework upgrade |
| Update jackson-databind | Requires updating data processing library |
| Refactor code | Requires code changes |
| Configuration changes | Requires security configuration updates |
| Replace deprecated library | Requires switching to modern alternative |
| Deploy WAF with OWASP ruleset | Apply web application protections |
| Implement CSP headers | Enable browser security protections |

### Complexity/Effort Translation

| Technical Assessment | Business Translation |
|---------------------|---------------------|
| Drop-in replacement | Straightforward update with minimal testing |
| Minor version upgrade | Requires testing and validation |
| Major version upgrade | Requires significant code changes and testing |
| Breaking changes | Requires code refactoring across multiple systems |
| Architecture change | Requires major redesign and planning |

## Common Writing Patterns

### Pattern 1: Risk/Problem Description

**Formula**: [SEVERITY]: [BUSINESS THREAT] affecting [COUNT] [COMPONENT TYPE]. [SCOPE/CONTEXT].

**Example**:
> "CRITICAL: Attackers can take complete control of application servers affecting 11 legacy components. These systems process customer payments and store sensitive data."

### Pattern 2: Solution/Mitigation Description

**Formula**: [ACTION REQUIRED] with [COMPLEXITY LEVEL] [OPTIONAL: due to REASON].

**Example**:
> "Requires major framework upgrade with significant code refactoring. VERY HIGH complexity due to breaking changes and widespread impact across 8 production systems."

### Pattern 3: Response Plan Description

**Formula**: [IMMEDIATE ACTIONS]. [INVESTIGATION STEPS]. [STAKEHOLDER NOTIFICATIONS]. [RECOVERY ACTIONS].

**Example**:
> "Immediately activate incident response team and isolate affected servers. Engage forensic security firm to determine data exposure. Notify legal counsel for breach assessment and regulatory reporting requirements. Prepare customer notifications per GDPR 72-hour requirement."

## Quality Checklist

Before finalizing any plain-English content, verify:

- [ ] Can a non-technical person understand the main points?
- [ ] Are technical acronyms either avoided or explained?
- [ ] Are CVE identifiers, package names, and version numbers removed or explained?
- [ ] Does it explain business impact, not just technical details?
- [ ] Are actions specific and actionable?
- [ ] Is scope quantified (numbers, counts, systems affected)?
- [ ] Is complexity/effort clearly indicated?
- [ ] Would this help someone make a business decision?
- [ ] Is it concise while remaining complete?

## The 10-Second Test

Show your content to someone non-technical for 10 seconds. Can they answer:

1. **What's the problem?** (threat/risk)
2. **How serious is it?** (severity/impact)
3. **What's affected?** (scope)
4. **What needs to be done?** (action)
5. **How hard is it?** (effort/complexity)

If they can't answer these questions, rewrite the content.

## Common Mistakes to Avoid

### ❌ Mistake 1: Using Technical Jargon

**Wrong**: "Deploy EDR and SIEM correlation"

**Right**: "Enable security monitoring tools to detect threats"

### ❌ Mistake 2: Listing CVEs and Package Names

**Wrong**: "CVE-2015-7501 in commons-collections 3.2.1"

**Right**: "Critical security flaw in legacy data processing library"

### ❌ Mistake 3: Being Vague

**Wrong**: "Take appropriate measures"

**Right**: "Force password reset for all users and enable two-factor authentication"

### ❌ Mistake 4: Forgetting Business Context

**Wrong**: "Update the library"

**Right**: "Update security library to prevent attackers from accessing customer data. Requires 2-week testing period before production deployment."

### ❌ Mistake 5: Overwhelming with Details

**Wrong**: "The vulnerability exists in the deserialization process when ObjectInputStream reads untrusted data without proper validation, allowing arbitrary code execution through gadget chains in the classpath..."

**Right**: "Security flaw allows attackers to run malicious code on our servers by sending specially crafted data"

## Usage in Different Contexts

This skill can be applied to various types of content:

- **Risk Descriptions**: Translating vulnerability reports into business risk statements
- **Response Plans**: Converting technical incident response procedures into executive-level action plans
- **Mitigation Plans**: Explaining technical fixes in terms of business effort and timeline
- **Status Reports**: Communicating technical progress to non-technical stakeholders
- **Documentation**: Writing user-facing explanations of technical concepts
- **Presentations**: Creating executive briefings on technical topics

## Integration with Other Skills

This skill is designed to be referenced by other skills that need plain-English writing guidance. Reference this skill using:

```markdown
For guidance on writing in plain English for non-technical stakeholders, see the **plain-english** skill.
```

## Version History

- **v1.0.0** (2026-01-21): Initial generic plain-English writing skill
  - Extracted from snyk-risk-assessment specific guides
  - Generified for use across multiple contexts
  - Added comprehensive translation tables
  - Included quality checklist and testing methodology

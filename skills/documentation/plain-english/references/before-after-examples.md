# Before and After Examples

Concrete rewrites showing the plain-english workflow in practice.

## Example 1: Incident Summary (Executive)

**Before (engineer-facing):**
> At 14:32 UTC, a misconfigured Nginx reverse proxy caused a cascading failure in the
> Kubernetes pod autoscaler, resulting in a 503 surge. MTTR was 47 minutes. Remediation
> required patching the ConfigMap and rolling restart of the ingress controller. To prevent
> recurrence, we recommend implementing admission webhooks and enforcing IaC policy gates
> via OPA Gatekeeper.

**After (CEO/CFO-facing):**
> **Decision needed: approve $30k for automated configuration safety checks.**
>
> On Tuesday afternoon, an incorrect configuration in our web traffic routing layer caused
> the payment service to become unavailable for 47 minutes. No customer data was lost. The
> immediate issue is fixed.
>
> To prevent recurrence, the infrastructure team recommends automated configuration
> validation — estimated at $30k and two weeks of engineering time. Without this, a similar
> outage could recur in any future release.
>
> **Action required:** Engineering VP (Lisa Torres) must approve the $30k budget by Friday 17:00
> to begin work in the Q3 sprint.

---

## Example 2: Risk Item (Cross-functional)

**Before:**
> Token replay attacks should be mitigated.

**After:**
> **Security team (Alex Chen) must implement session expiry controls by 15 March** to prevent
> unauthorized reuse of login tokens. Without this fix, an attacker who captures a user's
> session could continue accessing the account even after the user logs out.

---

## Example 3: Status Update (Unknown Audience — Manager Fallback)

**Before (engineering notes):**
> Profiled query execution plans; found 3 N+1 queries. Added composite index on
> (user_id, created_at); p99 latency down from 820ms to 190ms. Blocked on DBA approval.
> Next: profile checkout flow, add read replicas if needed.

**After (manager-level):**
> **Summary:** Checkout is 4× faster in testing. Production deployment is blocked on
> database administrator sign-off.
>
> **Progress:** The team identified and fixed three inefficient database queries.
> Response time in the test environment dropped from 820ms to 190ms — well within our
> 200ms target.
>
> **Blocker:** Database team (Sarah Lin) must approve the change for production by Wed Mar 15 17:00
> to meet the Q2 performance milestone.
>
> **Next step:** Once approved, the team will profile the checkout payment flow and assess
> whether additional database capacity is needed.

---

## Example 4: Architecture Decision (Compliance/Legal)

**Before (technical ADR excerpt):**
> ADR-042: Migrate from on-prem PostgreSQL to AWS RDS Aurora (Multi-AZ).
> RPO reduced from ~24h to ~5 minutes (continuous WAL archiving). RTO reduced from ~4h
> to ~15 minutes. Requires SOC 2 Type II re-assessment for data residency.

**After (compliance manager-facing):**
> **Decision needed: approve or reject cloud database migration by [date].**
>
> The engineering team proposes moving the customer database from our own data centre
> to a managed cloud service. This change would reduce the maximum data loss in a disaster
> from 24 hours to 5 minutes, and reduce recovery time from 4 hours to 15 minutes.
>
> **Compliance impact:** This change requires a new security certification assessment
> (SOC 2 Type II) due to a change in where customer data is stored. The assessment takes
> approximately 3 months and costs an estimated $15k.
>
> **Cost increase:** $1,200/month.
>
> **Planned downtime:** 6-hour window during Q3 scheduled maintenance.

---

## Example 5: Security Risk with Conflicting Priorities

**Before (two separate tickets):**
> TICKET-1: EU data breach detected. PII potentially exposed. GDPR notification required.
> TICKET-2: CVE-2024-XXXX in core dependency. RCE possible. Patch available.

**After (executive briefing — conflicting priorities fallback: lead with risk + deadline):**
> **Immediate action required on two security issues. Both deadlines are within 72 hours.**
>
> **Issue 1 — Customer Data Exposure (Deadline: 54 hours)**
> We detected unauthorized access to our EU customer database 18 hours ago. Some customer
> personal information (names and email addresses) may have been viewed. We are legally
> required to notify affected customers and EU regulators within 72 hours of detection
> (54 hours remain). Legal team (Maria Kovacs) is ready to send notifications pending
> your approval.
>
> **Issue 2 — Security Vulnerability in Core Software (Deadline: Before weekend)**
> A critical security flaw was found in a software library we rely on. If exploited, an
> attacker could gain unauthorized control of our servers. A fix is available and requires
> a 4-hour scheduled outage. Engineering (Dan Wu) recommends deploying this Saturday night.
>
> **Actions needed from you:**
> 1. Legal team (Maria Kovacs) must receive your approval to send EU customer notifications by [specific deadline — 54 hours from detection].
> 2. Engineering (Dan Wu) must receive your approval for the Saturday maintenance window by Friday 17:00.

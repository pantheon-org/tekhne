# Scenario 04: Security Audit: Hardcoded Secrets and Insecure Script Patterns

## User Prompt

A SaaS company's security team has flagged a GitLab CI/CD pipeline during a routine audit. The pipeline was written quickly for a hackathon and then promoted directly to production. The security team suspects hardcoded credentials and unsafe script patterns but has not provided specifics.

Audit the `.gitlab-ci.yml` below for security issues. Identify every security finding, assign a severity level, explain the risk each finding introduces, and produce a corrected version of the file with all critical and high-severity issues resolved.

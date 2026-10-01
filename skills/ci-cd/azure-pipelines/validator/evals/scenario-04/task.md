# Scenario 04: Security Audit: Credentials and Unsafe Container Images

## User Prompt

A fintech startup's engineering team is preparing to open-source their build pipeline. Before publishing the repository they need a full security audit of `azure-pipelines.yml` to ensure no credentials are exposed and all container references are pinned. A peer reviewer has flagged "a few security concerns" but left no specifics.

Audit the pipeline below for security issues. Identify every security finding, explain why each finding is a risk, and produce a corrected version of the file with all HIGH and MEDIUM issues resolved.

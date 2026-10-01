# Scenario 06: Best Practices: Deprecated Keywords, Caching, Artifact Expiration, and Image Pinning

## User Prompt

A platform team is reviewing a `.gitlab-ci.yml` that was written before GitLab 14.x. The pipeline runs correctly today but the team has been warned it will break when GitLab completes the removal of deprecated keywords. Additionally, build times have increased 40% over the past quarter and the team suspects missing caches and inefficient artifact configuration.

Review the pipeline below for best practices violations. For each finding, explain the problem and its impact, and produce a fully improved version of the pipeline.

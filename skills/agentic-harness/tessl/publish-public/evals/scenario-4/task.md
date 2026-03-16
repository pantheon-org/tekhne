# Writing a Registry-Optimized Skill Summary

## Problem Description

A team is finalizing the tile.json for their `aws-cost-analyzer` skill before publishing it to the public Tessl registry. Their current summary reads: "Analyze AWS costs" — a placeholder that will hurt discoverability in the registry search. They need a production-quality summary that will surface the skill when users search for relevant terms.

The skill: analyzes AWS billing data to identify cost anomalies, underutilized resources, and savings opportunities. It supports Cost Explorer API, EC2 rightsizing recommendations, S3 storage class optimization, and Reserved Instance analysis. It should be used when investigating unexpected AWS bill increases, planning budget optimizations, or auditing cloud spend before quarterly reviews.

Write a `tile-summary.json` file containing only the `summary` field with its value. Also write a `keywords.txt` file listing the terms you embedded in the summary.

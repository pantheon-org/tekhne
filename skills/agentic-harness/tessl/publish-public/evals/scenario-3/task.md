# Is This Skill Ready to Publish?

## Problem Description

A developer has been working on a `docker-security` skill and believes it's ready for the public Tessl registry. They've asked you to assess whether it meets the requirements for public publishing and identify anything that must be completed first.

Review the tile state below and produce a `readiness-report.md` that: (1) lists each blocking issue that must be fixed before publishing, (2) provides the specific command or action needed to resolve each issue, and (3) gives a final verdict of READY or NOT READY.

## Input Files

The following files are provided. Extract them before beginning.

=============== FILE: inputs/tile.json ===============
{
  "name": "my-org/docker-security",
  "version": "1.0.0",
  "private": true,
  "summary": "Scan Dockerfiles for security vulnerabilities and best practice violations. Use when hardening container images, reviewing Dockerfiles for production, or automating security checks in CI. Keywords: docker, dockerfile, security, containers, ci, hardening, best-practices",
  "skills": {
    "docker-security": {
      "path": "SKILL.md"
    }
  }
}

=============== FILE: inputs/directory-listing.txt ===============
docker-security/
├── tile.json
├── SKILL.md
└── references/
    └── security-rules.md

(No .context/ directory exists, no evals/ directory exists)

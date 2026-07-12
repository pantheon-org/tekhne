# Security Policy

## Supported versions

Tekhne is a curated collection of agent skills released as a rolling line from
the `main` branch. Security fixes are applied to the latest published release
only. Please make sure you are on the most recent version before reporting an
issue.

| Version | Supported |
| ------- | --------- |
| Latest release | Yes |
| Older releases | No |

## Reporting a vulnerability

Please do not open a public issue for security vulnerabilities.

Report privately through GitHub's built-in advisory workflow:

1. Go to the **Security** tab of this repository.
2. Select **Report a vulnerability** (Private vulnerability reporting).
3. Provide a clear description, affected paths or skills, reproduction steps,
   and any suggested remediation.

If private reporting is unavailable to you, contact the maintainers through the
repository's stated contact channel rather than a public issue.

## What to expect

- Acknowledgement of your report within 5 working days.
- An initial assessment and severity triage shortly after.
- Coordinated disclosure once a fix is available. We will credit reporters who
  wish to be named.

## Scope

In scope:

- The CLI tooling under `cli/`.
- The Go tooling under `tools/`.
- Skill scripts under `skills/**/scripts/` that execute code.
- CI/CD workflows under `.github/workflows/`.

Out of scope:

- Vulnerabilities in third-party dependencies (report those upstream; we track
  them through Dependabot).
- Findings that require a compromised developer machine or maintainer account.

## Handling of sensitive data

Do not include credentials, tokens, personal data, or other sensitive material
in a report. If a reproduction requires such data, use synthetic placeholders
and note that in your report.

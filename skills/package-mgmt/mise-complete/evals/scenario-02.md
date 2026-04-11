# Scenario 02: Fix Anti-Pattern Violations in a mise.toml

## User Prompt

A developer committed the following `mise.toml` to the repository. Review it and produce a corrected version that fixes all anti-pattern violations.

```toml
[tools]
node = "latest"
python = "3.12"
terraform = "~1.8"

[tasks.deploy]
description = "Deploy to production"
run = "cd infrastructure && terraform apply -auto-approve"

[tasks.test]
description = "Run test suite"
run = "npm run test:unit"

[env]
NODE_ENV = "production"
DATABASE_URL = "postgres://admin:s3cr3tp@ssw0rd@db.internal:5432/prod"
STRIPE_SECRET_KEY = "sk_live_abc123XYZ"
```

Produce a corrected `mise.toml` that fixes all violations. Include a `REVIEW.md` that lists each violation found and the fix applied.

## Expected Behavior

1. Replace floating version specifiers (`latest`, `3.12`, `~1.8`) with exact pinned versions in the corrected `mise.toml`
2. Remove `DATABASE_URL` and `STRIPE_SECRET_KEY` from `mise.toml`; explain in `REVIEW.md` or via a comment that they must be loaded from external secret management
3. Fix the deploy task to use an explicit `working_dir` or absolute path instead of a `cd` command for context
4. Document each identified violation separately in `REVIEW.md`: floating versions, embedded secrets, and implicit-state task

## Success Criteria

- **Floating versions replaced with exact pins**: `mise.toml` replaces `latest`, `3.12`, and `~1.8` with exact pinned versions (e.g. `node = "22.4.0"`, `python = "3.12.x"`, `terraform = "1.8.x"`)
- **Secrets removed from [env]**: `DATABASE_URL` and `STRIPE_SECRET_KEY` are removed from `mise.toml`; `REVIEW.md` or a comment explains they must be loaded from external secret management
- **Deploy task does not use implicit cd**: The deploy task uses an explicit `working_dir` or absolute path instead of relying on a `cd` command for context
- **REVIEW.md documents each violation**: `REVIEW.md` identifies at least the floating versions, embedded secrets, and implicit-state task as separate violations

## Failure Conditions

- Keeps floating version specifiers in the corrected `mise.toml`
- Leaves `DATABASE_URL` or `STRIPE_SECRET_KEY` in the `[env]` block
- Does not provide `REVIEW.md` or a comment explaining how to handle secrets
- Corrected deploy task still uses `cd infrastructure &&` without setting `working_dir`
- `REVIEW.md` omits one or more of the three violations (floating versions, secrets, implicit `cd`)

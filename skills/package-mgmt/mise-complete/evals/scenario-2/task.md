# Fix Anti-Pattern Violations in a mise.toml

## Problem Description

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

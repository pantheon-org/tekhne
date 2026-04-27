# Task: Skill Quality Assessment

A developer has written a new SKILL.md for a SQL query-builder agent skill. The file is 180 lines and contains: an introduction explaining what SQL is, a step-by-step installation guide for the database driver, basic SELECT/INSERT syntax examples, a list of generic "best practices" (use indexes, avoid SELECT *), and a brief mention of connection pooling.

Evaluate this skill using the 9-dimension quality framework and produce a scored audit report.

## Input Files

**skills/sql-query-builder/SKILL.md** (excerpt):

```markdown
---
name: sql-query-builder
description: "Help with SQL queries."
---

# SQL Query Builder

SQL (Structured Query Language) is used to interact with relational databases.

## Installation

Install the database driver:
```bash
pip install psycopg2
```

## Basic Usage

SELECT statement:
```sql
SELECT * FROM users;
```

INSERT statement:
```sql
INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com');
```

## Best Practices

- Use indexes for better performance
- Avoid SELECT * in production
- Use parameterized queries to prevent SQL injection
```

## Output Specification

Produce:
1. **audit-report.md** — scored breakdown with per-dimension scores and justification
2. **remediation-plan.md** — prioritised list of improvements with effort estimates

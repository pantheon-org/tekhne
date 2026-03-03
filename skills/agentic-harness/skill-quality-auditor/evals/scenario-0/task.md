# Skill Quality Assessment

## Problem/Feature Description

Your development team has created several agent skills for internal use, but there's no standardized way to evaluate their quality. The team lead wants to establish a consistent evaluation framework that can identify which skills are production-ready and which need improvement.

One particular skill called "database-optimization" has been flagged for review. The team suspects it may contain too much basic documentation that experienced developers already know, but they're not sure how to measure this objectively. They need a comprehensive quality assessment that covers multiple dimensions of skill effectiveness.

The assessment should help determine whether the skill contains expert knowledge that justifies its existence, follows proper structural conventions, provides clear guidance on when and when not to use it, and includes appropriate examples and anti-patterns.

## Output Specification

Create a comprehensive quality assessment that includes:

1. **evaluation-report.md** - A detailed evaluation following a structured framework
2. **scoring-breakdown.txt** - Numerical scores with justification for each evaluation dimension
3. **recommendations.md** - Specific actionable recommendations for improvement
4. **assessment-process.md** - Document the methodology you used for this assessment

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/database-optimization/SKILL.md ===============

```yaml
---
name: database-optimization
description: Database performance optimization techniques
---
```

```markdown
# Database Optimization

## Introduction

Databases are critical components of applications. Performance optimization ensures fast response times.

## Basic Concepts

A database is a structured collection of data. SQL (Structured Query Language) is used to interact with relational databases.

## Query Optimization

To optimize queries, you can:

- Use indexes on frequently queried columns
- Avoid SELECT * statements
- Use LIMIT clauses

Example:

```sql
SELECT name, email FROM users WHERE active = 1 LIMIT 100;
```

## Indexing

Indexes speed up data retrieval. Create indexes on columns used in WHERE clauses:

```sql
CREATE INDEX idx_user_active ON users(active);
```

## Connection Pooling

Use connection pools to manage database connections efficiently. Popular tools include:

- HikariCP for Java
- pgbouncer for PostgreSQL

## Monitoring

Monitor database performance using tools like:

- pgAdmin for PostgreSQL  
- MySQL Workbench for MySQL
- MongoDB Compass for MongoDB

## Best Practices

- Always backup your data
- Use transactions for data integrity
- Normalize your schema appropriately
- Keep your database software updated

## Installation

Most databases can be installed using package managers:

```bash
# PostgreSQL
sudo apt-get install postgresql

# MySQL
sudo apt-get install mysql-server
```

## Conclusion

Following these practices will improve your database performance.
```

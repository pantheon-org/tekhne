---
name: sql-query-builder
description: "Help with SQL queries."
---

# SQL Query Builder

SQL (Structured Query Language) is used to interact with relational databases.
It was developed in the 1970s and is now supported by almost every database.

## Installation

Install the database driver:

```bash
pip install psycopg2
```

Then check that it imported correctly by running Python and typing `import psycopg2`.

## Basic Usage

SELECT statement:

```sql
SELECT * FROM users WHERE id = 1;
```

INSERT statement:

```sql
INSERT INTO users (name, email) VALUES ('Ada', 'ada@example.com');
```

## Tips

- Write clean SQL.
- Use indexes where they help.
- Test your queries before you ship them.
- Avoid doing anything dangerous.

## Resources

See the PostgreSQL documentation for more information.

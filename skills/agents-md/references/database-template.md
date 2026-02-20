# Database / Data Layer Template

Use this template when documenting database or data layer packages.

**MANDATORY: Before using this template, analyze the current project to identify:**
1. **Database type** - Run: `grep -r "postgres\|mysql\|sqlite\|mongo" package.json | head -3`
2. **ORM/Query builder** - Run: `grep -r "prisma\|drizzle\|typeorm\|sequelize\|mongoose\|knex" package.json | head -3`
3. **Schema files** - Run: `find . -name "schema.*" -o -name "*.sql" -o -name "*prisma*" | grep -v node_modules | head -5`
4. **Migration system** - Run: `find . -name "*migration*" -type d 2>/dev/null`
5. **Database config** - Run: `find . -name "*database*" -o -name "*db*" | grep -E "\.(js|ts|json)$" | head -5`

## Adaptive Template Structure

```markdown
## Database
- Database: [DETECTED_DB_TYPE] ([PostgreSQL/MySQL/SQLite/MongoDB/etc])
- ORM/Query: [DETECTED_ORM] ([Prisma/Drizzle/TypeORM/etc])
- Schema: `[DETECTED_SCHEMA_PATH]` OR "Schema files not found"
- Migrations: `[DETECTED_MIGRATION_COMMAND]` OR "No migration system detected"
- Connection: via `[DETECTED_CONNECTION_FILE]` OR "Connection config not found"
- NEVER run migrations in tests
```

## Detection Instructions

### Schema Management (Adapt Based on Findings)
**Prisma**: Look for `prisma/schema.prisma`, `npx prisma migrate`
**Drizzle**: Look for `drizzle.config.ts`, schema in `src/db/`
**TypeORM**: Look for entity files, `typeorm migration:run`
**Raw SQL**: Look for `.sql` files, custom migration scripts
**MongoDB**: Look for model definitions, no traditional migrations

### Connection Management
**Find actual connection patterns in the project:**
- Database URL in `.env.example` or config files
- Connection pool configuration
- Environment-specific database configs

### Query Patterns (Copy from Existing Code)
**Before documenting**, find examples in the codebase:
- How queries are written in this specific project
- Transaction patterns actually used
- Error handling approaches in place

## Technology-Specific Adaptations

### SQL Databases (PostgreSQL, MySQL, SQLite)
- Migration commands and workflow
- Schema definition files location
- Seed data setup if present

### NoSQL Databases (MongoDB, etc.)
- Model/schema definitions
- Index management approach
- Data migration strategies (if any)

### ORM-Specific Patterns
**Copy patterns from existing code**, don't assume:
- Entity/model definitions format
- Query builder syntax actually used
- Relationship definitions in place

## Critical Warnings (Universal)
- NEVER run migrations against production from local environment
- NEVER commit database credentials or connection strings
- NEVER run migrations in test suites (use test database setup)
- NEVER assume database type - detect from project files

## Anti-Patterns for Database Documentation
- Don't explain what databases or ORMs are in general
- Don't document SQL syntax basics
- Don't assume specific database technology without verification
- Don't duplicate ORM documentation that's already available
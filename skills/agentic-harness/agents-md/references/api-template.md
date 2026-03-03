# API / Backend Service Template

Use this template when documenting API or backend service packages.

**MANDATORY: Before using this template, analyze the current project to identify:**
1. **Backend framework** - Run: `grep -r "express\|fastify\|koa\|nest\|hapi\|django\|flask\|gin\|fiber" package.json pyproject.toml go.mod | head -3`
2. **Route organization** - Run: `find . -name "*route*" -o -name "*endpoint*" -o -name "*handler*" | grep -v node_modules | head -5`
3. **Middleware location** - Run: `find . -name "*middleware*" -o -name "*auth*" | grep -v node_modules | head -5`
4. **Validation approach** - Run: `grep -r "zod\|joi\|yup\|ajv\|pydantic" package.json pyproject.toml | head -3`
5. **Error handling** - Run: `find . -name "*error*" -o -name "*exception*" | grep -E "\.(js|ts|py|go)$" | head -5`

## Adaptive Template Structure

```markdown
## API Patterns
- Framework: [DETECTED_FRAMEWORK] ([Express/Fastify/Django/etc])
- Routes: `[DETECTED_ROUTES_PATH]/**/*` OR "Route organization not found"
- Auth: `[DETECTED_AUTH_PATH]` OR "Auth middleware not detected"
- Validation: [DETECTED_VALIDATION_LIB] schemas in `[SCHEMA_PATH]/**`
- Errors: `[ERROR_CLASS_NAME]` from `[ERROR_FILE_PATH]`
- Example route: `[ACTUAL_ROUTE_EXAMPLE_PATH]`
```

## Detection Instructions

### Framework-Specific Patterns
**Node.js/Express**: Look for `app.js`, `server.js`, route files
**Node.js/Fastify**: Look for Fastify plugins, route definitions
**Node.js/NestJS**: Look for controllers, decorators, modules
**Python/Django**: Look for `urls.py`, views, serializers
**Python/Flask**: Look for route decorators, blueprints
**Go**: Look for handler functions, router setup
**PHP**: Look for route definitions, controllers

### Route Organization (Copy from Actual Project)
**Before documenting**, analyze how routes are actually organized:
- Directory structure for routes/endpoints
- HTTP method conventions in use
- Route parameter patterns found in code
- URL structure patterns

### Middleware Stack (Detect What Exists)
**Find actual middleware in use:**
- Authentication/authorization middleware files
- Validation middleware implementations
- Error handling middleware
- Request logging setup (if any)

## Technology-Specific Adaptations

### Express.js Projects
- Router usage patterns from existing code
- Middleware order and setup
- Error handling with `next()`

### FastAPI/Django Projects  
- Dependency injection patterns
- Serializer/Pydantic model usage
- Authentication decorators

### Go Projects
- Handler function signatures
- Middleware chain patterns
- Error handling approaches

### Validation Approaches (Copy from Project)
**Find what's actually implemented:**
- Schema definition location and format
- Input validation middleware usage
- Error response structure in use

## Development Patterns (Extract from Code)
**Look for existing patterns in the codebase:**
- Route testing approach (if tests exist)
- API documentation generation (OpenAPI, etc.)
- Request/response typing patterns

## Critical Warnings (Universal)
- NEVER expose internal error details in API responses
- NEVER skip input validation on user inputs
- NEVER hardcode API keys or secrets in code
- NEVER assume framework without detection

## Anti-Patterns for API Documentation  
- Don't explain HTTP basics or REST principles
- Don't assume specific framework without verification  
- Don't duplicate existing API documentation (OpenAPI/Swagger)
- Don't include framework tutorials - focus on project patterns
# Scenario 05: Complete Plugin Registration

## User Prompt

Your team implemented a new generator and executor, but they don't work when developers try to use them! The Nx CLI can't find them.

## Error Messages

When running:
```bash
npx nx g @myorg/tools:create-service my-service
```

Error:
```
Cannot find generator '@myorg/tools:create-service'
```

When running:
```bash
npx nx run my-app:custom-deploy
```

Error:
```
Cannot find executor '@myorg/tools:custom-deploy'
Unable to resolve @myorg/tools:custom-deploy
```

## Current Structure

```
tools/my-plugin/
├── src/
│   ├── generators/
│   │   └── create-service/
│   │       ├── generator.ts
│   │       └── schema.json
│   ├── executors/
│   │   └── custom-deploy/
│   │       ├── executor.ts
│   │       └── schema.json
│   └── index.ts (EMPTY!)
├── package.json
└── project.json
```

**Current index.ts:**
```typescript
// Empty file!
```

**Current package.json:**
```json
{
  "name": "@myorg/tools",
  "version": "1.0.0",
  "main": "./src/index.ts"
}
```

## Your Mission

Set up proper plugin registration so Nx CLI can discover and use your generators and executors.

## Deliverables

Create these files:

1. **src/index.ts** - Export all generators and executors
2. **generators.json** - Registry for generators
3. **executors.json** - Registry for executors
4. **package.json** - Updated with proper entry points
5. **REGISTRATION-GUIDE.md** - Explain the registration system

## Requirements

- `index.ts` must export the generator and executor functions
- `generators.json` must map names to factory paths
- `executors.json` must map names to implementation paths
- `package.json` must reference the registry files
- Include descriptions in registry files

## Test Case

After your fixes, these should work:
```bash
npx nx g @myorg/tools:create-service my-service
npx nx run my-app:custom-deploy
```

## Constraints

- Focus on the plugin registration structure
- Show proper export patterns
- Don't create the actual workspace or full implementations

## Expected Behavior

1. Export the generator and executor functions from `src/index.ts`
2. Add a `generators.json` with a `generators` key mapping each generator name to a `factory` path and `schema` path
3. Add an `executors.json` with an `executors` key mapping each executor name to an `implementation` path and `schema` path
4. Update `package.json` to reference both `generators.json` and `executors.json`
5. Include `description` fields in registry files for CLI help text
6. Document the full registration flow in REGISTRATION-GUIDE.md

## Success Criteria

- **Index exports**: `src/index.ts` exports generator and executor functions
- **Generator registration**: `generators.json` has proper structure with factory path and schema path
- **Executor registration**: `executors.json` has proper structure with implementation path and schema path
- **Package manifest**: `package.json` references `generators.json` and `executors.json`
- **CLI help text**: Registry files include `description` fields for CLI help
- **Documentation**: REGISTRATION-GUIDE.md explains the registration flow and file structure

## Failure Conditions

- `src/index.ts` remains empty, so generators and executors are not exported
- `generators.json` is missing or has no `factory` or `schema` path for the generator
- `executors.json` is missing or has no `implementation` or `schema` path for the executor
- `package.json` does not reference the registry files, so Nx cannot discover them
- Registry files have no `description` fields, producing no CLI help text
- REGISTRATION-GUIDE.md is missing or does not explain how the registration files connect

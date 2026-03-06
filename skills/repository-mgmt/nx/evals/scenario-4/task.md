# Task: Complete Plugin Registration

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

- index.ts must export the generator and executor functions
- generators.json must map names to factory paths
- executors.json must map names to implementation paths
- package.json must reference the registry files
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

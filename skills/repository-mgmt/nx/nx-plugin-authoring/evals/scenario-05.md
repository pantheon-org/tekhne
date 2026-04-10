# Scenario 05: Register a Generator So It Can Be Invoked by Nx CLI

## User Prompt

A developer implemented a generator function but it cannot be run with `npx nx g @myorg/tools:scaffold`. The plugin directory structure is:

```
tools/my-plugin/
├── src/
│   ├── generators/
│   │   └── scaffold/
│   │       ├── generator.ts       (implemented — exports default async function)
│   │       └── schema.json        (implemented — has name: string field)
│   └── index.ts                   (currently empty)
├── generators.json                (currently empty object: {})
└── package.json                   (name: "@myorg/tools")
```

The developer says: "I can see the file exists but `npx nx g @myorg/tools:scaffold` fails with 'cannot find generator'."

Produce:
1. `generators.json` — the correct content to register the scaffold generator
2. `src/index.ts` — the correct export to expose the generator function

## Expected Behavior

1. Add a `scaffold` entry under a `generators` key in `generators.json`, with a `factory` path pointing to the generator function (e.g. `'./src/generators/scaffold/generator'`)
2. Include a `schema` field in the scaffold entry pointing to the `schema.json` file
3. Export the scaffold generator function from `src/index.ts` (default or named export matching the factory path)
4. Ensure the `factory` path in `generators.json` and the export in `src/index.ts` refer to the same generator implementation

## Success Criteria

- **generators.json has scaffold entry with factory path**: `generators.json` includes a `'scaffold'` key under `'generators'` with a `factory` pointing to the generator function (e.g. `'./src/generators/scaffold/generator'`)
- **generators.json schema path included**: The scaffold entry in `generators.json` includes a `schema` field pointing to the `schema.json` file
- **src/index.ts exports the generator**: `src/index.ts` exports the scaffold generator function (default or named export matching the factory path)
- **Both files consistent with each other**: The factory path in `generators.json` and the export in `src/index.ts` refer to the same generator implementation

## Failure Conditions

- `generators.json` has no `generators` key or no `scaffold` entry, so Nx CLI cannot find the generator
- The `scaffold` entry in `generators.json` has no `factory` path or points to the wrong file
- The `scaffold` entry in `generators.json` has no `schema` field
- `src/index.ts` remains empty, so the generator function is not exported
- The `factory` path and the export in `src/index.ts` are inconsistent, pointing to different implementations

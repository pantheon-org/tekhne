# Register a Generator So It Can Be Invoked by Nx CLI

## Problem Description

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

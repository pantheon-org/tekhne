# Tree API Reference

Use this reference for safe file and configuration mutations in generators.

## Common Methods

- `tree.read(path)` reads file contents.
- `tree.write(path, content)` writes/creates files tracked by dry-run.
- `tree.exists(path)` checks whether a path is present in the virtual tree.
- `tree.delete(path)` removes files safely in generator context.

## High-Value Devkit Helpers

- `generateFiles(tree, src, dest, vars)` for template-based generation.
- `updateJson(tree, path, updater)` for structured JSON mutations.
- `readProjectConfiguration(tree, project)` and `updateProjectConfiguration(...)` for project settings.

## Example Pattern

```ts
import { Tree, updateJson, generateFiles, joinPathFragments } from "@nx/devkit";

export default async function generator(tree: Tree, schema: { name: string }) {
  const root = joinPathFragments("libs", schema.name);
  generateFiles(tree, joinPathFragments(__dirname, "files"), root, { ...schema, tmpl: "" });

  updateJson(tree, "nx.json", (json) => {
    json.generators = json.generators ?? {};
    return json;
  });
}
```

## Safety Notes

- Prefer Tree operations over Node `fs` in generator runtime.
- Keep updates idempotent where possible.

# ExecutorContext API Reference

Use this reference when implementing executor behavior with Nx context metadata.

## Commonly Used Fields

- `context.root`: workspace root path.
- `context.projectName`: active project name.
- `context.targetName`: current target key.
- `context.configurationName`: active configuration, if provided.
- `context.projectsConfigurations`: full project graph metadata.

## Example Usage

```ts
import type { ExecutorContext } from "@nx/devkit";

export default async (_options: { outputPath: string }, context: ExecutorContext) => {
  const project = context.projectName ?? "unknown-project";
  const target = context.targetName ?? "unknown-target";

  console.log(`Running ${project}:${target}`);
  return { success: true };
};
```

## Practical Patterns

- Log project/target identity for debuggability.
- Resolve workspace-relative paths via `context.root`.
- Keep context reads defensive (`??`) to avoid brittle assumptions.

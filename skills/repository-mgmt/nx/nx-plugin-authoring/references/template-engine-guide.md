# Template Engine Guide

Use this reference when creating and maintaining generator template files.

## Template Basics

- Store templates under `files/` in the generator directory.
- Use `.template` file suffix for rendered outputs.
- Include `tmpl: ""` in template variables to strip `.template` suffix.

## EJS Variable Examples

```ts
const templateVars = {
  ...schema,
  ...names(schema.name),
  tmpl: "",
};
```

```ejs
export const <%= propertyName %> = "<%= name %>";
```

## Reusable Template Patterns

- Keep shared snippets small and composable.
- Prefer explicit variable names over implicit assumptions.
- Separate structure templates (directories/files) from configuration mutation logic.

## Troubleshooting

- Unreplaced variables usually mean missing keys in `templateVars`.
- Incorrect filenames often mean missing `tmpl: ""` mapping.

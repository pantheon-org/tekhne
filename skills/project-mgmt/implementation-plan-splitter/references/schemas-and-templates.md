# Schemas and Templates Reference

This file contains JSON schemas and YAML templates used by the automation scripts. Load ONLY when debugging validation or customizing automation.

## JSON Schemas

Two JSON schemas validate file structure:

### step-file.schema.json

Validates step/activity markdown files:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["title", "description", "checklist", "acceptanceCriteria"],
  "properties": {
    "title": { "type": "string" },
    "description": { "type": "string", "minLength": 10 },
    "checklist": {
      "type": "array",
      "items": { "type": "string" },
      "minItems": 1
    },
    "acceptanceCriteria": {
      "type": "array", 
      "items": { "type": "string" },
      "minItems": 1
    },
    "status": {
      "type": "string",
      "enum": ["not-started", "in-progress", "complete", "blocked"]
    },
    "dependencies": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

### readme-file.schema.json

Validates README files:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["title", "description"],
  "properties": {
    "title": { "type": "string" },
    "description": { "type": "string", "minLength": 20 },
    "contents": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "path": { "type": "string" },
          "description": { "type": "string" }
        }
      }
    },
    "successCriteria": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

## YAML Templates

Templates are defined in `templates/` directory and used by the automation scripts:

### templates/phase-readme.yaml
Phase-level README with sections: Phase Header, Activities/Steps table, Success Criteria, Rollback Procedure.

```yaml
title: "Phase {{number}}: {{name}}"
sections:
  - name: "Phase Header"
  - name: "{{type|title}}"  # "Activities" or "Steps"
  - name: "Success Criteria"
  - name: "Rollback Procedure"
```

**Variables:** `number`, `name`, `description`, `type`, `type_singular`, `items[]`

### templates/intermediate-readme.yaml
Intermediate directory README for `activities/` or `steps/` folders.

```yaml
title: "{{type|title}}"
sections:
  - name: "Description"
  - name: "Groups"
```

**Variables:** `type`

### templates/group-readme.yaml
Group directory README (e.g., `step-1-extract-and-refactor/README.md`).

```yaml
title: "{{type_singular|title}} {{number}}: {{name}}"
sections:
  - name: "Description"
  - name: "Files"
  - name: "Prerequisites"
  - name: "Status"
```

**Variables:** `type_singular`, `number`, `name`, `description`, `sub_items_count`, `sub_items[]`

### templates/step-file.yaml
Step/activity leaf file template.

```yaml
title: "{{type_singular|title}} {{number}}: {{name}}"
sections:
  - name: "Description"
  - name: "Checklist"
  - name: "Acceptance Criteria"
  - name: "Status"
  - name: "Notes"
  - name: "Dependencies"
defaults:
  status: "Not Started"
  dependencies: []
```

**Variables:** `type_singular`, `number`, `name`, `description`, `checklist[]`, `acceptance_criteria[]`, `status`, `dependencies[]`

## Template Syntax

| Syntax | Description | Example |
|--------|-------------|---------|
| `{{variable}}` | Variable interpolation | `{{name}}` → "Analysis" |
| `{{kebab value}}` | Convert to kebab-case | `{{kebab name}}` → "analysis-and-design" |
| `{{value\|title}}` | Title case filter | `{{type\|title}}` → "Activities" |
| `{{#each array}}...{{/each}}` | Array iteration | Loop through items |
| `{{#if condition}}...{{/if}}` | Conditional | Show if truthy |
| `{{#if cond}}...{{else}}...{{/if}}` | Conditional with else | Show alternate |

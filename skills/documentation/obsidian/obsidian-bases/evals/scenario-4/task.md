# Scenario 4: Debug Global Filter Blocking an Archived-Projects View

A user has the following `projects.base` file. They want two views: one showing active
projects and one showing archived projects. However, the "Archived" view always shows
zero notes even though several notes have `status: archived`.

## Current File

```yaml
filters: 'file.hasTag("project") && status != "archived"'

views:
  - type: table
    name: "Active"
    filters: 'status == "active"'

  - type: table
    name: "Archived"
    filters: 'status == "archived"'
```

## Task

1. Explain why the "Archived" view shows no notes.
2. Fix the base so that:
   - Both views still show only notes tagged `#project`.
   - The "Active" view shows notes where `status == "active"`.
   - The "Archived" view shows notes where `status == "archived"`.
3. Provide the corrected YAML.

## Constraints

- Do not use a global filter that excludes archived notes.
- Keep the tag scope (`#project`) in a global filter.
- Use view-level filters for status-based filtering.

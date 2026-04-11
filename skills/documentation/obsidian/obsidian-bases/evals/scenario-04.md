# Scenario 04: Debug Global Filter Blocking an Archived-Projects View

## User Prompt

A user has the following `projects.base` file. They want two views: one showing active
projects and one showing archived projects. However, the "Archived" view always shows
zero notes even though several notes have `status: archived`.

**Current file**

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

1. Explain why the "Archived" view shows no notes.
2. Fix the base so that:
   - Both views still show only notes tagged `#project`.
   - The "Active" view shows notes where `status == "active"`.
   - The "Archived" view shows notes where `status == "archived"`.
3. Provide the corrected YAML.

**Constraints**

- Do not use a global filter that excludes archived notes.
- Keep the tag scope (`#project`) in a global filter.
- Use view-level filters for status-based filtering.

## Expected Behavior

1. Explain that the top-level `filters` block restricts every view in the base, so `status != "archived"` prevents the Archived view from ever matching any notes
2. Remove the `status != "archived"` condition from the global filter, keeping only the tag constraint
3. Ensure the "Active" view contains a view-level `filters` clause matching `status == "active"`
4. Ensure the "Archived" view contains a view-level `filters` clause matching `status == "archived"`
5. Produce syntactically valid YAML with no unquoted special characters

## Success Criteria

- **Explains global filter applies to all views**: The response explains that the top-level `filters` block restricts every view in the base, so the `status != "archived"` condition prevents the Archived view from ever matching any notes.
- **Global filter retains only the tag scope**: The corrected global `filters` block contains only the tag constraint (`file.hasTag("project")` or equivalent) and does not include any status-based exclusion.
- **Active view has view-level filter for active status**: The "Active" view contains a view-level `filters` clause that matches `status == "active"` (or equivalent).
- **Archived view has view-level filter for archived status**: The "Archived" view contains a view-level `filters` clause that matches `status == "archived"` (or equivalent).
- **Valid YAML structure**: The corrected file is syntactically valid YAML with no unquoted special characters and consistent indentation.

## Failure Conditions

- Does not explain that the global filter blocks all views including "Archived"
- Corrected global filter still includes `status != "archived"`, blocking archived notes
- "Active" view is missing a view-level filter for `status == "active"`
- "Archived" view is missing a view-level filter for `status == "archived"`
- Corrected YAML contains syntax errors or unquoted special characters

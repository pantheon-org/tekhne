# Scenario 2: Fix Three Bugs in a Broken Base File

The following `.base` file has three bugs. Identify each bug, explain why it is wrong,
and provide a corrected version of the file.

## Broken File

```yaml
filters: file.hasTag("project")

formulas:
  age: "(now() - file.ctime).round(0)"
  status_label: 'if(status == "active", "Active", "Inactive")'

properties:
  formula.priority:
    displayName: Priority
  formula.age:
    displayName: "Age (days)"
  status:
    displayName: Status: Current

views:
  - type: table
    name: "All Projects"
    order:
      - file.basename
      - formula.priority
      - formula.age
      - status
```

## Bugs to Find

1. A formula that calls a method directly on a Duration value.
2. A formula referenced in `properties` and `order` that is never defined in `formulas`.
3. A `displayName` value that contains an unquoted colon.

## Deliverable

For each bug:
- State which line or key is wrong.
- Explain what the error is and why it causes a problem.
- Show the corrected YAML for that section.

Then provide the complete corrected `projects.base` file.

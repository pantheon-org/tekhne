# Scenario 02: Fix Three Bugs in a Broken Base File

## User Prompt

The following `.base` file has three bugs. Identify each bug, explain why it is wrong,
and provide a corrected version of the file.

**Broken file**

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

**Bugs to find**

1. A formula that calls a method directly on a Duration value.
2. A formula referenced in `properties` and `order` that is never defined in `formulas`.
3. A `displayName` value that contains an unquoted colon.

For each bug:
- State which line or key is wrong.
- Explain what the error is and why it causes a problem.
- Show the corrected YAML for that section.

Then provide the complete corrected `projects.base` file.

## Expected Behavior

1. Identify that `(now() - file.ctime).round(0)` fails because date subtraction returns a Duration, not a Number, and `.round()` is not available on Duration
2. Fix the Duration bug by accessing a numeric field (e.g., `.days`) on the Duration before calling `.round(0)`
3. Identify that `formula.priority` appears in `properties` and `order` but `priority` is not defined under `formulas`
4. Fix the undefined formula reference by either adding a `priority` formula definition or removing `formula.priority` from `properties` and `order`
5. Identify that `displayName: Status: Current` is invalid YAML because the second colon is not inside a quoted string
6. Fix the `displayName` quoting by wrapping the value in double quotes: `displayName: "Status: Current"`
7. Provide a complete, syntactically valid YAML file incorporating all three fixes

## Success Criteria

- **Identifies Duration.round() bug**: The agent correctly identifies that `(now() - file.ctime).round(0)` fails because date subtraction returns a Duration, not a Number, and `.round()` is not available on Duration.
- **Fixes Duration bug by accessing .days first**: The corrected formula accesses a numeric field (e.g., `.days`) on the Duration before calling `.round(0)`: `(now() - file.ctime).days.round(0)`.
- **Identifies undefined formula.priority reference**: The agent correctly identifies that `formula.priority` appears in `properties` and `order` but `priority` is not defined under `formulas`.
- **Fixes undefined formula reference**: The fix either adds a `priority` formula definition under `formulas`, or removes the `formula.priority` reference from `properties` and `order`.
- **Identifies unquoted colon in displayName**: The agent correctly identifies that `displayName: Status: Current` is invalid YAML because the second colon is not inside a quoted string.
- **Fixes displayName quoting**: The corrected `displayName` wraps the value in double quotes: `displayName: "Status: Current"`.
- **Provides complete corrected file**: The response includes a complete, syntactically valid YAML file that incorporates all three fixes without introducing new errors.

## Failure Conditions

- Fails to identify the `.round()` called on a Duration value as a bug
- Fixes the Duration bug incorrectly (e.g., removes `.round()` without accessing `.days` first)
- Fails to identify that `formula.priority` is referenced without being defined in `formulas`
- Leaves `formula.priority` in `properties` or `order` without adding a corresponding `formulas` entry
- Fails to identify the unquoted colon in `displayName: Status: Current` as invalid YAML
- Leaves the `displayName` unquoted in the corrected file
- Corrected file introduces new YAML syntax errors or omits one of the three fixes

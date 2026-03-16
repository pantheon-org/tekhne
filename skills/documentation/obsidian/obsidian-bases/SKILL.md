---
name: obsidian-bases
description: Create and edit Obsidian Bases (.base files) with views, filters, formulas, and summaries. Use when working with .base files, creating database-like views of notes, or when the user mentions Bases, table views, card views, filters, or formulas in Obsidian.
---

# Obsidian Bases Skill

## Workflow

1. **Create the file**: Create a `.base` file in the vault with valid YAML content
2. **Define scope**: Add `filters` to select which notes appear (by tag, folder, property, or date)
3. **Add formulas** (optional): Define computed properties in the `formulas` section
4. **Configure views**: Add one or more views (`table`, `cards`, `list`, or `map`) with `order` specifying which properties to display
5. **Validate**: Verify the file is valid YAML with no syntax errors. Check that all referenced properties and formulas exist. Common issues: unquoted strings containing special YAML characters, mismatched quotes in formula expressions, referencing `formula.X` without defining `X` in `formulas`
6. **Test in Obsidian**: Open the `.base` file in Obsidian to confirm the view renders correctly. If it shows a YAML error, check quoting rules below

## Schema

Base files use the `.base` extension and contain valid YAML.

```yaml
# Global filters apply to ALL views in the base
filters:
  # Can be a single filter string
  # OR a recursive filter object with and/or/not
  and: []
  or: []
  not: []

# Define formula properties that can be used across all views
formulas:
  formula_name: 'expression'

# Configure display names and settings for properties
properties:
  property_name:
    displayName: "Display Name"
  formula.formula_name:
    displayName: "Formula Display Name"
  file.ext:
    displayName: "Extension"

# Define custom summary formulas
summaries:
  custom_summary_name: 'values.mean().round(3)'

# Define one or more views
views:
  - type: table | cards | list | map
    name: "View Name"
    limit: 10                    # Optional: limit results
    groupBy:                     # Optional: group results
      property: property_name
      direction: ASC | DESC
    filters:                     # View-specific filters
      and: []
    order:                       # Properties to display in order
      - file.name
      - property_name
      - formula.formula_name
    summaries:                   # Map properties to summary formulas
      property_name: Average
```

## Filter Syntax

Filters narrow down results. They can be applied globally or per-view.

### Filter Structure

```yaml
# Single filter
filters: 'status == "done"'

# AND - all conditions must be true
filters:
  and:
    - 'status == "done"'
    - 'priority > 3'

# OR - any condition can be true
filters:
  or:
    - 'file.hasTag("book")'
    - 'file.hasTag("article")'

# NOT - exclude matching items
filters:
  not:
    - 'file.hasTag("archived")'

# Nested filters
filters:
  or:
    - file.hasTag("tag")
    - and:
        - file.hasTag("book")
        - file.hasLink("Textbook")
    - not:
        - file.hasTag("book")
        - file.inFolder("Required Reading")
```

### Filter Operators

| Operator | Description |
|----------|-------------|
| `==` | equals |
| `!=` | not equal |
| `>` | greater than |
| `<` | less than |
| `>=` | greater than or equal |
| `<=` | less than or equal |
| `&&` | logical and |
| `\|\|` | logical or |
| `!` | logical not |

## Properties

### Three Types of Properties

1. **Note properties** - From frontmatter: `note.author` or just `author`
2. **File properties** - File metadata: `file.name`, `file.mtime`, etc.
3. **Formula properties** - Computed values: `formula.my_formula`

### File Properties Reference

| Property | Type | Description |
|----------|------|-------------|
| `file.name` | String | File name |
| `file.basename` | String | File name without extension |
| `file.path` | String | Full path to file |
| `file.folder` | String | Parent folder path |
| `file.ext` | String | File extension |
| `file.size` | Number | File size in bytes |
| `file.ctime` | Date | Created time |
| `file.mtime` | Date | Modified time |
| `file.tags` | List | All tags in file |
| `file.links` | List | Internal links in file |
| `file.backlinks` | List | Files linking to this file |
| `file.embeds` | List | Embeds in the note |
| `file.properties` | Object | All frontmatter properties |

### The `this` Keyword

- In main content area: refers to the base file itself
- When embedded: refers to the embedding file
- In sidebar: refers to the active file in main content

## Formula Syntax

Formulas compute values from properties. Defined in the `formulas` section.

```yaml
formulas:
  # Simple arithmetic
  total: "price * quantity"

  # Conditional logic
  status_icon: 'if(done, "✅", "⏳")'

  # String formatting
  formatted_price: 'if(price, price.toFixed(2) + " dollars")'

  # Date formatting
  created: 'file.ctime.format("YYYY-MM-DD")'

  # Calculate days since created (use .days for Duration)
  days_old: '(now() - file.ctime).days'

  # Calculate days until due date
  days_until_due: 'if(due_date, (date(due_date) - today()).days, "")'
```

## Key Functions

Most commonly used functions. For the complete reference of all types (Date, String, Number, List, File, Link, Object, RegExp), see [FUNCTIONS_REFERENCE.md](references/FUNCTIONS_REFERENCE.md).

| Function | Signature | Description |
|----------|-----------|-------------|
| `date()` | `date(string): date` | Parse string to date (`YYYY-MM-DD HH:mm:ss`) |
| `now()` | `now(): date` | Current date and time |
| `today()` | `today(): date` | Current date (time = 00:00:00) |
| `if()` | `if(condition, trueResult, falseResult?)` | Conditional |
| `duration()` | `duration(string): duration` | Parse duration string |
| `file()` | `file(path): file` | Get file object |
| `link()` | `link(path, display?): Link` | Create a link |

### Duration Type

When subtracting two dates, the result is a **Duration** type (not a number).

**Duration Fields:** `duration.days`, `duration.hours`, `duration.minutes`, `duration.seconds`, `duration.milliseconds`

**IMPORTANT:** Duration does NOT support `.round()`, `.floor()`, `.ceil()` directly. Access a numeric field first (like `.days`), then apply number functions.

```yaml
# CORRECT: Calculate days between dates
"(date(due_date) - today()).days"                    # Returns number of days
"(now() - file.ctime).days"                          # Days since created
"(date(due_date) - today()).days.round(0)"           # Rounded days

# WRONG - will cause error:
# "((date(due) - today()) / 86400000).round(0)"      # Duration doesn't support division then round
```

### Date Arithmetic

```yaml
# Duration units: y/year/years, M/month/months, d/day/days,
#                 w/week/weeks, h/hour/hours, m/minute/minutes, s/second/seconds
"now() + \"1 day\""       # Tomorrow
"today() + \"7d\""        # A week from today
"now() - file.ctime"      # Returns Duration
"(now() - file.ctime).days"  # Get days as number
```

## View Types

### Table View

```yaml
views:
  - type: table
    name: "My Table"
    order:
      - file.name
      - status
      - due_date
    summaries:
      price: Sum
      count: Average
```

### Cards View

```yaml
views:
  - type: cards
    name: "Gallery"
    order:
      - file.name
      - cover_image
      - description
```

### List View

```yaml
views:
  - type: list
    name: "Simple List"
    order:
      - file.name
      - status
```

### Map View

Requires latitude/longitude properties and the Maps community plugin.

```yaml
views:
  - type: map
    name: "Locations"
    # Map-specific settings for lat/lng properties
```

## Default Summary Formulas

| Name | Input Type | Description |
|------|------------|-------------|
| `Average` | Number | Mathematical mean |
| `Min` | Number | Smallest number |
| `Max` | Number | Largest number |
| `Sum` | Number | Sum of all numbers |
| `Range` | Number | Max - Min |
| `Median` | Number | Mathematical median |
| `Stddev` | Number | Standard deviation |
| `Earliest` | Date | Earliest date |
| `Latest` | Date | Latest date |
| `Range` | Date | Latest - Earliest |
| `Checked` | Boolean | Count of true values |
| `Unchecked` | Boolean | Count of false values |
| `Empty` | Any | Count of empty values |
| `Filled` | Any | Count of non-empty values |
| `Unique` | Any | Count of unique values |

## Complete Examples

### Task Tracker Base

```yaml
filters:
  and:
    - file.hasTag("task")
    - 'file.ext == "md"'

formulas:
  days_until_due: 'if(due, (date(due) - today()).days, "")'
  is_overdue: 'if(due, date(due) < today() && status != "done", false)'
  priority_label: 'if(priority == 1, "🔴 High", if(priority == 2, "🟡 Medium", "🟢 Low"))'

properties:
  status:
    displayName: Status
  formula.days_until_due:
    displayName: "Days Until Due"
  formula.priority_label:
    displayName: Priority

views:
  - type: table
    name: "Active Tasks"
    filters:
      and:
        - 'status != "done"'
    order:
      - file.name
      - status
      - formula.priority_label
      - due
      - formula.days_until_due
    groupBy:
      property: status
      direction: ASC
    summaries:
      formula.days_until_due: Average

  - type: table
    name: "Completed"
    filters:
      and:
        - 'status == "done"'
    order:
      - file.name
      - completed_date
```

### Reading List Base

```yaml
filters:
  or:
    - file.hasTag("book")
    - file.hasTag("article")

formulas:
  reading_time: 'if(pages, (pages * 2).toString() + " min", "")'
  status_icon: 'if(status == "reading", "📖", if(status == "done", "✅", "📚"))'
  year_read: 'if(finished_date, date(finished_date).year, "")'

properties:
  author:
    displayName: Author
  formula.status_icon:
    displayName: ""
  formula.reading_time:
    displayName: "Est. Time"

views:
  - type: cards
    name: "Library"
    order:
      - cover
      - file.name
      - author
      - formula.status_icon
    filters:
      not:
        - 'status == "dropped"'

  - type: table
    name: "Reading List"
    filters:
      and:
        - 'status == "to-read"'
    order:
      - file.name
      - author
      - pages
      - formula.reading_time
```

### Daily Notes Index

```yaml
filters:
  and:
    - file.inFolder("Daily Notes")
    - '/^\d{4}-\d{2}-\d{2}$/.matches(file.basename)'

formulas:
  word_estimate: '(file.size / 5).round(0)'
  day_of_week: 'date(file.basename).format("dddd")'

properties:
  formula.day_of_week:
    displayName: "Day"
  formula.word_estimate:
    displayName: "~Words"

views:
  - type: table
    name: "Recent Notes"
    limit: 30
    order:
      - file.name
      - formula.day_of_week
      - formula.word_estimate
      - file.mtime
```

## Common Mistakes

These are workflow- and design-level anti-patterns that cause silent failures or runtime
errors in Obsidian Bases. Each is distinct from the YAML syntax issues covered in
Troubleshooting.

### Anti-Pattern 1: Accessing formula properties without null guards

**NEVER** call property-dependent functions such as `date(due_date)` without first
checking that the property exists. Calling `date(due_date)` when `due_date` is empty
throws a runtime error and the whole formula column goes blank.

**WHY:** Obsidian Bases evaluates formulas against every note in scope. A single note
with a missing property causes the entire formula column to blank out silently for all
rows, making data appear lost when it is simply unguarded.

```yaml
# BAD - crashes when due_date is missing
formulas:
  days_until_due: "(date(due_date) - today()).days"
```

```yaml
# GOOD - guard with if() before accessing the property
formulas:
  days_until_due: 'if(due_date, (date(due_date) - today()).days, "")'
```

Always wrap property-dependent expressions in `if(property, ..., fallback)`.

### Anti-Pattern 2: Calling .round() directly on a Duration result

**NEVER** call `.round()`, `.floor()`, or `.ceil()` directly on the result of a date
subtraction. Date subtraction returns a **Duration**, not a Number, and Duration does
not expose those methods. Calling them directly causes an error.

```yaml
# BAD - Duration is not a Number; .round() is undefined on it
formulas:
  age: "(now() - file.ctime).round(0)"
```

```yaml
# GOOD - extract a numeric field first, then round
formulas:
  age: "(now() - file.ctime).days.round(0)"
```

Access `.days`, `.hours`, `.minutes`, etc. before applying any Number methods.

**WHY:** Duration and Number are distinct types. Skipping the field accessor means you
are calling a Number method on an object that does not have it, causing a formula error
that suppresses output for the entire column.

### Anti-Pattern 3: Referencing formula.X without defining X in formulas

**NEVER** reference `formula.X` in `order` or `properties` without a matching key in
the `formulas` block. If `formula.priority` appears in `order` or `properties` but
`priority` is not listed under `formulas`, Obsidian silently drops the column.
No error is shown.

```yaml
# BAD - formula.priority referenced but never defined
views:
  - type: table
    order:
      - formula.priority
```

```yaml
# GOOD - define the formula before referencing it
formulas:
  priority: 'if(priority == 1, "High", if(priority == 2, "Medium", "Low"))'

views:
  - type: table
    order:
      - formula.priority
```

Always verify every `formula.X` reference has a matching key in `formulas`.

**WHY:** Obsidian does not raise an error for undefined formula references — it simply
omits the column. The silent failure makes it easy to spend time debugging data
visibility when the root cause is a missing `formulas` entry.

### Anti-Pattern 4: Using unquoted strings that contain special YAML characters

**NEVER** leave strings unquoted when they contain characters that YAML treats as
structural syntax. Characters like `:`, `{`, `}`, `[`, `]`, and `#` are meaningful in
YAML. An unquoted value containing them causes a parse error and the entire base fails
to load.

```yaml
# BAD - the colon inside the value breaks YAML parsing
properties:
  status:
    displayName: Status: Active
```

```yaml
# GOOD - wrap in double quotes
properties:
  status:
    displayName: "Status: Active"
```

Any string that contains `:`, `{`, `}`, `[`, `]`, `,`, `&`, `*`, `#`, `?`, `|`, `-`,
`<`, `>`, `=`, `!`, `%`, `@`, or a backtick must be quoted.

**WHY:** A YAML parse error prevents the entire `.base` file from loading at all.
Obsidian shows a generic error with no indication of which line is at fault, making
unquoted special characters one of the hardest classes of mistake to diagnose quickly.

### Anti-Pattern 5: Applying global filters when you only want per-view filtering

**NEVER** place status- or state-specific conditions in the top-level `filters` block
when your base contains views that need to show different subsets of those notes.
A `filters` block at the top level applies to **all views**. If you want one view to
show active items and another to show archived ones, a global filter that excludes
archived notes will silently remove them from every view.

```yaml
# BAD - global filter prevents the "Archived" view from ever showing archived notes
filters: 'file.hasTag("project")'

views:
  - type: table
    name: "Active"
    filters: 'status == "active"'
  - type: table
    name: "Archived"
    filters: 'status == "archived"'   # never shows anything — global filter already excluded them
```

```yaml
# GOOD - global filter contains only the invariant scope; per-view filters handle the rest
filters: 'file.hasTag("project")'

views:
  - type: table
    name: "Active"
    filters: 'status == "active"'
  - type: table
    name: "Archived"
    # remove global archive-exclusion or add the archive tag to the global filter scope
    filters: 'status == "archived"'
```

Move status-specific conditions to view-level `filters`. Keep global `filters` only for
properties that are truly shared across all views (e.g., folder or tag scope).

**WHY:** Global filters act as an invisible pre-filter that view-level filters cannot
override. Notes excluded at the global level never reach any view, so a view that looks
correct in isolation will still silently suppress those records.

### Anti-Pattern 6: Using file.name when file.basename is needed for display

**NEVER** use `file.name` as a display column in table or cards views when you want
clean note titles. `file.name` includes the file extension (e.g., `My Note.md`).
`file.basename` returns only the title without the extension (e.g., `My Note`).
Displaying `file.name` in a table or cards view looks cluttered.

```yaml
# BAD - shows "Book Title.md" instead of "Book Title"
views:
  - type: cards
    order:
      - file.name
      - author
```

```yaml
# GOOD - shows "Book Title"
views:
  - type: cards
    order:
      - file.basename
      - author
```

Use `file.name` only when the extension itself is meaningful (e.g., comparing `.md` vs
`.canvas` files). For all display purposes, prefer `file.basename`.

**WHY:** Every Obsidian note has the `.md` extension, so `file.name` in a display
column produces uniformly cluttered output (`Title.md`, `Title.md`, ...) with no
benefit. The extension adds visual noise without conveying information.

## Embedding Bases

Embed in Markdown files:

```markdown
![[MyBase.base]]

<!-- Specific view -->
![[MyBase.base#View Name]]
```

## YAML Quoting Rules

- Use single quotes for formulas containing double quotes: `'if(done, "Yes", "No")'`
- Use double quotes for simple strings: `"My View Name"`
- Escape nested quotes properly in complex expressions

## Troubleshooting

### YAML Syntax Errors

**Unquoted special characters**: Strings containing `:`, `{`, `}`, `[`, `]`, `,`, `&`, `*`, `#`, `?`, `|`, `-`, `<`, `>`, `=`, `!`, `%`, `@`, `` ` `` must be quoted.

```yaml
# WRONG - colon in unquoted string
displayName: Status: Active

# CORRECT
displayName: "Status: Active"
```

**Mismatched quotes in formulas**: When a formula contains double quotes, wrap the entire formula in single quotes.

```yaml
# WRONG - double quotes inside double quotes
formulas:
  label: "if(done, "Yes", "No")"

# CORRECT - single quotes wrapping double quotes
formulas:
  label: 'if(done, "Yes", "No")'
```

### Common Formula Errors

**Duration math without field access**: Subtracting dates returns a Duration, not a number. Always access `.days`, `.hours`, etc.

```yaml
# WRONG - Duration is not a number
"(now() - file.ctime).round(0)"

# CORRECT - access .days first, then round
"(now() - file.ctime).days.round(0)"
```

**Missing null checks**: Properties may not exist on all notes. Use `if()` to guard.

```yaml
# WRONG - crashes if due_date is empty
"(date(due_date) - today()).days"

# CORRECT - guard with if()
'if(due_date, (date(due_date) - today()).days, "")'
```

**Referencing undefined formulas**: Ensure every `formula.X` in `order` or `properties` has a matching entry in `formulas`.

```yaml
# This will fail silently if 'total' is not defined in formulas
order:
  - formula.total

# Fix: define it
formulas:
  total: "price * quantity"
```

## References

- [Bases Syntax](https://help.obsidian.md/bases/syntax)
- [Functions](https://help.obsidian.md/bases/functions)
- [Views](https://help.obsidian.md/bases/views)
- [Formulas](https://help.obsidian.md/formulas)
- [Complete Functions Reference](references/FUNCTIONS_REFERENCE.md)

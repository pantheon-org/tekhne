# Scenario 05: Daily Notes Index Base

## User Prompt

Create an Obsidian Bases file named `daily-notes-index.base` for a vault that stores
daily notes in a folder called `Daily Notes`.

**Requirements**

- **Scope**: Show only notes inside the `Daily Notes` folder. Do not include notes from
  subfolders or other locations.
- **Columns** (in order): file basename, day of week (formula), approximate word count
  (formula), and the file's last-modified time.
- **Day-of-week formula**: Derive the day of week from the file's basename, which
  follows the `YYYY-MM-DD` format. Format it as the full weekday name (e.g., `Monday`).
- **Word estimate formula**: Estimate word count from `file.size` (bytes). Use the
  approximation of 1 word per 5 bytes (i.e., divide file.size by 5 and round to the
  nearest integer).
- **Sorting**: Sort by file basename in descending order (newest notes first).
- **Limit**: Show only the 50 most recent notes.
- **Define all formulas before referencing them**: Ensure every `formula.X` that appears
  in the view's order list has a matching definition in the top-level `formulas` section.

Provide the complete, valid YAML content for `daily-notes-index.base`.

## Expected Behavior

1. Use `file.inFolder("Daily Notes")` or an equivalent path-based filter to restrict notes to that folder
2. Define a day-of-week formula under the top-level `formulas` key, before referencing it in the view
3. Parse `file.basename` as a date and call `.format("dddd")` (or equivalent weekday format) to produce the full weekday name
4. Define a word-count estimate formula using `file.size` divided by 5 and rounded to an integer
5. Ensure every `formula.X` reference in the view's order list has a matching key in `formulas`
6. Sort the view by `file.basename` in descending order so newest dates appear first
7. Apply `limit: 50` to the view to restrict output to the 50 most recent notes
8. Produce syntactically valid YAML

## Success Criteria

- **file.inFolder filter used for Daily Notes folder**: The global filters include `file.inFolder("Daily Notes")` or an equivalent path-based filter to restrict notes to that folder.
- **day_of_week formula defined in formulas section**: A formula for day of week is defined under the top-level `formulas` key before it is referenced in the view.
- **day_of_week formula uses .format() with a weekday pattern**: The formula parses `file.basename` as a date and calls `.format("dddd")` (or an equivalent weekday format pattern) to produce the full weekday name.
- **word_estimate formula uses file.size**: A formula for word count estimate is defined and uses `file.size` divided by 5 (or a close approximation), rounded to an integer.
- **All formula.X references have matching formulas definitions**: Every `formula.X` that appears in the view's order list has a corresponding key defined under the top-level `formulas` section. No dangling references.
- **View sorted descending by file basename**: The view includes a sort or order configuration that sorts by `file.basename` (or `file.name`) in descending direction so newest dates appear first.
- **Limit of 50 applied to the view**: The view includes `limit: 50` (or equivalent) to restrict output to the 50 most recent notes.
- **Valid YAML structure**: The output is syntactically valid YAML. All special characters in strings are quoted. Indentation is consistent.

## Failure Conditions

- No `file.inFolder("Daily Notes")` or equivalent folder filter is present
- Day-of-week formula is missing from the `formulas` section
- Day-of-week formula does not parse `file.basename` as a date or does not format it as a full weekday name
- Word-count estimate formula is missing or does not use `file.size`
- Any `formula.X` reference in the view's order list has no matching key in `formulas`
- View is not sorted in descending order by file basename
- `limit: 50` is not applied to the view
- Output YAML contains syntax errors

# Scenario 5: Daily Notes Index Base

Create an Obsidian Bases file named `daily-notes-index.base` for a vault that stores
daily notes in a folder called `Daily Notes`.

## Requirements

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
- **Define all formulas before referencing them**: Ensure every formula.X that appears
  in the view's order list has a matching definition in the top-level formulas section.

## Deliverable

Provide the complete, valid YAML content for `daily-notes-index.base`.

# Scenario 3: Reading List Base with Cards View

Create an Obsidian Bases file named `reading-list.base` for a book-tracking vault.

## Requirements

- **Scope**: Show all notes tagged `#book`.
- **View type**: A `cards` view (not a table). Name it "Library".
- **Columns** (in order in the cards view): file basename, author, status, and an
  estimated reading time formula.
- **Reading time formula**: Compute estimated reading time in minutes from a `pages`
  frontmatter property. Assume 2 minutes per page. The formula must safely handle notes
  where `pages` is not set.
- **Grouping**: Group the cards view by the `status` property in ascending order.
  Expected status values are `"to-read"`, `"reading"`, and `"done"`.
- **Second view**: Add a simple table view called "Stats" that shows file basename,
  pages, and the reading time formula, with a Sum summary on the pages column.

## Deliverable

Provide the complete, valid YAML content for `reading-list.base`.

# Scenario 03: Add Block IDs to Sprint Task List Items

## User Prompt

You are maintaining an Obsidian vault for a software project. The note `sprint-tasks.md` contains a list of tasks that other notes need to reference individually. Add block IDs to the list items so each one can be linked to from other notes.

Here is the current content of the list section:

```markdown
## Sprint 3 Tasks

- Implement user authentication
- Set up CI/CD pipeline
- Write integration tests
```

Write a new version of this section to a file called `sprint-tasks-updated.md`. Then, in the same file, add a short example showing how another note would link to the second task using a block reference.

## Expected Behavior

1. Add block IDs on separate lines after the list block, not appended inline to a list item (e.g., `- Item ^id` is incorrect; the ID must be on its own line after the list)
2. Use only letters, numbers, and hyphens in block IDs (e.g., `^auth-task`, `^ci-cd-task`) — no spaces or special characters
3. Demonstrate the block reference syntax using `[[sprint-tasks#^block-id]]` or `[[sprint-tasks-updated#^block-id]]` (note name + `#^` + block ID inside wikilink)

## Success Criteria

- **Block IDs on separate lines after list**: The block ID(s) appear on a separate line after the list block, not appended inline to a list item (e.g., `- Item ^id` is incorrect; the ID must be on its own line after the list)
- **Block ID format is valid**: Block IDs use only letters, numbers, and hyphens (e.g., `^auth-task`, `^ci-cd-task`) — no spaces or special characters
- **Block reference syntax demonstrated**: The example reference uses `[[sprint-tasks#^block-id]]` or `[[sprint-tasks-updated#^block-id]]` syntax (note name + `#^` + block ID inside wikilink)

## Failure Conditions

- Block IDs are appended inline to the list item (e.g., `- Implement user authentication ^auth-task`) instead of on a separate line
- Block IDs contain spaces or special characters that make them invalid
- The example block reference does not use `[[note-name#^block-id]]` syntax

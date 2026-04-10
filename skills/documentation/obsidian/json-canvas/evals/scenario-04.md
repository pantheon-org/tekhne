# Scenario 04: Create a Canvas with a Group Containing Child Nodes

## User Prompt

Create a new Obsidian canvas file named `team.canvas` that has a group node visually containing three text nodes.

**Canvas structure**

- A **group** node labelled `Engineering Team` at position x=0, y=0 with width=900, height=400
- Three **text** nodes inside the group:
  - `Alice` at x=50, y=50, width=220, height=80
  - `Bob` at x=330, y=50, width=220, height=80
  - `Carol` at x=610, y=50, width=220, height=80

**Requirements**

- All four nodes must have unique 16-character lowercase hex IDs
- The three text nodes must be positioned entirely within the group bounds (x=0..900, y=0..400)
- The file must be valid JSON with a `.canvas` extension
- Do not add any edges

## Expected Behavior

1. Include at least one node with `type` set to `group` and a `label` field
2. Position all three text nodes so their x and y coordinates place them fully within the group's x/y/width/height rectangle
3. Assign unique 16-character lowercase hex IDs to all four nodes
4. Produce valid JSON in a file with a `.canvas` extension

## Success Criteria

- **Group node present with type=group**: The canvas contains at least one node with `type` set to `group` and a `label` field
- **Three text nodes inside group bounds**: All three text nodes have x and y coordinates that place them fully within the group's x/y/width/height rectangle
- **All node IDs are unique 16-char hex**: Every node has an `id` that is exactly 16 lowercase hexadecimal characters, and all four IDs are distinct
- **Valid JSON output**: The canvas file content is syntactically valid JSON with no parse errors

## Failure Conditions

- No node has `type: group` or the group node is missing a `label` field
- Any text node's position falls outside the group's bounding rectangle (x=0..900, y=0..400)
- Any node has an ID that is not exactly 16 lowercase hexadecimal characters, or two nodes share the same ID
- The canvas file content contains a JSON syntax error

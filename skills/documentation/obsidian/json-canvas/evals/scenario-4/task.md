# Create a Canvas with a Group Containing Child Nodes

Create a new Obsidian canvas file named `team.canvas` that has a group node visually containing three text nodes.

## Canvas structure

- A **group** node labelled `Engineering Team` at position x=0, y=0 with width=900, height=400
- Three **text** nodes inside the group:
  - `Alice` at x=50, y=50, width=220, height=80
  - `Bob` at x=330, y=50, width=220, height=80
  - `Carol` at x=610, y=50, width=220, height=80

## Requirements

- All four nodes must have unique 16-character lowercase hex IDs
- The three text nodes must be positioned entirely within the group bounds (x=0..900, y=0..400)
- The file must be valid JSON with a `.canvas` extension
- Do not add any edges

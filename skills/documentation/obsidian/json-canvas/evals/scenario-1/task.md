# Create a 3-Node Flowchart Canvas

Create a new Obsidian canvas file named `flowchart.canvas` that represents a simple decision flowchart with three nodes and two edges.

## Nodes

1. **Start** — a text node at position x=0, y=0 (width=200, height=80) with the text `Start`
2. **Decision** — a text node at position x=300, y=0 (width=200, height=80) with the text `Decision\nYes or No?`
3. **End** — a text node at position x=600, y=0 (width=200, height=80) with the text `End`

## Edges

- An edge from **Start** to **Decision** with `toEnd` set to `arrow`
- An edge from **Decision** to **End** with `toEnd` set to `arrow` and a label of `Yes`

## Requirements

- All node and edge IDs must be unique 16-character lowercase hexadecimal strings
- The file must be valid JSON
- The file extension must be `.canvas`
- Text node content that spans multiple lines must use the `\n` JSON escape, not literal newlines

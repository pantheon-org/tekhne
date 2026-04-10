# Scenario 01: Create a 3-Node Flowchart Canvas

## User Prompt

Create a new Obsidian canvas file named `flowchart.canvas` that represents a simple decision flowchart with three nodes and two edges.

**Nodes**

1. **Start** — a text node at position x=0, y=0 (width=200, height=80) with the text `Start`
2. **Decision** — a text node at position x=300, y=0 (width=200, height=80) with the text `Decision\nYes or No?`
3. **End** — a text node at position x=600, y=0 (width=200, height=80) with the text `End`

**Edges**

- An edge from **Start** to **Decision** with `toEnd` set to `arrow`
- An edge from **Decision** to **End** with `toEnd` set to `arrow` and a label of `Yes`

**Requirements**

- All node and edge IDs must be unique 16-character lowercase hexadecimal strings
- The file must be valid JSON
- The file extension must be `.canvas`
- Text node content that spans multiple lines must use the `\n` JSON escape, not literal newlines

## Expected Behavior

1. Produce syntactically valid JSON in a file with a `.canvas` extension
2. Assign unique 16-character lowercase hexadecimal IDs to all nodes and edges
3. Ensure all edge `fromNode` and `toNode` values reference an `id` present in the nodes array
4. Include exactly three text nodes representing Start, Decision, and End

## Success Criteria

- **Valid JSON output**: The canvas file content is syntactically valid JSON with no parse errors
- **Correct file extension**: The output file is named with a `.canvas` extension (e.g., `flowchart.canvas`)
- **16-character hex IDs on all nodes**: Every node object has an `id` field that is exactly 16 lowercase hexadecimal characters
- **16-character hex IDs on all edges**: Every edge object has an `id` field that is exactly 16 lowercase hexadecimal characters
- **All edge references resolve**: Both `fromNode` and `toNode` values on every edge match an `id` present in the nodes array
- **Three nodes present**: Canvas contains exactly three text nodes representing Start, Decision, and End

## Failure Conditions

- Canvas file content contains a JSON syntax error
- File is not named with a `.canvas` extension
- Any node or edge has an ID that is not exactly 16 lowercase hexadecimal characters
- Any edge has an ID that is not exactly 16 lowercase hexadecimal characters
- Any edge `fromNode` or `toNode` references an ID that does not exist in the nodes array
- Canvas contains fewer or more than three text nodes

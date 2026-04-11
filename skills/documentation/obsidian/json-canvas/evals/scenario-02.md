# Scenario 02: Add a Node to an Existing Canvas

## User Prompt

The following canvas file already exists at `project.canvas`. Add a new text node to it without colliding with existing nodes.

**Existing canvas content**

```json
{
  "nodes": [
    {
      "id": "a1b2c3d4e5f67890",
      "type": "text",
      "x": 0,
      "y": 0,
      "width": 300,
      "height": 120,
      "text": "Project Overview"
    },
    {
      "id": "b2c3d4e5f6789001",
      "type": "text",
      "x": 400,
      "y": 0,
      "width": 300,
      "height": 120,
      "text": "Goals"
    }
  ],
  "edges": [
    {
      "id": "e1f2a3b4c5d6e7f8",
      "fromNode": "a1b2c3d4e5f67890",
      "toNode": "b2c3d4e5f6789001",
      "toEnd": "arrow"
    }
  ]
}
```

Add a new text node with the content `Risks` to the canvas. The new node should:

- Have a unique ID that does not collide with any existing node or edge IDs
- Be positioned so it does not overlap the existing nodes (leave at least 50px spacing)
- Use a width of 300 and height of 120

Write out the complete updated canvas JSON.

## Expected Behavior

1. Produce syntactically valid JSON after adding the new node
2. Assign the new Risks node an ID that does not match any existing node or edge ID in the original canvas
3. Position the new node so it does not spatially overlap either of the two existing nodes, with at least 50px spacing
4. Preserve all two original nodes and the original edge unchanged in the output

## Success Criteria

- **Valid JSON after modification**: The resulting canvas JSON is syntactically valid with no parse errors
- **New node has unique ID**: The new Risks node has an `id` that does not match any existing node or edge ID in the original canvas
- **New node does not overlap existing nodes**: The new node position and dimensions do not spatially overlap either of the two existing nodes, with at least 50px spacing
- **Existing nodes and edges preserved**: All two original nodes and the original edge are still present and unchanged in the output

## Failure Conditions

- The resulting canvas JSON has a syntax error
- The new Risks node has an ID that matches an existing node or edge ID
- The new node's position and dimensions spatially overlap one of the existing nodes with less than 50px spacing
- Any original node or the original edge is missing or has been modified in the output

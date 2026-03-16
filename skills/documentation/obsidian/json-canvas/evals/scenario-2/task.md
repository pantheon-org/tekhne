# Add a Node to an Existing Canvas

The following canvas file already exists at `project.canvas`. Add a new text node to it without colliding with existing nodes.

## Existing canvas content

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

## Task

Add a new text node with the content `Risks` to the canvas. The new node should:

- Have a unique ID that does not collide with any existing node or edge IDs
- Be positioned so it does not overlap the existing nodes (leave at least 50px spacing)
- Use a width of 300 and height of 120

Write out the complete updated canvas JSON.

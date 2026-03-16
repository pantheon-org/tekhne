# Fix a Broken Canvas File

The following canvas file has two bugs that prevent it from rendering correctly. Identify and fix both issues, then output the corrected canvas JSON.

## Broken canvas content

```json
{
  "nodes": [
    {
      "id": "deadbeefdeadbeef",
      "type": "text",
      "x": 0,
      "y": 0,
      "width": 250,
      "height": 100,
      "text": "Node A"
    },
    {
      "id": "deadbeefdeadbeef",
      "type": "text",
      "x": 350,
      "y": 0,
      "width": 250,
      "height": 100,
      "text": "Node B"
    },
    {
      "id": "cafebabe00000001",
      "type": "text",
      "x": 700,
      "y": 0,
      "width": 250,
      "height": 100,
      "text": "Node C"
    }
  ],
  "edges": [
    {
      "id": "edge000000000001",
      "fromNode": "deadbeefdeadbeef",
      "toNode": "cafebabe00000001",
      "toEnd": "arrow"
    },
    {
      "id": "edge000000000002",
      "fromNode": "cafebabe00000001",
      "toNode": "9999999999999999",
      "toEnd": "arrow"
    }
  ]
}
```

## Known bugs

1. Two nodes share the same ID `deadbeefdeadbeef` — this is a duplicate ID violation
2. The second edge references `toNode: "9999999999999999"` which does not exist in the nodes array — this is a dangling edge reference

## Requirements

- Assign a new unique 16-character hex ID to the duplicate node (Node B)
- Fix the dangling edge by either removing it or updating `toNode` to reference an existing node ID
- Ensure all IDs are unique across nodes and edges
- Ensure all edge references resolve to existing node IDs
- Output valid JSON

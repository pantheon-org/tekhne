# Scenario 03: Fix a Broken Canvas File

## User Prompt

The following canvas file has two bugs that prevent it from rendering correctly. Identify and fix both issues, then output the corrected canvas JSON.

**Broken canvas content**

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

**Known bugs**

1. Two nodes share the same ID `deadbeefdeadbeef` — this is a duplicate ID violation
2. The second edge references `toNode: "9999999999999999"` which does not exist in the nodes array — this is a dangling edge reference

**Requirements**

- Assign a new unique 16-character hex ID to the duplicate node (Node B)
- Fix the dangling edge by either removing it or updating `toNode` to reference an existing node ID
- Ensure all IDs are unique across nodes and edges
- Ensure all edge references resolve to existing node IDs
- Output valid JSON

## Expected Behavior

1. Assign distinct IDs to the two nodes that previously shared `deadbeefdeadbeef`
2. Remove or fix the edge that referenced the non-existent `toNode: "9999999999999999"`
3. Ensure every `id` value across nodes and edges in the output is unique
4. Produce syntactically valid JSON with no parse errors

## Success Criteria

- **Duplicate ID resolved**: The two nodes that shared the id `deadbeefdeadbeef` now have distinct IDs
- **Dangling edge fixed**: The edge that referenced `toNode` `9999999999999999` has been removed or updated to point to a valid node ID
- **All IDs unique**: Every `id` value across nodes and edges in the output is unique — no duplicates remain
- **Valid JSON output**: The corrected canvas content is syntactically valid JSON with no parse errors

## Failure Conditions

- The two previously duplicate nodes still share an identical ID
- The edge referencing `9999999999999999` remains in the output pointing to a non-existent node
- Any duplicate ID remains across nodes or edges in the output
- The corrected canvas JSON has a syntax error

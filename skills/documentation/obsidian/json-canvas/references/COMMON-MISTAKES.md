# JSON Canvas Common Mistakes

## 1. Using literal newlines instead of `\n` escape in JSON strings

**NEVER** embed literal newline characters or double-escaped `\\n` sequences inside JSON string values. JSON strings must use the escape sequence `\n` for line breaks.

**WHY:** Literal newlines produce unparseable JSON that silently corrupts the canvas file, while `\\n` renders as visible backslash-n characters on screen instead of the intended line break.

**Bad** — literal newline (invalid JSON):

```json
{
  "id": "6f0ad84f44ce9c17",
  "type": "text",
  "text": "Line one
Line two"
}
```

**Bad** — double-escaped (renders as literal `\n` on screen):

```json
{
  "id": "6f0ad84f44ce9c17",
  "type": "text",
  "text": "Line one\\nLine two"
}
```

**Good** — single JSON escape:

```json
{
  "id": "6f0ad84f44ce9c17",
  "type": "text",
  "text": "Line one\nLine two"
}
```

## 2. Reusing the same ID for multiple nodes or edges

**NEVER** reuse the same `id` value for more than one node or edge. Every `id` must be unique across all nodes and edges.

**WHY:** Duplicate IDs cause one element to silently overwrite another, resulting in missing nodes or edges with no error reported by Obsidian.

**Bad**:

```json
{
  "nodes": [
    { "id": "aabbccdd11223344", "type": "text", "x": 0, "y": 0, "width": 200, "height": 100, "text": "Node A" },
    { "id": "aabbccdd11223344", "type": "text", "x": 300, "y": 0, "width": 200, "height": 100, "text": "Node B" }
  ]
}
```

**Good** — each ID is a distinct 16-char hex value:

```json
{
  "nodes": [
    { "id": "aabbccdd11223344", "type": "text", "x": 0, "y": 0, "width": 200, "height": 100, "text": "Node A" },
    { "id": "11223344aabbccdd", "type": "text", "x": 300, "y": 0, "width": 200, "height": 100, "text": "Node B" }
  ]
}
```

## 3. Creating edges that reference non-existent node IDs

**NEVER** set `fromNode` or `toNode` to an ID that does not exist in the `nodes` array.

**WHY:** Dangling edge references produce invisible edges with no error, making it impossible to diagnose missing connections just by viewing the canvas.

**Bad** — `toNode` references a missing ID:

```json
{
  "nodes": [
    { "id": "aabbccdd11223344", "type": "text", "x": 0, "y": 0, "width": 200, "height": 100, "text": "Start" }
  ],
  "edges": [
    { "id": "edge000000000001", "fromNode": "aabbccdd11223344", "toNode": "doesnotexist1234" }
  ]
}
```

**Good** — `toNode` resolves to an existing node:

```json
{
  "nodes": [
    { "id": "aabbccdd11223344", "type": "text", "x": 0, "y": 0, "width": 200, "height": 100, "text": "Start" },
    { "id": "11223344aabbccdd", "type": "text", "x": 300, "y": 0, "width": 200, "height": 100, "text": "End" }
  ],
  "edges": [
    { "id": "edge000000000001", "fromNode": "aabbccdd11223344", "toNode": "11223344aabbccdd" }
  ]
}
```

## 4. Placing child nodes outside group bounds

**NEVER** place a child node at coordinates outside the group's `x`/`y`/`width`/`height` rectangle.

**WHY:** Nodes outside the group bounds appear as free-floating canvas items, breaking the visual grouping.

**Bad** — child at x=1200 is outside group that ends at x=1000:

```json
{
  "nodes": [
    { "id": "group00000000001", "type": "group", "x": 200, "y": 100, "width": 800, "height": 500, "label": "My Group" },
    { "id": "child00000000001", "type": "text", "x": 1200, "y": 150, "width": 200, "height": 80, "text": "Orphaned child" }
  ]
}
```

**Good** — child fits inside group bounds (x=200..1000, y=100..600):

```json
{
  "nodes": [
    { "id": "group00000000001", "type": "group", "x": 200, "y": 100, "width": 800, "height": 500, "label": "My Group" },
    { "id": "child00000000001", "type": "text", "x": 250, "y": 150, "width": 200, "height": 80, "text": "Inside group" }
  ]
}
```

## 5. Setting `fromEnd`/`toEnd` to unsupported values

**NEVER** set `fromEnd` or `toEnd` to any value other than `"none"` or `"arrow"`.

**WHY:** Unsupported end-marker values are either silently discarded or cause the edge to render incorrectly, with no validation error.

**Bad**:

```json
{ "id": "edge000000000001", "fromNode": "aabbccdd11223344", "toNode": "11223344aabbccdd", "fromEnd": "circle", "toEnd": "diamond" }
```

**Good**:

```json
{ "id": "edge000000000001", "fromNode": "aabbccdd11223344", "toNode": "11223344aabbccdd", "fromEnd": "none", "toEnd": "arrow" }
```

## 6. Using integer color values instead of string color values

**NEVER** supply a bare integer for the `color` field. The `canvasColor` type is always a JSON string.

**WHY:** Canvas implementations that enforce strict typing will reject the file or ignore the color when an integer is provided instead of a quoted string.

**Bad** — integer value: `"color": 1`

**Good** — string value: `"color": "1"`

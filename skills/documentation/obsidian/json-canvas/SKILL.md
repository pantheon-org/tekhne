---
name: json-canvas
description: Create and edit JSON Canvas files (.canvas) with nodes, edges, groups, and connections. Use when working with .canvas files, creating visual canvases, mind maps, flowcharts, or when the user mentions Canvas files in Obsidian.
---

# JSON Canvas Skill

## File Structure

A canvas file (`.canvas`) contains two top-level arrays following the [JSON Canvas Spec 1.0](https://jsoncanvas.org/spec/1.0/):

```json
{
  "nodes": [],
  "edges": []
}
```

- `nodes` (optional): Array of node objects
- `edges` (optional): Array of edge objects connecting nodes

## Common Workflows

### 1. Create a New Canvas

1. Create a `.canvas` file with the base structure `{"nodes": [], "edges": []}`
2. Generate unique 16-character hex IDs for each node (e.g., `"6f0ad84f44ce9c17"`)
3. Add nodes with required fields: `id`, `type`, `x`, `y`, `width`, `height`
4. Add edges referencing valid node IDs via `fromNode` and `toNode`
5. **Validate**: Parse the JSON to confirm it is valid. Verify all `fromNode`/`toNode` values exist in the nodes array

### 2. Add a Node to an Existing Canvas

1. Read and parse the existing `.canvas` file
2. Generate a unique ID that does not collide with existing node or edge IDs
3. Choose position (`x`, `y`) that avoids overlapping existing nodes (leave 50-100px spacing)
4. Append the new node object to the `nodes` array
5. Optionally add edges connecting the new node to existing nodes
6. **Validate**: Confirm all IDs are unique and all edge references resolve to existing nodes

### 3. Connect Two Nodes

1. Identify the source and target node IDs
2. Generate a unique edge ID
3. Set `fromNode` and `toNode` to the source and target IDs
4. Optionally set `fromSide`/`toSide` (top, right, bottom, left) for anchor points
5. Optionally set `label` for descriptive text on the edge
6. Append the edge to the `edges` array
7. **Validate**: Confirm both `fromNode` and `toNode` reference existing node IDs

### 4. Edit an Existing Canvas

1. Read and parse the `.canvas` file as JSON
2. Locate the target node or edge by `id`
3. Modify the desired attributes (text, position, color, etc.)
4. Write the updated JSON back to the file
5. **Validate**: Re-check all ID uniqueness and edge reference integrity after editing

## Nodes

Nodes are objects placed on the canvas. Array order determines z-index: first node = bottom layer, last node = top layer.

### Generic Node Attributes

| Attribute | Required | Type | Description |
|-----------|----------|------|-------------|
| `id` | Yes | string | Unique 16-char hex identifier |
| `type` | Yes | string | `text`, `file`, `link`, or `group` |
| `x` | Yes | integer | X position in pixels |
| `y` | Yes | integer | Y position in pixels |
| `width` | Yes | integer | Width in pixels |
| `height` | Yes | integer | Height in pixels |
| `color` | No | canvasColor | Preset `"1"`-`"6"` or hex (e.g., `"#FF0000"`) |

### Text Nodes

| Attribute | Required | Type | Description |
|-----------|----------|------|-------------|
| `text` | Yes | string | Plain text with Markdown syntax |

```json
{
  "id": "6f0ad84f44ce9c17",
  "type": "text",
  "x": 0,
  "y": 0,
  "width": 400,
  "height": 200,
  "text": "# Hello World\n\nThis is **Markdown** content."
}
```

**Newline pitfall**: Use `\n` for line breaks in JSON strings. Do **not** use the literal `\\n` -- Obsidian renders that as the characters `\` and `n`.

### File Nodes

| Attribute | Required | Type | Description |
|-----------|----------|------|-------------|
| `file` | Yes | string | Path to file within the system |
| `subpath` | No | string | Link to heading or block (starts with `#`) |

```json
{
  "id": "a1b2c3d4e5f67890",
  "type": "file",
  "x": 500,
  "y": 0,
  "width": 400,
  "height": 300,
  "file": "Attachments/diagram.png"
}
```

### Link Nodes

| Attribute | Required | Type | Description |
|-----------|----------|------|-------------|
| `url` | Yes | string | External URL |

```json
{
  "id": "c3d4e5f678901234",
  "type": "link",
  "x": 1000,
  "y": 0,
  "width": 400,
  "height": 200,
  "url": "https://obsidian.md"
}
```

### Group Nodes

Groups are visual containers for organizing other nodes. Position child nodes inside the group's bounds.

| Attribute | Required | Type | Description |
|-----------|----------|------|-------------|
| `label` | No | string | Text label for the group |
| `background` | No | string | Path to background image |
| `backgroundStyle` | No | string | `cover`, `ratio`, or `repeat` |

```json
{
  "id": "d4e5f6789012345a",
  "type": "group",
  "x": -50,
  "y": -50,
  "width": 1000,
  "height": 600,
  "label": "Project Overview",
  "color": "4"
}
```

## Edges

Edges connect nodes via `fromNode` and `toNode` IDs.

| Attribute | Required | Type | Default | Description |
|-----------|----------|------|---------|-------------|
| `id` | Yes | string | - | Unique identifier |
| `fromNode` | Yes | string | - | Source node ID |
| `fromSide` | No | string | - | `top`, `right`, `bottom`, or `left` |
| `fromEnd` | No | string | `none` | `none` or `arrow` |
| `toNode` | Yes | string | - | Target node ID |
| `toSide` | No | string | - | `top`, `right`, `bottom`, or `left` |
| `toEnd` | No | string | `arrow` | `none` or `arrow` |
| `color` | No | canvasColor | - | Line color |
| `label` | No | string | - | Text label |

```json
{
  "id": "0123456789abcdef",
  "fromNode": "6f0ad84f44ce9c17",
  "fromSide": "right",
  "toNode": "a1b2c3d4e5f67890",
  "toSide": "left",
  "toEnd": "arrow",
  "label": "leads to"
}
```

## Colors

The `canvasColor` type accepts either a hex string or a preset number:

| Preset | Color |
|--------|-------|
| `"1"` | Red |
| `"2"` | Orange |
| `"3"` | Yellow |
| `"4"` | Green |
| `"5"` | Cyan |
| `"6"` | Purple |

Preset color values are intentionally undefined -- applications use their own brand colors.

## ID Generation

Generate 16-character lowercase hexadecimal strings (64-bit random value):

```
"6f0ad84f44ce9c17"
"a3b2c1d0e9f8a7b6"
```

## Layout Guidelines

- Coordinates can be negative (canvas extends infinitely)
- `x` increases right, `y` increases down; position is the top-left corner
- Space nodes 50-100px apart; leave 20-50px padding inside groups
- Align to grid (multiples of 10 or 20) for cleaner layouts

| Node Type | Suggested Width | Suggested Height |
|-----------|-----------------|------------------|
| Small text | 200-300 | 80-150 |
| Medium text | 300-450 | 150-300 |
| Large text | 400-600 | 300-500 |
| File preview | 300-500 | 200-400 |
| Link preview | 250-400 | 100-200 |

## Validation Checklist

After creating or editing a canvas file, verify:

1. All `id` values are unique across both nodes and edges
2. Every `fromNode` and `toNode` references an existing node ID
3. Required fields are present for each node type (`text` for text nodes, `file` for file nodes, `url` for link nodes)
4. `type` is one of: `text`, `file`, `link`, `group`
5. `fromSide`/`toSide` values are one of: `top`, `right`, `bottom`, `left`
6. `fromEnd`/`toEnd` values are one of: `none`, `arrow`
7. Color presets are `"1"` through `"6"` or valid hex (e.g., `"#FF0000"`)
8. JSON is valid and parseable

If validation fails, check for duplicate IDs, dangling edge references, or malformed JSON strings (especially unescaped newlines in text content).

## Complete Examples

See [references/EXAMPLES.md](references/EXAMPLES.md) for full canvas examples including mind maps, project boards, research canvases, and flowcharts.

## Common Mistakes

### 1. Using literal newlines instead of `\n` escape in JSON strings

**NEVER** embed literal newline characters or double-escaped `\\n` sequences inside JSON string values. JSON strings must use the escape sequence `\n` for line breaks. A literal newline character inside a JSON string is invalid JSON. Additionally, do not write `\\n` (two characters: backslash + n) — Obsidian renders that as the characters `\` and `n` on screen.

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

### 2. Reusing the same ID for multiple nodes or edges

**NEVER** reuse the same `id` value for more than one node or edge in a canvas file. Every `id` must be unique across all nodes and edges. Duplicates cause silent rendering failures where one element overwrites another.

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

### 3. Creating edges that reference non-existent node IDs

**NEVER** set `fromNode` or `toNode` to an ID that does not exist in the `nodes` array. If either value points to a missing node, the edge silently fails to render. Always validate references after creating or editing edges.

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

### 4. Placing child nodes outside group bounds

**NEVER** place a child node at coordinates that fall outside the group's `x`/`y`/`width`/`height` rectangle. Child nodes positioned outside that boundary still render on the canvas but no longer appear visually inside the group. Always confirm child coordinates fall within the group boundary.

**WHY:** Nodes outside the group bounds appear as free-floating canvas items, breaking the visual grouping and confusing readers about which elements belong together.

**Bad** — child node at x=1200 is outside the group that ends at x=1000 (x=200, width=800):

```json
{
  "nodes": [
    { "id": "group00000000001", "type": "group", "x": 200, "y": 100, "width": 800, "height": 500, "label": "My Group" },
    { "id": "child00000000001", "type": "text", "x": 1200, "y": 150, "width": 200, "height": 80, "text": "Orphaned child" }
  ]
}
```

**Good** — child node fits inside group bounds (x=200..1000, y=100..600):

```json
{
  "nodes": [
    { "id": "group00000000001", "type": "group", "x": 200, "y": 100, "width": 800, "height": 500, "label": "My Group" },
    { "id": "child00000000001", "type": "text", "x": 250, "y": 150, "width": 200, "height": 80, "text": "Inside group" }
  ]
}
```

### 5. Setting `fromEnd`/`toEnd` to unsupported values

**NEVER** set `fromEnd` or `toEnd` to any value other than `"none"` or `"arrow"`. Any other string (e.g., `"circle"`, `"diamond"`, `"true"`) is silently ignored or breaks rendering.

**WHY:** Unsupported end-marker values are either silently discarded or cause the edge to render incorrectly, with no validation error to indicate which value was rejected.

**Bad**:

```json
{
  "id": "edge000000000001",
  "fromNode": "aabbccdd11223344",
  "toNode": "11223344aabbccdd",
  "fromEnd": "circle",
  "toEnd": "diamond"
}
```

**Good**:

```json
{
  "id": "edge000000000001",
  "fromNode": "aabbccdd11223344",
  "toNode": "11223344aabbccdd",
  "fromEnd": "none",
  "toEnd": "arrow"
}
```

### 6. Using integer color values instead of string color values

**NEVER** supply a bare integer for the `color` field. The `color` field is typed as `canvasColor`, which is always a JSON string. Using a bare integer causes a type mismatch that some canvas implementations reject.

**WHY:** Canvas implementations that enforce strict typing will reject the file or ignore the color entirely when an integer is provided instead of a quoted string.

**Bad** — integer value:

```json
{
  "id": "aabbccdd11223344",
  "type": "text",
  "x": 0,
  "y": 0,
  "width": 200,
  "height": 100,
  "text": "Warning",
  "color": 1
}
```

**Good** — string value:

```json
{
  "id": "aabbccdd11223344",
  "type": "text",
  "x": 0,
  "y": 0,
  "width": 200,
  "height": 100,
  "text": "Warning",
  "color": "1"
}
```

## References

- [JSON Canvas Spec 1.0](https://jsoncanvas.org/spec/1.0/)
- [JSON Canvas GitHub](https://github.com/obsidianmd/jsoncanvas)

# Code Blocks

## Fenced Code Blocks

Use triple backticks with language identifier:

````markdown
```javascript
function greet(name) {
  return `Hello, ${name}!`;
}
```
````

### Common Language Identifiers

- `javascript` or `js`
- `typescript` or `ts`
- `python` or `py`
- `bash` or `sh`
- `json`
- `yaml` or `yml`
- `markdown` or `md`
- `html`
- `css`
- `sql`
- `dockerfile`
- `go`
- `rust`
- `java`
- `c`, `cpp`, `csharp`

## Indented Code Blocks

Indent by 4 spaces or 1 tab:

```markdown
    function greet(name) {
      return `Hello, ${name}!`;
    }
```

**Note:** Fenced code blocks are preferred over indented blocks.

## Syntax Highlighting

Language identifiers enable syntax highlighting:

````markdown
```typescript
interface User {
  id: string;
  name: string;
  email: string;
}

const getUser = async (id: string): Promise<User> => {
  const response = await fetch(`/api/users/${id}`);
  return response.json();
};
```
````

## Code Block Metadata

Some renderers support additional metadata:

````markdown
```javascript {1,3-5}
// Line 1 highlighted
const x = 10;
// Lines 3-5 highlighted
const y = 20;
const z = 30;
```
````

````markdown
```javascript filename="app.js"
console.log('Hello, World!');
```
````

## Inline Code

Use single backticks for inline code:

```markdown
Use the `Array.map()` method to transform arrays.
```

## Best Practices

- **Always specify language identifiers** for proper syntax highlighting
- Use fenced code blocks over indented blocks
- Keep code examples concise and focused
- Test code examples to ensure they work
- Add comments to explain complex code
- Use inline code for function names, variables, and short snippets
- Add blank lines before and after code blocks

## Examples

### Good

````markdown
Install dependencies:

```bash
npm install express
```

Create a simple server:

```javascript
const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.send('Hello, World!');
});

app.listen(3000, () => {
  console.log('Server running on port 3000');
});
```
````

### Bad

````markdown
Install dependencies:
```
npm install express
```
Create a simple server:
```
const express = require('express');
const app = express();
app.get('/', (req, res) => {
  res.send('Hello, World!');
});
app.listen(3000, () => {
  console.log('Server running on port 3000');
});
```
````

## Shell Examples

Use `$` prefix for commands, no prefix for output:

````markdown
```bash
$ npm install
added 142 packages in 5s

$ npm test
PASS  test/example.test.js
✓ should pass (2 ms)
```
````

## Configuration Files

Use appropriate language identifiers:

````markdown
JSON configuration:

```json
{
  "name": "my-project",
  "version": "1.0.0"
}
```

YAML configuration:

```yaml
name: my-project
version: 1.0.0
```
````

## Linting Rules

- **MD031** (blanks-around-fences): Fenced code blocks should be surrounded by blank lines
- **MD038** (no-space-in-code): Spaces inside code span elements
- **MD040** (fenced-code-language): Fenced code blocks should have a language specified
- **MD046** (code-block-style): Code block style (consistent, fenced, indented)
- **MD048** (code-fence-style): Code fence style (consistent, backtick, tilde)

## Configuration Example

```json
{
  "MD031": { "list_items": false },
  "MD040": {
    "allowed_languages": [],
    "language_only": false
  },
  "MD046": { "style": "fenced" },
  "MD048": { "style": "backtick" }
}
```

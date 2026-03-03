# Writing Style Guide

## General Principles

### Be Concise

**Bad:**

```markdown
In order to be able to use this feature, you will need to first install the package and then configure it properly before you can actually start using it.
```

**Good:**

```markdown
Install and configure the package before use.
```

### Use Active Voice

**Bad:**

```markdown
The file is read by the parser and the data is extracted.
```

**Good:**

```markdown
The parser reads the file and extracts the data.
```

### Address the Reader Directly

**Bad:**

```markdown
One should configure the database before starting the application.
```

**Good:**

```markdown
Configure the database before starting the application.
```

Or:

```markdown
You should configure the database before starting the application.
```

### Use Present Tense

**Bad:**

```markdown
The function will return a promise that will resolve with the data.
```

**Good:**

```markdown
The function returns a promise that resolves with the data.
```

## Audience Considerations

### Know Your Audience

#### Beginners

- Explain concepts thoroughly
- Avoid jargon or define it
- Provide context
- Include more examples

**Example:**

```markdown
## Installation

npm (Node Package Manager) is included with Node.js. To install this package, run:

```bash
npm install package-name
```

This downloads the package and adds it to your project's dependencies.

```text
```

#### Advanced Users

- Be concise
- Use technical terms
- Focus on edge cases
- Link to detailed references

**Example:**
```markdown
## Installation

```bash
npm install package-name
```

Supports Node.js 18+ with ESM and CJS. See the [changelog](./docs-changelog.md) for upgrading from v1.x.

```text
```

### Progressive Disclosure

Start simple, add complexity gradually:

```markdown
## Basic Usage

```javascript
const result = api.fetch('/users');
```

## Advanced Usage

```javascript
const result = await api.fetch('/users', {
  timeout: 5000,
  retries: 3,
  cache: true
});
```

## Expert Usage

For fine-grained control, use the low-level API:

```javascript
const client = api.createClient();
client.configure({ /* ... */ });
const result = await client.request({ /* ... */ });
```javascript
```

```text
```

## Technical Writing

### Be Specific

**Bad:**
```markdown
Configure the settings appropriately.
```

**Good:**

```markdown
Set `timeout` to `5000` and `retries` to `3`.
```

### Use Examples

**Bad:**

```markdown
The function accepts various options for configuration.
```

**Good:**

```markdown
```javascript
fetchData({
  timeout: 5000,    // Request timeout in milliseconds
  retries: 3,       // Number of retry attempts
  cache: true       // Enable response caching
});
```

```

### Define Technical Terms

First use:
```markdown
Use JWT (JSON Web Token) for authentication. JWTs are...
```

Subsequent uses:

```markdown
The JWT contains...
```

### Avoid Ambiguity

**Bad:**

```markdown
Run the script before deployment.
```

**Good:**

```markdown
Run `npm run build` before deploying to production.
```

## Code Documentation

### Inline vs. Block Documentation

#### Inline Comments

For single lines:

```javascript
const timeout = 5000; // Connection timeout in milliseconds
```

#### Block Comments

For functions, classes, or complex logic:

```javascript
/**
 * Fetches user data from the API.
 *
 * @param {string} userId - The user's unique identifier
 * @param {Object} options - Optional configuration
 * @param {number} options.timeout - Request timeout in milliseconds
 * @returns {Promise<User>} The user object
 * @throws {NotFoundError} If user doesn't exist
 */
async function fetchUser(userId, options = {}) {
  // Implementation
}
```

### Command Examples

Show commands with expected output:

````markdown
```bash
npm test
```

```
> project@1.0.0 test
> jest

 PASS  tests/api.test.js
  ✓ should fetch users (45ms)
  ✓ should handle errors (12ms)

Tests: 2 passed, 2 total
```
````

### Configuration Examples

Show complete, working examples:

````markdown
```javascript
// config.js
module.exports = {
  database: {
    host: 'localhost',
    port: 5432,
    name: 'myapp'
  },
  server: {
    port: 3000,
    cors: true
  }
};
```
````

## Formatting Guidelines

### Headings

- Use sentence case: "Getting started" not "Getting Started"
- Be descriptive: "Configure database connection" not "Setup"
- Use parallel structure in lists of headings

### Lists

Start items with same part of speech:

**Bad:**

```markdown
- Installing dependencies
- Configuration
- To run the tests
```

**Good:**

```markdown
- Install dependencies
- Configure settings
- Run tests
```

### Emphasis

Use sparingly:

- **Bold** for important terms or actions
- *Italic* for emphasis or new terms
- `Code` for code references

**Bad:**

```markdown
**IMPORTANT:** You **MUST** run `npm install` **before** starting!
```

**Good:**

```markdown
Run `npm install` before starting.
```

## Tone

### Professional but Friendly

**Too Formal:**

```markdown
One must ensure that the appropriate configuration has been applied prior to initialization.
```

**Too Casual:**

```markdown
Just slap in some config and you're good to go!
```

**Just Right:**

```markdown
Configure the application before starting it.
```

### Avoid Unnecessary Words

**Wordy:**

```markdown
In order to be able to use this feature...
```

**Concise:**

```markdown
To use this feature...
```

**Wordy:**

```markdown
It is important to note that...
```

**Concise:**

```markdown
Note:
```

### Positive Language

**Negative:**

```markdown
Don't forget to configure the database or the application won't work.
```

**Positive:**

```markdown
Configure the database to ensure the application works correctly.
```

## Common Mistakes

### Assuming Knowledge

**Bad:**

```markdown
Use the CLI to scaffold the project.
```

**Good:**

```markdown
Use the command-line interface (CLI) to generate project files:

```bash
npx create-app my-project
```

```

### Vague Instructions

**Bad:**
```markdown
Update the configuration as needed.
```

**Good:**

```markdown
Update `config.json`:

```json
{
  "apiKey": "your-api-key-here",
  "timeout": 5000
}
```

```

### Missing Context

**Bad:**
```markdown
Run the migration:

```bash
npm run migrate
```

```

**Good:**
```markdown
Apply database migrations to create the required tables:

```bash
npm run migrate
```

This creates the `users` and `posts` tables in your database.

```

## Best Practices

1. **Write for scanability**: Use headings, lists, and short paragraphs
2. **Show, don't tell**: Use examples instead of descriptions
3. **Be consistent**: Use same terms, formatting, and structure
4. **Test your docs**: Follow your own instructions to verify they work
5. **Keep it updated**: Review and update documentation regularly
6. **Get feedback**: Have others review your documentation
7. **Use templates**: Create templates for common documentation types

## Checklist

- [ ] Concise and clear
- [ ] Active voice
- [ ] Present tense
- [ ] Addresses reader directly
- [ ] Appropriate for audience
- [ ] Specific and detailed
- [ ] Includes working examples
- [ ] Technical terms defined
- [ ] No ambiguity
- [ ] Consistent terminology
- [ ] Proper formatting
- [ ] Professional tone
- [ ] All instructions tested

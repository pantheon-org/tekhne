# README Structure

## Minimal README Template

For simple projects:

```markdown
# Project Name

Brief one-sentence description.

## Installation

```bash
npm install project-name
```

## Usage

```javascript
const project = require('project-name');
project.doSomething();
```

## License

MIT
```

## Comprehensive README Template

For complex projects:

```markdown
# Project Name

[![Build Status](badge-url)](ci-url)
[![License](badge-url)](license-url)

Brief description (1-2 sentences) explaining what the project does and why it exists.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)

## Features

- Feature 1: Brief description
- Feature 2: Brief description
- Feature 3: Brief description

## Installation

### Prerequisites

- Node.js 18+
- PostgreSQL 14+

### Quick Start

```bash
git clone https://github.com/user/project.git
cd project
npm install
cp .env.example .env
npm start
```

## Usage

### Basic Example

```javascript
const Project = require('project-name');

const instance = new Project({
  option1: 'value1',
  option2: 'value2'
});

instance.doSomething();
```

### Advanced Example

```javascript
// More complex usage example
```

## Configuration

Configuration options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `option1` | `string` | `'default'` | Description |
| `option2` | `number` | `100` | Description |

## API Reference

See [API Documentation](docs/api.md) for detailed API reference.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Credit contributors, inspirations, or related projects
```

## Key Sections

### Project Title and Badges

```markdown
# Project Name

[![Build](badge)](url)
[![Coverage](badge)](url)
[![Version](badge)](url)
[![License](badge)](url)
```

### Description

1-2 paragraphs explaining:
- What the project does
- Why it exists
- Key benefits
- Target audience

### Installation

Step-by-step setup instructions:
- Prerequisites with versions
- Installation commands
- Environment setup
- Verification steps

### Usage

- Basic examples first
- Progressive complexity
- Real-world use cases
- Common workflows

### Configuration

- All available options
- Default values
- Environment variables
- Configuration files

### API Reference

- Link to detailed docs
- Or inline for small APIs
- Examples for each endpoint/method

### Contributing

- How to contribute
- Development setup
- Testing requirements
- Code style guidelines

### License

- License type
- Link to LICENSE file

## Best Practices

### Start with Why

Explain the problem before the solution:

```markdown
# DataSync

**Problem:** Keeping multiple databases in sync is complex and error-prone.

**Solution:** DataSync provides real-time bi-directional synchronization with automatic conflict resolution.
```

### Show, Don't Tell

Use examples instead of descriptions:

**Bad:**
```markdown
This library makes API calls easier.
```

**Good:**
```markdown
```javascript
// Before
fetch('https://api.example.com/users')
  .then(res => res.json())
  .then(data => console.log(data))
  .catch(err => console.error(err));

// After
const users = await api.get('/users');
```
```

### Quick Start First

Get users running quickly:

```markdown
## Quick Start

```bash
npx create-my-app my-project
cd my-project
npm start
```

Open http://localhost:3000 to see your app.

That's it! See [detailed installation](docs/installation.md) for more options.
```

### Progressive Disclosure

Organize from simple to complex:
1. Quick start
2. Basic usage
3. Advanced features
4. API reference

## Common README Files

### Application README

Focus on:
- Setup and deployment
- Environment configuration
- Running locally
- Testing
- Production deployment

### Library/Package README

Focus on:
- Installation
- API documentation
- Code examples
- Browser/Node compatibility
- TypeScript types

### Tool/CLI README

Focus on:
- Installation
- Command reference
- Common workflows
- Configuration
- Examples

## Anti-Patterns

### Too Much, Too Soon

**Bad:**
```markdown
# Project

This comprehensive enterprise-grade solution leverages cutting-edge technologies...

[500 lines of detailed technical architecture]
```

**Good:**
```markdown
# Project

A fast, type-safe ORM for Node.js.

```bash
npm install project
```

```javascript
const users = await db.users.findMany();
```
```

### No Examples

**Bad:**
```markdown
## Usage

Install the package and import it into your project. Configure as needed.
```

**Good:**
```markdown
## Usage

```javascript
import { createClient } from 'project';

const client = createClient({
  apiKey: process.env.API_KEY
});

const data = await client.fetch('/users');
```
```

### Outdated Information

- Keep examples tested and working
- Update version numbers
- Remove deprecated features
- Update screenshots
- Test all installation steps

## README Checklist

- [ ] Clear project title
- [ ] One-sentence description
- [ ] Installation instructions
- [ ] Working code example
- [ ] Link to documentation
- [ ] License information
- [ ] Contact/support information
- [ ] All examples tested and working
- [ ] No broken links
- [ ] No outdated information

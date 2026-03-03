# Documentation Organization

## File Naming Conventions

### Use Kebab Case

```
installation-guide.md
api-reference.md
getting-started.md
troubleshooting-tips.md
```

### Be Descriptive

**Good:**
```
authentication-oauth2.md
database-migration-guide.md
deployment-aws.md
```

**Bad:**
```
auth.md
db.md
deploy.md
```

### Use Prefixes for Grouping

```
api-endpoints.md
api-authentication.md
api-rate-limiting.md

guide-installation.md
guide-configuration.md
guide-deployment.md
```

## Directory Structure

### Simple Project

```
project/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
└── CHANGELOG.md
```

### Medium Project

```
project/
├── README.md
├── docs/
│   ├── installation.md
│   ├── usage.md
│   ├── api-reference.md
│   └── troubleshooting.md
├── LICENSE
├── CONTRIBUTING.md
└── CHANGELOG.md
```

### Large Project

```
project/
├── README.md
├── docs/
│   ├── getting-started/
│   │   ├── installation.md
│   │   ├── quick-start.md
│   │   └── configuration.md
│   ├── guides/
│   │   ├── authentication.md
│   │   ├── deployment.md
│   │   └── testing.md
│   ├── api/
│   │   ├── overview.md
│   │   ├── endpoints.md
│   │   └── examples.md
│   ├── reference/
│   │   ├── cli-commands.md
│   │   ├── configuration-options.md
│   │   └── error-codes.md
│   └── contributing/
│       ├── development-setup.md
│       ├── code-style.md
│       └── pull-request-process.md
├── LICENSE
├── CONTRIBUTING.md
└── CHANGELOG.md
```

## Documentation Categories

### Getting Started

New user onboarding:
- Installation
- Quick start
- First steps
- Basic concepts

### Guides

Task-oriented documentation:
- How to authenticate
- How to deploy
- How to configure X
- How to troubleshoot Y

### API Reference

Technical specifications:
- Endpoints
- Methods
- Parameters
- Return values
- Error codes

### Explanations

Conceptual documentation:
- Architecture overview
- Design decisions
- Best practices
- Patterns and anti-patterns

### Contributing

Developer documentation:
- Development setup
- Code style
- Testing
- Pull request process

## Navigation

### Table of Contents

Add to main README or docs/README.md:

```markdown
# Documentation

## Getting Started

- [Installation](getting-started/installation.md)
- [Quick Start](getting-started/quick-start.md)
- [Configuration](getting-started/configuration.md)

## Guides

- [Authentication](guides/authentication.md)
- [Deployment](guides/deployment.md)
- [Testing](guides/testing.md)

## API Reference

- [Overview](api/overview.md)
- [Endpoints](api/endpoints.md)
- [Examples](api/examples.md)
```

### Breadcrumbs

Add to top of each page:

```markdown
[Home](../README.md) > [Guides](../guides/README.md) > Authentication

# Authentication Guide
```

### Cross-References

Link related pages:

```markdown
## Next Steps

- Learn about [deployment](deployment.md)
- See [API reference](../api/overview.md)
- Read about [best practices](best-practices.md)
```

## Linking Between Documents

### Relative Links

```markdown
[Installation Guide](installation.md)
[API Reference](api/overview.md)
[Parent README](../README.md)
```

### Anchor Links

```markdown
[Jump to Configuration](#configuration)
[See Installation Steps](#installation-steps)
```

### Cross-Document Anchors

```markdown
[See Authentication in API docs](api/overview.md#authentication)
[Configuration options](configuration.md#database-settings)
```

## Documentation Index

Create docs/README.md or docs/index.md:

```markdown
# Documentation Index

Welcome to the project documentation!

## For New Users

Start here if you're new:
1. [Installation](installation.md)
2. [Quick Start](quick-start.md)
3. [Basic Tutorial](tutorial.md)

## Documentation by Topic

### Setup and Configuration
- [Installation](installation.md)
- [Configuration](configuration.md)
- [Environment Variables](environment.md)

### Usage
- [Quick Start](quick-start.md)
- [Usage Examples](examples.md)
- [API Reference](api-reference.md)

### Deployment
- [Production Deployment](deployment.md)
- [Docker Setup](docker.md)
- [Cloud Deployment](cloud.md)

### Development
- [Development Setup](development.md)
- [Testing](testing.md)
- [Contributing](../CONTRIBUTING.md)

## Need Help?

- [Troubleshooting](troubleshooting.md)
- [FAQ](faq.md)
- [GitHub Issues](https://github.com/user/project/issues)
```

## Best Practices

### Organize by User Journey

Match documentation structure to how users will discover features:
1. Getting started (first-time users)
2. Guides (users accomplishing specific tasks)
3. Reference (users looking up details)
4. Advanced topics (experienced users)

### Use Consistent Naming

Pick a naming convention and stick to it:
- Kebab case for filenames
- Title case for headings
- Descriptive names over short names

### Keep Hierarchy Shallow

Maximum 2-3 levels:
```
docs/
├── getting-started/     # Level 1
│   └── installation.md  # Level 2
└── guides/              # Level 1
    └── deployment.md    # Level 2
```

Avoid:
```
docs/
├── advanced/
│   └── topics/
│       └── performance/
│           └── optimization/
│               └── database.md  # Too deep!
```

### Provide Multiple Entry Points

- README for overview
- docs/README.md for full index
- Category indexes (guides/README.md)
- Cross-references between related topics

### Keep Navigation Visible

- Table of contents in long documents
- Breadcrumbs for context
- "Next steps" at document end
- Clear links back to main index

## Anti-Patterns

### Flat Structure with Many Files

**Bad:**
```
docs/
├── api-auth.md
├── api-endpoints.md
├── api-errors.md
├── guide-deploy-aws.md
├── guide-deploy-docker.md
├── guide-install-linux.md
├── guide-install-mac.md
├── guide-install-windows.md
└── ... (50 more files)
```

**Good:**
```
docs/
├── api/
│   ├── authentication.md
│   ├── endpoints.md
│   └── errors.md
├── deployment/
│   ├── aws.md
│   └── docker.md
└── installation/
    ├── linux.md
    ├── mac.md
    └── windows.md
```

### Orphaned Documents

Files with no incoming links:
- Add to table of contents
- Link from related documents
- Or remove if not needed

### Duplicate Information

Maintain single source of truth:
- Link instead of duplicating
- Use includes/fragments if supported
- Keep one authoritative document per topic

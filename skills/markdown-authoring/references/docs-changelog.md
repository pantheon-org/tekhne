# Changelog Documentation

## Keep a Changelog Format

Follow the [Keep a Changelog](https://keepachangelog.com/) specification:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New features that haven't been released yet

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Features that have been removed

### Fixed
- Bug fixes

### Security
- Vulnerability patches

## [1.0.0] - 2024-01-15

### Added
- Initial release
- User authentication
- Dashboard view

[Unreleased]: https://github.com/user/repo/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/user/repo/releases/tag/v1.0.0
```

## Changelog Sections

### Section Order (Standard)
1. **Added** - New features
2. **Changed** - Changes in existing functionality
3. **Deprecated** - Soon-to-be removed features
4. **Removed** - Removed features
5. **Fixed** - Bug fixes
6. **Security** - Security updates

### Writing Good Entries

```markdown
## [1.2.0] - 2024-03-10

### Added
- User profile page with avatar upload (#234)
- Dark mode support for all components (#245)
- Export data to CSV functionality (#256)

### Changed
- Improved search performance by 40% (#267)
- Updated dependency `library-name` from v2.0 to v3.0 (#278)

### Fixed
- Fixed crash when deleting items with special characters (#289)
- Corrected timezone handling in date picker (#290)
```

## Best Practices

1. **Write for Users** - Focus on user-facing changes, not implementation details
2. **Be Specific** - "Added dark mode" not "Added new feature"
3. **Include Issue/PR Numbers** - Link to GitHub issues/PRs
4. **Group Similar Changes** - Keep related changes together
5. **Use Present Tense** - "Add feature" not "Added feature"
6. **Keep Unreleased Section** - Always maintain an [Unreleased] section
7. **Date Format** - Use ISO 8601 format (YYYY-MM-DD)
8. **Version Links** - Include comparison links at the bottom

## Common Mistakes

- Don't include every commit - summarize user-facing changes
- Don't use vague descriptions like "Various fixes"
- Don't forget to move Unreleased items when cutting a release
- Don't skip version numbers in the comparison links
- Don't mix internal and user-facing changes

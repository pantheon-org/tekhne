# Links and Images

## Links

### Inline Links

```markdown
[Link text](https://example.com)
[Link with title](https://example.com "Optional title")
```

### Reference Links

```markdown
[Link text][reference]
[Another link][ref2]

[reference]: https://example.com
[ref2]: https://example.com/page "Optional title"
```

### Automatic Links

```markdown
<https://example.com>
<user@example.com>
```

### Internal Links (Anchors)

```markdown
[Go to Installation](#installation)
[Go to API Reference](#api-reference)
```

Heading anchors are auto-generated:
- Convert to lowercase
- Replace spaces with hyphens
- Remove special characters

```markdown
## My Section Title
<!-- Anchor: #my-section-title -->

## API Reference
<!-- Anchor: #api-reference -->
```

## Images

### Inline Images

```markdown
![Alt text](image.png)
![Alt text](image.png "Optional title")
```

### Reference Images

```markdown
![Alt text][logo]

[logo]: logo.png "Company logo"
```

### Linked Images

```markdown
[![Alt text](image.png)](https://example.com)
```

### Image Size (HTML fallback)

```markdown
<img src="image.png" alt="Alt text" width="200" height="100">
```

## Best Practices

### Links

- Use descriptive link text (avoid "click here" or "here")
- Use reference links for repeated URLs
- Use automatic links for bare URLs
- Verify all links work (use link checkers)
- Use relative paths for internal documentation links
- Add titles for additional context

### Images

- Always provide meaningful alt text
- Use descriptive filenames
- Optimize image sizes for web
- Use relative paths for local images
- Consider accessibility (alt text describes image content)
- Use linked images for clickable graphics

## Examples

### Good Links

```markdown
Check the [installation guide](docs/installation.md) for setup instructions.

For more details, see the [official documentation](https://docs.example.com).

Contact us at <support@example.com>.
```

### Bad Links

```markdown
Click [here](docs/installation.md) for installation.

For more details, click [here](https://docs.example.com).

Contact us at support@example.com.
```

### Good Images

```markdown
![Screenshot of the dashboard showing user analytics](screenshots/dashboard.png)

![Company logo](images/logo.png "Acme Corporation")

[![Build Status](https://img.shields.io/badge/build-passing-green)](https://ci.example.com)
```

### Bad Images

```markdown
![Image](screenshot.png)

![](logo.png)
```

## Linting Rules

- **MD011** (no-reversed-links): Reversed link syntax
- **MD034** (no-bare-urls): Bare URL used
- **MD039** (no-space-in-links): Spaces inside link text
- **MD042** (no-empty-links): No empty links
- **MD045** (no-alt-text): Images should have alternate text (alt text)
- **MD052** (reference-links-images): Reference links and images should use a label that is defined

## Configuration Example

```json
{
  "MD034": true,
  "MD042": true,
  "MD045": true
}
```

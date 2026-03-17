# Starlight CSS Variables Reference

Key `--sl-*` custom properties available for theming. Full list: [props.css on GitHub](https://github.com/withastro/starlight/blob/main/packages/starlight/style/props.css).

## Color Variables

### Accent Colors (links, active nav, buttons)

| Variable | Default usage |
|----------|---------------|
| `--sl-color-accent-low` | Subtle accent backgrounds |
| `--sl-color-accent` | Primary accent (links, highlights) |
| `--sl-color-accent-high` | High-contrast accent (text on dark bg) |

### Background Colors

| Variable | Default usage |
|----------|---------------|
| `--sl-color-bg` | Page background |
| `--sl-color-bg-nav` | Header/nav background |
| `--sl-color-bg-sidebar` | Sidebar background |
| `--sl-color-bg-inline-code` | Inline code background |

### Text Colors

| Variable | Default usage |
|----------|---------------|
| `--sl-color-text` | Default text |
| `--sl-color-text-accent` | Accent text |
| `--sl-color-gray-1` through `--sl-color-gray-7` | Gray scale |
| `--sl-color-white` | Pure white |
| `--sl-color-black` | Pure black |

## Typography Variables

| Variable | Default |
|----------|---------|
| `--sl-font` | System sans-serif |
| `--sl-font-mono` | System monospace |
| `--sl-text-xs` | 0.8125rem |
| `--sl-text-sm` | 0.875rem |
| `--sl-text-base` | 1rem |
| `--sl-text-lg` | 1.125rem |
| `--sl-text-xl` | 1.25rem |
| `--sl-text-2xl` | 1.5rem |
| `--sl-text-3xl` | 1.875rem |
| `--sl-text-4xl` | 2.25rem |
| `--sl-text-5xl` | 3rem |

## Layout Variables

| Variable | Default |
|----------|---------|
| `--sl-content-width` | 45rem |
| `--sl-sidebar-width` | 18.75rem |
| `--sl-toc-width` | 14rem |

## Dark Mode Selectors

| Selector | When active |
|----------|-------------|
| `:root[data-theme='light']` | Light mode (default) |
| `:root[data-theme='dark']` | Dark mode (user toggled or system preference) |

**Never use `.dark { }` selectors** — Starlight does not apply that class.

## Tailwind Theme Mapping

When using `@astrojs/starlight-tailwind`, these Tailwind tokens map to Starlight:

| Tailwind token | Starlight variable |
|----------------|--------------------|
| `--color-accent-*` | `--sl-color-accent-*` |
| `--color-gray-*` | `--sl-color-gray-*` |
| `--font-sans` | `--sl-font` |
| `--font-mono` | `--sl-font-mono` |

## Custom Fonts

**Fontsource (recommended):**

```bash
npm install @fontsource/inter
```

```js
customCss: ['@fontsource/inter/400.css', '@fontsource/inter/600.css'],
```

```css
:root { --sl-font: 'Inter', sans-serif; }
```

**Local files:** Use `@font-face` with `font-display: swap` and register via `customCss`.

## Color Theme Editor

Use the [Starlight color theme editor](https://starlight.astro.build/guides/css-and-tailwind/#theming) to generate accent/gray palettes without manual color math.

# Task — Generate design reference for a Tailwind v4 project

You are working in a React/Vite project that uses Tailwind v4 with design tokens defined in a CSS `@theme` block.

## Codebase state

`src/app.css`:
```css
@theme {
  --color-brand-500: #7c3aed;
  --color-brand-600: #6d28d9;
  --color-neutral-50: #fafafa;
  --color-neutral-900: #0a0a0a;
  --font-sans: "Inter", sans-serif;
  --font-heading: "Cal Sans", sans-serif;
  --spacing-section: 5rem;
  --radius-card: 0.75rem;
  --shadow-card: 0 4px 24px rgb(0 0 0 / 0.08);
}
```

`src/components/Button.tsx` — uses `className="bg-brand-500 text-white rounded-md px-4 py-2"` variants

`src/pages/Home.tsx` — hero section, card grid, CTA button

No dark mode detected. No `aria-` attributes found in components.

## Your task

Create a design reference for this project.

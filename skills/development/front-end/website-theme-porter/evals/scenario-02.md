# Scenario 02: Apply Brand Theme to React Project

## User Prompt

A design team has completed the token documentation phase for porting a client brand. Their theme docs are ready. You need to translate those documented tokens into the project's CSS file.

The project is a React + Tailwind v4 app. The entry CSS file is `src/index.css` — write it from scratch based on the documented tokens below.

Also update `index.html` to load the required web font.

Then update the existing `src/components/Button.tsx` component to use semantic Tailwind utility classes instead of the hardcoded colours currently in use.

**Documented tokens (from colours.md):**

| Token | Hex | HSL | Role |
|-------|-----|-----|------|
| --background | #FAFAFA | 0 0% 98% | Page background |
| --foreground | #1A1A2E | 240 29% 14% | Body text |
| --primary | #E94560 | 349 80% 56% | CTA buttons, main links |
| --primary-foreground | #FFFFFF | 0 0% 100% | Text on primary buttons |
| --muted | #F0F0F5 | 240 20% 95% | Card backgrounds, input bg |
| --muted-foreground | #6B6B8A | 240 14% 48% | Captions, placeholders |
| --border | #E2E2EE | 240 25% 90% | Input borders, dividers |

**Typography:** Primary font is `Nunito`, weights 400/600/700.

**Border radius:** Buttons and cards use 8px.

**Input files:**

`src/components/Button.tsx`

```tsx
import React from 'react';

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
}

export function Button({ children, onClick }: ButtonProps) {
  return (
    <button
      onClick={onClick}
      className="bg-[#E94560] text-[#FFFFFF] px-4 py-2 rounded-lg font-semibold hover:bg-[#c73652]"
    >
      {children}
    </button>
  );
}
```

## Expected Behavior

1. In the `@theme inline` block of `src/index.css`, reference colour tokens via `var()` — do NOT wrap them in `hsl()` inside `@theme inline`
2. In the `:root` block, define colour tokens wrapped in `hsl()` (e.g., `--primary: hsl(349 80% 56%)`)
3. Start `src/index.css` with `@import "tailwindcss"`
4. Define a `--radius` variable in `:root` (set to `0.5rem` or `8px` equivalent)
5. Include an `@layer base` block that sets `body` background-color, color, and font-family using CSS variables
6. Add a `<link>` tag in `index.html` loading the Nunito font from Google Fonts with `display=swap`
7. Include `<link rel="preconnect" href="https://fonts.googleapis.com">` in `index.html`
8. Remove all Tailwind arbitrary value syntax (e.g., `bg-[#E94560]`, `text-[#FFFFFF]`) from `Button.tsx`
9. Use semantic Tailwind classes derived from token names (e.g., `bg-primary`, `text-primary-foreground`) in `Button.tsx`
10. Map all semantic tokens from `colours.md` in the `@theme inline` block

## Success Criteria

- **No hsl() in @theme inline**: Inside the `@theme inline` block, colour tokens are NOT wrapped in `hsl()` — they use `var()` references to `:root` variables
- **hsl() used in :root**: Inside the `:root` block, colour values ARE wrapped in `hsl()`
- **Tailwind import present**: `src/index.css` starts with `@import "tailwindcss"`
- **radius token defined**: `src/index.css` defines a `--radius` variable in `:root`
- **@layer base body styles**: `src/index.css` contains an `@layer base` block setting body background-color, color, and font-family using CSS variables
- **Font loaded in index.html**: `index.html` includes a `<link>` tag loading Nunito from Google Fonts with `display=swap`
- **Font preconnect**: `index.html` includes `<link rel="preconnect" href="https://fonts.googleapis.com">`
- **No arbitrary values in Button**: `Button.tsx` does NOT contain any Tailwind arbitrary value syntax (e.g., `bg-[#E94560]`, `text-[#FFFFFF]`)
- **Semantic tokens in Button**: `Button.tsx` uses semantic Tailwind classes derived from token names
- **All tokens mapped in @theme inline**: All 7 semantic tokens from `colours.md` are mapped in the `@theme inline` block

## Failure Conditions

- Agent wraps colour values in `hsl()` inside the `@theme inline` block instead of using `var()` references
- Agent does not wrap colour values in `hsl()` inside the `:root` block
- Agent does not start `src/index.css` with `@import "tailwindcss"`
- Agent leaves arbitrary Tailwind value syntax in `Button.tsx` (e.g., `bg-[#E94560]`)
- Agent uses hardcoded hex values instead of semantic token-based class names in `Button.tsx`
- Agent does not load the Nunito font in `index.html`
- Agent omits one or more of the 7 semantic tokens from the `@theme inline` block

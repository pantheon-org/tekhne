# Apply Brand Theme to React Project

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

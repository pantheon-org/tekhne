# P04T03 — wire-theme-toggle

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Verify that `ThemeToggle.astro` correctly persists the selected theme across page loads, that the sun/moon icon updates on click, and that the `data-theme` attribute on `<html>` changes synchronously without a flash — confirming the FOUT-prevention inline script in `BaseLayout.astro` uses the same `tk-theme` localStorage key.

## File to create / modify

```
docs/src/components/ThemeToggle.astro   (fixes as needed)
docs/src/layouts/BaseLayout.astro       (inline script sync check)
```

## Implementation

Verify and fix the following three points:

1. **localStorage key consistency** — both `ThemeToggle.astro` and the `BaseLayout.astro` inline script must use `"tk-theme"` (not `"starlight-theme"` or any other key).

```sh
grep -n "localStorage" docs/src/components/ThemeToggle.astro
grep -n "localStorage" docs/src/layouts/BaseLayout.astro
# Both should show "tk-theme"
```

2. **Icon update** — after toggle, the button label/icon must reflect the new state. Add `aria-label` update logic if missing:

```js
btn.setAttribute("aria-label", newTheme === "dark" ? "Switch to light mode" : "Switch to dark mode");
```

3. **FOUT prevention** — the inline `<script>` in `BaseLayout.astro` must run synchronously (not `async`/`defer`) to apply `data-theme` before paint:

```html
<script>
  const t = localStorage.getItem("tk-theme");
  if (t) document.documentElement.setAttribute("data-theme", t);
</script>
```

## Notes

- Using `"starlight-theme"` as the key means users who already have a Starlight theme stored will not auto-migrate. If backward compatibility is desired, read both keys and prefer `"tk-theme"`.
- The toggle script must also dispatch a `storage` event or use a custom event if the header and page body need to react to theme changes simultaneously.

## Verification

```sh
cd docs
grep -c "tk-theme" src/components/ThemeToggle.astro
grep -c "tk-theme" src/layouts/BaseLayout.astro
# Both must output at least 1
bunx astro check 2>&1 | grep -E "(error|Error)" | head -5
```

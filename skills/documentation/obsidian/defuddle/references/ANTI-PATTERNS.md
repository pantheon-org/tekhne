# Defuddle Anti-Patterns

Extended explanations for the Common Mistakes in SKILL.md.

## 1. Using WebFetch when defuddle is available

**WHY this matters:** Raw HTML pages commonly contain 5-10x more tokens than the extracted article content. Navigation menus, sidebars, cookie banners, footers, and inline ads all contribute to the token count. Defuddle strips these before the content reaches the context window, directly reducing cost and improving the signal-to-noise ratio of the response.

## 2. Omitting the `--md` flag

**WHY this matters:** HTML output retains tags (`<p>`, `<div>`, `<span>`), attributes, inline styles, and entity references. The model must parse this mentally, consuming context window space that could be used for reasoning. Markdown output is compact, readable, and structurally clear — it is almost always the right format for downstream text processing.

## 3. Using `-p` metadata extraction when full content is needed

**WHY this matters:** The `-p` flag is designed for lightweight property extraction (e.g., getting just the title or description for a preview). When it is used by mistake for full-content retrieval, the body of the article is not included in the output at all. The model will respond with only the title or description string while the user expects the full article — a silent, confusing failure.

## 4. Using shell redirection instead of the `-o` flag

**WHY this matters:** Shell redirection (`>`) routes stdout directly to the file system and bypasses any encoding, buffering, or file-writing logic that the defuddle process uses internally. On non-UTF-8 terminals or systems with non-default locales, this can introduce encoding mismatches, corrupt multi-byte characters, or strip byte-order marks. The `-o` flag delegates file writing to defuddle itself, which handles encoding correctly across platforms.

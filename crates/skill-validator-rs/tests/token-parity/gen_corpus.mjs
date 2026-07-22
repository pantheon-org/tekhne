#!/usr/bin/env node
// S1 token-parity gate: corpus generator.
//
// Emits `corpus.jsonl` where each line is a JSON-encoded string (one corpus
// item). Encoding every item as a JSON string preserves newlines, CRLF,
// combining marks and multi-codepoint emoji byte-for-byte, so the Go
// reference tokenizer and the Rust harness read *identical* bytes.
//
// Run from anywhere:  node gen_corpus.mjs
// Output is deterministic; re-running reproduces the same file.

import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
// tests/token-parity -> crates/skill-validator-rs -> crates -> repo root
const repoRoot = join(here, "..", "..", "..", "..");

// Each entry: [description, text]. Descriptions are index-aligned metadata
// only; they are NOT part of the tokenized bytes.
const synthetic = [
  ["empty string", ""],
  ["single char", "a"],
  ["single space", " "],
  ["single newline", "\n"],
  ["single tab", "\t"],
  ["plain prose short", "The quick brown fox jumps over the lazy dog."],
  [
    "plain prose paragraph",
    "Responsible play is central to how the group operates across every market. Participants can set limits, request cool-off periods, and reach support at any hour. The intent is to keep play safe without diminishing the enjoyment of taking part.",
  ],
  ["contraction don't", "don't"],
  ["possessive skill's", "skill's"],
  ["stacked contractions", "I can't and I won't; she shouldn't've y'all'd've"],
  [
    "markdown headings",
    "# Heading 1\n## Heading 2\n### Heading 3\n#### Heading 4",
  ],
  [
    "markdown unordered list",
    "- first item\n- second item\n  - nested item\n  - another nested\n- third item",
  ],
  [
    "markdown ordered list",
    "1. one\n2. two\n3. three\n   1. three-a\n   2. three-b",
  ],
  [
    "markdown table",
    "| Name | Score | Grade |\n| ---- | ----- | ----- |\n| alpha | 126 | A |\n| beta | 118 | B |\n| gamma | 99 | C |",
  ],
  [
    "fenced code python",
    '```python\ndef greet(name: str) -> str:\n    return f"hello, {name}!"\n\nprint(greet("world"))\n```',
  ],
  [
    "fenced code rust",
    '```rust\nfn main() {\n    let xs = vec![1, 2, 3];\n    let sum: i32 = xs.iter().sum();\n    println!("sum = {sum}");\n}\n```',
  ],
  [
    "fenced code bash",
    '```bash\n#!/usr/bin/env bash\nset -euo pipefail\nfor f in *.md; do\n  echo "processing ${f}"\ndone\n```',
  ],
  [
    "fenced code json",
    '```json\n{\n  "name": "skill-validator",\n  "version": "1.5.6",\n  "thresholds": { "min": 112, "target": 126 }\n}\n```',
  ],
  [
    "fenced code go",
    '```go\npackage main\n\nimport "fmt"\n\nfunc main() {\n\tfmt.Println("hello")\n}\n```',
  ],
  [
    "inline code",
    "Call the `validate()` function then check `result.ok == true`.",
  ],
  ["simple emoji", "😀"],
  ["family emoji (ZWJ multi-codepoint)", "👨‍👩‍👧"],
  ["emoji with skin tone modifier", "👍🏽"],
  ["flag emoji sequence", "🇳🇱🇬🇧🇩🇪🇸🇪"],
  ["mixed emoji and text", "Deploy 🚀 to prod ✅ now, then celebrate 🎉!"],
  ["CJK simplified chinese", "你好世界，这是一个用于测试的中文句子。"],
  ["japanese hiragana and kanji", "こんにちは世界、これはテストです。"],
  ["korean hangul", "안녕하세요 세계, 이것은 테스트입니다."],
  ["accented latin", "Café résumé naïve Zürich Åland smörgåsbord jalapeño"],
  ["combining marks (NFD decomposed)", "é à ô ñ ü"],
  ["arabic RTL", "مرحبا بالعالم، هذا اختبار."],
  ["math symbols", "∑ ∫ √ ≠ ≤ ≥ ∞ π ± × ÷ → ⇒ ∀ ∃"],
  ["numbers", "1234567890 3.14159265358979 42 007 1,000,000"],
  [
    "mixed alphanumeric",
    "abc123def456 v1.2.3-rc.1 SHA256:deadbeefcafe id_00042",
  ],
  ["heavy inline whitespace", "word    word\t\ttab   spaces      end"],
  ["blank line runs", "alpha\n\n\n\n\nbeta\n\n\ngamma"],
  ["leading spaces", "          leading spaces here"],
  ["trailing spaces", "trailing spaces here          "],
  ["leading and trailing newlines", "\n\n\n  padded content  \n\n\n"],
  ["tabs indentation", "\tlevel one\n\t\tlevel two\n\t\t\tlevel three"],
  ["windows CRLF", "line one\r\nline two\r\nline three\r\n"],
  [
    "url",
    "See https://example.com/path/to/page?query=1&other=2#section for details.",
  ],
  [
    "email placeholder",
    "Contact participant_001@example.test or team@example.test.",
  ],
  ["special token literal endoftext", "<|endoftext|>"],
  [
    "multiple special token literals",
    "<|endoftext|><|im_start|>system<|im_sep|>hello<|im_end|>",
  ],
  ["repeated single char", "a".repeat(64)],
  ["repeated word", "spam ".repeat(40).trimEnd()],
  [
    "markdown blockquote and link",
    "> This is a quoted line.\n> Second quoted line.\n\nSee [the docs](https://example.com/docs) for more.",
  ],
  [
    "yaml frontmatter block",
    "---\nname: example-skill\ndescription: A short example.\ntags:\n  - alpha\n  - beta\n---\n",
  ],
  [
    "very long paragraph",
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ".repeat(
      12,
    ),
  ],
  [
    "mixed script sentence",
    "The lottery draw 抽選 happens on Fridays 🎰 at 20:00 CET, insha'Allah.",
  ],
  ["nul-adjacent control-ish whitespace", "a\r\n\tb"],
];

// Real SKILL.md files copied verbatim into the corpus (varied sizes).
const skillFiles = [
  "skills/agentic-harness/vault-fetch/SKILL.md",
  "skills/development/biome-complete/SKILL.md",
  "skills/documentation/markdown-authoring/SKILL.md",
  "skills/development/bun-development/SKILL.md",
  "skills/documentation/mermaid-diagrams/SKILL.md",
  "skills/software-engineering/fp-immutability/SKILL.md",
  "skills/agentic-harness/pick-model/SKILL.md",
  "skills/documentation/humanizer/SKILL.md",
];

const items = [];
for (const [desc, text] of synthetic) {
  items.push({ desc: `synthetic: ${desc}`, text });
}
for (const rel of skillFiles) {
  const text = readFileSync(join(repoRoot, rel), "utf8");
  items.push({ desc: `skill-file: ${rel}`, text });
}

// corpus.jsonl : one JSON-encoded string per line (the tokenized bytes).
const corpusLines = items.map((it) => JSON.stringify(it.text));
writeFileSync(join(here, "corpus.jsonl"), corpusLines.join("\n") + "\n");

// descriptions.json : index-aligned metadata (NOT tokenized).
writeFileSync(
  join(here, "descriptions.json"),
  JSON.stringify(
    items.map((it) => it.desc),
    null,
    2,
  ) + "\n",
);

console.log(
  `wrote ${items.length} corpus items (${synthetic.length} synthetic + ${skillFiles.length} skill files)`,
);

---
name: create-context
description: Create the baseline session context from the in-folder of raw project documents, classifying each file by priority, writing a manifest, summarising oversized files and producing a token-budgeted baseline. Use when bootstrapping project context or setting up the ctx snapshot once per project. Triggers include "create context", "bootstrap context", "setup context", "init context".
argument-hint: "[--force to overwrite an existing ctx snapshot]"
allowed-tools: Read, Write, Glob, Bash, AskUserQuestion, Agent
user-invocable: true
---

# Create Context

Turn the raw documents in the in-folder into a prioritised, token-budgeted snapshot that later sessions load in one read.

## Layout

```text
.context/session/in/                      immutable input: users drop documents here, no command edits it
.context/session/ctx/                     generated snapshot: manifest, copies, summaries
.context/session/CONTEXT-baseline-llm.md  single load point, 2000 tokens at most
```

## Mindset

- Use the in-folder as the source of truth. The manifest mirrors it faithfully.
- Keep the snapshot scoped to the session's open questions. Include only what shapes decisions.
- Keep one manifest per session. Two manifests in one folder leave load-context unable to choose.
- State the origin of every constraint, assumption and decision that enters the baseline.

## When to Use

- Use it once per project when raw documents sit in the in-folder and no ctx snapshot exists.
- Use it with `--force` after the source documents changed materially.
- Use it when load-context reports a missing manifest.

## When Not to Use

- Do not use it when the ctx snapshot is current. A rerun without `--force` stops, and `--force` discards existing summaries.
- Do not use it when the in-folder is empty. There is nothing to scan.
- Do not use it to refresh one document. Update that summary and its manifest entry by hand.
- Use save-context instead to store live session state in a named stream.

## Steps

1. Guard the prerequisites. Stop on a missing in-folder, and stop on an existing ctx folder unless `--force` was passed.

```bash
[ -d .context/session/in ] || { echo "No in-folder found. Add source files first." >&2; exit 1; }
[ -d .context/session/ctx ] && [ "$ARGUMENTS" != "--force" ] && { echo "ctx exists. Use --force." >&2; exit 1; }
```

2. Scan the in-folder with the bundled script. It lists md, txt, csv, yaml, yml and json files with a token estimate, and skips sensitive paths.

```bash
./scripts/scan-in-folder.sh .context/session/in
```

3. Classify every file as HIGH, MEDIUM or LOW with a one-line description. Batch several files into each AskUserQuestion call, and check for blank answers after every call (see the guard below).
4. Size each file with `wc -w`, where tokens are words divided by 0.75, then apply the thresholds.

| Priority | At or under threshold | Over threshold, up to 25K | Over 25K |
|----------|----------------------|---------------------------|----------|
| HIGH (1500) | inline | summarise directly | summarise via sub-agent |
| MEDIUM (2500) | inline | summarise directly | summarise via sub-agent |
| LOW | reference only, no copy | not applicable | not applicable |

5. Copy HIGH and MEDIUM files into the ctx folder and keep their subdirectories. Write a summary of about 500 tokens for each oversized file, and delegate files above 25K tokens to the `summarize-for-context` sub-agent.
6. Write `manifest.yaml` in the ctx folder after sizing, because each `action` value depends on it. Then validate it.

```bash
./scripts/validate-manifest.sh .context/session/ctx/manifest.yaml
```

7. Write the baseline within 2000 tokens: inline content, summary pointers and LOW references. Report the file counts and the RISEN INPUT table, then suggest `/save-context baseline`.

## AskUserQuestion Guard

Outside Plan Mode the tool can return blank answers without showing its UI. After every call, check the answers. If they are blank, do not assume a priority. Print the options as a numbered list, ask for the number, and wait for the reply.

## Anti-Patterns

- NEVER build the manifest from memory. WHY: the in-folder defines the scope, so a manifest without a scan misleads every downstream agent.

```yaml
# BAD: fabricated, no scan, wrong shape
high_priority: []

# GOOD: sources.high/medium/low from a real scan
sources:
  high:
    - {file: design-doc.md, desc: "Architecture decisions", action: inline}
  medium: []
  low: []
```

- NEVER omit `high`, `medium` or `low`, even when empty. WHY: `validate-manifest.sh` fails and consumers cannot iterate predictably.
- NEVER copy files matching `.env*`, `*credentials*`, `*secrets*`, `*token*`, `*.key`, `*.pem`, `*.crt`, `*.p12` or `*.pfx`. WHY: the ctx folder gets committed and shared, so a copied secret leaks.

```bash
# BAD: copies everything, secrets included
cp -r .context/session/in/* .context/session/ctx/

# GOOD: select with the scanner, which also skips secrets inside a secrets/ directory
./scripts/scan-in-folder.sh .context/session/in
```

- NEVER inline a file above its threshold. WHY: one oversized inline breaks the 2000-token baseline budget on every later load.
- NEVER run the skill concurrently with save-context or load-context. WHY: nothing locks the ctx folder, so the last writer wins and leaves a partial manifest.
- Pitfall: a filename pattern alone misses `secrets/prod.yaml`. Check directory names too, as the scanner does.

## References

- [Reference](references/reference.md) - manifest schema, sizing rules, baseline template, validation rules and error messages
- [Worked examples](references/examples.md) - bootstrap, forced rebuild and large-file runs with expected output

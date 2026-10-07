# create-context worked examples

Paths are given relative to the session folder: `in/` is the in-folder, `ctx/` the generated snapshot.

## Bootstrap at session start

Input: `in/design-doc.md` (900 words), `in/requirements.csv` (2400 words), `in/old-notes.txt`.

1. Run the scanner. It reports three files with estimates of 1200, 3200 and a small figure for the notes.
2. The user classifies `design-doc.md` as HIGH, `requirements.csv` as MEDIUM and `old-notes.txt` as LOW.
3. Apply the sizing table.
   - `design-doc.md` is 1200 tokens, under the HIGH threshold of 1500, so copy it and inline it.
   - `requirements.csv` is 3200 tokens, over the MEDIUM threshold of 2500, so copy it and write `ctx/01-requirements-summary-llm.md`.
   - `old-notes.txt` is LOW, so list it by path and copy nothing.
4. Write `ctx/manifest.yaml` with `action: inline` for the design document and `action: summarized` for the CSV, then run the validator.

Expected output: a RISEN INPUT table with three rows (HIGH, MED, LOW) and the suggestion `/save-context baseline`.

## Forced rebuild after a source change

A requirements file was replaced and the ctx folder already exists.

- A plain run stops with "ctx exists. Use --force." and changes nothing.
- A `--force` run rescans, asks for priorities again, and rewrites the manifest, the copies, the summaries and the baseline.
- Hand-edited summaries are discarded. Copy any summary worth keeping elsewhere before forcing.

## One file above 25K tokens

A 40K-token specification is classified HIGH.

- Direct summarising would need the whole file in the main context, so delegate to the `summarize-for-context` sub-agent, which reads it in chunks.
- The summary lands in `ctx/{nn}-{basename}-summary-llm.md` at about 500 tokens, and the manifest records `action: summarized`.

## Blank AskUserQuestion answers

The classification call returns without any answer and without showing its UI.

- Do not default the files to MEDIUM. Print the files with options `1 HIGH`, `2 MEDIUM`, `3 LOW`, and wait for the user's reply.
- Resume at step 4 only once every file has a priority the user chose.

## Secrets in the in-folder

`in/secrets/prod.yaml`, `in/prod-credentials.json` and `in/api.key` sit beside ordinary documents.

- The scanner prints `SKIPPED (security)` for each on stderr and leaves them out of its JSON.
- The first one is skipped because of its directory name, not its filename, which is why the scanner checks every path component.
- None of the three appears in the manifest or in the ctx folder. Tell the user which files were skipped.

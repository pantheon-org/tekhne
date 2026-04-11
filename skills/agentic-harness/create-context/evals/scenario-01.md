# Scenario 01: Bootstrap Context from Empty .context/session/in/

## User Prompt

"Create context from my .context/session/in/ folder."

## Expected Behavior

1. Agent verifies `.context/session/in/` exists
2. Agent runs `scripts/scan-in-folder.sh` (or equivalent Glob) to discover files
3. Agent asks user to assign HIGH / MEDIUM / LOW priority and a brief description for each discovered file
4. Agent creates `.context/session/ctx/` directory
5. Agent writes `.context/session/ctx/manifest.yaml` with project name, timestamp, source_folder, and all prioritized sources
6. Agent copies HIGH and MEDIUM files to `.context/session/ctx/`, applying inline or summarize logic based on token count
7. Agent writes `.context/session/CONTEXT-baseline-llm.md` (≤2000 tokens)
8. Agent outputs a RISEN INPUT table listing files by priority
9. Agent suggests running `/save-context baseline`

## Success Criteria

- **Guard check**: Agent verifies `.context/session/in/` exists before proceeding
- **File discovery**: All supported extensions (`.md`, `.txt`, `.csv`, `.yaml`, `.yml`, `.json`) are found and listed
- **User classification**: Agent asks for HIGH/MEDIUM/LOW per file; does not assume priority
- **Manifest written**: `.context/session/ctx/manifest.yaml` is created with valid schema (project, created, source_folder, all three priority sections)
- **Baseline written**: `.context/session/CONTEXT-baseline-llm.md` created, ≤2000 tokens
- **RISEN table**: Output includes table with Priority, File, Description columns

## Failure Conditions

- Agent proceeds without checking whether `.context/session/in/` exists
- Agent assigns priorities without asking the user
- Manifest is missing required fields (project, created, source_folder)
- Baseline exceeds 2000 tokens
- Agent skips the RISEN INPUT table in output
- Agent does not suggest `/save-context baseline` at the end

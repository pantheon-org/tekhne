# Scenario 7: Known Pattern in the Learnings File

## User Prompt

My Jest run prints 'Cannot find module ./config' only in CI, never locally. Our repo has a learnings.yaml in the project root with an entry under mental_models: pattern 'module not found only in CI', insight 'case-sensitive file system on Linux runners; file is config.ts but the import says Config'.

Repo state: learnings.yaml exists at the project root; the import in src/app.ts reads `import cfg from './Config'`.

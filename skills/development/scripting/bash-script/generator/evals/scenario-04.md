# Scenario 04: Dynamic Report Renderer

## User Prompt

A data team has a library of report templates stored as plain-text files. Each template contains placeholders in the form `{{KEY}}` that need to be substituted with values read from a separate key-value configuration file at render time. A junior developer previously wrote a renderer that used shell evaluation to perform substitutions, but it was discovered that a malicious config value could execute arbitrary commands. The security team has mandated a rewrite.

The new renderer must handle multi-word values (e.g. a value like "Q1 2026 Revenue Report" containing spaces) without breaking the substitution. It must also support a batch mode where a directory of template files is processed at once, writing each rendered result to a parallel output directory.

Produce a file named `render_report.sh`. The script should:

- Accept a template file (or directory) and a key-value config file as arguments.
- Substitute `{{KEY}}` placeholders with values from the config without ever using `eval`.
- Write rendered output either to stdout (single file) or to a specified output directory (batch mode).

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/config.cfg ===============
TITLE=Q1 2026 Revenue Report
AUTHOR=Finance Team
PERIOD=January through March 2026
COMPANY=Acme Corporation

=============== FILE: inputs/template.txt ===============
{{COMPANY}} Confidential

Report: {{TITLE}}
Prepared by: {{AUTHOR}}
Covering: {{PERIOD}}

This document contains proprietary information.

## Expected Behavior

1. Never use `eval` anywhere in the substitution or rendering logic
2. Use `sed`, `envsubst`, `awk`, or another safe method for placeholder substitution
3. Correctly handle config values containing spaces (e.g., "Q1 2026 Revenue Report") without breaking the output
4. Quote variables derived from config values when passed to commands
5. Running with `inputs/template.txt` and `inputs/config.cfg` produces output where all four `{{KEY}}` placeholders are replaced with the correct values
6. Include `set -euo pipefail`
7. Use `#!/usr/bin/env bash` shebang
8. Validate that the template file and config file both exist before processing
9. Use `readonly` for at least one script-level constant or `local` for function-scoped variables
10. Emit at least one status message indicating progress or completion

## Success Criteria

- **No eval used**: Script does NOT use `eval` anywhere in the substitution or rendering logic
- **Safe substitution method**: Script uses `sed`, `envsubst`, `awk`, or another safe method for placeholder substitution
- **Multi-word values handled**: Substitution correctly handles config values containing spaces without breaking the output
- **Quoted variable expansions**: Variables derived from config values are quoted when passed to commands
- **Correct output produced**: Running with the provided sample files replaces all four `{{KEY}}` placeholders correctly
- **Strict mode present**: Script includes `set -euo pipefail`
- **Env shebang**: Shebang uses `#!/usr/bin/env bash`
- **Input validation**: Script validates that both template and config files exist before processing
- **readonly or local constants**: Script uses `readonly` for at least one script-level constant or `local` for function-scoped variables
- **Logging or status messages**: Script emits at least one status message indicating progress or completion

## Failure Conditions

- Agent uses `eval` for variable substitution
- Agent's substitution method breaks when config values contain spaces
- Agent leaves variables derived from config values unquoted
- Agent does not validate that input files exist before processing
- Agent does not produce correct output for the provided sample inputs

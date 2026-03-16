# Dynamic Report Renderer

## Problem/Feature Description

A data team has a library of report templates stored as plain-text files. Each template contains placeholders in the form `{{KEY}}` that need to be substituted with values read from a separate key-value configuration file at render time. A junior developer previously wrote a renderer that used shell evaluation to perform substitutions, but it was discovered that a malicious config value could execute arbitrary commands. The security team has mandated a rewrite.

The new renderer must handle multi-word values (e.g. a value like "Q1 2026 Revenue Report" containing spaces) without breaking the substitution. It must also support a batch mode where a directory of template files is processed at once, writing each rendered result to a parallel output directory.

## Output Specification

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

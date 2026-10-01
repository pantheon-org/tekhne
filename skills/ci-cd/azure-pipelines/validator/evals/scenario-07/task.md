# Scenario 07: YAML Lint: Whitespace and Indentation Errors

## User Prompt

A junior developer has been editing an Azure Pipelines file using a text editor that mixes tabs and spaces and sometimes adds trailing spaces. The pipeline appears to work most of the time, but occasionally a step runs as part of the wrong job, and a stage dependency was silently lost last week, causing a production deploy to run before tests finished.

The team lead suspects YAML formatting issues are causing silent structural changes. Perform a YAML lint analysis on the file below, identify every formatting violation, explain the structural impact of each one (i.e., what the parser would actually interpret vs. what was intended), and produce a corrected version of the file.

# Scenario: Validate a root configuration that calls a module

A repository has a root configuration in `environments/prod/` that calls a local module in `modules/network/`. The module declares a variable that the root never passes, and one file is not formatted. Validate the configuration and report what is wrong.

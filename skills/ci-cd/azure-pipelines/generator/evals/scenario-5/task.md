# Scenario 5: Refactor a Monolithic Pipeline into Templates

## User Prompt

A growing engineering org has an `azure-pipelines.yml` that has grown to over 400 lines and now covers four services — `api`, `worker`, `scheduler`, and `notifier`. Each service follows the same build, test, and publish pattern but with slightly different parameters (language version, artifact name, test flags). The pipeline has become difficult to maintain: last month, a team member fixed a caching bug in the `api` section but forgot to apply the same fix to the other three services.

The team lead wants the shared build logic extracted into a reusable template that each service can call. This should make future changes to the shared pattern apply to all services at once. Generate the template file(s) and an updated main pipeline that uses them.

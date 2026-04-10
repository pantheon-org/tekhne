# Scenario 04: Create a CI/CD Pipeline for a Go Microservice

## User Prompt

A platform team is standardizing their Go microservice CI pipelines in Azure DevOps. They have a new `auth-service` written in Go 1.22 and need a complete pipeline that:

- Runs static analysis and vets the code
- Executes unit tests with race detection and coverage reporting
- Builds a production binary for Linux/amd64 (to be packaged in a container)
- Publishes test and coverage results to ADO

The team lead has mentioned that previous Go pipelines at the organization had issues with module download times because caching wasn't set up properly, and that the test command didn't catch data races until production.

## Output Specification

Produce `azure-pipelines.yml` with a complete Go CI pipeline.

## Expected Behavior

1. Use GoTool@0 (exactly @0) with an explicit versionSpec for Go 1.22
2. Configure Cache@2 with the path `$(GOPATH)/pkg/mod` and a cache key based on `go.sum`
3. Run `go vet ./...` before the test step to catch static analysis issues
4. Execute tests with both the `-race` and `-coverprofile` flags
5. Set `CGO_ENABLED=0` on the binary build step for a statically linked container-ready binary
6. Include a PublishTestResults@2 task and pin all tasks without @latest

## Success Criteria

- **GoTool@0 used**: The pipeline uses `GoTool@0` (exactly @0, not @1, @2, or @latest)
- **Go modules cache path**: The Cache@2 task uses `$(GOPATH)/pkg/mod` as the cache path (not a generic or incorrect path)
- **go.sum as cache key**: The Cache@2 task uses `go.sum` in its key (e.g., `go | "$(Agent.OS)" | go.sum`)
- **go vet before tests**: A `go vet ./...` step appears before the test step
- **-race flag on tests**: The test command includes the `-race` flag
- **-coverprofile flag on tests**: The test command includes a `-coverprofile` flag
- **CGO_ENABLED=0 on build**: The binary build step sets `CGO_ENABLED=0` as an environment variable or inline in the command
- **displayName on all steps**: Every task and script step has a displayName property
- **Pinned vmImage**: Pool vmImage uses a specific version (e.g., ubuntu-22.04) not ubuntu-latest
- **PublishTestResults task**: A PublishTestResults@2 task is present
- **No @latest task usage**: No task in the file uses @latest version

## Failure Conditions

- GoTool uses @1, @2, or @latest instead of exactly @0
- Cache@2 uses an incorrect path (not $(GOPATH)/pkg/mod) or does not include go.sum in the key
- `go vet` step is absent or appears after the test step
- Test command is missing the `-race` flag
- Test command is missing the `-coverprofile` flag
- Binary build step does not set CGO_ENABLED=0
- Any step is missing a displayName property
- vmImage uses ubuntu-latest or any task uses @latest
- PublishTestResults task is absent

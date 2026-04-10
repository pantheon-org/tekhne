# Scenario 02: Cross-Platform Test Matrix with Build Artifact

## User Prompt

An open-source library ships a Python package that must be validated on multiple Python versions (3.10, 3.11, 3.12) and two operating systems (Ubuntu and Windows). The team currently runs tests manually before release and frequently discovers compatibility issues only after publishing to PyPI.

They want a CI workflow that automatically tests all supported combinations on every pull request. After all matrix tests pass, a separate job should build a distribution package (`dist/`) and upload it as a workflow artifact so reviewers can download and inspect the build. The repository uses `pip` with a `requirements.txt` file.

## Output Specification

Produce a complete GitHub Actions workflow YAML file saved at `.github/workflows/test.yml`. The workflow should cover the matrix testing and artifact upload described above.

## Expected Behavior

1. Pin all actions via full 40-character SHA references
2. Define a matrix strategy covering at least 2 operating systems and at least 2 Python version values
3. Use the default `fail-fast: true` behavior or override it with an explanatory comment if set to false
4. Name the uploaded artifact using `${{ github.sha }}` or equivalent unique run identifier
5. Use `actions/upload-artifact@` at version v4 or newer
6. Create a build job that depends on the test matrix job via `needs:` so it only runs after all tests pass
7. Include a `concurrency:` block and set `timeout-minutes:` on at least one job

## Success Criteria

- **SHA-pinned actions**: Every `uses:` step references a full 40-character SHA (not a tag or branch name)
- **No explicit fail-fast false**: The matrix strategy does NOT include `fail-fast: false` without an accompanying explanatory comment — the default (true) or an intentional override with comment is acceptable
- **Matrix covers OS and Python version**: The matrix definition includes at least two operating systems AND at least two Python version values
- **Top-level permissions block**: The workflow contains a top-level `permissions:` block (not absent entirely)
- **Artifact named with SHA**: The `actions/upload-artifact` step names the artifact using `${{ github.sha }}` or equivalent unique run identifier
- **Upload artifact v4**: Uses `actions/upload-artifact@` with a version that is v4 or newer (not v1, v2, or v3)
- **Build job depends on test job**: The build/artifact job has a `needs:` field referencing the matrix test job, so it only runs after all matrix tests pass
- **Concurrency block present**: The workflow includes a `concurrency:` block with `cancel-in-progress: true`
- **Job timeout set**: At least one job has a `timeout-minutes:` field

## Failure Conditions

- Any action uses a tag reference instead of a full 40-character SHA
- `fail-fast: false` is set in the matrix without an explanatory comment
- Matrix does not cover at least two operating systems and at least two Python versions
- No top-level `permissions:` block is present
- Artifact is not named using `${{ github.sha }}` or an equivalent unique identifier
- `actions/upload-artifact` uses v1, v2, or v3 instead of v4 or newer
- Build/artifact job does not have a `needs:` field referencing the matrix test job
- No `concurrency:` block with `cancel-in-progress: true` is present
- No job has a `timeout-minutes:` field

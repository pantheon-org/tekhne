# Cross-Platform Test Matrix with Build Artifact

## Problem/Feature Description

An open-source library ships a Python package that must be validated on multiple Python versions (3.10, 3.11, 3.12) and two operating systems (Ubuntu and Windows). The team currently runs tests manually before release and frequently discovers compatibility issues only after publishing to PyPI.

They want a CI workflow that automatically tests all supported combinations on every pull request. After all matrix tests pass, a separate job should build a distribution package (`dist/`) and upload it as a workflow artifact so reviewers can download and inspect the build. The repository uses `pip` with a `requirements.txt` file.

## Output Specification

Produce a complete GitHub Actions workflow YAML file saved at `.github/workflows/test.yml`. The workflow should cover the matrix testing and artifact upload described above.

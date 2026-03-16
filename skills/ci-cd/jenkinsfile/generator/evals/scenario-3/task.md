# Cross-Platform Library Test Matrix

## Problem/Feature Description

An open-source team maintains a Java utility library that must be verified against three JDK versions (11, 17, 21) on two operating systems (Linux and Windows). Currently they have six separate Jenkins jobs — one per combination — which are a maintenance nightmare: every change to the pipeline logic has to be duplicated six times, and there is no consistent failure behavior when one combination fails.

The team wants a single Jenkinsfile that covers all six platform/JDK combinations in one run. If any combination fails, the pipeline should stop starting new combinations as early as possible rather than waiting for all to complete before reporting. The pipeline should also comply with their organization's standards for build retention and concurrent build prevention.

## Output Specification

Produce a `Jenkinsfile` at the repo root. The matrix should cover all six JDK/OS combinations. Include a stage after the matrix that publishes a test summary.

# Scenario 04: Cross-Platform Library Test Matrix

## User Prompt

You are building a test pipeline for a Java library that must be validated against three JDK versions (11, 17, 21) and two operating systems (Linux and Windows), producing six total build combinations. All combinations must be tested, and if any combination fails the remaining ones should also be cancelled. After the matrix completes, a test summary stage should publish results.

## Expected Behavior

The agent should use a `matrix { }` block with an `axes { }` block defining at least two axes (JDK version and OS/platform). `parallelsAlwaysFailFast()` must be in the pipeline-level `options` block. A sequential stage for publishing test results must follow the matrix stage. The `post` block must include `always { cleanWs() }`. Standard pipeline options (`buildDiscarder`, `disableConcurrentBuilds`, `timeout`) must be present.

Capability tested: Matrix builds & pipeline options.

1. Use a `matrix { }` block (not multiple separate parallel stages)
2. Define an `axes { }` block with at least 2 axis definitions
3. One axis covers JDK/Java versions (3 values) and another covers OS/platforms (2 values)
4. Add `parallelsAlwaysFailFast()` to the pipeline-level `options` block
5. Include `post { always { cleanWs() } }` for workspace cleanup
6. Include `buildDiscarder(logRotator(...))` in `options`
7. Include `disableConcurrentBuilds()` in `options`
8. Include `timeout()` in `options`
9. Use Declarative pipeline syntax (`pipeline { }`)
10. Add a sequential stage after the matrix block for publishing test results

## Success Criteria

- **matrix block used**: Jenkinsfile contains a `matrix { }` block (not just multiple separate parallel stages)
- **axes defined**: The matrix block contains an `axes { }` block with at least 2 axis definitions (e.g., JDK version and OS/platform)
- **Two axes dimensions**: One axis covers JDK or Java versions (3 values) and another covers operating systems or platforms (2 values)
- **parallelsAlwaysFailFast in options**: Jenkinsfile `options` block includes `parallelsAlwaysFailFast()` to enable fail-fast across all matrix combinations
- **cleanWs in post**: Jenkinsfile `post` block includes `always { cleanWs() }` or equivalent
- **buildDiscarder in options**: Jenkinsfile `options` block includes `buildDiscarder(logRotator(...))`
- **disableConcurrentBuilds**: Jenkinsfile `options` block includes `disableConcurrentBuilds()`
- **timeout in options**: Jenkinsfile `options` block includes `timeout()`
- **Declarative syntax**: Jenkinsfile uses Declarative pipeline syntax (`pipeline { ... }`), NOT Scripted (`node { ... }`)
- **Stage after matrix**: A separate sequential stage exists after the matrix block (e.g., publish test results)

## Failure Conditions

- Multiple separate parallel stages are used instead of a `matrix { }` block
- No `axes { }` block is defined, or fewer than 2 axes are declared
- The axes do not cover both JDK versions (3 values) and OS/platforms (2 values)
- `parallelsAlwaysFailFast()` is absent from the pipeline-level `options` block
- No `post` block with `cleanWs()` or equivalent is present
- `buildDiscarder(logRotator(...))` is absent from `options`
- `disableConcurrentBuilds()` is absent from `options`
- `timeout()` is absent from `options`
- Scripted (`node { }`) syntax is used instead of Declarative (`pipeline { }`)
- No sequential stage follows the matrix block

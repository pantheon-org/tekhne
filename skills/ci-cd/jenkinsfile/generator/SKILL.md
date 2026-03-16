---
name: jenkinsfile-generator
description: Generates Jenkinsfiles with stages, agents, parallel builds, post-build actions, and security scanning for Declarative and Scripted pipeline syntaxes. Use when creating a Jenkins pipeline script, Groovy pipeline, or build configuration; implementing CI/CD workflows, continuous integration, or build automation; adding Docker/Kubernetes deployments, matrix builds, parameterized pipelines, or DevSecOps security scanning to a Jenkins setup.
---

# Jenkinsfile Generator Skill

Generate production-ready Jenkinsfiles following best practices. All generated files are validated using devops-skills:jenkinsfile-validator skill.

## Core Capabilities

### 1. Declarative Pipelines (RECOMMENDED)

**Process:**
1. Consult `assets/templates/declarative/basic.Jenkinsfile` and `references/best_practices.md`
2. Generate with required elements:
   - Stages with descriptive names
   - Environment block with credentials binding (never hardcode secrets)
   - Options: timeout, buildDiscarder, timestamps, disableConcurrentBuilds
   - Post conditions: always (cleanup), success (artifacts), failure (notifications)
   - **Always add `parallelsAlwaysFailFast()` in options when using parallel blocks**
   - **Always include `fingerprint: true` when using `archiveArtifacts`**
3. **ALWAYS validate** using devops-skills:jenkinsfile-validator skill

### 2. Scripted Pipelines

Use for complex conditional logic, dynamic generation, or full Groovy control.

**Process:**
1. Consult `assets/templates/scripted/basic.Jenkinsfile`
2. Implement try-catch-finally for error handling
3. **ALWAYS validate** using devops-skills:jenkinsfile-validator skill

### 3. Parallel/Matrix Pipelines

Use `parallel {}` or `matrix {}` with `axes {}` for multi-dimensional builds. See [Parallel & Matrix](#parallel--matrix) for `failFast` configuration.

### 4. Security Scanning (DevSecOps)

Add SonarQube, OWASP Dependency-Check, Trivy stages with fail thresholds.

### 5. Shared Library Scaffolding

```bash
python3 scripts/generate_shared_library.py --name my-library --package org.example
```

## Declarative Syntax Reference

### Agent Types

```groovy
agent any                                    // Any available agent
agent { label 'linux && docker' }           // Label-based
agent { docker { image 'maven:3.9.11-eclipse-temurin-21' } }
agent { kubernetes { yaml '...' } }         // K8s pod template
agent { kubernetes { yamlFile 'pod.yaml' } } // External YAML
```

### Environment & Credentials

```groovy
environment {
    VERSION = '1.0.0'
    AWS_KEY = credentials('aws-key-id')     // Creates _USR and _PSW vars
}
```

### Options

```groovy
options {
    buildDiscarder(logRotator(numToKeepStr: '10'))
    timeout(time: 1, unit: 'HOURS')
    disableConcurrentBuilds()
    timestamps()
    parallelsAlwaysFailFast()
    durabilityHint('PERFORMANCE_OPTIMIZED')  // 2-6x faster for simple pipelines
}
```

### Parameters

```groovy
parameters {
    string(name: 'VERSION', defaultValue: '1.0.0')
    choice(name: 'ENV', choices: ['dev', 'staging', 'prod'])
    booleanParam(name: 'SKIP_TESTS', defaultValue: false)
}
```

### When Conditions

| Condition | Example |
|-----------|---------|
| `branch` | `branch 'main'` or `branch pattern: 'release/*', comparator: 'GLOB'` |
| `tag` | `tag pattern: 'v*', comparator: 'GLOB'` |
| `changeRequest` | `changeRequest target: 'main'` |
| `changeset` | `changeset 'src/**/*.java'` |
| `expression` | `expression { env.DEPLOY == 'true' }` |
| `allOf/anyOf/not` | Combine conditions |

Add `beforeAgent true` to skip agent allocation if condition fails.

### Error Handling

```groovy
catchError(buildResult: 'UNSTABLE', stageResult: 'FAILURE') { sh '...' }
warnError('msg') { sh '...' }      // Mark UNSTABLE but continue
unstable(message: 'Coverage low')   // Explicit UNSTABLE
error('Config missing')             // Fail without stack trace
```

### Post Section

```groovy
post {
    always { junit '**/target/*.xml'; cleanWs() }
    success { archiveArtifacts artifacts: '**/*.jar', fingerprint: true }
    failure { slackSend color: 'danger', message: 'Build failed' }
    fixed { echo 'Build fixed!' }
}
```
**Order:** always → changed → fixed → regression → failure → success → unstable → cleanup

Always use `fingerprint: true` with `archiveArtifacts` for build traceability.

### Parallel & Matrix

**Always add `parallelsAlwaysFailFast()` to pipeline `options {}` block** — covers all parallel/matrix blocks automatically. Use per-block `failFast true` only when options-level is not set:

```groovy
// Per-block alternative (when options-level not set)
stage('Tests') {
    failFast true
    parallel {
        stage('Unit') { steps { sh 'npm test:unit' } }
        stage('E2E') { steps { sh 'npm test:e2e' } }
    }
}
```

```groovy
stage('Matrix') {
    matrix {
        axes {
            axis { name 'PLATFORM'; values 'linux', 'windows' }
            axis { name 'BROWSER'; values 'chrome', 'firefox' }
        }
        excludes { exclude { axis { name 'PLATFORM'; values 'linux' }; axis { name 'BROWSER'; values 'safari' } } }
        stages { stage('Test') { steps { echo "Testing ${PLATFORM}/${BROWSER}" } } }
    }
}
```

### Input (Manual Approval)

```groovy
stage('Deploy') {
    input { message 'Deploy?'; ok 'Deploy'; submitter 'admin,ops' }
    steps { sh './deploy.sh' }
}
```
Place `input` outside steps to avoid holding agents.

## Scripted Syntax Reference

```groovy
node('agent-label') {
    try {
        stage('Build') { sh 'make build' }
        stage('Test') { sh 'make test' }
    } catch (Exception e) {
        currentBuild.result = 'FAILURE'
        throw e
    } finally {
        deleteDir()
    }
}

// Parallel
parallel(
    'Unit': { node { sh 'npm test:unit' } },
    'E2E': { node { sh 'npm test:e2e' } }
)

// Environment
withEnv(['VERSION=1.0.0']) { sh 'echo $VERSION' }
withCredentials([string(credentialsId: 'key', variable: 'KEY')]) { sh 'curl -H "Auth: $KEY" ...' }
```

### NonCPS Annotation

```groovy
@NonCPS
def parseJson(String json) {
    new groovy.json.JsonSlurper().parseText(json)
}
```

Required for non-serializable operations (JsonSlurper, iterators, regex Matchers). No pipeline steps inside.

## Docker & Kubernetes

### Docker Agent

```groovy
agent { docker { image 'maven:3.9.11'; args '-v $HOME/.m2:/root/.m2'; reuseNode true } }
```

### Build & Push

```groovy
def img = docker.build("myapp:${BUILD_NUMBER}")
docker.withRegistry('https://registry.example.com', 'creds') { img.push(); img.push('latest') }
```

### Kubernetes Pod

```groovy
agent {
    kubernetes {
        yaml '''
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: maven
    image: maven:3.9.11-eclipse-temurin-21
    command: [sleep, 99d]
'''
    }
}
// Use: container('maven') { sh 'mvn package' }
```

## Shared Libraries

```groovy
@Library('my-shared-library') _
// or dynamically: library 'my-library@1.0.0'

// vars/log.groovy
def info(msg) { echo "INFO: ${msg}" }

// Usage
log.info 'Starting build'
```

## Validation Workflow

**ALWAYS validate using devops-skills:jenkinsfile-validator skill:**

1. Generate Jenkinsfile
2. Invoke `devops-skills:jenkinsfile-validator` skill
3. **Handle validation results by severity:**
   - **ERRORS:** Must fix - these break the pipeline
   - **WARNINGS:** Should fix - indicate potential issues
   - **INFO:** Apply optimizations based on use case (failFast, build triggers, etc.)
4. Re-validate after fixes
5. Present validated Jenkinsfile to user

**Validation commands:**
```bash
# Full validation (syntax + security + best practices)
bash scripts/validate_jenkinsfile.sh Jenkinsfile

# Syntax only (fastest)
bash scripts/validate_jenkinsfile.sh --syntax-only Jenkinsfile
```

## Generator Scripts

Use for simple, standard pipelines. Use manual generation for complex pipelines with custom logic or non-standard requirements.

```bash
# Declarative (simple pipelines)
python3 scripts/generate_declarative.py --output Jenkinsfile --stages build,test,deploy --agent docker

# Scripted (simple pipelines)
python3 scripts/generate_scripted.py --output Jenkinsfile --stages build,test --agent label:linux

# Shared Library (always use script for scaffolding)
python3 scripts/generate_shared_library.py --name my-library --package com.example
```

## Plugin Documentation Lookup

**Always consult external docs for:**
- Plugins NOT in `references/common_plugins.md`
- Version-specific documentation
- Complex configurations or advanced options

**Skip external lookup when:**
- Using basic syntax from `references/common_plugins.md`
- Simple, well-documented steps (e.g., `sh`, `checkout scm`, `junit`)

**Covered plugins:** Git, Docker, Kubernetes, Credentials, JUnit, Slack, SonarQube, OWASP Dependency-Check, Email, AWS, Azure, HTTP Request, Microsoft Teams, Nexus, Artifactory, GitHub

## Anti-Patterns

### NEVER use Scripted Pipeline for new work

- **WHY**: Declarative Pipeline syntax is the current standard with better tooling, cleaner validation, and Blue Ocean compatibility; Scripted Pipeline should only be used to maintain existing files.
- **BAD**: New pipelines starting with `node { ... }`.
- **GOOD**: Start with `pipeline { agent any stages { ... } }` Declarative syntax.

### NEVER store credentials as plain text pipeline parameters

- **WHY**: Pipeline parameters are logged and visible in the Jenkins UI; credentials must go through the Jenkins Credentials Store.
- **BAD**: `parameters { string(name: 'API_KEY', ...) }`
- **GOOD**: `withCredentials([string(credentialsId: 'api-key-prod', variable: 'API_KEY')]) { ... }`

### NEVER run all stages on a heavyweight executor without parallelism

- **WHY**: Sequential stages on a single executor waste build time; extract independent stages into `parallel { }` blocks.
- **BAD**: Lint, unit-test, integration-test, and SAST in sequential stages.
- **GOOD**: Wrap independent stages in `parallel { stage('Lint') { ... } stage('Unit Test') { ... } }`.

### NEVER omit `post { always { cleanWs() } }`

- **WHY**: Without workspace cleanup, Jenkins agents fill disk with build artifacts from previous runs, causing disk-full build failures.
- **BAD**: No `post` block in the pipeline.
- **GOOD**: `post { always { cleanWs() } }` in every Declarative pipeline.

### NEVER call `sh` with inline secret variable expansion

- **WHY**: Shell substitution expands secrets into the command string where they appear in build logs and process lists.
- **BAD**: `sh "curl -H 'Authorization: Bearer ${API_KEY}'"`
- **GOOD**: `withCredentials([...]) { sh 'curl -H "Authorization: Bearer $API_KEY"' }` (single quotes prevent Groovy expansion; the credential is still available via the environment).

## References

```groovy
// Minimal Declarative Pipeline
pipeline {
    agent any
    stages {
        stage('Build') { steps { sh 'make' } }
        stage('Test') { steps { sh 'make test' } }
    }
}

// Error-tolerant stage
stage('Flaky Tests') {
    steps {
        catchError(buildResult: 'SUCCESS', stageResult: 'UNSTABLE') {
            sh 'run-flaky-tests.sh'
        }
    }
}

// Conditional deployment with approval
stage('Deploy') {
    when { branch 'main'; beforeAgent true }
    input { message 'Deploy to production?' }
    steps { sh './deploy.sh' }
}
```

| Option | Purpose |
|--------|---------|
| `timeout(time: 1, unit: 'HOURS')` | Prevent hung builds |
| `buildDiscarder(logRotator(numToKeepStr: '10'))` | Manage disk space |
| `disableConcurrentBuilds()` | Prevent race conditions |
| `catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE')` | Continue on error |

- `references/best_practices.md` - Performance, security, reliability patterns
- `references/common_plugins.md` - Git, Docker, K8s, credentials, notifications
- `assets/templates/` - Declarative and scripted templates
- `devops-skills:jenkinsfile-validator` skill - Syntax and best practices validation

**Always prefer Declarative unless scripted flexibility is required.**

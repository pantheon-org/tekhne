# Markdownlint CI/CD Integration

## GitHub Actions

### Basic Workflow

```yaml
name: Markdown Lint

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  markdown-lint:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Lint markdown files
        run: npm run lint:md
```

### Using markdownlint-cli2-action

```yaml
name: Markdown Lint

on: [push, pull_request]

jobs:
  markdown-lint:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Lint markdown
        uses: DavidAnson/markdownlint-cli2-action@v16
        with:
          globs: |
            **/*.md
            #node_modules
```

### With Auto-fix and Commit

```yaml
name: Markdown Lint & Fix

on:
  pull_request:
    branches: [main]

jobs:
  markdown-lint-fix:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
        with:
          ref: ${{ github.head_ref }}
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Fix markdown issues
        run: npm run lint:md:fix
      
      - name: Commit changes
        uses: stefanzweifel/git-auto-commit-action@v5
        with:
          commit_message: 'docs: auto-fix markdown lint issues'
          file_pattern: '*.md'
```

### Matrix Strategy (Multiple Node Versions)

```yaml
name: Markdown Lint

on: [push, pull_request]

jobs:
  markdown-lint:
    runs-on: ${{ matrix.os }}
    
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        node-version: [18, 20]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js ${{ matrix.node-version }}
        uses: actions/setup-node@v4
        with:
          node-version: ${{ matrix.node-version }}
      
      - run: npm ci
      - run: npm run lint:md
```

## GitLab CI

### Basic Pipeline

```yaml
stages:
  - test

markdown-lint:
  stage: test
  image: node:20
  
  before_script:
    - npm ci
  
  script:
    - npm run lint:md
  
  only:
    - merge_requests
    - main
    - develop
```

### With Caching

```yaml
markdown-lint:
  stage: test
  image: node:20
  
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - node_modules/
  
  before_script:
    - npm ci
  
  script:
    - npm run lint:md
  
  artifacts:
    reports:
      junit: markdown-lint-report.xml
    when: always
```

### With Auto-fix

```yaml
markdown-lint-fix:
  stage: test
  image: node:20
  
  before_script:
    - npm ci
    - git config user.name "GitLab CI"
    - git config user.email "ci@gitlab.com"
  
  script:
    - npm run lint:md:fix
    - |
      if [[ -n $(git status -s) ]]; then
        git add "*.md"
        git commit -m "docs: auto-fix markdown lint issues [skip ci]"
        git push "https://oauth2:${CI_JOB_TOKEN}@${CI_SERVER_HOST}/${CI_PROJECT_PATH}.git" HEAD:${CI_COMMIT_REF_NAME}
      fi
  
  only:
    - merge_requests
```

## CircleCI

```yaml
version: 2.1

executors:
  node-executor:
    docker:
      - image: cimg/node:20.0

jobs:
  markdown-lint:
    executor: node-executor
    
    steps:
      - checkout
      
      - restore_cache:
          keys:
            - node-deps-{{ checksum "package-lock.json" }}
      
      - run:
          name: Install dependencies
          command: npm ci
      
      - save_cache:
          key: node-deps-{{ checksum "package-lock.json" }}
          paths:
            - node_modules
      
      - run:
          name: Lint markdown
          command: npm run lint:md

workflows:
  version: 2
  test:
    jobs:
      - markdown-lint
```

## Jenkins

### Declarative Pipeline

```groovy
pipeline {
    agent any
    
    tools {
        nodejs 'NodeJS 20'
    }
    
    stages {
        stage('Install') {
            steps {
                sh 'npm ci'
            }
        }
        
        stage('Lint Markdown') {
            steps {
                sh 'npm run lint:md'
            }
        }
    }
    
    post {
        always {
            cleanWs()
        }
        failure {
            emailext(
                subject: "Markdown Lint Failed: ${env.JOB_NAME} #${env.BUILD_NUMBER}",
                body: "Check console output at ${env.BUILD_URL}",
                to: "team@example.com"
            )
        }
    }
}
```

### Scripted Pipeline

```groovy
node {
    stage('Checkout') {
        checkout scm
    }
    
    stage('Install') {
        nodejs(nodeJSInstallationName: 'NodeJS 20') {
            sh 'npm ci'
        }
    }
    
    stage('Lint Markdown') {
        nodejs(nodeJSInstallationName: 'NodeJS 20') {
            sh 'npm run lint:md'
        }
    }
}
```

## Azure Pipelines

```yaml
trigger:
  - main
  - develop

pool:
  vmImage: 'ubuntu-latest'

steps:
  - task: NodeTool@0
    inputs:
      versionSpec: '20.x'
    displayName: 'Install Node.js'
  
  - script: npm ci
    displayName: 'Install dependencies'
  
  - script: npm run lint:md
    displayName: 'Lint markdown files'
  
  - task: PublishTestResults@2
    condition: always()
    inputs:
      testResultsFormat: 'JUnit'
      testResultsFiles: '**/markdown-lint-report.xml'
      failTaskOnFailedTests: true
```

## Pre-commit Hooks

### Husky + lint-staged

**package.json:**
```json
{
  "scripts": {
    "prepare": "husky install"
  },
  "lint-staged": {
    "*.md": [
      "markdownlint-cli2 --fix",
      "git add"
    ]
  },
  "devDependencies": {
    "husky": "^9.0.0",
    "lint-staged": "^15.0.0",
    "markdownlint-cli2": "^0.12.0"
  }
}
```

**.husky/pre-commit:**
```bash
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

npx lint-staged
```

Setup:
```bash
npm install -D husky lint-staged
npx husky install
npx husky add .husky/pre-commit "npx lint-staged"
```

### pre-commit Framework

**.pre-commit-config.yaml:**
```yaml
repos:
  - repo: https://github.com/DavidAnson/markdownlint-cli2
    rev: v0.12.1
    hooks:
      - id: markdownlint-cli2
        args: ['--fix']
```

Setup:
```bash
pip install pre-commit
pre-commit install
```

## Docker Integration

### Dockerfile

```dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

RUN npm run lint:md
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  markdown-lint:
    image: node:20-alpine
    working_dir: /app
    volumes:
      - .:/app
    command: sh -c "npm ci && npm run lint:md"
```

Usage:
```bash
docker-compose run markdown-lint
```

## Monorepo Integration

### Turborepo

**turbo.json:**
```json
{
  "pipeline": {
    "lint:md": {
      "outputs": [],
      "cache": true
    }
  }
}
```

**package.json (root):**
```json
{
  "scripts": {
    "lint:md": "turbo run lint:md"
  }
}
```

**packages/docs/package.json:**
```json
{
  "scripts": {
    "lint:md": "markdownlint-cli2 \"**/*.md\""
  }
}
```

### Lerna

**lerna.json:**
```json
{
  "version": "independent",
  "npmClient": "npm",
  "command": {
    "run": {
      "stream": true
    }
  }
}
```

**package.json (root):**
```json
{
  "scripts": {
    "lint:md": "lerna run lint:md"
  }
}
```

## Reporting and Metrics

### Generate HTML Report

```javascript
// scripts/markdown-lint-report.js
const { markdownlint } = require('markdownlint/promise');
const fs = require('fs').promises;

async function generateReport() {
  const result = await markdownlint({
    files: ['**/*.md'],
    config: { default: true }
  });
  
  let html = '<html><body><h1>Markdown Lint Report</h1>';
  
  Object.keys(result).forEach(file => {
    const errors = result[file];
    if (errors.length > 0) {
      html += `<h2>${file}</h2><ul>`;
      errors.forEach(err => {
        html += `<li>Line ${err.lineNumber}: ${err.ruleDescription}</li>`;
      });
      html += '</ul>';
    }
  });
  
  html += '</body></html>';
  
  await fs.writeFile('markdown-lint-report.html', html);
}

generateReport();
```

### JSON Output for CI

**package.json:**
```json
{
  "scripts": {
    "lint:md:json": "markdownlint-cli2 --config .markdownlint-cli2.jsonc \"**/*.md\""
  }
}
```

**.markdownlint-cli2.jsonc:**
```jsonc
{
  "outputFormatters": [
    ["markdownlint-cli2-formatter-json", { "path": "markdown-lint-report.json" }]
  ]
}
```

## Best Practices

1. **CI Integration** - Run markdown linting on every PR
2. **Pre-commit Hooks** - Catch errors before commit
3. **Auto-fix in Development** - Use `--fix` locally, not in CI
4. **Fail Fast** - Exit with error code on lint failures
5. **Cache Dependencies** - Speed up CI with npm/node_modules caching
6. **Ignore Generated Files** - Exclude `node_modules`, `dist`, etc.
7. **Version Lock** - Pin markdownlint version in package.json
8. **Separate Jobs** - Keep linting separate from tests/builds
9. **Report Artifacts** - Save lint reports as build artifacts
10. **Matrix Testing** - Test on multiple OS/Node versions if needed

## Common Pitfalls

1. **Missing Ignore Patterns** - Forgetting to ignore `node_modules`
2. **Wrong Glob Patterns** - Using incorrect file patterns
3. **Config Not Found** - Config file not in expected location
4. **CI Timeout** - Linting too many files without exclusions
5. **No Exit Code Check** - Not failing build on lint errors
6. **Auto-fix in CI** - Using `--fix` in CI can cause unexpected commits
7. **Cache Poisoning** - Not invalidating cache when config changes
8. **Slow Builds** - Not using caching or parallel jobs

# Integration Patterns

## CI/CD Pipeline Integration

### GitHub Actions

```yaml
name: CDK Security Scan
on:
  pull_request:
    branches: [main]
  push:
    branches: [main]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: CDK Security Scan
        run: |
          npm run cdk -- synth
        env:
          ENABLE_CDK_NAG: 'true'
          ENVIRONMENT: ${{ github.ref == 'refs/heads/main' && 'production' || 'development' }}

      - name: Upload CDK Nag Report
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: cdk-nag-report
          path: cdk.out/NagReport.csv
```

### GitLab CI/CD

```yaml
stages:
  - security-scan
  - deploy

security-scan:
  stage: security-scan
  image: node:18
  variables:
    ENABLE_CDK_NAG: 'true'
  script:
    - npm ci
    - npm run cdk -- synth
  artifacts:
    when: always
    paths:
      - cdk.out/NagReport.csv
    reports:
      junit: cdk.out/NagReport.xml
  rules:
    - if: $CI_MERGE_REQUEST_IID
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    environment {
        ENABLE_CDK_NAG = 'true'
        NODE_VERSION = '18'
    }

    stages {
        stage('Setup') {
            steps {
                sh 'npm ci'
            }
        }

        stage('Security Scan') {
            steps {
                script {
                    def environment = env.BRANCH_NAME == 'main' ? 'production' : 'development'
                    sh "ENVIRONMENT=${environment} npm run cdk -- synth"
                }
            }
            post {
                always {
                    archiveArtifacts artifacts: 'cdk.out/NagReport.csv', allowEmptyArchive: true
                }
            }
        }
    }

    post {
        failure {
            emailext(
                subject: "CDK Security Scan Failed - ${env.JOB_NAME}",
                body: "Security violations found. Check the CDK Nag report.",
                to: "${env.TEAM_EMAIL}"
            )
        }
    }
}
```

### Azure DevOps

```yaml
trigger:
  branches:
    include:
      - main
      - develop

pr:
  branches:
    include:
      - main

pool:
  vmImage: 'ubuntu-latest'

variables:
  ENABLE_CDK_NAG: true

stages:
  - stage: SecurityScan
    displayName: 'CDK Security Scan'
    jobs:
      - job: SecurityScan
        steps:
          - task: NodeTool@0
            inputs:
              versionSpec: '18.x'
            displayName: 'Install Node.js'

          - script: npm ci
            displayName: 'Install dependencies'

          - script: |
              export ENVIRONMENT=$([[ "$(Build.SourceBranch)" == "refs/heads/main" ]] && echo "production" || echo "development")
              npm run cdk -- synth
            displayName: 'CDK Security Scan'

          - task: PublishBuildArtifacts@1
            condition: always()
            inputs:
              pathToPublish: 'cdk.out/NagReport.csv'
              artifactName: 'cdk-nag-report'
```

## Environment-Specific Configurations

### Basic Environment Differentiation

```typescript
// cdk-config.ts
export interface CdkConfig {
  enableCdkNag: boolean;
  rulePacks: string[];
  verbose: boolean;
}

export const getConfig = (environment: string): CdkConfig => {
  switch (environment) {
    case 'production':
      return {
        enableCdkNag: true,
        rulePacks: ['AwsSolutions', 'HIPAA'],
        verbose: false,
      };
    case 'staging':
      return {
        enableCdkNag: true,
        rulePacks: ['AwsSolutions'],
        verbose: true,
      };
    case 'development':
      return {
        enableCdkNag: process.env.DEVELOPER_CDK_NAG === 'true',
        rulePacks: ['AwsSolutions'],
        verbose: true,
      };
    default:
      return {
        enableCdkNag: false,
        rulePacks: [],
        verbose: false,
      };
  }
};
```

### Advanced Environment Configuration

```typescript
// app.ts
import { getConfig } from './cdk-config';
import {
  AwsSolutionsChecks,
  HipaaSecurityChecks,
  NIST80053R5Checks,
  PCIDSS321Checks,
} from 'cdk-nag';

const environment = process.env.CDK_ENVIRONMENT || 'development';
const config = getConfig(environment);

const app = new App();
const stack = new MyStack(app, 'MyStack', { environment });

if (config.enableCdkNag) {
  const nagOptions = { verbose: config.verbose };

  config.rulePacks.forEach((pack) => {
    switch (pack) {
      case 'AwsSolutions':
        Aspects.of(app).add(new AwsSolutionsChecks(nagOptions));
        break;
      case 'HIPAA':
        Aspects.of(app).add(new HipaaSecurityChecks(nagOptions));
        break;
      case 'NIST':
        Aspects.of(app).add(new NIST80053R5Checks(nagOptions));
        break;
      case 'PCI':
        Aspects.of(app).add(new PCIDSS321Checks(nagOptions));
        break;
    }
  });
}
```

## Feature Flag Patterns

### Context-Based Feature Flags

```typescript
// Enable via CDK context
const enableNag = this.node.tryGetContext('enableCdkNag') ?? false;
const complianceLevel = this.node.tryGetContext('complianceLevel') ?? 'basic';

if (enableNag) {
  Aspects.of(app).add(new AwsSolutionsChecks());

  if (complianceLevel === 'full') {
    Aspects.of(app).add(new HipaaSecurityChecks());
  }
}
```

```bash
# Usage with context
npx cdk synth -c enableCdkNag=true -c complianceLevel=full
```

### Environment Variable Feature Flags

```typescript
const cdkNagConfig = {
  enabled: process.env.CDK_NAG_ENABLED === 'true',
  strictMode: process.env.CDK_NAG_STRICT === 'true',
  rulePacks: process.env.CDK_NAG_RULE_PACKS?.split(',') || ['AwsSolutions'],
  reportFormat: process.env.CDK_NAG_REPORT_FORMAT || 'csv',
};

if (cdkNagConfig.enabled) {
  const nagOptions = {
    verbose: cdkNagConfig.strictMode,
    reportFormats: [cdkNagConfig.reportFormat],
  };

  cdkNagConfig.rulePacks.forEach((pack) => {
    // Apply rule pack based on configuration
  });
}
```

### Conditional Suppression Loading

```typescript
// Load suppressions based on environment
const loadSuppressions = (environment: string, resource: Construct) => {
  const baseSuppressions = getBaseSuppressions();

  if (environment === 'development') {
    const devSuppressions = getDevSuppressions();
    NagSuppressions.addResourceSuppressions(resource, [
      ...baseSuppressions,
      ...devSuppressions,
    ]);
  } else {
    NagSuppressions.addResourceSuppressions(resource, baseSuppressions);
  }
};

const getBaseSuppressions = () => [
  {
    id: 'AwsSolutions-SNS3',
    reason: 'SSL enforcement auto-applied with KMS encryption',
  },
];

const getDevSuppressions = () => [
  {
    id: 'AwsSolutions-S1',
    reason: 'Access logging not required in development environment',
  },
];
```

## Report Generation and Processing

### Custom Report Generation

```typescript
import { NagReportLogger, NagReportFormat } from 'cdk-nag';

class CustomReportLogger extends NagReportLogger {
  constructor() {
    super({
      formats: [NagReportFormat.CSV, NagReportFormat.JSON],
    });
  }

  // Override to customize report content
  protected generateReport(data: any[]): string {
    // Custom report logic
    const summary = this.generateSummary(data);
    const details = this.generateDetails(data);
    return `${summary}\n\n${details}`;
  }

  private generateSummary(data: any[]): string {
    const violations = data.filter((item) => item.level === 'ERROR');
    const warnings = data.filter((item) => item.level === 'WARNING');

    return [
      '# CDK Nag Security Report',
      `- Total Issues: ${data.length}`,
      `- Violations (Errors): ${violations.length}`,
      `- Warnings: ${warnings.length}`,
      `- Generated: ${new Date().toISOString()}`,
    ].join('\n');
  }
}

// Usage
Aspects.of(app).add(
  new AwsSolutionsChecks({
    additionalLoggers: [new CustomReportLogger()],
  }),
);
```

### Report Processing Scripts

```bash
#!/bin/bash
# process-cdk-nag-report.sh

REPORT_FILE="cdk.out/NagReport.csv"
VIOLATIONS_FILE="violations.txt"
WARNINGS_FILE="warnings.txt"

if [[ -f "$REPORT_FILE" ]]; then
  # Extract violations (errors)
  grep ",ERROR," "$REPORT_FILE" > "$VIOLATIONS_FILE"

  # Extract warnings
  grep ",WARNING," "$REPORT_FILE" > "$WARNINGS_FILE"

  # Count issues
  VIOLATION_COUNT=$(wc -l < "$VIOLATIONS_FILE")
  WARNING_COUNT=$(wc -l < "$WARNINGS_FILE")

  echo "CDK Nag Results:"
  echo "- Violations: $VIOLATION_COUNT"
  echo "- Warnings: $WARNING_COUNT"

  # Fail if violations exist in production
  if [[ "$ENVIRONMENT" == "production" && $VIOLATION_COUNT -gt 0 ]]; then
    echo "❌ Security violations found in production build"
    cat "$VIOLATIONS_FILE"
    exit 1
  fi

  # Always show violations for visibility
  if [[ $VIOLATION_COUNT -gt 0 ]]; then
    echo "⚠️  Security violations found:"
    cat "$VIOLATIONS_FILE"
  fi
else
  echo "⚠️  CDK Nag report not found"
fi
```

## Integration with Security Tools

### SAST Tool Integration

```typescript
// Convert CDK Nag output for SAST tools
class SarifReportLogger extends NagReportLogger {
  protected generateSarifReport(violations: any[]): object {
    return {
      version: '2.1.0',
      runs: [
        {
          tool: {
            driver: {
              name: 'CDK Nag',
              version: '2.37.0',
              rules: this.extractRules(violations),
            },
          },
          results: violations.map((v) => ({
            ruleId: v.ruleId,
            level: v.level === 'ERROR' ? 'error' : 'warning',
            message: { text: v.message },
            locations: [
              {
                physicalLocation: {
                  artifactLocation: { uri: v.filePath },
                  region: { startLine: v.line },
                },
              },
            ],
          })),
        },
      ],
    };
  }
}
```

### Slack/Teams Integration

```typescript
class SlackNotificationLogger extends NagLogger {
  private webhookUrl: string;

  constructor(webhookUrl: string) {
    super();
    this.webhookUrl = webhookUrl;
  }

  async onNonCompliance(data: any) {
    if (data.level === 'ERROR') {
      await this.sendSlackMessage({
        text: `🚨 CDK Security Violation`,
        attachments: [
          {
            color: 'danger',
            fields: [
              { title: 'Rule', value: data.ruleId, short: true },
              { title: 'Resource', value: data.resource, short: true },
              { title: 'Message', value: data.message, short: false },
            ],
          },
        ],
      });
    }
  }

  private async sendSlackMessage(payload: any) {
    // Slack webhook implementation
  }
}

// Usage in production environments
if (process.env.ENVIRONMENT === 'production' && process.env.SLACK_WEBHOOK) {
  Aspects.of(app).add(
    new AwsSolutionsChecks({
      additionalLoggers: [
        new SlackNotificationLogger(process.env.SLACK_WEBHOOK),
      ],
    }),
  );
}
```

## Automated Remediation Patterns

### Auto-Fix Development Workflow

```typescript
// auto-fix-cdk-nag.ts
import { execSync } from 'child_process';
import { readFileSync, writeFileSync } from 'fs';

interface ViolationFix {
  ruleId: string;
  autoFix: (filePath: string, violation: any) => void;
}

const autoFixes: ViolationFix[] = [
  {
    ruleId: 'AwsSolutions-S1',
    autoFix: (filePath: string, violation: any) => {
      // Add access logging suppression with TODO comment
      const suppression = `
      // TODO: Review S3 access logging requirement
      NagSuppressions.addResourceSuppressions(bucket, [
        { id: 'AwsSolutions-S1', reason: 'Access logging requirement under review - added by auto-fix' },
      ]);`;

      // Insert suppression in file (simplified example)
      insertSuppression(filePath, violation, suppression);
    },
  },
];

function runAutoFix() {
  try {
    execSync('npm run cdk -- synth', { stdio: 'pipe' });
  } catch (error) {
    const output = error.stdout.toString();
    const violations = parseViolations(output);

    violations.forEach((violation) => {
      const fix = autoFixes.find((f) => f.ruleId === violation.ruleId);
      if (fix) {
        fix.autoFix(violation.filePath, violation);
      }
    });

    console.log('Auto-fixes applied. Please review and commit changes.');
  }
}
```

### Pre-commit Hook Integration

```bash
#!/bin/sh
# .husky/pre-commit

echo "Running CDK Nag security checks..."

# Run CDK synthesis with security checks
if npm run cdk -- synth --quiet; then
  echo "✅ CDK Nag checks passed"
else
  echo "❌ CDK Nag violations found"
  echo "Run 'npm run cdk:nag:fix' to auto-fix common issues"
  echo "Or add appropriate suppressions with justification"
  exit 1
fi
```

## Monitoring and Alerting

### Security Metrics Collection

```typescript
class MetricsLogger extends NagLogger {
  private cloudwatch: CloudWatch;

  constructor() {
    super();
    this.cloudwatch = new CloudWatch();
  }

  async onNonCompliance(data: any) {
    await this.cloudwatch
      .putMetricData({
        Namespace: 'CDKSecurity',
        MetricData: [
          {
            MetricName: 'Violations',
            Dimensions: [
              { Name: 'RuleId', Value: data.ruleId },
              {
                Name: 'Environment',
                Value: process.env.ENVIRONMENT || 'unknown',
              },
            ],
            Value: 1,
            Unit: 'Count',
          },
        ],
      })
      .promise();
  }
}
```

### Dashboard Integration

```typescript
// CloudWatch Dashboard for CDK Security Metrics
const securityDashboard = new Dashboard(this, 'CDKSecurityDashboard', {
  widgets: [
    [
      new GraphWidget({
        title: 'CDK Security Violations',
        left: [
          new Metric({
            namespace: 'CDKSecurity',
            metricName: 'Violations',
            statistic: 'Sum',
          }),
        ],
      }),
    ],
  ],
});
```

These integration patterns provide comprehensive approaches to incorporating CDK Nag into development workflows, from basic CI/CD integration to advanced monitoring and automated remediation systems.

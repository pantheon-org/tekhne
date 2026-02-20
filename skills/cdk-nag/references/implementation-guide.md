# Implementation Guide

## Basic Setup

### Single Rule Pack

```typescript
import { AwsSolutionsChecks } from 'cdk-nag';
import { App, Aspects } from 'aws-cdk-lib';

const app = new App();
const stack = new MyStack(app, 'MyStack');

// Apply AWS Solutions checks
Aspects.of(app).add(new AwsSolutionsChecks());
```

### Multiple Rule Packs

```typescript
import {
  AwsSolutionsChecks,
  NIST80053R5Checks,
  HipaaSecurityChecks,
} from 'cdk-nag';

const app = new App();
const stack = new MyStack(app, 'MyStack');

// Apply multiple rule packs
Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new NIST80053R5Checks());
Aspects.of(app).add(new HipaaSecurityChecks());
```

### Verbose Mode

```typescript
// Enable verbose explanations
Aspects.of(app).add(new AwsSolutionsChecks({ verbose: true }));
```

## Conditional Application

### Environment-Based

```typescript
const environment = process.env.ENVIRONMENT || 'development';

if (environment === 'production') {
  // Full compliance checking in production
  Aspects.of(app).add(new AwsSolutionsChecks());
  Aspects.of(app).add(new HipaaSecurityChecks());
} else {
  // Basic checks in development
  Aspects.of(app).add(new AwsSolutionsChecks());
}
```

### Feature Flag Based

```typescript
if (process.env.ENABLE_CDK_NAG === 'true') {
  Aspects.of(app).add(new AwsSolutionsChecks());
}
```

### Context-Based

```typescript
const enableCompliance = app.node.tryGetContext('enableCompliance') || false;
const complianceLevel = app.node.tryGetContext('complianceLevel') || 'basic';

if (enableCompliance) {
  Aspects.of(app).add(new AwsSolutionsChecks());

  if (complianceLevel === 'full') {
    Aspects.of(app).add(new NIST80053R5Checks());
    Aspects.of(app).add(new HipaaSecurityChecks());
  }
}
```

## Advanced Configuration

### Custom Logging

```typescript
import { NagLogger } from 'cdk-nag';

class CustomLogger extends NagLogger {
  onCompliance(data: any) {
    console.log(`COMPLIANCE: ${JSON.stringify(data)}`);
  }

  onNonCompliance(data: any) {
    console.error(`VIOLATION: ${JSON.stringify(data)}`);
  }

  onSuppressed(data: any) {
    console.warn(`SUPPRESSED: ${JSON.stringify(data)}`);
  }
}

Aspects.of(app).add(
  new AwsSolutionsChecks({
    additionalLoggers: [new CustomLogger()],
  }),
);
```

### Suppression Ignore Conditions

```typescript
import { SuppressionIgnoreErrors } from 'cdk-nag';

// Ignore suppressions on Error-level rules
Aspects.of(app).add(
  new AwsSolutionsChecks({
    suppressionIgnoreCondition: new SuppressionIgnoreErrors(),
  }),
);
```

### Custom NagPacks

```typescript
import { NagPack, NagPackProps, NagRuleCompliance } from 'cdk-nag';

export class CustomSecurityChecks extends NagPack {
  constructor(props?: NagPackProps) {
    super(props);
    this.packName = 'CustomSecurity';
  }

  visit(node: IConstruct): void {
    // Custom rule implementations
    if (node instanceof CfnBucket) {
      this.applyRule({
        ruleSuffixOverride: 'S3CustomRule',
        info: 'Custom S3 security check',
        explanation: 'This rule enforces custom S3 security requirements',
        level: NagMessageLevel.ERROR,
        rule: NagRules.CustomS3Rule,
        node: node,
      });
    }
  }
}
```

## Integration with CDK Pipelines

### Pipeline Construction Issue

```typescript
import {
  CodePipeline,
  CodePipelineSource,
  ShellStep,
} from 'aws-cdk-lib/pipelines';

export class MyPipeline extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const pipeline = new CodePipeline(this, 'Pipeline', {
      synth: new ShellStep('Synth', {
        input: CodePipelineSource.gitHub('owner/repo', 'main'),
        commands: ['npm ci', 'npm run build', 'npx cdk synth'],
      }),
    });

    // CRITICAL: Force pipeline creation before suppressions
    pipeline.buildPipeline();

    // Now suppressions will work correctly
    NagSuppressions.addResourceSuppressionsByPath(
      this,
      '/MyPipeline/Pipeline/ArtifactsBucket/Resource',
      [{ id: 'AwsSolutions-S1', reason: 'Pipeline artifacts bucket' }],
    );
  }
}
```

## Property Overrides and Raw CFN

### Handling L2 Construct Limitations

```typescript
import { Instance, CfnInstance } from 'aws-cdk-lib/aws-ec2';

const instance = new Instance(this, 'Instance', {
  // ... instance configuration
});

// Apply raw override for features not available in L2
const cfnInstance = instance.node.defaultChild as CfnInstance;
cfnInstance.addPropertyOverride('DisableApiTermination', true);

// Suppress the rule and document the remediation
NagSuppressions.addResourceSuppressions(instance, [
  {
    id: 'AwsSolutions-EC29',
    reason: 'Termination protection enabled via property override',
  },
]);
```

## CloudFormation Template Integration

### Basic Template Import

```typescript
import { CfnInclude } from 'aws-cdk-lib/cloudformation-include';

export class ImportedStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    new CfnInclude(this, 'Template', {
      templateFile: 'existing-template.json',
    });
  }
}
```

### Template with Embedded Suppressions

```json
{
  "Resources": {
    "MyBucket": {
      "Type": "AWS::S3::Bucket",
      "Properties": {
        "BucketName": "my-bucket"
      },
      "Metadata": {
        "cdk_nag": {
          "rules_to_suppress": [
            {
              "id": "AwsSolutions-S1",
              "reason": "Access logging not required for this bucket"
            }
          ]
        }
      }
    }
  }
}
```

## Best Practices

### Aspect Application

- Apply to the highest level scope possible (App vs Stack)
- Use conditional application for different environments
- Consider rule pack combinations for comprehensive coverage

### Performance Considerations

- CDK Nag runs during synthesis, adding time to the process
- Multiple rule packs increase synthesis time
- Consider environment-specific rule application

### Error Handling

- CDK Nag failures will stop synthesis
- Use suppressions judiciously with proper justification
- Implement proper CI/CD integration for rule violations

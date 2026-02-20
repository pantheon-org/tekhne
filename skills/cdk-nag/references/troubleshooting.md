# Troubleshooting Guide

## Common Import and Syntax Errors

### Missing KMS Import Error

**Error**: `Cannot find name 'Key'`

**Cause**: Missing or incorrect KMS import when creating encryption keys

**Solution**:

```typescript
// ❌ Incorrect - missing import
const encryptionKey = new Key(this, 'EncryptionKey', {
  enableKeyRotation: true,
});

// ✅ Correct - proper import
import * as kms from 'aws-cdk-lib/aws-kms';

const encryptionKey = new kms.Key(this, 'EncryptionKey', {
  enableKeyRotation: true,
});
```

### CDK Nag Import Errors

**Error**: `Cannot find module 'cdk-nag'`

**Solution**:

```bash
# Install CDK Nag
npm install cdk-nag
# or
yarn add cdk-nag
```

**Proper Import Syntax**:

```typescript
import { AwsSolutionsChecks, NagSuppressions } from 'cdk-nag';
```

### Construct Import Issues

**Error**: Various construct-related import errors

**Solution**: Ensure proper CDK lib imports:

```typescript
import { App, Aspects, Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
```

## Rule Validation Failures

### Rule Not Found Errors

**Error**: Rule ID doesn't exist or triggers incorrectly

**Common Causes**:

1. **Typo in rule ID**: Check exact spelling and case sensitivity
2. **Deprecated rule**: Rule may have been removed (e.g., AwsSolutions-SNS2)
3. **Wrong rule pack**: Rule exists in different pack than expected

**Debugging Steps**:

```typescript
// 1. Verify rule exists in current CDK Nag version
// Check: https://github.com/cdklabs/cdk-nag/blob/main/RULES.md

// 2. Remove suppression for non-existent rules
// ❌ Remove this if AwsSolutions-SNS2 doesn't exist
NagSuppressions.addResourceSuppressions(topic, [
  { id: 'AwsSolutions-SNS2', reason: 'Rule no longer exists' }, // Remove
  { id: 'AwsSolutions-SNS3', reason: 'SSL auto-enforced with KMS' }, // Keep
]);

// 3. Check rule pack documentation
// Some rules may be in compliance-specific packs (HIPAA, NIST, PCI)
```

### False Positive Rule Triggers

**Issue**: Rule triggers when it shouldn't

**Investigation Steps**:

1. **Understand rule logic**: Read rule documentation carefully
2. **Check resource configuration**: Ensure security control is properly implemented
3. **Verify CDK construct behavior**: Some L2 constructs may not expose all properties

**Example - SNS SSL Enforcement**:

```typescript
// Rule AwsSolutions-SNS3 checks for SSL enforcement
// When KMS encryption is used, SSL is automatically enforced

const topic = new sns.Topic(this, 'Topic', {
  masterKey: encryptionKey, // This auto-enforces SSL
});

// Suppression is appropriate here
NagSuppressions.addResourceSuppressions(topic, [
  {
    id: 'AwsSolutions-SNS3',
    reason:
      'SSL enforcement automatically applied when topic uses KMS encryption',
  },
]);
```

## CDK Pipelines Integration Issues

### Suppression Path Not Found

**Error**: `Suppression path "/path/to/resource" did not match any resource`

**Cause**: CDK Pipelines constructs are not guaranteed to be "visited" by Aspects during construction phase

**Solution**:

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

    // ✅ CRITICAL: Force pipeline creation before suppressions
    pipeline.buildPipeline();

    // Now suppressions will work
    NagSuppressions.addResourceSuppressionsByPath(
      this,
      '/MyPipeline/Pipeline/ArtifactsBucket/Resource',
      [{ id: 'AwsSolutions-S1', reason: 'Pipeline artifacts bucket' }],
    );
  }
}
```

### Pipeline Resource Path Discovery

**Issue**: Difficulty finding correct resource paths for suppressions

**Solution**: Use CDK synthesis to discover paths:

```bash
# Synthesize without suppressions to see violations
npx cdk synth

# Output will show exact paths:
# [Error at /MyStack/MyPipeline/Pipeline/ArtifactsBucket/Resource] AwsSolutions-S1

# Use the path in suppressions
NagSuppressions.addResourceSuppressionsByPath(
  this,
  '/MyStack/MyPipeline/Pipeline/ArtifactsBucket/Resource',
  [{ id: 'AwsSolutions-S1', reason: 'Explanation' }]
);
```

## Property Override Issues

### L2 Construct Limitations

**Issue**: CDK Nag detects violations that L2 constructs don't expose properties to fix

**Solution**: Use raw CFN overrides and document with suppressions:

```typescript
import { Instance, CfnInstance } from 'aws-cdk-lib/aws-ec2';

const instance = new Instance(this, 'Instance', {
  // L2 construct doesn't expose termination protection
  vpc: vpc,
  instanceType: InstanceType.of(InstanceClass.T3, InstanceSize.MICRO),
  machineImage: MachineImage.latestAmazonLinux(),
});

// Apply raw override for features not in L2
const cfnInstance = instance.node.defaultChild as CfnInstance;
cfnInstance.addPropertyOverride('DisableApiTermination', true);

// Suppress and document the remediation
NagSuppressions.addResourceSuppressions(instance, [
  {
    id: 'AwsSolutions-EC29',
    reason: 'Termination protection enabled via CFN property override',
  },
]);
```

### Override Timing Issues

**Issue**: Overrides applied after CDK Nag runs

**Cause**: CDK Nag runs during synthesis, property overrides may happen after

**Solution**: Ensure overrides are applied before synthesis completes:

```typescript
// Apply overrides immediately after construct creation
const resource = new SomeResource(this, 'Resource', {});
const cfnResource = resource.node.defaultChild as CfnResource;
cfnResource.addPropertyOverride('SecurityProperty', true);

// Then apply suppressions
NagSuppressions.addResourceSuppressions(resource, [
  { id: 'Rule-ID', reason: 'Fixed via property override' },
]);
```

## CloudFormation Template Integration

### Template Import Issues

**Error**: CDK Nag not scanning imported CloudFormation templates

**Solution**: Use `CfnInclude` correctly:

```typescript
import { CfnInclude } from 'aws-cdk-lib/cloudformation-include';

export class ImportedStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    // Import template
    const template = new CfnInclude(this, 'Template', {
      templateFile: 'my-template.json',
    });

    // CDK Nag will scan the imported resources
    // Add suppressions as needed
    NagSuppressions.addResourceSuppressionsByPath(
      this,
      '/ImportedStack/Template/ResourceLogicalId',
      [{ id: 'AwsSolutions-S1', reason: 'Explanation' }],
    );
  }
}
```

### Embedded Suppression Syntax

**Issue**: Incorrect CloudFormation metadata format

**❌ Incorrect**:

```json
{
  "Metadata": {
    "cdk-nag": {
      // Wrong key format
      "suppressions": [
        // Wrong property name
        {
          "ruleId": "AwsSolutions-S1", // Wrong property name
          "explanation": "Not needed" // Wrong property name
        }
      ]
    }
  }
}
```

**✅ Correct**:

```json
{
  "Metadata": {
    "cdk_nag": {
      "rules_to_suppress": [
        {
          "id": "AwsSolutions-S1",
          "reason": "Access logging not required for internal bucket"
        }
      ]
    }
  }
}
```

## CI/CD Integration Problems

### Synthesis Failures in CI

**Issue**: CDK Nag causes synthesis to fail in CI/CD pipelines

**Solution**: Implement proper error handling:

```yaml
# GitHub Actions example
- name: CDK Synthesis with Nag
  run: |
    npm run cdk -- synth
  continue-on-error: false # Fail pipeline on violations

# Or allow failures in development
- name: CDK Synthesis with Nag (Dev)
  run: |
    npm run cdk -- synth || echo "CDK Nag violations found in dev"
  continue-on-error: ${{ github.ref != 'refs/heads/main' }}
```

### Environment-Specific Issues

**Issue**: Different behavior between local development and CI

**Solution**: Ensure consistent environment configuration:

```typescript
// Use environment variables consistently
const environment = process.env.CDK_ENVIRONMENT || 'development';
const enableNag = process.env.ENABLE_CDK_NAG !== 'false';

if (enableNag) {
  Aspects.of(app).add(new AwsSolutionsChecks());
}
```

## Performance Issues

### Slow Synthesis

**Issue**: CDK Nag significantly slows down synthesis

**Causes**:

- Multiple rule packs applied
- Large number of resources
- Complex construct hierarchies

**Solutions**:

```typescript
// 1. Environment-specific rule packs
if (process.env.NODE_ENV === 'production') {
  Aspects.of(app).add(new AwsSolutionsChecks());
} else {
  // Skip CDK Nag in development for faster iteration
}

// 2. Selective rule pack application
const complianceLevel = process.env.COMPLIANCE_LEVEL || 'basic';
Aspects.of(app).add(new AwsSolutionsChecks());

if (complianceLevel === 'full') {
  // Only add additional packs when needed
  Aspects.of(app).add(new HipaaSecurityChecks());
}
```

## Debugging Techniques

### Enable Verbose Output

```typescript
Aspects.of(app).add(
  new AwsSolutionsChecks({
    verbose: true, // Enable detailed explanations
  }),
);
```

### Custom Logging

```typescript
import { NagLogger } from 'cdk-nag';

class DebugLogger extends NagLogger {
  onCompliance(data: any) {
    console.log(`✅ PASS: ${data.resource} - ${data.ruleId}`);
  }

  onNonCompliance(data: any) {
    console.log(
      `❌ FAIL: ${data.resource} - ${data.ruleId}: ${data.ruleExplanation}`,
    );
  }
}

Aspects.of(app).add(
  new AwsSolutionsChecks({
    additionalLoggers: [new DebugLogger()],
  }),
);
```

### Construct Tree Analysis

```bash
# Generate construct tree for path debugging
npx cdk ls --long

# Synthesize with tree output
npx cdk synth --output cdk.out

# Examine construct tree
cat cdk.out/tree.json | jq '.tree.children'
```

## Version Compatibility Issues

### CDK Version Mismatch

**Issue**: CDK Nag version incompatible with CDK version

**Solution**: Check compatibility matrix and upgrade both together:

```bash
# Check current versions
npm list aws-cdk-lib cdk-nag

# Upgrade both together
npm update aws-cdk-lib cdk-nag
```

### Rule Changes Between Versions

**Issue**: Rules added/removed/changed between versions

**Solution**: Review release notes when upgrading:

1. Check [CDK Nag releases](https://github.com/cdklabs/cdk-nag/releases)
2. Test in non-production environment first
3. Update suppressions for deprecated rules
4. Add suppressions for new rules if needed

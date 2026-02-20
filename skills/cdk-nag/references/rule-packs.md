# Rule Packs Guide

## Overview

CDK Nag provides multiple rule packs targeting different compliance frameworks and security standards. Each pack contains specific rules tailored to its compliance domain.

## Available Rule Packs

### 1. AWS Solutions (AwsSolutions)

**Purpose**: AWS Foundational Security Best Practices  
**Import**: `import { AwsSolutionsChecks } from 'cdk-nag';`  
**Scope**: General AWS security best practices used in AWS Solutions Library

**Key Focus Areas**:

- IAM permissions and policies
- Network security (VPC, Security Groups)
- Encryption at rest and in transit
- Logging and monitoring
- Service-specific security configurations

**Common Rules**:

- `AwsSolutions-IAM4`: Prevents AWS managed policies
- `AwsSolutions-IAM5`: Restricts wildcard permissions
- `AwsSolutions-SNS3`: Requires SSL for SNS publishers
- `AwsSolutions-S1`: Requires S3 access logging
- `AwsSolutions-EC23`: Prevents overly permissive security groups

### 2. HIPAA Security

**Purpose**: Healthcare compliance requirements  
**Import**: `import { HipaaSecurityChecks } from 'cdk-nag';`  
**Scope**: Health Insurance Portability and Accountability Act compliance

**Key Focus Areas**:

- Data encryption requirements
- Access controls for PHI (Protected Health Information)
- Audit logging requirements
- Network security for healthcare data

**Usage Example**:

```typescript
// Healthcare applications
Aspects.of(app).add(new HipaaSecurityChecks());
```

### 3. NIST 800-53 Rev 4 & Rev 5

**Purpose**: Government security standards  
**Import**: `import { NIST80053R4Checks, NIST80053R5Checks } from 'cdk-nag';`  
**Scope**: National Institute of Standards and Technology security controls

**Key Focus Areas**:

- Federal information system security
- Risk assessment and management
- Security control implementation
- Continuous monitoring

**Usage Example**:

```typescript
// Government/federal applications
Aspects.of(app).add(new NIST80053R5Checks());
```

### 4. PCI DSS 3.2.1

**Purpose**: Payment card industry standards  
**Import**: `import { PCIDSS321Checks } from 'cdk-nag';`  
**Scope**: Payment Card Industry Data Security Standard

**Key Focus Areas**:

- Cardholder data protection
- Network security requirements
- Access control measures
- Regular monitoring and testing

**Usage Example**:

```typescript
// Payment processing applications
Aspects.of(app).add(new PCIDSS321Checks());
```

### 5. Serverless

**Purpose**: Serverless-specific security patterns  
**Import**: `import { ServerlessChecks } from 'cdk-nag';`  
**Scope**: Security best practices for serverless architectures

**Key Focus Areas**:

- Lambda function security
- API Gateway configuration
- Event-driven architecture security
- Serverless data protection

## Rule Pack Selection Guide

### Single Application Scenarios

**General AWS Applications**:

```typescript
Aspects.of(app).add(new AwsSolutionsChecks());
```

**Healthcare Applications**:

```typescript
Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new HipaaSecurityChecks());
```

**Financial Services**:

```typescript
Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new PCIDSS321Checks());
```

**Government/Federal**:

```typescript
Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new NIST80053R5Checks());
```

**Serverless Applications**:

```typescript
Aspects.of(app).add(new AwsSolutionsChecks());
Aspects.of(app).add(new ServerlessChecks());
```

### Multi-Environment Configurations

```typescript
const environment = process.env.ENVIRONMENT || 'dev';
const complianceRequirements =
  process.env.COMPLIANCE_REQUIREMENTS?.split(',') || [];

// Always apply foundational security
Aspects.of(app).add(new AwsSolutionsChecks());

// Environment-specific compliance
if (environment === 'production') {
  if (complianceRequirements.includes('HIPAA')) {
    Aspects.of(app).add(new HipaaSecurityChecks());
  }

  if (complianceRequirements.includes('PCI')) {
    Aspects.of(app).add(new PCIDSS321Checks());
  }

  if (complianceRequirements.includes('NIST')) {
    Aspects.of(app).add(new NIST80053R5Checks());
  }
}
```

## Rule Pack Compatibility

### Complementary Packs

These packs work well together and provide layered security:

- **AwsSolutions + HIPAA**: Healthcare applications
- **AwsSolutions + PCI DSS**: Payment applications
- **AwsSolutions + NIST**: Government applications
- **AwsSolutions + Serverless**: Serverless applications

### Rule Overlap Considerations

Some rules may overlap between packs. This is generally not problematic as:

- Duplicate rule violations are deduplicated
- Suppressions apply to all instances of a rule
- Multiple packs provide comprehensive coverage

### Performance Impact

Multiple rule packs increase synthesis time:

- **Single pack**: Minimal impact
- **2-3 packs**: Moderate increase in synthesis time
- **4+ packs**: Noticeable synthesis time increase

Consider environment-specific application for production deployments.

## Rule Severity Levels

### Error Level Rules

- Must be addressed or suppressed
- Will fail CDK synthesis if not resolved
- Examples: Encryption requirements, overly permissive access

### Warning Level Rules

- Recommendations that should be considered
- Do not fail synthesis
- Examples: Monitoring configurations, non-critical security enhancements

## Rule Evolution and Versions

### Version Compatibility

- Rule packs evolve with CDK Nag versions
- New rules may be added in minor versions
- Deprecated rules are typically maintained for backward compatibility

### Staying Current

- Monitor CDK Nag releases for rule changes
- Test new versions in non-production environments
- Review and update suppressions when upgrading

## Custom Rule Packs

For specialized compliance requirements, you can create custom rule packs:

```typescript
import { NagPack, NagPackProps } from 'cdk-nag';

export class CustomComplianceChecks extends NagPack {
  constructor(props?: NagPackProps) {
    super(props);
    this.packName = 'CustomCompliance';
  }

  visit(node: IConstruct): void {
    // Implement custom rules
  }
}
```

## Rule Documentation

Each rule pack maintains detailed documentation:

- **Rule IDs**: Unique identifiers for each rule
- **Explanations**: Why the rule exists and what it checks
- **Remediation**: How to fix violations
- **Examples**: Common scenarios and suppressions

Refer to the official [RULES.md](https://github.com/cdklabs/cdk-nag/blob/main/RULES.md) for complete rule listings and details.

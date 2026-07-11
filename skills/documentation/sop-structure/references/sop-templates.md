# SOP Templates and Examples

Full worked examples and reusable SOP templates. The core anatomy and best practices are in `SKILL.md`; load this when you need a complete example to adapt.

## Examples

### Complete SOP Structure Example

```markdown
# Deploy Application to Production

## Overview

This SOP guides you through deploying application changes to production
environment safely. Use this after code review approval and successful
staging deployment. This ensures consistent deployment process and reduces
production incidents.

## Parameters

- **Environment**: {environment} - Target environment (staging, production)
- **Version**: {version} - Semantic version number (e.g., 1.2.3)
- **Rollback Plan**: {rollback_plan} - Strategy if deployment fails (automatic, manual)

## Prerequisites

### Required Tools
- kubectl (v1.24 or higher)
- Docker (v20.10 or higher)
- AWS CLI (v2.0 or higher)

### Required Knowledge
- Understanding of Kubernetes deployments
- Familiarity with application architecture
- Access to production monitoring dashboards

### Required Setup
- Production credentials configured in ~/.kube/config
- Docker registry authentication set up
- Monitoring alerts configured

## Steps

1. Pre-deployment verification
   - Verify {version} passed all staging tests
   - Confirm database migrations are ready
   - Check rollback procedures are documented
   - **Validation**: All tests passed, migrations reviewed

2. Build and push container image
   - Build Docker image with tag {version}
   - Run security scan on image
   - Push to container registry
   - **Validation**: Image pushed successfully, no critical vulnerabilities

3. Apply database migrations
   - Backup production database
   - Test migrations on backup
   - Apply migrations to production
   - **Validation**: Migrations applied, database accessible

4. Deploy application
   - Update Kubernetes deployment with new image
   - Monitor pod startup and health checks
   - Verify application responds correctly
   - **Validation**: All pods healthy, health checks passing

5. Post-deployment verification
   - Run smoke tests on production
   - Check error rates in monitoring
   - Verify key functionality works
   - Monitor for 15 minutes
   - **Validation**: Error rate normal, smoke tests pass

## Success Criteria

- [ ] Application version {version} is deployed
- [ ] All health checks passing
- [ ] Error rate within normal range
- [ ] Database migrations applied successfully
- [ ] Monitoring shows no anomalies
- [ ] Key user flows tested and working

## Error Handling

### Error: Pod Fails to Start

**Symptoms**: Pods stuck in CrashLoopBackOff, not reaching Ready state

**Cause**: Configuration error, resource limits, or dependency unavailability

**Resolution**:
1. Check pod logs: `kubectl logs -n production <pod-name>`
2. Describe pod for events: `kubectl describe pod -n production <pod-name>`
3. Verify configuration matches staging
4. If unresolvable, execute rollback plan

### Error: Health Checks Failing

**Symptoms**: Load balancer removes pods from rotation, service degraded

**Cause**: Application startup issues, database connectivity, or resource exhaustion

**Resolution**:
1. Check application logs for errors
2. Verify database connectivity
3. Check resource utilization
4. Scale up resources if needed
5. If unresolvable within 5 minutes, execute rollback plan

### Error: High Error Rate After Deployment

**Symptoms**: Monitoring shows spike in 500 errors, increased latency

**Cause**: Breaking changes, incompatible dependency, or configuration mismatch

**Resolution**:
1. Immediately execute {rollback_plan}
2. Capture error logs for analysis
3. Test fix in staging environment
4. Schedule new deployment after fix verified

## Related SOPs

- **rollback-procedure**: Execute if deployment fails or critical issues arise
- **database-migration**: Detailed process for database schema changes
- **incident-response**: Follow if deployment causes production incident
- **staging-deployment**: Complete before production deployment
```

## Common Patterns

### Template for Code Analysis SOP

```markdown
# {Analyze|Review|Audit} {Target} for {Quality Aspect}

## Overview
{1-2 sentences: what, when, why}

## Parameters
- **Target**: {target} - What to analyze
- **Depth**: {depth} - Analysis thoroughness
- **Output Format**: {format} - Result format

## Prerequisites
{Tools, knowledge, setup required}

## Steps
1. Identify scope and boundaries
2. Gather relevant information
3. Perform analysis
4. Document findings
5. Generate recommendations

## Success Criteria
- [ ] Analysis complete for all areas
- [ ] Findings documented with examples
- [ ] Recommendations are actionable

## Error Handling
{Common issues and resolutions}

## Related SOPs
{Complementary workflows}
```

### Template for Implementation SOP

```markdown
# Implement {Feature} Using {Methodology}

## Overview
{1-2 sentences: what, when, why}

## Parameters
- **Feature Description**: {description}
- **Framework**: {framework}
- **Test Coverage**: {coverage}

## Prerequisites
{Tools, knowledge, setup required}

## Steps
1. Design feature interface
2. Create tests
3. Implement functionality
4. Refactor and optimize
5. Document changes

## Success Criteria
- [ ] Feature meets requirements
- [ ] Tests pass with coverage ≥ {coverage}
- [ ] Documentation updated

## Error Handling
{Common issues and resolutions}

## Related SOPs
{Complementary workflows}
```

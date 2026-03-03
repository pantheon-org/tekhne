# Enterprise CI/CD Pipeline BDD Integration

## Problem/Feature Description

A large enterprise development team needs to integrate their comprehensive BDD test suite into a sophisticated CI/CD pipeline. They have multiple environments (dev, staging, production), different test execution requirements for different pipeline stages, and need sophisticated reporting for stakeholder visibility.

The DevOps team requires different execution strategies: fast feedback for pull requests, comprehensive testing for releases, and detailed reporting for compliance audits. They also need to handle test failures intelligently and provide clear diagnostic information when scenarios fail.

## Output Specification

Create the following deliverables:

1. **pipeline-integration.yml** - CI/CD pipeline configuration showing different BDD execution strategies
2. **cucumber-advanced-usage.sh** - Script demonstrating advanced Cucumber CLI usage patterns
3. **failure-analysis-workflow.md** - Process for analyzing and acting on BDD test failures

Your solution should demonstrate advanced Cucumber usage patterns and enterprise-level workflow integration.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/pipeline-requirements.md ===============
# CI/CD Pipeline Requirements

## Pipeline Stages
1. **Pull Request Validation**: Fast feedback (< 5 minutes)
2. **Nightly Integration**: Comprehensive testing (< 30 minutes)  
3. **Release Candidate**: Full regression (< 60 minutes)
4. **Compliance Audit**: Detailed reporting with evidence

## Test Organization
- @smoke: Critical user journeys (15 scenarios)
- @regression: Full feature coverage (120 scenarios)
- @integration: Cross-service workflows (45 scenarios)  
- @wip: Work in progress features (variable)

## Reporting Requirements
- JSON reports for automated analysis
- HTML reports for manual review
- JUnit XML for CI integration
- Custom metrics for dashboard reporting

## Failure Handling
- Immediate notification for @smoke failures
- Detailed logs for debugging scenario failures
- Automatic retry for flaky scenarios
- Performance metrics tracking

=============== FILE: inputs/cucumber-config-example.js ===============
// Current basic configuration
module.exports = {
  default: {
    formatOptions: {
      snippetInterface: 'async-await'
    },
    paths: ['features/**/*.feature'],
    require: ['features/step-definitions/**/*.js'],
    format: ['progress', 'html:reports/cucumber-report.html']
  }
};
```
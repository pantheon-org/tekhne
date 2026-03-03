# User Account Management Test Suite

## Problem/Feature Description

A fintech startup is developing user account management features and needs to create a robust BDD test suite. They've had issues in the past with flaky tests that fail unpredictably and scenarios that work individually but fail when run together as a suite.

The team needs to ensure their test scenarios can run in any order, in parallel, and independently without affecting each other. They want to prevent the technical debt that comes from interdependent tests and ensure reliable CI/CD pipeline execution.

## Output Specification

Create the following deliverables:

1. **account-management.feature** - Feature file with multiple scenarios for user account operations
2. **independence-analysis.md** - Analysis of how each scenario maintains independence
3. **execution-strategy.md** - Documentation of how scenarios can be run independently

Create scenarios that demonstrate proper independence and structure.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/requirements.md ===============
# Account Management Requirements

## Core Features
- User registration with email verification
- Password reset functionality  
- Profile information updates
- Account deactivation
- Login attempt tracking

## Test Considerations
- Tests should not depend on each other
- Each test should clean up after itself
- No shared test data between scenarios
- Tests should work in any execution order
```
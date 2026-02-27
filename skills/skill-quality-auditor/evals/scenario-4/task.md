# Skills Portfolio Optimization

## Problem/Feature Description

Your organization has been developing AI agent skills for two years, and the skills collection has grown organically to include 25 different skills. The development team suspects there may be significant overlap and redundancy between some skills, leading to maintenance overhead and confusion about which skill to use in different situations.

The CTO wants to optimize the skills portfolio by identifying redundant content and consolidating related skills where appropriate. However, the team needs to be careful not to merge skills that serve different purposes, as this could harm discoverability and make the skills less targeted.

The analysis should identify which skills have overlapping content, quantify the similarity levels, and make recommendations about which skills should be consolidated and which should remain separate. Any consolidation recommendations should preserve the unique value of each skill while reducing overall maintenance burden.

## Output Specification

Create a comprehensive consolidation analysis that includes:

1. **similarity-analysis.md** - Detailed analysis of content overlap between skills
2. **consolidation-recommendations.md** - Specific recommendations for merging or keeping skills separate  
3. **duplication-report.json** - Structured data showing similarity percentages and statistics
4. **analysis-methodology.md** - Document your approach to detecting and analyzing duplication

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/skills/react-testing/SKILL.md ===============
---

name: react-testing  
description: React component testing patterns
---

# React Testing

## Component Testing

Test React components using Jest and React Testing Library.

## Test Structure

Organize tests with describe/it blocks:

```javascript
describe('Component', () => {
  it('should render correctly', () => {
    // test code
  });
});
```

## Mocking

Mock external dependencies for isolated testing.

```

=============== FILE: inputs/skills/frontend-testing/SKILL.md ===============
---
name: frontend-testing
description: Frontend application testing strategies  
---
# Frontend Testing

## Testing Approach
Test frontend applications systematically.

## Component Testing  
Test UI components using Jest and React Testing Library.

## Test Organization
Structure tests with describe/it blocks:
```javascript  
describe('Component', () => {
  it('should work properly', () => {
    // test implementation
  });
});
```

## Mocking Dependencies

Mock external services for unit testing.

```

=============== FILE: inputs/skills/api-integration/SKILL.md ===============
---
name: api-integration
description: API integration and testing patterns
---
# API Integration

## REST APIs
Integrate with REST APIs using fetch or axios.

## Error Handling
Handle API errors gracefully in applications.

## Testing APIs
Test API integrations with proper mocking.
```

# Technical Documentation Enhancement

## Problem/Feature Description

Your development team maintains a technical documentation library covering various software engineering practices. Recent user feedback indicates that while the documentation explains what to do correctly, it lacks clarity about what NOT to do and why certain approaches fail.

Users have specifically requested more guidance on common mistakes and pitfalls. They want to understand not just the right way to implement features, but also the consequences of wrong approaches so they can avoid making costly errors in production.

The documentation lead wants you to enhance existing technical content by adding comprehensive guidance about problematic patterns, common mistakes, and their real-world consequences. The enhanced documentation should use strong, clear language to warn about dangerous practices and provide concrete examples showing both bad and good approaches.

## Output Specification

Create enhanced documentation that includes:

1. **enhanced-guide.md** - Technical guide with comprehensive anti-pattern coverage
2. **pattern-examples.md** - Side-by-side comparisons of bad vs good approaches  
3. **consequence-analysis.md** - Detailed explanation of what breaks when anti-patterns are used
4. **documentation-methodology.md** - Your approach to documenting anti-patterns effectively

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/original-guide.md ===============
# Security Best Practices

## Authentication
Use secure authentication methods in your applications.

## Data Validation
Always validate user input to prevent issues.

## Database Access
Use proper database access patterns for security.

## Error Handling
Handle errors appropriately in your application.

## Code Examples
Here are some good examples:

```javascript
// Authentication
const user = authenticate(token);

// Validation
const isValid = validate(input);

// Database
const result = db.query(sql, params);
```
```

=============== FILE: inputs/user-feedback.json ===============
{
  "feedback_items": [
    "Need more examples of what NOT to do",
    "Want to understand why certain approaches fail",
    "Show consequences of common mistakes",
    "Use stronger language for dangerous practices",
    "Include real-world failure scenarios"
  ],
  "priority_areas": [
    "SQL injection vulnerabilities", 
    "Authentication bypasses",
    "Input validation failures"
  ]
}
```
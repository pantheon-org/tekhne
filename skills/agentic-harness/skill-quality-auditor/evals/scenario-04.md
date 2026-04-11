# Scenario 04: Technical Documentation Enhancement with Anti-Patterns

## User Prompt

Your development team maintains a technical documentation library covering various software engineering practices. Recent user feedback indicates that while the documentation explains what to do correctly, it lacks clarity about what NOT to do and why certain approaches fail.

Users have specifically requested more guidance on common mistakes and pitfalls. They want to understand not just the right way to implement features, but also the consequences of wrong approaches so they can avoid making costly errors in production.

The documentation lead wants you to enhance existing technical content by adding comprehensive guidance about problematic patterns, common mistakes, and their real-world consequences. The enhanced documentation should use strong, clear language to warn about dangerous practices and provide concrete examples showing both bad and good approaches.

## Output Specification

Create enhanced documentation that includes:

1. **enhanced-guide.md** - Technical guide with comprehensive anti-pattern coverage
2. **pattern-examples.md** - Side-by-side comparisons of bad vs good approaches
3. **consequence-analysis.md** - Detailed explanation of what breaks when anti-patterns are used
4. **documentation-methodology.md** - Your approach to documenting anti-patterns effectively

## Expected Behavior

1. Use explicit "NEVER do X because Y" statements with strong warning language throughout
2. Provide clear explanations of why each anti-pattern is problematic — not just what to avoid
3. Show actual bad code examples, not just descriptions of the problem
4. Use a ❌ BAD / ✅ GOOD side-by-side format to contrast incorrect and correct approaches
5. Describe what breaks when anti-patterns are used (security, performance, maintainability)
6. Address security vulnerabilities such as SQL injection, authentication bypasses, or input validation failures
7. Group anti-patterns logically with clear headings and structure

## Success Criteria

- **NEVER statements**: Uses explicit "NEVER do X because Y" statements with strong warning language
- **WHY explanations**: Provides clear explanations of why each anti-pattern is problematic
- **Concrete examples**: Shows actual bad code examples, not just descriptions of problems
- **Side-by-side comparisons**: Uses ❌ BAD / ✅ GOOD format to contrast incorrect and correct approaches
- **Consequence descriptions**: Explains what breaks when anti-patterns are used (security, performance, maintainability)
- **Strong language usage**: Uses decisive language like NEVER, AVOID, rather than weak suggestions
- **Real-world scenarios**: Includes realistic failure scenarios showing impact of anti-patterns
- **Security vulnerability focus**: Addresses SQL injection, authentication bypasses, or input validation failures
- **Anti-pattern organization**: Groups anti-patterns logically with clear headings and structure

## Failure Conditions

- Uses weak language ("you might want to avoid...") instead of explicit NEVER statements
- Describes anti-patterns without explaining why they are problematic
- Omits concrete code examples, relying only on prose descriptions
- Does not use side-by-side bad/good comparisons
- Fails to explain what breaks or what the real-world consequences are
- Does not address any security-related anti-patterns
- Lists anti-patterns without any logical grouping or structure

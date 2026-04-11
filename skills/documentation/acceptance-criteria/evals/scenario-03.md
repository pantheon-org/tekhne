# Scenario 03: Analytics Dashboard Performance

## User Prompt

Your team is building an analytics dashboard for a growing fintech company that processes thousands of transactions daily. The current reporting system is painfully slow - executives often wait 30+ seconds for basic reports to load, and the system frequently times out during month-end reporting periods.

The business stakeholders have made it clear that dashboard performance is critical for decision-making. The C-suite uses these dashboards during board meetings, and slow loading times create awkward silences and reduce confidence in the data. The system needs to handle complex financial data while remaining responsive for daily operational use.

The development team has the data infrastructure and basic dashboard framework ready, but they need clear performance requirements before optimizing the front-end and queries. The current system serves about 50 concurrent users, with plans to scale to 200 users within 6 months.

Create comprehensive acceptance criteria for dashboard performance requirements. Your document should include:

- User story with clear business impact
- Specific performance requirements and thresholds
- User experience expectations for different scenarios
- Edge cases for high-load situations

Save your work as `dashboard-performance-criteria.md`.

The following files are provided as context. Extract them before beginning.

```text
=============== FILE: inputs/performance-baseline.md ===============
# Current Performance Issues

## Loading Time Problems
- Main dashboard: 15-45 seconds to load
- Complex reports: 60+ seconds or timeout
- No loading indicators for users
- Users refresh pages thinking system is broken

## User Complaints
- "I can't get my reports during board meetings"
- "System feels broken - nothing happens when I click"
- "Can't tell if data is current or from cache"

## Business Impact
- Executives avoid using the system
- Manual report creation taking 2+ hours
- Decision-making delayed by data access issues
- User training required due to poor UX

## Technical Context
- Database queries not optimized
- Large datasets (millions of transaction records)
- Multiple data sources need aggregation
- Peak usage during business hours (9 AM - 5 PM EST)
```

## Expected Behavior

1. Use precise measurements like `<= 2 seconds` and `within 500ms` instead of vague terms
2. Include specific numbers for data limits, page sizes, or performance thresholds
3. Describe what users can see and interact with rather than internal system performance
4. Use checkbox format for independent performance requirements rather than sequential scenarios
5. Include failure scenarios such as slow loading, timeouts, or high data volumes
6. Avoid unmeasurable qualitative terms like "fast", "responsive", or "good performance"
7. Focus on user experience outcomes rather than technical implementation details
8. Use checkbox format (`- [ ]`) to enable clear pass/fail validation of performance criteria
9. Cover different measurement categories (time, quantity, percentage)
10. Define boundaries about which performance aspects are in or out of scope

## Success Criteria

- **Specific timing requirements**: Uses precise measurements like `<= 2 seconds`, `within 500ms` instead of vague terms like `fast`
- **Measurable quantities**: Includes specific numbers for data limits, page sizes, or performance thresholds
- **Observable UI behavior**: Describes what users can see/interact with rather than internal system performance
- **Rule-oriented format**: Uses checkbox format for independent performance requirements rather than sequential scenarios
- **Negative performance cases**: Includes failure scenarios like slow loading, timeouts, or high data volumes
- **Avoids vague terms**: Does NOT use unmeasurable qualitative terms like `fast`, `responsive`, `good performance`
- **User outcome focus**: Focuses on user experience outcomes rather than technical implementation details
- **Checkbox validation format**: Uses checkbox format (`- [ ]`) to enable clear pass/fail validation of performance criteria
- **Multiple measurement types**: Covers different measurement categories (time, quantity, percentage, etc.)
- **Scope definition**: Includes boundaries about what performance aspects are in/out of scope

## Failure Conditions

- Uses vague terms like "fast", "responsive", or "good performance" without measurable thresholds
- Omits specific numbers for page size limits, result counts, or concurrency thresholds
- Describes internal system metrics (database query time, cache hit rates) instead of user-visible behavior
- Uses Given/When/Then scenarios instead of rule-oriented checkbox format for independent performance requirements
- Covers only normal-load scenarios without addressing slow loading, timeouts, or peak traffic
- Uses qualitative adjectives that cannot be objectively tested
- Focuses on how the system works rather than what the user experiences
- Uses prose descriptions instead of checkbox format, making pass/fail unclear
- Covers only one type of performance measurement (e.g., only timing)
- Omits boundaries, leaving unclear which performance aspects are in scope

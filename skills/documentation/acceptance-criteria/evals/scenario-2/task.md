# Analytics Dashboard Performance

## Problem Description

Your team is building an analytics dashboard for a growing fintech company that processes thousands of transactions daily. The current reporting system is painfully slow - executives often wait 30+ seconds for basic reports to load, and the system frequently times out during month-end reporting periods.

The business stakeholders have made it clear that dashboard performance is critical for decision-making. The C-suite uses these dashboards during board meetings, and slow loading times create awkward silences and reduce confidence in the data. The system needs to handle complex financial data while remaining responsive for daily operational use.

The development team has the data infrastructure and basic dashboard framework ready, but they need clear performance requirements before optimizing the front-end and queries. The current system serves about 50 concurrent users, with plans to scale to 200 users within 6 months.

## Output Specification

Create comprehensive acceptance criteria for dashboard performance requirements. Your document should include:

- User story with clear business impact
- Specific performance requirements and thresholds
- User experience expectations for different scenarios
- Edge cases for high-load situations

Save your work as `dashboard-performance-criteria.md`.

## Input Files

The following files are provided as context. Extract them before beginning.

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


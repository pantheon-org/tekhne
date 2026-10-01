# Scenario 06: Analytics Dashboard Performance

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

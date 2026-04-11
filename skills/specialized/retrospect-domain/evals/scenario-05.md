# Scenario 05: Cross-Referencing Learnings from Two Different Domains

## User Prompt

"Compare what I learned about database optimization this month versus what I learned about caching strategies — are there any overlapping insights?"

## Expected Behavior

1. Agent runs the session filter script with `--month` twice (or once with broad scope), separating sessions by domain relevance: database optimization vs caching strategies
2. Reads the relevant session files for each domain subset and applies the Technical Domain framework independently
3. Produces a domain learnings summary for each domain independently: What I Learned, What Worked, What Didn't Work, Decisions Made, Patterns Emerged
4. Performs a cross-domain comparison: identifies insights that appear in both domain summaries (overlapping patterns, shared trade-offs, complementary decisions)
5. Highlights divergences: decisions or approaches that contradict each other across domains
6. Produces a combined Start/Stop/Continue that acknowledges both domains
7. Writes a single unified insights report to `.retro/insights/domain/{PERIOD}.md` with clearly labelled per-domain sections and a cross-domain synthesis section
8. Reports the file path

## Success Criteria

- **Two domain subsets identified**: Report has clearly labelled sections for database optimization and caching strategies respectively
- **Independent analysis per domain**: Each domain section addresses the Technical Domain framework questions independently
- **Cross-domain comparison present**: Report includes a section explicitly comparing the two domains and identifying overlapping or contradictory insights
- **At least one overlap identified**: If evidence exists, at least one shared insight is surfaced (or agent explicitly states no overlap was found)
- **Combined SSC provided**: Start/Stop/Continue section acknowledges both domains
- **Single output file written**: One unified report at `.retro/insights/domain/{PERIOD}.md`, not two separate files
- **File path reported**: Agent outputs the path to the written file

## Failure Conditions

- Agent merges the two domains without independent analysis of each
- Cross-domain comparison section is absent
- Agent fabricates overlaps not supported by session evidence
- Agent writes two separate output files instead of one unified report
- Either domain section is empty or contains only placeholder text
- Agent applies the generic framework instead of the Technical Domain framework to technical sessions

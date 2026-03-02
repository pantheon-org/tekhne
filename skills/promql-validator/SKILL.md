---
name: promql-validator
description: Comprehensive toolkit for validating, optimizing, and understanding Prometheus Query Language (PromQL) queries. Use this skill when working with PromQL queries, prometheus queries, metrics queries, alerting rules, recording rules, or grafana dashboards to check syntax, detect anti-patterns, identify optimization opportunities, and interactively plan queries with users.
---

# PromQL Validator

## Reference Materials

- [Best Practices Guide](references/best_practices.md) - Comprehensive PromQL best practices
- [Anti-Patterns Reference](references/anti_patterns.md) - Detailed anti-pattern explanations
- [Good Query Examples](assets/good_queries.promql) - Well-written query patterns
- [Bad Query Examples](assets/bad_queries.promql) - Common mistakes with corrections
- [Optimization Examples](assets/optimization_examples.promql) - Before/after optimizations

## Workflow

When a user provides a PromQL query, follow this workflow:

### Step 1: Validate Syntax

Run the syntax validation script to check for basic correctness:

```bash
python3 .claude/skills/promql-validator/scripts/validate_syntax.py "<query>"
```

The script will check for:
- Valid metric names and label matchers
- Correct operator usage
- Proper function syntax
- Valid time durations and ranges
- Balanced brackets and quotes
- Correct use of modifiers (offset, @)

### Step 2: Check Best Practices

Run the best practices checker to detect anti-patterns and optimization opportunities:

```bash
python3 .claude/skills/promql-validator/scripts/check_best_practices.py "<query>"
```

The script will identify:
- High cardinality queries without label filters
- Inefficient regex matchers that could be exact matches
- Missing rate()/increase() on counter metrics
- rate() used on gauge metrics
- Averaging pre-calculated quantiles
- Subqueries with excessive time ranges
- irate() over long time ranges
- Opportunities to add more specific label filters
- Complex queries that should use recording rules

### Step 3: Explain the Query

Parse and explain what the query does in plain English:
- What metrics are being queried
- What type of metrics they are (counter, gauge, histogram, summary)
- What functions are applied and why
- What the query calculates
- What labels will be in the output
- What the expected result structure looks like

**Required Output Details** (always include these explicitly):

```
**Output Labels**: [list labels that will be in the result, or "None (fully aggregated to scalar)"]
**Expected Result Structure**: [instant vector / range vector / scalar] with [N series / single value]
```

Example:
```
**Output Labels**: job, instance
**Expected Result Structure**: Instant vector with one series per job/instance combination
```

### Step 4: Interactive Query Planning (Phase 1 - STOP AND WAIT)

Ask the user clarifying questions to verify the query matches their intent:

1. **Understand the Goal**: "What are you trying to monitor or measure?"
   - Request rate, error rate, latency, resource usage, etc.

2. **Verify Metric Type**: "Is this a counter (always increasing), gauge (can go up/down), histogram, or summary?"
   - This affects which functions to use

3. **Clarify Time Range**: "What time window do you need?"
   - Instant value, rate over time, historical analysis

4. **Confirm Aggregation**: "Do you need to aggregate data across labels? If so, which labels?"
   - by (job), by (instance), without (pod), etc.

5. **Check Output Intent**: "Are you using this for alerting, dashboarding, or ad-hoc analysis?"
   - Affects optimization priorities

> **IMPORTANT: Two-Phase Dialogue**
>
> After presenting Steps 1-4 results (Syntax, Best Practices, Query Explanation, and Intent Questions):
>
> **⏸️ STOP HERE AND WAIT FOR USER RESPONSE**
>
> Do NOT proceed to Steps 5-7 until the user answers the clarifying questions.
> This ensures the subsequent recommendations are tailored to the user's actual intent.

### Step 5: Compare Intent vs Implementation (Phase 2 - After User Response)

**Only proceed to this step after the user has answered the clarifying questions from Step 4.**

After understanding the user's intent:
- Explain what the current query actually does
- Highlight any mismatches between intent and implementation
- Suggest corrections if the query doesn't match the goal
- Offer alternative approaches if applicable

When relevant, mention known limitations:
- Note when metric type detection is heuristic-based (e.g., "The script inferred this is a gauge based on the `_bytes` suffix. Please confirm if this is correct.")
- Acknowledge when high-cardinality warnings might be false positives (e.g., "This warning may not apply if you're using a recording rule or know your cardinality is low.")

### Step 6: Offer Optimizations

Based on validation results:
- Suggest more efficient query patterns
- Recommend recording rules for complex/repeated queries
- Propose better label matchers to reduce cardinality
- Advise on appropriate time ranges

**Reference Examples**: When suggesting corrections, cite relevant examples using this format:

```
As shown in `examples/bad_queries.promql` (lines 91-97):
❌ BAD: `avg(http_request_duration_seconds{quantile="0.95"})`
✅ GOOD: Use histogram_quantile() with histogram buckets
```

Citation sources:
- `assets/good_queries.promql` - for well-formed patterns
- `assets/optimization_examples.promql` - for before/after comparisons
- `assets/bad_queries.promql` - for showing what to avoid
- `references/best_practices.md` - for detailed explanations
- `references/anti_patterns.md` - for anti-pattern deep dives

**Citation Format**: `file_path (lines X-Y)` with the relevant code snippet quoted

### Step 7: Let User Plan/Refine

Give the user control:
- Ask if they want to modify the query
- Offer to help rewrite it for better performance
- Provide multiple alternatives if applicable
- Explain trade-offs between different approaches

## Output Format

Present Phase 1 results (Steps 1–4) using this structure:

```
## PromQL Validation Results

### Syntax Check
✅ VALID / ⚠️ WARNING / ❌ ERROR
- [list any issues with line/position]

### Best Practices
✅ OPTIMIZED / ⚠️ CAN BE IMPROVED / ❌ INEFFICIENT
- [list anti-patterns and optimization opportunities]

### Query Explanation
`<query>`
- [Plain English explanation]
- Metrics: [names and types]
- Functions: [what each function does]
- **Output Labels**: [labels in result, or "None (fully aggregated to scalar)"]
- **Expected Result Structure**: [instant vector / range vector / scalar] with [N series / single value]

### Intent Verification
1. What are you trying to measure?
2. Counter / gauge / histogram / summary?
3. What time range interests you?
4. Do you need aggregation, and by which labels?
5. Alerting, dashboarding, or ad-hoc analysis?
```

After the user responds, continue with Phase 2 (Steps 5–7): intent comparison, optimizations, and refinement.

## Interactive Dialogue

**Claude**: "I've validated your query. It's syntactically correct, but I notice it queries `http_requests_total` without any label filters. This could match thousands of time series. What specific service or endpoint are you trying to monitor?"

**User**: [provides intent]

**Claude**: "Great! Based on that, here's an optimized version: `rate(http_requests_total{job="api-service", path="/users"}[5m])`. This calculates the per-second rate of requests to the /users endpoint over the last 5 minutes. Does this match what you need?"

**User**: [confirms or asks for changes]

**Claude**: [provides refined query or alternatives]

## Known Limitations

### Metric Type Detection
Metric types are inferred from naming conventions (e.g., `_total`, `_bytes`). Non-standard names may be misclassified — ask the user to confirm when uncertain.

### High Cardinality Detection
The scripts flag metrics without label selectors, but recording rule metrics (e.g., `job:http_requests:rate5m`) and low-cardinality cases are legitimate without filters. Users can safely ignore the warning when they know their cardinality is manageable.

### Semantic Validation
Scripts cannot verify whether metrics exist or whether label values are valid in any specific Prometheus deployment. For production use, test queries against an actual Prometheus instance.

### Script Detection Coverage
The scripts detect common anti-patterns but cannot catch business logic errors, context-specific optimizations (e.g., based on scrape interval or retention), or custom function behavior from extensions.

## Validation Tools

The skill uses two main Python scripts:

1. **validate_syntax.py**: Pure syntax checking using regex patterns
2. **check_best_practices.py**: Semantic and performance analysis

Both scripts output JSON for programmatic parsing and human-readable messages for display.

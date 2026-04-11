# Search Protocol

Multi-source search before investigating. 80% of bugs already solved online.

## Query Construction

```
[exact error message] [stack] [framework] [version]
```

Strip paths, keep error text. Add context keywords.

## Sources (in order)

| Source | Best for | Query tip |
|--------|----------|-----------|
| **Stack Overflow** | Common errors, how-to | `site:stackoverflow.com` |
| **GitHub Issues** | Library-specific bugs | `site:github.com [repo] issues` |
| **Official Docs** | Config, API usage | `site:[framework].dev` or `site:[docs-url]` |
| **Reddit** | Weird edge cases, opinions | `site:reddit.com r/[language]` |

## Workflow

1. **WebSearch** with constructed query
2. **Scan titles** for exact match to symptom
3. **WebFetch** top 2-3 promising results
4. **Extract** solution or keywords for next search

## No Results?

| Situation | Action |
|-----------|--------|
| Zero hits | Broaden query (remove version, simplify error) |
| Hits but wrong context | Add stack/framework to query |
| Outdated answers | Add year: `[query] 2024 2025` |
| Too generic | Add specific error code or function name |

## Solution Found?

If solution found → verify it applies to user's context → skip to Learn phase

If partial match → extract keywords → refine search → try again (max 3 iterations)

If nothing after 3 searches → move to Qualify phase

# Scenario 03: Article Summary Documentation Task

## User Prompt

Your architecture team is evaluating different approaches to implementing event-driven microservices. You've just finished reading an insightful article titled "Event Sourcing Patterns in Distributed Systems" from a well-known tech blog that covered several implementation strategies, common pitfalls, and performance considerations.

The article discussed CQRS implementation patterns, event store design choices, and how companies like Netflix and Spotify handle event ordering challenges. It also covered practical considerations around event versioning, replay mechanisms, and the trade-offs between different persistence strategies.

Your team maintains a shared knowledge repository where summaries of relevant technical articles are stored for future reference during architecture discussions and technology decisions.

Create an article summary that captures:

- The main concepts and insights from the article
- Key takeaways that are relevant to your team's work
- Any practical examples or case studies mentioned
- Your assessment of how this applies to current projects
- Reference information for the original source

The summary should be structured for easy scanning and reference during team discussions about event-driven architecture approaches.

The following content represents key points from the article you read. Extract this information before beginning.

```text
=============== FILE: inputs/article-notes.md ===============

**Source**: "Event Sourcing Patterns in Distributed Systems" by Jane Tech (https://example-tech-blog.com/event-sourcing-patterns)
**Date Read**: Current date
**Author**: Jane Tech, Senior Architect at TechCorp

## Key Points

- CQRS separates read/write models for better scalability
- Netflix uses event-first approach for user recommendation pipeline
- Event versioning strategies: upcast vs downcast patterns
- Snapshot frequency affects replay performance significantly
- Eventual consistency requires careful UX design considerations

## Technical Details

- Event stores: PostgreSQL vs specialized solutions like EventStore
- Ordering guarantees: per-stream vs global ordering trade-offs
- Schema evolution: backward compatibility strategies
```

## Expected Behavior

1. Show evidence of loading the `article-summary.yaml` template schema
2. Correctly identify the entry type as an article summary based on the URL/source context
3. Include the required `article` tag in frontmatter (not video/podcast/talk)
4. Create the file in the correct `YYYY/MM/` directory structure
5. Use a lowercase slug with hyphens, no uppercase letters or underscores in the filename
6. Ensure the date matches exactly in filename, frontmatter date field, and H1 title
7. Format the H1 using the correct `Topic - Month D, YYYY` format (not `Month DD`)
8. Include exactly one H1 heading
9. Ensure the frontmatter tags array matches the Tags section with pipe separators
10. Format all tags in lowercase-hyphenated style (e.g., `event-sourcing` not `EventSourcing`)
11. Include language specifiers on any code blocks, no bare backticks
12. Format the metadata block using `**Key:** Value` with proper bold formatting

## Success Criteria

- **Article-summary schema**: Shows evidence of loading article-summary.yaml template schema
- **Entry type identification**: Correctly identified as article summary type based on URL/source context
- **Article tag inclusion**: Includes required `article` tag in frontmatter (not video/podcast/talk)
- **Directory placement**: File created in correct YYYY/MM/ directory structure
- **Filename slug format**: Uses lowercase slug with hyphens, no uppercase letters or underscores
- **Triple date consistency**: Date matches exactly in filename, frontmatter date field, and H1 title
- **H1 format compliance**: H1 uses correct `Topic - Month D, YYYY` format (not `Month DD`)
- **Unique H1 heading**: Document has exactly one H1 heading, no multiples
- **Tag synchronization**: Frontmatter tags array matches Tags section with pipe separators
- **Hyphenated tags**: All tags use lowercase-hyphenated format (event-sourcing not EventSourcing)
- **Code block languages**: Any code blocks include language specifiers, no bare backticks
- **Metadata formatting**: Metadata block uses `**Key:** Value` format with proper bold formatting

## Failure Conditions

- No evidence of loading the article-summary.yaml template schema
- Entry type is misidentified (e.g., treated as a learning entry instead of article summary)
- `article` tag absent; a different tag type (video, podcast, talk) used instead
- File placed in wrong directory or missing the YYYY/MM/ structure
- Filename slug contains uppercase letters or underscores
- Date is inconsistent between filename, frontmatter, or H1 title
- H1 uses zero-padded day (`Month 01`) instead of `Month 1`
- Document contains more than one H1 heading
- Tags section does not match frontmatter array or uses wrong separator
- Any tag contains uppercase letters or camelCase (e.g., EventSourcing)
- Any code block is missing a language specifier
- Metadata block uses bullet points or unbolded keys

# Scenario 03: Write a Commit Message for a Performance Fix

## User Prompt

Your team has been investigating a slow endpoint in the search service. After profiling, an engineer discovered that the `/search` endpoint was executing an unbounded database query on every request, with no caching layer. Under sustained load the average response time was 480ms with a 99th percentile of 2.1 seconds.

The fix adds Redis caching with a 5-minute TTL and limits the maximum result set to 100 records. After the change, benchmarks show an average response time of 45ms and a 99th percentile of 190ms under the same load.

The change touches `services/search.ts`, `config/redis.ts`, and the query builder. The Redis connection pool is initialized lazily to avoid startup overhead.

Write a commit message for this change that will be used in the project's auto-generated CHANGELOG and will help future engineers understand why this optimization was made.

Create a file called `commit-message.txt` containing the complete commit message.

## Expected Behavior

1. Use the `perf` type to correctly classify this performance improvement
2. Include a body section explaining the motivation (not just a one-liner header)
3. Separate the header from the body with exactly one blank line
4. Write the body explaining the motivation (high latency, unbounded queries) rather than listing which files changed
5. Keep specific file names out of the header
6. Wrap every body line at 72 characters or fewer
7. Use imperative mood in the header description (e.g., `cache`, `add`, `reduce`)
8. Keep the header line at 72 characters or fewer
9. Start the description portion of the header with a lowercase letter

## Success Criteria

- **perf type used**: The commit type is `perf` (not `fix`, `refactor`, or `chore`)
- **Body present**: The commit message includes a body section (not just a one-liner header)
- **Blank line separates header/body**: There is exactly one blank line between the header and the body
- **Body explains why**: The body explains the motivation (e.g., high latency, unbounded queries) rather than listing which files were changed
- **No file names in header**: The header does NOT mention specific file names (e.g., `search.ts`, `redis.ts`)
- **Body lines <= 72 chars**: Every line in the body section is 72 characters or fewer
- **Imperative header**: The header description uses imperative mood (e.g., `cache`, `add`, `reduce`) not past tense
- **Header <= 72 chars**: The header line is 72 characters or fewer
- **Lowercase description**: The description portion of the header (after the colon and space) begins with a lowercase letter

## Failure Conditions

- Uses `fix`, `refactor`, or `chore` type instead of `perf` for a performance improvement
- Writes only a one-liner header without any body section
- Missing or multiple blank lines between the header and body
- Body lists file names or implementation details instead of explaining why the change was needed
- Header mentions specific file names like `search.ts` or `redis.ts`
- Any body line exceeds 72 characters
- Header description uses past tense (e.g., `cached`, `added`)
- Header line exceeds 72 characters
- Description portion begins with an uppercase letter

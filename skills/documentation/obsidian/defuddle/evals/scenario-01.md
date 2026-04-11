# Scenario 01: Summarize a Web Article

## User Prompt

"Can you read this article and give me a summary? https://www.theverge.com/2024/1/1/24020042/ai-year-in-review"

Read the page at that URL and produce a concise markdown summary of the article's key points.

Write your summary to a file called `summary.md` in the current working directory.

## Expected Behavior

1. Use `defuddle parse <url>` rather than WebFetch or curl to retrieve the page content
2. Include the `--md` flag to request markdown output from defuddle
3. Write a file named `summary.md` in the working directory containing a readable summary
4. Structure the summary with headings or bullet points and substantive content rather than a raw page dump

## Success Criteria

- **Used defuddle instead of WebFetch**: The agent invoked `defuddle parse <url>` rather than WebFetch or curl to retrieve the page content.
- **Used --md flag**: The defuddle command included the `--md` flag to request markdown output.
- **Produced summary.md**: A file named `summary.md` exists in the working directory containing a readable summary.
- **Summary contains meaningful content**: `summary.md` contains structured markdown (headings or bullet points) with substantive content rather than just the raw page dump.

## Failure Conditions

- Uses WebFetch or curl instead of `defuddle parse` to retrieve the page
- Runs defuddle without the `--md` flag
- Does not produce a `summary.md` file in the working directory
- `summary.md` contains only raw unstructured page content without any summarization

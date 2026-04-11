# Scenario 04: Code Review vs Discussion Separator

## User Prompt

Your team uses merge requests for both code review and general project discussions. Reviewers sometimes leave inline comments on specific lines of code, while other times they leave general comments in the MR discussion thread. The problem is that these get mixed together, making it hard to track which feedback is about specific code vs general architectural concerns.

The tech lead wants a tool that can separate these two types of feedback so that:
- Developers can focus on specific code-level comments when making changes
- Product owners can focus on high-level discussion comments
- The team can track how much code-level vs architectural feedback is being given

Create a Ruby script called `comment_classifier.rb` that:
1. Takes a GitLab merge request URL as input
2. Analyzes all the comments on that MR
3. Produces two output files:
   - `inline_comments.txt` - Comments that are about specific code lines
   - `discussion_comments.txt` - General discussion comments

Each file should list the comments with their authors, but exclude any automated bot messages.

```bash
ruby comment_classifier.rb "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Outputs: inline_comments.txt and discussion_comments.txt
```

The script should handle authentication and provide clear error messages if it can't access the merge request.

## Expected Behavior

1. Call `get_mr_comments.sh` in the solution to fetch comment data
2. Extract and use the `Type` field from comment output to distinguish between comment types
3. Treat `Type: DiffNote` as inline/code-level comments specifically
4. Treat `Type: comment` as general discussion comments specifically
5. Exclude comments where `System: true` to remove automated bot messages
6. Extract and use the `System` field from comment output
7. Extract author information from the `Author` field in output
8. Reference or handle the `GITLAB_TOKEN`/`GITLAB_PAT` environment variable
9. Include error handling with clear messages for access issues

## Success Criteria

- **Uses get_mr_comments.sh**: Calls `get_mr_comments.sh` in the solution
- **Checks Type field**: Extracts and uses the `Type` field from comment output to distinguish between comment types
- **Identifies DiffNote as inline**: Treats `Type: DiffNote` as inline/code-level comments specifically
- **Identifies comment as general**: Treats `Type: comment` as general discussion comments specifically
- **Filters System: true**: Excludes comments where `System: true` to remove automated bot messages
- **Checks System field**: Extracts and uses the `System` field from comment output
- **Parses Author field**: Extracts author information from the `Author` field in output
- **Authentication handling**: References or handles `GITLAB_TOKEN`/`GITLAB_PAT` environment variable
- **Error messages**: Includes error handling with clear messages for access issues

## Failure Conditions

- Script does not call `get_mr_comments.sh` for data retrieval
- `Type` field is not extracted or used to classify comments
- `DiffNote` type is not specifically treated as inline code comments
- `comment` type is not specifically treated as general discussion
- Automated bot messages (`System: true`) are not filtered out of both output files
- `System` field is not extracted or checked
- Author information is not extracted from the `Author` field
- `GITLAB_TOKEN`/`GITLAB_PAT` requirement is not referenced or handled
- Script fails silently without clear error messages for authentication or access issues

# Code Review vs Discussion Separator

## Problem Description

Your team uses merge requests for both code review and general project discussions. Reviewers sometimes leave inline comments on specific lines of code, while other times they leave general comments in the MR discussion thread. The problem is that these get mixed together, making it hard to track which feedback is about specific code vs general architectural concerns.

The tech lead wants a tool that can separate these two types of feedback so that:
- Developers can focus on specific code-level comments when making changes
- Product owners can focus on high-level discussion comments
- The team can track how much code-level vs architectural feedback is being given

## Output Specification

Create a Ruby script called `comment_classifier.rb` that:
1. Takes a GitLab merge request URL as input
2. Analyzes all the comments on that MR
3. Produces two output files:
   - `inline_comments.txt` - Comments that are about specific code lines
   - `discussion_comments.txt` - General discussion comments

Each file should list the comments with their authors, but exclude any automated bot messages.

## Example Usage

```bash
ruby comment_classifier.rb "https://gitlab.com/gitlab-org/gitlab/-/merge_requests/123456"
# Outputs: inline_comments.txt and discussion_comments.txt
```

The script should handle authentication and provide clear error messages if it can't access the merge request.

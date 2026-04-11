# Scenario 02: Learning Documentation Task

## User Prompt

Your team is adopting a new containerization strategy for the microservices platform. After spending the last two weeks researching Docker multi-stage builds, Kubernetes deployment patterns, and container security best practices, you've gained valuable insights that need to be captured for the team.

You've discovered several key concepts including how multi-stage builds can reduce image sizes by 70%, the importance of using non-root users for security, and specific patterns for health checks and resource limits in Kubernetes. You also learned about some common pitfalls like forgetting to use .dockerignore files and the security implications of using latest tags.

The engineering team wants to build a knowledge base of these learnings so other developers can benefit from your research and avoid common mistakes when implementing similar containerization approaches.

Create a learning documentation entry that captures:

- The key concepts and techniques discovered
- Practical examples or code snippets where relevant
- Important gotchas or pitfalls to avoid
- Resources or references that were particularly helpful
- Next steps for applying this knowledge

The documentation should be structured for easy reference and knowledge sharing across the development team.

## Expected Behavior

1. Show evidence of loading the `learning.yaml` template before generation
2. Demonstrate a systematic approach with gathering, schema loading, generation, and validation phases
3. Create the proper `YYYY/MM/` directory structure for the file location
4. Use `YYYY-MM-DD-slug.md` format with a lowercase-only slug and hyphens
5. Maintain exact date consistency across filename, frontmatter, and H1 header
6. Include the required `learning` tag in the frontmatter tags array
7. Format all tags in lowercase-hyphenated style (no underscores or camelCase)
8. Ensure the Tags section matches the frontmatter array exactly, using pipe-separated format
9. Include language identifiers on all code blocks (bash, yaml, etc.)
10. Show evidence of running the validation script on the generated file
11. Show evidence of prettier and markdownlint execution after validation

## Success Criteria

- **Schema consultation**: Shows evidence of loading learning.yaml template before generation
- **Four-phase workflow**: Demonstrates systematic approach with gathering, schema loading, generation, validation phases
- **File organization**: Creates proper YYYY/MM/ directory structure for the file location
- **Filename compliance**: Uses YYYY-MM-DD-slug.md format with lowercase-only slug and hyphens
- **Date synchronization**: Maintains exact date match across filename, frontmatter, and H1 header
- **Learning tag inclusion**: Includes required `learning` tag in frontmatter tags array
- **Tag format compliance**: Uses lowercase-hyphenated format for all tags (no underscores or camelCase)
- **Tag section consistency**: Tags section matches frontmatter array exactly with pipe-separated format
- **Code language specifiers**: All code blocks include language identifiers (bash, yaml, etc.)
- **Validation execution**: Shows evidence of running validation script on generated file
- **Formatting compliance**: Evidence of prettier and markdownlint execution after validation

## Failure Conditions

- No evidence of loading the learning.yaml template schema before generation
- Skips one or more of the four workflow phases (gather, schema, generate, validate)
- File placed in wrong directory or missing the YYYY/MM/ structure
- Filename does not follow YYYY-MM-DD-slug.md, or slug contains uppercase or underscores
- Date is inconsistent between filename, frontmatter, or H1 header
- `learning` tag is absent from the frontmatter tags array
- Any tag contains uppercase letters, underscores, or camelCase
- Tags section does not match frontmatter array or uses wrong format
- Any code block is missing a language identifier
- No evidence of running the validation script on the generated file
- No evidence of prettier or markdownlint execution

# Scenario 02: Restrict Agent to Documentation Writing with Minimal Permissions

## User Prompt

Create an OpenCode agent called "docs-writer" that only has access to read files and the "markdown-authoring" skill. It should NOT be able to run shell commands or access any other skills. Show the correct permission configuration.

## Expected Behavior

1. Deny all skills by default with `skill: { '*': 'deny' }` before whitelisting specific ones
2. Whitelist only `'markdown-authoring': 'allow'` after the wildcard deny
3. Block or restrict bash commands to prevent shell access (not `allow-all`)
4. Configure read access to allow file reading
5. Write a `description` with concrete trigger phrases for documentation tasks

## Success Criteria

- **Skill wildcard denied first**: Permission block includes `skill: { '*': 'deny' }` before whitelisting
- **Specific skill whitelisted**: After the deny, `'markdown-authoring': 'allow'` is added
- **Bash restricted or denied**: Bash commands are blocked or set to ask (not allow-all)
- **Read permission configured**: Read access is configured to allow file reading
- **Description has triggers**: The description includes concrete trigger phrases for documentation tasks

## Failure Conditions

- Whitelists `markdown-authoring` without first denying all other skills with `'*': 'deny'`
- Allows all bash commands by setting `bash: { '*': 'allow' }` instead of restricting shell access
- Omits read permission, preventing the agent from reading files it needs to document
- Writes a vague description with no user-facing trigger phrases
- Grants access to additional skills beyond `markdown-authoring`

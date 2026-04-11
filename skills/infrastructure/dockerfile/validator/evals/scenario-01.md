# Scenario 01: Hadolint Syntax Validation (Stage 1/4)

## User Prompt

You are given the following Dockerfile:

```dockerfile
FROM ubuntu
RUN apt-get update
RUN apt-get install -y curl git wget
RUN apt-get install -y python3
COPY . /app
WORKDIR /app
RUN pip3 install -r requirements.txt
CMD python3 app.py
```

Perform Stage 1 (Hadolint Syntax Validation) on this Dockerfile.

Run hadolint rules mentally and identify all violations. For each violation, report:
- The rule ID (e.g., DL3006)
- The line number where the violation occurs
- A brief description of why it is a violation
- The recommended fix

Order findings by severity (errors before warnings before info/style). Do not modify the Dockerfile — only report findings.

## Expected Behavior

1. Flag `FROM ubuntu` (no tag) as a violation of DL3006 (always tag the version of an image explicitly) at line 1
2. Flag the apt-get install commands as violating DL3008 (pin versions in apt-get install) for curl, git, wget, and python3
3. Identify that apt-get update and install are not followed by `rm -rf /var/lib/apt/lists/*` in the same RUN layer, violating DL3009 or DL3027
4. Note that splitting apt-get install across two RUN commands (lines 3 and 4) creates unnecessary layers and recommend combining them
5. List findings ordered by severity (errors before warnings before info/style) and not modify the Dockerfile

## Success Criteria

- **DL3006: untagged base image**: Agent flags `FROM ubuntu` (no tag) as a violation of DL3006 (always tag the version of an image explicitly) at line 1
- **DL3008: unpinned apt packages**: Agent flags the apt-get install commands as violating DL3008 (pin versions in apt-get install) for curl, git, wget, and python3
- **DL3009 or DL3027: apt-get cache not cleaned**: Agent identifies that apt-get update and install are not followed by `rm -rf /var/lib/apt/lists/*` in the same RUN layer (DL3009 or DL3027)
- **Multiple separate RUN commands flagged**: Agent notes that splitting apt-get install across two RUN commands (lines 3 and 4) creates unnecessary layers and recommends combining them
- **Severity ordering and read-only posture**: Agent lists findings ordered by severity (errors before warnings before info/style) and does not modify the Dockerfile

## Failure Conditions

- DL3006 violation for the untagged `FROM ubuntu` is not reported
- DL3008 violation for unpinned apt packages is not identified
- DL3009/DL3027 violation for missing apt cache cleanup is not flagged
- Split RUN commands are not identified as creating unnecessary layers
- Findings are not ordered by severity, or the agent attempts to modify the Dockerfile instead of reporting only

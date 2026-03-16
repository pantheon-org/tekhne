# Task: Hadolint Syntax Validation (Stage 1/4)

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

## Your Task

Perform Stage 1 (hadolint syntax validation) on this Dockerfile.

1. Identify every hadolint rule violation (DL-prefixed rules). For each violation state:
   - The rule ID (e.g., DL3006, DL3008)
   - The line number in the Dockerfile
   - A plain-English description of the violation
   - The severity level (error, warning, info, or style)

2. Identify any ShellCheck (SC-prefixed) violations in the shell commands used in RUN instructions.

3. List all violations in order of severity (errors first, then warnings, then info/style).

Do NOT modify the Dockerfile. Present findings only.

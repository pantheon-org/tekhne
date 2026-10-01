# Scenario 04: Hadolint Syntax Validation (Stage 1/4)

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

# Scenario 05: Full Validation Report with Severity Categories

## User Prompt

You are given the following Dockerfile for a production Java service:

```dockerfile
FROM openjdk:17

WORKDIR /app

ENV JAVA_OPTS="-Xmx512m"
ENV DB_PASSWORD=secret123

COPY target/app.jar /app/app.jar
COPY config/ /app/config/

EXPOSE 8080
EXPOSE 8443
EXPOSE 9090

HEALTHCHECK --interval=30s --timeout=10s CMD curl -f http://localhost:8080/health || exit 1

CMD ["sh", "-c", "java $JAVA_OPTS -jar /app/app.jar"]
```

Pre-validation check: no `.dockerignore` file exists in the build context.

Run the complete 4-stage validation workflow mentally and produce a full findings report.

1. **Stage 1 (hadolint):** List all DL-rule violations with line numbers.
2. **Stage 2 (Checkov):** List all CKV_DOCKER_* security findings.
3. **Stage 3 (Best Practices):** Check base image, USER directive, HEALTHCHECK, layer count, and .dockerignore.
4. **Stage 4 (Optimization):** Assess image size, multi-stage opportunity, and layer efficiency.

Then produce the final summary report with:
- A severity table: Critical | High | Medium | Low with finding counts
- Detailed findings list ordered Critical → Low, each with file/line, description, and recommended fix code block
- A confirmation that no files were modified and a question asking the user whether to apply the proposed fixes

## Expected Behavior

1. Identify `ENV DB_PASSWORD=secret123` as a Critical security finding and propose using BuildKit secrets or runtime environment injection instead
2. Flag the absence of a USER directive, classify it as High severity, and propose adding a non-root user before CMD
3. Flag that `openjdk:17` without a patch version tag is imprecise and recommend a pinned minimal variant
4. Flag the absence of `.dockerignore` and list recommended patterns to include (`.git`, `.env`, `*.log`, `target/` for Java)
5. Note that a pre-built JAR is copied in (`COPY target/app.jar`) and assess whether an additional build stage is relevant
6. Include a severity table showing finding counts per severity tier (Critical, High, Medium, Low)
7. End the report with a statement that no files were modified and a question asking the user whether to apply the proposed fixes

## Success Criteria

- **Hardcoded DB_PASSWORD identified as Critical**: Agent identifies `ENV DB_PASSWORD=secret123` as a Critical security finding and proposes using BuildKit secrets or runtime environment injection instead
- **Missing USER identified as High**: Agent flags the absence of a USER directive, classifies it as High severity, and proposes adding a non-root user before CMD
- **openjdk:17 untagged or deprecated base image flagged**: Agent flags that `openjdk:17` without a patch version tag (e.g., `17-jdk-slim`) is imprecise and recommends a pinned minimal variant
- **Missing .dockerignore flagged**: Agent flags the absence of `.dockerignore` and lists recommended patterns to include (node_modules, .git, .env, *.log, target/ for Java)
- **Multi-stage build opportunity identified**: Agent notes that a pre-built JAR is copied in (`COPY target/app.jar`) which is already a build artefact, and assesses whether an additional build stage is relevant
- **Severity table present**: Report includes a table showing finding counts per severity tier (Critical, High, Medium, Low)
- **User asked before applying fixes**: Report ends with a statement that no files were modified and a question asking the user whether to apply the proposed fixes

## Failure Conditions

- `ENV DB_PASSWORD=secret123` is not identified as a Critical security finding
- Missing USER directive is not flagged or is not classified as High severity
- `openjdk:17` without a patch version is not identified as an imprecise or risky base image tag
- Missing `.dockerignore` is not flagged
- Report does not include a severity table with counts per tier
- Report does not end with a confirmation that no files were modified and a question to the user before applying fixes

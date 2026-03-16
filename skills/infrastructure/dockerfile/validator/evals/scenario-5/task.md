# Task: Full Validation Report with Severity Categories

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

## Your Task

Run the complete 4-stage validation workflow mentally and produce a full findings report.

1. **Stage 1 (hadolint):** List all DL-rule violations with line numbers.
2. **Stage 2 (Checkov):** List all CKV_DOCKER_* security findings.
3. **Stage 3 (Best Practices):** Check base image, USER directive, HEALTHCHECK, layer count, and .dockerignore.
4. **Stage 4 (Optimization):** Assess image size, multi-stage opportunity, and layer efficiency.

Then produce the final summary report with:
- A severity table: Critical | High | Medium | Low with finding counts
- Detailed findings list ordered Critical → Low, each with file/line, description, and recommended fix code block
- A confirmation that no files were modified and a question asking the user whether to apply the proposed fixes

# Scenario 04: Add Health Checking and Production Hardening to a Java Service

## User Prompt

The platform team is onboarding a Spring Boot application (`payment-service`) onto a new Kubernetes cluster that uses liveness and readiness probes. The team's SRE discovered that the current Dockerfile has no health check instruction, so Kubernetes cannot verify the container is actually ready to serve traffic. Additionally, the container registry compliance scanner is blocking the image because the CMD uses shell-form syntax, which makes it impossible for the container runtime to forward signals correctly — causing slow shutdowns and goroutine leaks.

The team wants a production-hardened Dockerfile that includes a proper HEALTHCHECK, uses the correct CMD syntax for clean signal handling, explicitly documents the port the service listens on, and restricts filesystem access by running as a dedicated service account.

The Spring Boot application JAR is built as `target/payment-service.jar` by `mvn package -DskipTests`. It listens on port 8080 and exposes a `/actuator/health` endpoint. Use Java 21 and a JRE-only runtime image.

Produce a `Dockerfile` for the `payment-service` Spring Boot application.

Also produce a `.dockerignore` appropriate for a Maven Java project.

Place both files in the current directory.

## Expected Behavior

1. Include a `HEALTHCHECK` instruction (e.g., `HEALTHCHECK CMD curl --fail http://localhost:8080/actuator/health`)
2. Use exec-form (JSON array) syntax for `CMD` (e.g., `CMD ["java", "-jar", "/app/payment-service.jar"]`)
3. Include `EXPOSE 8080` (or appropriate port)
4. Create a non-root user/group and place the `USER` instruction before `CMD`/`ENTRYPOINT`
5. Use a JRE-only runtime image (`eclipse-temurin:*-jre`, `amazoncorretto`, or similar) — not a full JDK
6. Use at least two `FROM` instructions separating the build stage from the runtime stage
7. Pin all `FROM` base image tags — no `:latest`
8. Include a `.dockerignore` with at least one Maven-specific entry (`target/` or `*.class`)

## Success Criteria

- **HEALTHCHECK present**: Dockerfile contains a HEALTHCHECK instruction (e.g., HEALTHCHECK CMD curl --fail http://localhost:8080/actuator/health or equivalent)
- **Exec-form CMD**: CMD uses JSON array syntax (e.g., CMD ["java", "-jar", "/app/payment-service.jar"]) rather than shell string form
- **EXPOSE port documented**: Dockerfile contains EXPOSE 8080 (or appropriate port)
- **Non-root user**: A non-root user/group is created and USER instruction is placed before CMD/ENTRYPOINT
- **JRE-only runtime stage**: The final/runtime stage uses a JRE image (eclipse-temurin:*-jre, amazoncorretto, or similar JRE-only) rather than a full JDK image
- **Multi-stage build**: Dockerfile contains at least two FROM instructions separating the build stage from the runtime stage
- **Pinned base image tags**: All FROM instructions use specific version tags and NOT :latest
- **.dockerignore for Maven**: .dockerignore is present and contains at least one Maven-specific entry such as target/ or *.class

## Failure Conditions

- No `HEALTHCHECK` instruction is present
- `CMD` uses shell string form instead of JSON array syntax
- No `EXPOSE` instruction for port 8080
- No non-root user, or `USER` instruction is absent or placed after `CMD`
- Runtime stage uses a full JDK image instead of a JRE-only image
- Only one `FROM` instruction (no multi-stage build)
- Any `FROM` uses `:latest` or no tag
- `.dockerignore` is missing Maven-specific entries (`target/` or `*.class`)

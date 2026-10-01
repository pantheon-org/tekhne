# Scenario 07: Add Health Checking and Production Hardening to a Java Service

## User Prompt

The platform team is onboarding a Spring Boot application (`payment-service`) onto a new Kubernetes cluster that uses liveness and readiness probes. The team's SRE discovered that the current Dockerfile has no health check instruction, so Kubernetes cannot verify the container is actually ready to serve traffic. Additionally, the container registry compliance scanner is blocking the image because the CMD uses shell-form syntax, which makes it impossible for the container runtime to forward signals correctly — causing slow shutdowns and goroutine leaks.

The team wants a production-hardened Dockerfile that includes a proper HEALTHCHECK, uses the correct CMD syntax for clean signal handling, explicitly documents the port the service listens on, and restricts filesystem access by running as a dedicated service account.

The Spring Boot application JAR is built as `target/payment-service.jar` by `mvn package -DskipTests`. It listens on port 8080 and exposes a `/actuator/health` endpoint. Use Java 21 and a JRE-only runtime image.

Produce a `Dockerfile` for the `payment-service` Spring Boot application.

Also produce a `.dockerignore` appropriate for a Maven Java project.

Place both files in the current directory.

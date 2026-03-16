# Add Health Checking and Production Hardening to a Java Service

## Problem/Feature Description

The platform team is onboarding a Spring Boot application (`payment-service`) onto a new Kubernetes cluster that uses liveness and readiness probes. The team's SRE discovered that the current Dockerfile has no health check instruction, so Kubernetes cannot verify the container is actually ready to serve traffic. Additionally, the container registry compliance scanner is blocking the image because the CMD uses shell-form syntax, which makes it impossible for the container runtime to forward signals correctly — causing slow shutdowns and goroutine leaks.

The team wants a production-hardened Dockerfile that includes a proper HEALTHCHECK, uses the correct CMD syntax for clean signal handling, explicitly documents the port the service listens on, and restricts filesystem access by running as a dedicated service account.

The Spring Boot application JAR is built as `target/payment-service.jar` by `mvn package -DskipTests`. It listens on port 8080 and exposes a `/actuator/health` endpoint.

## Output Specification

Produce a `Dockerfile` for the `payment-service` Spring Boot application. Use Java 21 and a JRE-only runtime image.

Also produce a `.dockerignore` appropriate for a Maven Java project.

Place both files in the current directory.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: pom.xml ===============
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.acme</groupId>
    <artifactId>payment-service</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>3.2.3</version>
    </parent>

    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-actuator</artifactId>
        </dependency>
    </dependencies>
</project>

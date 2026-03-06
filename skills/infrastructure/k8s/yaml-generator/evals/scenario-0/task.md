# Microservice Deployment Manifest

## Problem Description

Your team is deploying a new payment processing microservice to production. The application runs on port 8080, requires specific resource allocation due to processing load, and needs health monitoring to ensure high availability. The security team requires all production workloads to follow least-privilege principles.

Create a production-ready Kubernetes Deployment manifest that the platform team can use for the initial rollout. The manifest must follow industry best practices for resource management, health monitoring, and security.

## Output Specification

Generate a file named `payment-service-deployment.yaml` containing a complete Deployment manifest with the following requirements:

- Application: payment-processor
- Image: payment-app:2.1.0
- Port: 8080
- Production namespace: payments
- 3 replicas
- Health endpoint: /health (for both liveness and readiness)
- Include appropriate metadata labels

The manifest should be ready for production deployment without requiring modifications for basic security or reliability concerns.

# Complete Application Stack

## Problem Description

Your team needs to deploy a new web application to Kubernetes. The application requires a Deployment for the workload, a Service to expose it internally, and a ConfigMap to store non-sensitive configuration. The infrastructure team has requested manifests that follow a consistent structure for easier management.

Create a complete set of Kubernetes manifests for this application stack. The application is a Node.js web server running on port 3000 that needs configuration for database connection strings and feature flags.

## Output Specification

Generate a single multi-document YAML file named `webapp-stack.yaml` containing three resources:
1. ConfigMap with sample configuration (database URL and feature flags)
2. Deployment for the web application (3 replicas)
3. Service to expose the application (ClusterIP type)

Application details:
- Name: webapp
- Namespace: apps
- Image: webapp:1.0.0
- Port: 3000
- Health endpoint: /healthz

Ensure all resources use consistent labeling that connects them as part of the same application stack.

# GitOps Application Configuration

## Problem Description

Your platform team is adopting ArgoCD for GitOps-based deployments. You need to create an ArgoCD Application resource that will automatically sync a microservice from a Git repository to the production cluster. The application manifests are stored in a GitHub repository and need automatic synchronization with pruning enabled.

The DevOps team needs this Application resource to follow ArgoCD best practices for automated deployments with self-healing capabilities.

## Output Specification

Generate a file named `microservice-app.yaml` containing an ArgoCD Application CRD with the following requirements:

- Application name: user-service
- Target namespace: production
- Source Git repo: <https://github.com/company/user-service>
- Path in repo: k8s/manifests
- Target revision: main
- Destination cluster: in-cluster (<https://kubernetes.default.svc>)
- Enable automated sync with pruning and self-healing

The manifest should use the correct ArgoCD API group and version, and include all required fields for the Application CRD.

Create a second file named `generation-notes.md` that documents:
- The API version and kind used
- Why this specific configuration is appropriate for production
- Any required fields that were included

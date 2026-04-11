# Scenario 04: GitOps Application Configuration with ArgoCD CRD

## User Prompt

Your platform team is adopting ArgoCD for GitOps-based deployments. You need to create an ArgoCD Application resource that will automatically sync a microservice from a Git repository to the production cluster. The application manifests are stored in a GitHub repository and need automatic synchronization with pruning enabled.

The DevOps team needs this Application resource to follow ArgoCD best practices for automated deployments with self-healing capabilities.

Generate a file named `microservice-app.yaml` containing an ArgoCD Application CRD with the following requirements:

- Application name: user-service
- Target namespace: production
- Source Git repo: https://github.com/company/user-service
- Path in repo: k8s/manifests
- Target revision: main
- Destination cluster: in-cluster (https://kubernetes.default.svc)
- Enable automated sync with pruning and self-healing

The manifest should use the correct ArgoCD API group and version, and include all required fields for the Application CRD.

Create a second file named `generation-notes.md` that documents:
- The API version and kind used
- Why this specific configuration is appropriate for production
- Any required fields that were included

## Expected Behavior

1. Use `apiVersion: argoproj.io/v1alpha1`
2. Use `kind: Application`
3. Include `spec.project` field (typically `default`)
4. Include `spec.source` with `repoURL`, `path`, and `targetRevision` fields
5. Include `spec.destination` with `server` and `namespace` fields
6. Include `spec.syncPolicy.automated` section
7. Set `prune: true` in `syncPolicy.automated`
8. Set `selfHeal: true` in `syncPolicy.automated`
9. Place the Application resource in the `argocd` namespace (`metadata.namespace`)
10. Include `generation-notes.md` documenting the API version, required fields, or CRD specifics

## Success Criteria

- **Correct API group**: Uses `apiVersion: argoproj.io/v1alpha1`
- **Correct kind**: Uses `kind: Application`
- **Project field present**: Includes `spec.project` field (typically `default`)
- **Source configuration**: Includes `spec.source` with `repoURL`, `path`, and `targetRevision` fields
- **Destination configuration**: Includes `spec.destination` with `server` and `namespace` fields
- **Automated sync**: Includes `spec.syncPolicy.automated` section
- **Prune enabled**: Sets `prune: true` in `syncPolicy.automated`
- **Self-heal enabled**: Sets `selfHeal: true` in `syncPolicy.automated`
- **ArgoCD namespace**: Application resource placed in `argocd` namespace (`metadata.namespace`)
- **Documentation included**: `generation-notes.md` documents the API version, required fields, or CRD specifics

## Failure Conditions

- Wrong `apiVersion` is used (not `argoproj.io/v1alpha1`)
- `kind: Application` is not used
- `spec.project` is absent
- `spec.source` is missing `repoURL`, `path`, or `targetRevision`
- `spec.destination` is missing `server` or `namespace`
- `spec.syncPolicy.automated` is absent
- `prune: true` or `selfHeal: true` is not set in the sync policy
- Application is placed in the wrong namespace (not `argocd`)

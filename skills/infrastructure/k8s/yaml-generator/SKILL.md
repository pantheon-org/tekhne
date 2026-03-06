---
name: k8s-yaml-generator
description: Comprehensive toolkit for generating, validating, and managing Kubernetes YAML resources. Use this skill when creating Kubernetes manifests (Deployments, Services, ConfigMaps, StatefulSets, etc.), working with Custom Resource Definitions (CRDs), or generating production-ready K8s configurations.
---

# K8s Generator

## Core Workflow

### 1. Understand Requirements

Gather: resource type, target K8s version, app requirements (replicas, ports, volumes), namespace/labels/annotations, and CRD specifics (kind, apiVersion, version).

### 2. Fetch CRD Documentation (if needed)

Query library documentation for CRD specifications:
```
tessl_query_library_docs: query: "<project-name> <CRD-kind> <version> specification"
# e.g. "argo-cd Application v1alpha1 specification"
# e.g. "istio VirtualService v1beta1 specification"
# e.g. "cert-manager Certificate v1 specification"
```

The query should include:
- Project/operator name (e.g., "argo-cd", "istio", "cert-manager")
- CRD kind (e.g., "Application", "Certificate", "VirtualService")
- API version (e.g., "v1alpha1", "v1", "v1beta1")
- Context about what you need (e.g., "spec fields", "examples", "configuration")

If documentation is insufficient, fall back to web search:
```
WebSearch: "<CRD-name> <version> spec documentation"
# e.g. "ArgoCD Application v1alpha1 spec documentation"
```
Always include the version in the query for compatibility.

### 3. Generate YAML Resource

**Recommended labels** (use consistently across all resources):
```yaml
labels:
  app.kubernetes.io/name: myapp
  app.kubernetes.io/instance: myapp-prod
  app.kubernetes.io/version: "1.0.0"
  app.kubernetes.io/component: backend
  app.kubernetes.io/part-of: myplatform
  app.kubernetes.io/managed-by: claude-code
```

**Best practices checklist:**
- Explicit, non-deprecated API versions
- Resource requests/limits on all containers
- Liveness and readiness probes
- Non-root `securityContext`; Pod Security Standards
- Secrets in Secret objects (not ConfigMaps)
- Namespace specified on all namespace-scoped resources
- For CRDs: include all required fields; add comments on complex configs

**Deployment template:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  namespace: default
  labels:
    app.kubernetes.io/name: myapp
    app.kubernetes.io/instance: myapp-prod
spec:
  replicas: 3
  selector:
    matchLabels:
      app.kubernetes.io/name: myapp
      app.kubernetes.io/instance: myapp-prod
  template:
    metadata:
      labels:
        app.kubernetes.io/name: myapp
        app.kubernetes.io/instance: myapp-prod
    spec:
      containers:
      - name: myapp
        image: myapp:1.0.0
        ports:
        - containerPort: 8080
        resources:
          requests: { memory: "64Mi", cpu: "250m" }
          limits:   { memory: "128Mi", cpu: "500m" }
        livenessProbe:
          httpGet: { path: /health, port: 8080 }
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet: { path: /ready, port: 8080 }
          initialDelaySeconds: 5
          periodSeconds: 5
```

**Service template:**
```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
  namespace: default
spec:
  type: ClusterIP  # or LoadBalancer, NodePort
  selector:
    app.kubernetes.io/name: myapp
    app.kubernetes.io/instance: myapp-prod
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
    name: http
```

**ConfigMap template:**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: myapp-config
  namespace: default
data:
  app.properties: |
    key1=value1
  config.json: |
    { "setting": "value" }
```

### 4. Validate Generated YAML

**CRITICAL: Always validate using the k8s-yaml-validator workflow immediately after generation.**

```
See: yaml-validator/SKILL.md (in this tile)
```

The validator runs `yamllint` (syntax), `kubeconform` (schema/API compliance), best-practice checks, and optional cluster dry-run. Address all reported errors—fix, re-validate, repeat until clean.

### 5. Deliver the Resource

Present the validated YAML with a brief summary, key configuration choices, and next steps:

```bash
kubectl apply -f <filename>.yaml
kubectl get <resource-type> <name> -n <namespace>
kubectl describe <resource-type> <name> -n <namespace>
```

## Common CRD Examples

### ArgoCD Application

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: myapp
  namespace: argocd
  labels:
    app.kubernetes.io/managed-by: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/org/repo
    targetRevision: HEAD
    path: manifests
  destination:
    server: https://kubernetes.default.svc
    namespace: myapp
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

### Istio VirtualService

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: myapp
spec:
  hosts:
  - myapp.example.com
  gateways:
  - myapp-gateway
  http:
  - route:
    - destination:
        host: myapp-service
        port:
          number: 8080
```

### Cert-Manager Certificate

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: myapp-tls
  namespace: default
spec:
  secretName: myapp-tls-secret
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsNames:
  - myapp.example.com
```

## Advanced Features

### Multi-Resource Generation

1. Generate each resource following the core workflow
2. Use consistent labels across all resources
3. Respect dependency order (e.g., ConfigMaps before Deployments)
4. Validate each resource individually using the k8s-yaml-validator workflow
5. Combine into a single multi-document YAML with `---` separators if desired

### Version-Specific Generation

- Use appropriate API versions for the target K8s version (check deprecations)
- Example: Ingress moved from `extensions/v1beta1` to `networking.k8s.io/v1` in K8s 1.19+

## Troubleshooting

| Issue | Solution |
|---|---|
| CRD docs not found | Try name variations; fall back to WebSearch with version-specific query |
| Validation failures | Read errors carefully; verify field names/types/required fields; re-validate |
| Wrong API version | Confirm target K8s version; check deprecation status; update `apiVersion`; re-validate |

## Integration

- **k8s-yaml-validator** (in this tile) — automatic validation of generated resources
- **k8s-debug** — troubleshooting deployed resources
- **helm-validator** — validating Helm charts using these resources

---

**Workflow summary:** Understand → Fetch CRD Docs (if needed) → Generate → Validate → Deliver

---
name: k8s-yaml-generator
description: Comprehensive toolkit for generating, validating, and managing Kubernetes YAML resources. Use this skill when creating Kubernetes manifests (Deployments, Services, ConfigMaps, StatefulSets, etc.), working with Custom Resource Definitions (CRDs), or generating production-ready K8s configurations.
---

# K8s YAML Generator

## Generation Mindset

**Mental Model**: Kubernetes YAML generation is about translating application requirements into declarative infrastructure. Think in terms of desired state, not imperative commands.

**Decision Framework**:
1. **Start with the workload** (Pod, Deployment, StatefulSet)—this defines what runs
2. **Add access layer** (Service, Ingress)—this defines how it's accessed
3. **Configure runtime** (ConfigMap, Secret)—this defines how it behaves
4. **Set constraints** (ResourceQuota, LimitRange, NetworkPolicy)—this defines boundaries

**When to use this skill**:
- Creating new Kubernetes resources from scratch
- Converting Docker Compose or other formats to K8s manifests
- Generating CRD instances (ArgoCD, Istio, Cert-Manager, etc.)
- Building templates for Helm charts or Kustomize bases
- Scaffolding multi-resource applications

**Generation philosophy**: Generate correct, complete, validated YAML on first pass. Never output YAML that hasn't been validated.

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

## Common Anti-Patterns

### NEVER: Generate Without Resource Limits

**BAD**:
```yaml
containers:
- name: app
  image: myapp:1.0
  # No resources defined - pods can consume unlimited CPU/memory
```

**GOOD**:
```yaml
containers:
- name: app
  image: myapp:1.0
  resources:
    requests: { memory: "64Mi", cpu: "250m" }
    limits:   { memory: "128Mi", cpu: "500m" }
```

**Why**: Unlimited resources lead to noisy neighbor issues and OOMKilled pods.

### NEVER: Hardcode Secrets in ConfigMaps

**BAD**:
```yaml
kind: ConfigMap
data:
  DATABASE_PASSWORD: "my-secret-password"  # Visible in etcd
```

**GOOD**:
```yaml
kind: Secret
type: Opaque
stringData:
  DATABASE_PASSWORD: "my-secret-password"  # Base64 encoded, separate RBAC
```

### NEVER: Use `latest` Tag in Production

**BAD**:
```yaml
containers:
- name: app
  image: myapp:latest  # Non-deterministic, breaks rollbacks
```

**GOOD**:
```yaml
containers:
- name: app
  image: myapp:1.2.3  # Immutable, traceable
  imagePullPolicy: IfNotPresent
```

### NEVER: Skip Health Probes

**BAD**:
```yaml
containers:
- name: app
  image: myapp:1.0
  # No probes - K8s can't detect failures
```

**GOOD**:
```yaml
containers:
- name: app
  image: myapp:1.0
  livenessProbe:
    httpGet: { path: /health, port: 8080 }
    initialDelaySeconds: 30
  readinessProbe:
    httpGet: { path: /ready, port: 8080 }
    initialDelaySeconds: 5
```

### NEVER: Mismatch Selector Labels

**BAD**:
```yaml
# Deployment
selector:
  matchLabels: { app: myapp }
# Service
selector: { app: my-app }  # Typo breaks routing
```

**GOOD**:
```yaml
# Deployment
selector:
  matchLabels: { app: myapp, version: v1 }
# Service
selector: { app: myapp }  # Matches all versions
```

### ALWAYS: Validate Before Delivery

Never output YAML without running it through the validation workflow:

```bash
# Use yaml-validator from this tile
tessl_query_library_docs: query: "kubernetes yaml validation kubeconform yamllint"
```

### ALWAYS: Use Namespaces Explicitly

```yaml
metadata:
  name: myapp
  namespace: production  # Never rely on default namespace
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

## Verification Checklist

Before delivering generated YAML, confirm:

- [ ] All required fields are present (metadata.name, metadata.namespace, spec)
- [ ] API version is correct for target K8s version
- [ ] Resource requests/limits are defined on all containers
- [ ] Health probes (liveness/readiness) are configured
- [ ] Selectors match labels exactly (no typos)
- [ ] Secrets are in Secret objects, not ConfigMaps
- [ ] Image tags are specific versions, not `latest`
- [ ] SecurityContext is set (runAsNonRoot, read-only filesystem where appropriate)
- [ ] Validation passes with zero errors using k8s-yaml-validator

## Troubleshooting

| Issue | Solution |
|---|---|
| CRD docs not found | Try name variations; fall back to WebSearch with version-specific query |
| Validation failures | Read errors carefully; verify field names/types/required fields; re-validate |
| Wrong API version | Confirm target K8s version; check deprecation status; update `apiVersion`; re-validate |

## Integration

- **k8s-yaml-validator** (in this tile) — automatic validation of generated resources
- **k8s-debug** (in this tile) — troubleshooting deployed resources
- **helm-generator** — generating Helm charts using these resources as templates

---

**Workflow summary:** Understand → Fetch CRD Docs (if needed) → Generate → Validate → Deliver

## Anti-Patterns

### NEVER omit `resources:` limits and requests from container specs

- **WHY**: Containers without resource constraints are evicted under node pressure, cannot be autoscaled predictably, and fail Kubernetes best-practice audits (Polaris, kube-score).
- **BAD**: A container spec with no `resources:` block.
- **GOOD**: Set `requests` for scheduling and `limits` for protection: `resources: {requests: {cpu: 100m, memory: 128Mi}, limits: {cpu: 500m, memory: 512Mi}}`

### NEVER use `latest` as the image tag in Kubernetes manifests

- **WHY**: `imagePullPolicy: Always` with `:latest` makes every pod start non-deterministic; `imagePullPolicy: IfNotPresent` with `:latest` uses a stale local image silently.
- **BAD**: `image: myapp:latest`
- **GOOD**: `image: myapp:v1.2.3` with a specific, immutable tag and `imagePullPolicy: IfNotPresent`

### NEVER expose sensitive configuration as plain environment variables

- **WHY**: Config mounted from a literal `env` value appears in pod descriptions, process listings, and logs; it bypasses Kubernetes RBAC for Secret objects.
- **BAD**: `env: [{name: DB_PASSWORD, value: "mysecret"}]`
- **GOOD**: `env: [{name: DB_PASSWORD, valueFrom: {secretKeyRef: {name: db-secret, key: password}}}]`

### NEVER create Deployments without liveness and readiness probes

- **WHY**: Without probes, Kubernetes cannot detect unhealthy pods and continues routing traffic to them even when the application is broken or deadlocked.
- **BAD**: A Deployment spec with no `livenessProbe` or `readinessProbe`.
- **GOOD**: Add `readinessProbe` to control traffic routing and `livenessProbe` to trigger pod restarts on deadlocks.

### NEVER inline secret values in CI pipeline commands using heredocs

- **WHY**: Heredoc content containing environment variable expansions appears in CI logs and shell history, leaking secrets.
- **BAD**: `kubectl apply -f - <<EOF\nenv:\n  value: $SECRET\nEOF`
- **GOOD**: Use Kubernetes Secrets, Sealed Secrets, or external-secrets-operator; reference secrets by name rather than inlining values in pipeline commands.

## References

For detailed examples and templates:
- Standard resource templates: See "Deployment template", "Service template", "ConfigMap template" sections above
- CRD examples: See "Common CRD Examples" section (ArgoCD, Istio, Cert-Manager)
- Validation: See `yaml-validator/SKILL.md` in this tile
- Debugging: See `debug/SKILL.md` in this tile for troubleshooting deployed resources

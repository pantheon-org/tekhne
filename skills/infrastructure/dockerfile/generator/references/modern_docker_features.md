# Modern Docker Features (2025)

## Multi-Platform Builds with BuildX

**Use Case:** Build images that work on both AMD64 and ARM64 architectures (e.g., x86 servers and Apple Silicon Macs).

**Enable BuildX:**
```bash
# BuildX is included in Docker Desktop by default
# For Linux, ensure BuildX is installed
docker buildx version
```

**Create Multi-Platform Images:**
```bash
# Build for multiple platforms
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t myapp:latest \
  --push \
  .

# Build and load for current platform (testing)
docker buildx build \
  --platform linux/amd64 \
  -t myapp:latest \
  --load \
  .
```

**Dockerfile Considerations:**
```dockerfile
# Most Dockerfiles work across platforms automatically
# Use platform-specific base images when needed
FROM --platform=$BUILDPLATFORM node:20-alpine AS builder

# Access build arguments for platform info
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN echo "Building on $BUILDPLATFORM for $TARGETPLATFORM"
```

**When to Use:**
- Deploying to mixed infrastructure (x86 + ARM)
- Supporting Apple Silicon Macs in development
- Optimizing for AWS Graviton (ARM-based) instances
- Building cross-platform CLI tools

## Software Bill of Materials (SBOM)

**Use Case:** Generate SBOM for supply chain security and compliance (increasingly required in 2025).

**Generate SBOM During Build:**
```bash
# Generate SBOM with BuildKit (Docker 24.0+)
docker buildx build \
  --sbom=true \
  -t myapp:latest \
  .

# SBOM is attached as attestation to the image
# View SBOM
docker buildx imagetools inspect myapp:latest --format "{{ json .SBOM }}"
```

**Generate SBOM from Existing Image:**
```bash
# Using Syft
syft myapp:latest -o json > sbom.json

# Using Docker Scout
docker scout sbom myapp:latest
```

**SBOM Benefits:** Vulnerability tracking, license compliance, dependency transparency, and audit trail for security reviews.

**Integration with CI/CD:**
```yaml
# GitHub Actions example
- name: Build with SBOM
  run: |
    docker buildx build \
      --sbom=true \
      --provenance=true \
      -t myapp:latest \
      --push \
      .
```

## BuildKit Cache Mounts (Advanced)

**Use Case:** Persist package manager caches across builds for faster iteration.

**Already covered in detail in `optimization_patterns.md`.**

**Quick reference:**
```dockerfile
# syntax=docker/dockerfile:1

# NPM cache mount
RUN --mount=type=cache,target=/root/.npm \
    npm ci

# Go module cache
RUN --mount=type=cache,target=/go/pkg/mod \
    go mod download

# Pip cache
RUN --mount=type=cache,target=/root/.cache/pip \
    pip install -r requirements.txt
```

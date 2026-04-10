# Scenario 04: Multi-Stage Build Optimisation (Stage 4/4)

## User Prompt

You are given the following single-stage Dockerfile for a Go application:

```dockerfile
FROM golang:1.21

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go build -o server ./cmd/server

RUN apt-get update && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["/app/server"]
```

Perform Stage 4 (Optimization Analysis) on this Dockerfile.

1. Identify that this is a compiled language (Go) and that a multi-stage build would eliminate the Go toolchain from the final image.

2. Propose a multi-stage build using:
   - A named `build` stage (`AS build`) with `golang:1.21` for compilation
   - A minimal final stage using `gcr.io/distroless/base-debian12` or `alpine:3.21`
   - Only the compiled binary and required certificates copied to the final stage

3. Estimate the image size reduction that would result from moving to a distroless or Alpine final stage (provide a rough estimate and explain why).

4. Explain the CI debugging benefit of named build stages (the `--target build` flag).

5. Note any security improvements from the distroless approach.

Provide the complete optimised Dockerfile as a code block.

## Expected Behavior

1. Correctly identify that Go produces a statically compiled binary and the build toolchain (`golang:1.21` ~900MB) does not need to be in the final image
2. Propose a Dockerfile using `FROM golang:1.21 AS build` and a separate final FROM stage with `COPY --from=build`
3. Use `gcr.io/distroless/base-debian12`, `distroless/static`, or `alpine:3.x` as the minimal final base image rather than the full golang image
4. Provide a reasonable estimate of image size reduction (e.g., from ~900MB to ~20-30MB for distroless static) and explain why the toolchain layers are dropped
5. Explain that named stages allow `docker build --target build` to access the intermediate build stage for CI debugging without exposing the final runtime image
6. Note that distroless images have no shell, package manager, or unnecessary system libraries, reducing the attack surface

## Success Criteria

- **Multi-stage build opportunity identified**: Agent correctly identifies that Go produces a statically compiled binary and the build toolchain (`golang:1.21` ~900MB) does not need to be in the final image
- **Named AS build stage used**: Proposed Dockerfile uses `FROM golang:1.21 AS build` and then a separate final FROM stage with `COPY --from=build`
- **Minimal final base image chosen**: Final stage uses `gcr.io/distroless/base-debian12`, `distroless/static`, or `alpine:3.x` rather than the full golang image
- **Image size reduction estimate provided**: Agent provides a reasonable estimate of image size reduction (e.g., from ~900MB to ~20-30MB for distroless static) and explains why the toolchain layers are dropped
- **--target build debugging benefit explained**: Agent explains that named stages allow `docker build --target build` to access the intermediate build stage for CI debugging without exposing the final runtime image
- **Security improvements noted**: Agent notes that distroless images have no shell, package manager, or unnecessary system libraries, reducing the attack surface

## Failure Conditions

- Multi-stage build opportunity is not identified for the Go application
- Proposed Dockerfile does not use a named build stage (`AS build`) or does not use `COPY --from=build`
- Final stage uses the full `golang` image instead of a minimal distroless or Alpine image
- No image size reduction estimate is provided
- The `--target build` CI debugging benefit is not explained
- Security improvements from distroless are not mentioned

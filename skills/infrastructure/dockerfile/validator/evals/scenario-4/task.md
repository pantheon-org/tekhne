# Task: Multi-Stage Build Optimisation (Stage 4/4)

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

## Your Task

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

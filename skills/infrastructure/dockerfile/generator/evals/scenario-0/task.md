# Containerize a Go Microservice for Production

## Problem/Feature Description

Your team has built a small Go REST API called `inventory-service` that exposes an HTTP server on port 8080. The service is currently deployed by copying the binary to VMs, but the team wants to migrate to Kubernetes and needs a production-quality container image. The existing CI pipeline builds and tests the Go binary, but no Docker infrastructure exists yet.

The security team has flagged that all container images must run as unprivileged users and must use minimal base images to reduce the attack surface. The platform team requires that image tags be deterministic and reproducible — they've had incidents in the past where builds produced different images on different days due to floating base image tags.

## Output Specification

Produce a `Dockerfile` that containerizes the Go application. The application entry point is `cmd/server/main.go`, the module name in `go.mod` is `github.com/acme/inventory-service`, and the binary should be built with `go build -o /app/server ./cmd/server`. The service starts and accepts traffic when it responds to a GET request at `/healthz`.

Also produce a `.dockerignore` file appropriate for a Go project.

Place both files in the current directory.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: go.mod ===============
module github.com/acme/inventory-service

go 1.22

require (
	github.com/go-chi/chi/v5 v5.0.12
	github.com/jmoiron/sqlx v1.3.5
)
=============== FILE: cmd/server/main.go ===============
package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "ok")
	})
	http.HandleFunc("/items", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, `[]`)
	})
	http.ListenAndServe(":8080", nil)
}

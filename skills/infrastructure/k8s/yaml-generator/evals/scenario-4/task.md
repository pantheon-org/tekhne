# Scenario 4: Converts a docker-compose file without using removed API versions

## User Prompt

"Convert this docker-compose.yml to Kubernetes manifests. Our cluster is on Kubernetes 1.27."

## Repository State

The compose file defines two services. `web` uses image `web:3.1.0`, listens on port 80 and mounts `./nginx.conf` as its configuration. `cache` uses image `redis:7.2.4` and listens on 6379. Only `web` must be reachable from outside, at `shop.example.com`.

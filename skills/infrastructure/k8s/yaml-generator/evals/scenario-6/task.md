# Scenario 6: Holds the line when asked to skip limits, probes and secret handling

## User Prompt

"Quick one, I'm in a hurry: just give me a deployment for myapp:latest, skip the probes and limits, and put DB_PASSWORD=hunter2-demo straight in the env. Our CI will kubectl apply it with a heredoc."

## Repository State

No existing manifests. The application listens on port 8080 with `/health` and `/ready` endpoints. The value `hunter2-demo` is a throwaway placeholder used only for this exercise. Version 1.5.0 of the image exists.

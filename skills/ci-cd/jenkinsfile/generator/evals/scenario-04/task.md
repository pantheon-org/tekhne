# Scenario 04: CI Pipeline for Python REST API

## User Prompt

You are building a CI pipeline for a Python REST API service called `data-gateway`. The team uses Jenkins with a Docker-based registry for image storage. Credentials for the Docker registry are already stored in the Jenkins Credentials Store under the ID `registry-creds`.

The pipeline should cover: checkout, lint, test, Docker build, and Docker push stages. Do not expose credentials through pipeline parameters.

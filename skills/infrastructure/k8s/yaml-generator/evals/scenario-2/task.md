# Scenario 2: Keeps secrets out of ConfigMaps and plain environment variables

## User Prompt

"Add the config for the orders API: it needs a database password and a feature-flags file."

## Repository State

A Deployment named `orders-api` already exists in the `shop` namespace with the labels `app.kubernetes.io/name: orders-api` and `app.kubernetes.io/instance: orders-api-prod`. The database password for this exercise is the placeholder `changeme-placeholder`. The feature flags are `new-checkout=true` and `legacy-export=false`.

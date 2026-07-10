# Task: Save a Step-by-Step Deployment Process

You are an AI coding assistant. The developer explains their deployment process for production:

> "Every time we deploy, we need to: (1) run `bun run build`, (2) tag the Docker image with the git SHA, (3) push to ECR, (4) update the ECS task definition, then (5) wait for the service to stabilise before closing the ticket."

The developer says: "Save this deployment process — I want you to remember it."

Write the shell command(s) you would run to save this process.

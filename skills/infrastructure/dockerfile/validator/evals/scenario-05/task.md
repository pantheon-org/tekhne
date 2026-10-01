# Scenario 05: Security Scan (Stage 2/4)

## User Prompt

You are given the following Dockerfile for a payment processing service:

```dockerfile
FROM node:18

WORKDIR /app

ENV NODE_ENV=production
ENV API_KEY=sk_live_abcdef1234567890
ENV DATABASE_URL=postgres://admin:P@ssw0rd!@db.internal:5432/payments

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3000
EXPOSE 22

CMD ["node", "server.js"]
```

Perform Stage 2 (Security Scan) on this Dockerfile using Checkov-equivalent analysis.

Identify all security findings, classify each by severity (Critical / High / Medium / Low), and provide a recommended fix or alternative for each finding. Do not modify the Dockerfile.

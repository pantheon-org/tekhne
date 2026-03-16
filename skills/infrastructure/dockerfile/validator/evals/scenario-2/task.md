# Task: Security Scan (Stage 2/4)

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

## Your Task

Perform Stage 2 (Checkov security scan) analysis on this Dockerfile.

1. Identify all Checkov (CKV_DOCKER_*) security violations. For each violation state:
   - The check ID
   - What is wrong
   - Why it is a security risk
   - The recommended fix

2. Specifically address:
   - Hardcoded secrets in ENV instructions
   - Missing USER directive
   - HEALTHCHECK absence
   - Exposed port 22 risk

3. For the hardcoded secrets, describe the correct alternative using BuildKit secret mounts.

4. Categorise all findings as Critical, High, Medium, or Low severity.

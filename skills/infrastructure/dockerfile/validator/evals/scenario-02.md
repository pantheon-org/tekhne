# Scenario 02: Security Scan (Stage 2/4)

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

## Expected Behavior

1. Identify `ENV API_KEY` and `ENV DATABASE_URL` as hardcoded secrets, classify them as Critical severity, and explain they will persist in all image layers
2. Describe using `--mount=type=secret` in a RUN instruction or runtime environment injection as the correct alternative to ENV for secrets
3. Identify the absence of a USER instruction as a High severity finding (container runs as root), and propose adding a non-root user
4. Flag `EXPOSE 22` as a security risk (SSH port exposure) and recommend removing it unless SSH access is explicitly required
5. Identify the absence of a HEALTHCHECK directive as a finding and propose an appropriate healthcheck for a Node.js HTTP service
6. Correctly assign Critical to hardcoded secrets, High to missing USER and EXPOSE 22, and Medium or lower to missing HEALTHCHECK

## Success Criteria

- **Hardcoded secrets identified as Critical**: Agent identifies `ENV API_KEY` and `ENV DATABASE_URL` as hardcoded secrets, classifies them as Critical severity, and explains they will persist in all image layers
- **BuildKit secret mount alternative described**: Agent describes using `--mount=type=secret` in a RUN instruction or runtime environment injection as the correct alternative to ENV for secrets
- **Missing USER directive identified**: Agent identifies the absence of a USER instruction as a High severity finding (container runs as root), and proposes adding a non-root user
- **EXPOSE 22 flagged**: Agent flags `EXPOSE 22` as a security risk (SSH port exposure) and recommends removing it unless SSH access is explicitly required
- **Missing HEALTHCHECK flagged**: Agent identifies the absence of a HEALTHCHECK directive as a finding and proposes an appropriate healthcheck for a Node.js HTTP service
- **Severity categorisation correct**: Agent correctly assigns Critical to hardcoded secrets, High to missing USER and EXPOSE 22, and Medium or lower to missing HEALTHCHECK

## Failure Conditions

- Hardcoded secrets in ENV instructions are not flagged or are not classified as Critical
- No alternative to ENV for secrets is described (BuildKit secret mount or runtime injection not mentioned)
- Missing USER instruction is not identified as a security finding
- EXPOSE 22 is not flagged as a risk
- Missing HEALTHCHECK is not mentioned
- Severity classification is incorrect (e.g., missing USER classified as Low instead of High)

# Speed Up Docker Builds for a Node.js API

## Problem/Feature Description

A backend team maintains a Node.js Express API (`order-service`) that is deployed via Docker. During a recent sprint retrospective, engineers complained that Docker builds take 3-4 minutes on every small code change — even when no dependencies have changed. The tech lead suspects the build cache is not being used effectively, since each rebuild reinstalls all npm packages from scratch.

The team needs the Dockerfile restructured so that `npm ci` (the dependency install step) is only re-executed when `package.json` or `package-lock.json` actually changes. Code-only changes should reuse the cached dependency layer and complete in under 30 seconds.

## Output Specification

Produce a new `Dockerfile` for the `order-service` Node.js application. The application listens on port 3000, the start command is `node src/index.js`, and Node.js 20 should be used.

Also produce an appropriate `.dockerignore` for a Node.js project.

Both files should be placed in the current directory.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: package.json ===============
{
  "name": "order-service",
  "version": "1.0.0",
  "description": "Order management REST API",
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js",
    "test": "jest"
  },
  "dependencies": {
    "express": "^4.18.2",
    "pg": "^8.11.3",
    "dotenv": "^16.3.1"
  },
  "devDependencies": {
    "jest": "^29.7.0"
  }
}
=============== FILE: src/index.js ===============
const express = require('express');
const app = express();

app.get('/health', (req, res) => res.json({ status: 'ok' }));
app.get('/orders', (req, res) => res.json([]));

app.listen(3000, () => console.log('order-service listening on port 3000'));

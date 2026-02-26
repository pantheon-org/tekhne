# Create AGENTS.md for pnpm Monorepo

<!-- markdownlint-disable MD032 -->
## Problem/Feature Description

Your team has set up a new monorepo using pnpm workspaces for a React application with a shared component library. The previous developer started creating AGENTS.md but left incomplete and possibly incorrect commands.

The tech lead has asked you to complete the AGENTS.md documentation, ensuring all commands are correct and validated. This is critical because broken commands in documentation have caused deployment issues in the past.

## Output Specification

Create/finish `AGENTS.md` with validated commands. The documentation should include:

- How to install dependencies
- How to run the application in development
- How to build packages
- How to run tests
- How to lint

IMPORTANT: Verify each command works correctly before including it. The tech lead will test every command by copy-pasting.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: package.json ===============
{
  "name": "pnpm-monorepo",
  "private": true,
  "packageManager": "pnpm@8.6.0",
  "scripts": {
    "build": "pnpm -r run build",
    "dev": "pnpm -r run dev",
    "test": "pnpm -r run test",
    "lint": "pnpm -r run lint",
    "format": "prettier --write .",
    "typecheck": "pnpm -r run typecheck"
  },
  "devDependencies": {
    "prettier": "^3.0.0",
    "typescript": "^5.1.0"
  }
}

=============== FILE: pnpm-workspace.yaml ===============
packages:
- 'packages/*'

=============== FILE: packages/web/package.json ===============
{
  "name": "@company/web",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint src --ext ts,tsx",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "@company/ui": "workspace:*",
    "@company/utils": "workspace:*",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.0.0",
    "typescript": "^5.1.0",
    "vite": "^4.4.0",
    "vitest": "^0.33.0",
    "eslint": "^8.45.0",
    "@typescript-eslint/parser": "^5.60.0",
    "@typescript-eslint/eslint-plugin": "^5.60.0"
  }
}

=============== FILE: packages/ui/package.json ===============
{
  "name": "@company/ui",
  "version": "1.0.0",
  "main": "dist/index.js",
  "module": "dist/index.mjs",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc && vite build",
    "test": "vitest",
    "lint": "eslint src --ext ts,tsx",
    "typecheck": "tsc --noEmit"
  },
  "peerDependencies": {
    "react": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "typescript": "^5.1.0",
    "vite": "^4.4.0",
    "vitest": "^0.33.0"
  }
}

=============== FILE: packages/utils/package.json ===============
{
  "name": "@company/utils",
  "version": "1.0.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "test": "vitest",
    "lint": "eslint src --ext ts",
    "typecheck": "tsc --noEmit"
  },
  "devDependencies": {
    "typescript": "^5.1.0",
    "vitest": "^0.33.0"
  }
}

=============== FILE: packages/web/vite.config.ts ===============
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
  },
});

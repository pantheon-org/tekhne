# Update AGENTS.md After Repository Changes

<!-- markdownlint-disable MD022 MD025 -->
## Problem/Feature Description

Six months ago, your team created an AGENTS.md file for the project. Since then, significant changes have been made:

- Migrated from Jest to Vitest for testing
- Added ESLint flat config
- Removed Storybook
- Added a new `lint:fix` script

The current AGENTS.md is now outdated and contains commands that no longer work. You need to update the documentation to reflect the current state of the repository.

## Output Specification

Update the existing `AGENTS.md` to:

- Replace Jest commands with Vitest equivalents
- Update linting commands for ESLint flat config
- Remove Storybook-related instructions
- Add the new `lint:fix` script
- Ensure all commands are still valid

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: AGENTS.md ===============
# Agent Documentation

## Project Overview

This is a React TypeScript application with a component library.

## Commands

### Install dependencies

```bash
npm install
```

### Run tests

```bash
npm test
```

### Run tests in watch mode

```bash
npm test -- --watch
```

### Lint code

```bash
npm run lint
```

### Build

```bash
npm run build
```

### Start Storybook

```bash
npm run storybook
```

### Build Storybook

```bash
npm run build-storybook
```

## Tech Stack

- React 18
- TypeScript
- Jest
- Storybook
- ESLint

=============== FILE: package.json ===============
{
  "name": "react-app",
  "version": "2.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage",
    "lint": "eslint src --ext ts,tsx",
    "lint:fix": "eslint src --ext ts,tsx --fix",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
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
    "@vitest/coverage-v8": "^0.33.0",
    "eslint": "^8.45.0",
    "@typescript-eslint/parser": "^5.60.0",
    "@typescript-eslint/eslint-plugin": "^5.60.0"
  }
}

=============== FILE: eslint.config.js ===============
import js from '@eslint/js';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsparser from '@typescript-eslint/parser';

export default [
  js.configs.recommended,
  {
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
    },
    rules: {
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      'no-console': 'warn',
    },
  },
];

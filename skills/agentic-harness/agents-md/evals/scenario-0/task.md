# Create AGENTS.md for Simple Node.js Project

<!-- markdownlint-disable MD033 MD050 -->
## Problem/Feature Description

Your team has been working on a new internal API service that handles user authentication and profile management. The service is a straightforward Node.js application with Express, TypeScript, and PostgreSQL. Several team members have been struggling to understand how to run tests, build the project, and follow coding conventions when making contributions.

The engineering lead has asked you to create an AGENTS.md file in the repository root to help AI assistants and new developers understand how to work with this codebase. This documentation should be practical and enable someone to start contributing quickly.

## Output Specification

Create an `AGENTS.md` file in the repository root that provides:

- How to set up the development environment
- How to run tests
- How to build the project
- Coding conventions and patterns used
- Any useful commands for development

The documentation should be concise but complete enough for someone to start contributing without asking questions.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: package.json ===============
{
  "name": "user-auth-service",
  "version": "1.0.0",
  "description": "Internal user authentication and profile management API",
  "main": "dist/index.js",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node src/index.ts",
    "test": "jest",
    "test:watch": "jest --watch",
    "lint": "eslint src --ext .ts",
    "lint:fix": "eslint src --ext .ts --fix"
  },
  "dependencies": {
    "express": "^4.18.2",
    "pg": "^8.11.0",
    "bcrypt": "^5.1.0",
    "jsonwebtoken": "^9.0.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/express": "^4.17.17",
    "@types/pg": "^8.10.0",
    "@types/bcrypt": "^5.0.0",
    "@types/jsonwebtoken": "^9.0.0",
    "@types/jest": "^29.5.0",
    "typescript": "^5.0.0",
    "ts-node": "^10.9.0",
    "jest": "^29.5.0",
    "ts-jest": "^29.1.0",
    "eslint": "^8.40.0",
    "@typescript-eslint/parser": "^5.59.0",
    "@typescript-eslint/eslint-plugin": "^5.59.0"
  }
}

=============== FILE: tsconfig.json ===============
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}

=============== FILE: jest.config.js ===============
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: ['<rootDir>/src'],
  testMatch: ['**/__tests__/**/*.ts', '**/?(*.)+(spec|test).ts'],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
  collectCoverageFrom: [
    'src/**/*.ts',
    '!src/**/*.d.ts',
    '!src/**/__tests__/**'
  ],
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov', 'html']
};

=============== FILE: .eslintrc.js ===============
module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  env: {
    node: true,
    es6: true,
  },
  rules: {
    '@typescript-eslint/explicit-function-return-type': 'warn',
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    'no-console': 'warn',
  },
};

# Create Hierarchical AGENTS.md for Monorepo

<!-- markdownlint-disable MD034 -->
## Problem/Feature Description

Your company maintains a monorepo containing multiple packages: a React UI component library, a shared utilities library, and a backend API service. The engineering team has grown frustrated with AI assistants generating incorrect code because they don't understand the project structure, and developers waste time figuring out which commands work in which package.

The tech lead wants you to set up a proper AGENTS.md documentation system. This should include a root AGENTS.md that provides an overview and shared conventions, plus package-specific AGENTS.md files in each subdirectory that contain locally relevant instructions.

## Output Specification

Create a hierarchical AGENTS.md structure:

- Root `AGENTS.md` with monorepo overview, shared conventions, and navigation to sub-packages
- `packages/ui/AGENTS.md` with UI library-specific instructions
- `packages/utils/AGENTS.md` with utilities library-specific instructions
- `packages/api/AGENTS.md` with API service-specific instructions

Ensure:

- Root file is concise (not a full manual)
- No duplication of instructions between files
- Each package file contains locally relevant commands
- Clear references/links between files

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: package.json ===============
{
  "name": "company-monorepo",
  "private": true,
  "workspaces": [
    "packages/*"
  ],
  "scripts": {
    "build": "turbo run build",
    "dev": "turbo run dev",
    "test": "turbo run test",
    "lint": "turbo run lint",
    "clean": "turbo run clean"
  },
  "devDependencies": {
    "turbo": "^1.10.0",
    "eslint": "^8.40.0",
    "typescript": "^5.0.0"
  }
}

=============== FILE: turbo.json ===============
{
  "$schema": "https://turbo.build/schema.json",
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", "build/**"]
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": ["coverage/**"]
    },
    "lint": {
      "outputs": []
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "clean": {
      "cache": false
    }
  }
}

=============== FILE: packages/ui/package.json ===============
{
  "name": "@company/ui",
  "version": "1.0.0",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "lint": "eslint src --ext .ts,.tsx",
    "storybook": "storybook dev -p 6006",
    "build-storybook": "storybook build"
  },
  "peerDependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "typescript": "^5.0.0",
    "jest": "^29.5.0",
    "@testing-library/react": "^14.0.0",
    "storybook": "^7.0.0",
    "@storybook/react": "^7.0.0",
    "@storybook/react-vite": "^7.0.0"
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
    "test": "jest",
    "lint": "eslint src --ext .ts"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "jest": "^29.5.0",
    "@types/jest": "^29.5.0"
  }
}

=============== FILE: packages/api/package.json ===============
{
  "name": "@company/api",
  "version": "1.0.0",
  "main": "dist/index.js",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node-dev --respawn src/index.ts",
    "test": "jest",
    "lint": "eslint src --ext .ts"
  },
  "dependencies": {
    "express": "^4.18.2",
    "pg": "^8.11.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "ts-node": "^10.9.0",
    "ts-node-dev": "^2.0.0",
    "jest": "^29.5.0",
    "@types/jest": "^29.5.0",
    "@types/express": "^4.17.17",
    "@types/pg": "^8.10.0"
  }
}

=============== FILE: packages/ui/tsconfig.json ===============
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "jsx": "react-jsx",
    "strict": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "bundler",
    "declaration": true,
    "declarationMap": true,
    "outDir": "./dist"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}

# Scenario 5: Move an existing Nx React app from Webpack to Vite

## User Prompt

The Nx app `storefront` (`apps/storefront`) is a React app built with Webpack. It has a few non-public files (`README.md` and `LICENSE`) that must end up in the build output, and a `src/environments/environment.prod.ts` file that replaces `environment.ts` for production builds. Write the step-by-step migration plan with the commands and the resulting `vite.config.ts`, and say how you will confirm the migration worked.

## Repo state

- Nx workspace, Node.js 20, Bun as the package manager.
- `storefront` currently has Webpack-based `build` and `serve` targets and a Jest `test` target.

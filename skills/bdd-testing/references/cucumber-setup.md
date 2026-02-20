# Cucumber.js Setup and Installation

## Installation

```bash
# Install Cucumber.js
bun add -D @cucumber/cucumber

# Install TypeScript support
bun add -D typescript ts-node @types/node
```

## Project Structure

```
features/
  lobby/
    create-lobby.feature
    join-lobby.feature
  game/
    player-movement.feature
    game-completion.feature
  support/
    world.ts           # Custom World class
    steps.ts           # Step definitions
    hooks.ts           # Before/After hooks
cucumber.js            # Cucumber configuration
```

## TypeScript Configuration

**cucumber.js**

```javascript
module.exports = {
  default: {
    require: ["features/support/**/*.ts"],
    requireModule: ["ts-node/register"],
    format: ["progress-bar", "html:reports/cucumber-report.html"],
    formatOptions: { snippetInterface: "async-await" },
  },
};
```

## Running Cucumber

```bash
# Run all features
npx cucumber-js

# Run specific feature file
npx cucumber-js features/lobby/create-lobby.feature

# Run specific scenario by line number
npx cucumber-js features/lobby/create-lobby.feature:10

# Run with tags
npx cucumber-js --tags "@mvp and not @wip"

# Dry run (validate syntax without running)
npx cucumber-js --dry-run

# Generate step definition snippets for undefined steps
npx cucumber-js --format snippets
```

## package.json Scripts

```json
{
  "scripts": {
    "test": "cucumber-js",
    "test:ci": "cucumber-js --profile ci",
    "test:smoke": "cucumber-js --tags @smoke",
    "test:wip": "cucumber-js --tags @wip"
  }
}
```

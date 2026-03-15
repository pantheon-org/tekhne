// Step definition files use `function` syntax for Cucumber's `this` binding.
// These files live outside cli/lib/ and are exempt from CLI TypeScript conventions.
import { Given, Then, When } from "@cucumber/cucumber";
import { runCli } from "./run-cli";
import type { CliWorld } from "./world";

Given("I am at the repository root", function (this: CliWorld) {
  // No-op: cucumber.js is at the repo root so process.cwd() is already correct.
});

When(
  "I run the CLI command {string}",
  function (this: CliWorld, command: string) {
    const args = command.split(/\s+/);
    this.lastResult = runCli(args, this.repoRoot);
  },
);

Then(
  "the exit code should be {int}",
  function (this: CliWorld, expectedCode: number) {
    if (!this.lastResult) throw new Error("No CLI command has been run yet");
    const actual = this.lastResult.exitCode;
    if (actual !== expectedCode) {
      throw new Error(
        `Expected exit code ${expectedCode} but got ${actual}\n` +
          `stdout: ${this.lastResult.stdout}\n` +
          `stderr: ${this.lastResult.stderr}`,
      );
    }
  },
);

Then(
  "the output should contain {string}",
  function (this: CliWorld, expected: string) {
    if (!this.lastResult) throw new Error("No CLI command has been run yet");
    const combined = this.lastResult.stdout + this.lastResult.stderr;
    if (!combined.toLowerCase().includes(expected.toLowerCase())) {
      throw new Error(
        `Expected output to contain "${expected}"\nActual output:\n${combined}`,
      );
    }
  },
);

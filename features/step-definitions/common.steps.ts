// Step definition files use `function` syntax for Cucumber's `this` binding.
import { Given, Then, When } from "@cucumber/cucumber";
import {
  runCatalogDryRun,
  runValidatorHelp,
  runValidatorStructure,
} from "./run";
import type { CliWorld } from "./world";

Given("I am at the repository root", function (this: CliWorld) {
  // No-op: cucumber.js is at the repo root so process.cwd() is already correct.
});

When(
  "I run the skill catalog update in dry-run mode",
  function (this: CliWorld) {
    this.lastResult = runCatalogDryRun(this.repoRoot);
  },
);

When("I show the skill validator help", function (this: CliWorld) {
  this.lastResult = runValidatorHelp(this.repoRoot);
});

When("I validate the skill {string}", function (this: CliWorld, skill: string) {
  this.lastResult = runValidatorStructure(this.repoRoot, skill);
});

Then(
  "the exit code should be {int}",
  function (this: CliWorld, expectedCode: number) {
    if (!this.lastResult) throw new Error("No command has been run yet");
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
    if (!this.lastResult) throw new Error("No command has been run yet");
    const combined = this.lastResult.stdout + this.lastResult.stderr;
    if (!combined.toLowerCase().includes(expected.toLowerCase())) {
      throw new Error(
        `Expected output to contain "${expected}"\nActual output:\n${combined}`,
      );
    }
  },
);

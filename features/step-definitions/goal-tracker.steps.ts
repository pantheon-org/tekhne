// Step definition files use `function` syntax for Cucumber's `this` binding.

import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { After, Given, Then, When } from "@cucumber/cucumber";
import { runGoalScript } from "./run";
import type { CliWorld } from "./world";

type GoalWorld = CliWorld & { goalRoot?: string };

const slug = (title: string): string =>
  title
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

const goalRoot = (world: GoalWorld): string => {
  if (!world.goalRoot) {
    world.goalRoot = mkdtempSync(join(tmpdir(), "goal-tracker-"));
    mkdirSync(join(world.goalRoot, ".context", "goals"), { recursive: true });
  }
  return world.goalRoot;
};

After(function (this: GoalWorld) {
  if (this.goalRoot) rmSync(this.goalRoot, { recursive: true, force: true });
});

// Write an active goal whose frontmatter carries `session: <sessionValue>`
// verbatim, so a scenario can supply a quoted YAML value.
const writeGoal = (
  world: GoalWorld,
  title: string,
  sessionValue: string,
): void => {
  const file = join(
    goalRoot(world),
    ".context",
    "goals",
    `2026-09-30-${slug(title)}.md`,
  );
  writeFileSync(
    file,
    [
      "---",
      `title: ${title}`,
      "type: goal",
      "date: 2026-09-30",
      "status: active",
      "goal-status: new",
      `session: ${sessionValue}`,
      "tags: []",
      "---",
      "",
      `# ${title}`,
      "",
      "## Done looks like",
      "",
      "The work is merged with its tests passing.",
      "",
      "## Items",
      "",
      "| # | Item | State | Reach | Evidence |",
      "| - | ---- | ----- | ----- | -------- |",
      "| 1 | Do the work | todo | local | |",
      "",
      "## Log",
      "",
      "- 2026-09-30 created, goal-status new",
      "",
    ].join("\n"),
  );
};

Given(
  "a goal {string} is active for session {string}",
  function (this: GoalWorld, title: string, session: string) {
    writeGoal(this, title, session);
  },
);

Given(
  "a goal {string} has its session written as {string}",
  function (this: GoalWorld, title: string, sessionValue: string) {
    writeGoal(this, title, sessionValue);
  },
);

When(
  "I run the goal status for session {string}",
  function (this: GoalWorld, session: string) {
    this.lastResult = runGoalScript(this.repoRoot, [
      "status",
      "--root",
      goalRoot(this),
      "--session",
      session,
    ]);
  },
);

When(
  "I run the goal check for session {string}",
  function (this: GoalWorld, session: string) {
    this.lastResult = runGoalScript(this.repoRoot, [
      "check",
      "--root",
      goalRoot(this),
      "--session",
      session,
    ]);
  },
);

When("I run the goal status without a session", function (this: GoalWorld) {
  this.lastResult = runGoalScript(this.repoRoot, [
    "status",
    "--root",
    goalRoot(this),
  ]);
});

When(
  "I create the goal {string} for session {string}",
  function (this: GoalWorld, title: string, session: string) {
    this.lastResult = runGoalScript(this.repoRoot, [
      "new",
      title,
      "--root",
      goalRoot(this),
      "--session",
      session,
    ]);
  },
);

Then(
  "the output should not contain {string}",
  function (this: GoalWorld, unexpected: string) {
    if (!this.lastResult) throw new Error("No command has been run yet");
    const combined = this.lastResult.stdout + this.lastResult.stderr;
    if (combined.includes(unexpected)) {
      throw new Error(
        `Expected output not to contain "${unexpected}"\nActual output:\n${combined}`,
      );
    }
  },
);

Then(
  "the new goal file should contain {string}",
  function (this: GoalWorld, expected: string) {
    if (!this.lastResult) throw new Error("No command has been run yet");
    const path = this.lastResult.stdout.trim().split("\n").pop() ?? "";
    const text = readFileSync(path, "utf-8");
    if (!text.includes(expected))
      throw new Error(
        `Expected ${path} to contain "${expected}"\nActual:\n${text}`,
      );
  },
);

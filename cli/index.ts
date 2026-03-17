#!/usr/bin/env bun

import { Command } from "@cliffy/command";
import { auditCommand } from "./commands/audit";
import { installCommand } from "./commands/install";
import { readmeCommand } from "./commands/readme";
import { syncCommand } from "./commands/sync";
import { tesslCommand } from "./commands/tessl";
import { uninstallCommand } from "./commands/uninstall";
import { validateCommand } from "./commands/validate";

await new Command()
  .name("tekhne")
  .description("Tekhne CLI - Skill management and quality tooling")
  .version("1.0.0")
  .command("audit", auditCommand)
  .command("tessl", tesslCommand)
  .command("install", installCommand)
  .command("uninstall", uninstallCommand)
  .command("readme", readmeCommand)
  .command("sync", syncCommand)
  .command("validate", validateCommand)
  .parse(process.argv.slice(2));

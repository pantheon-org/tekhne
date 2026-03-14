#!/usr/bin/env bun

import { Command } from "commander";
import { auditCommand } from "./commands/audit";
import { installCommand } from "./commands/install";
import { readmeCommand } from "./commands/readme";
import { syncCommand } from "./commands/sync";
import { tesslCommand } from "./commands/tessl";
import { validateCommand } from "./commands/validate";

const program = new Command();

program
  .name("tekhne")
  .description("Tekhne CLI - Skill management and quality tooling")
  .version("1.0.0");

program.addCommand(auditCommand);
program.addCommand(tesslCommand);
program.addCommand(installCommand);
program.addCommand(readmeCommand);
program.addCommand(syncCommand);
program.addCommand(validateCommand);

program.parse();

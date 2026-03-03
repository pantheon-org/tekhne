#!/usr/bin/env bun

import { Command } from "commander";
import { auditCommand } from "./commands/audit";
import { tesslCommand } from "./commands/tessl";

const program = new Command();

program
  .name("tekhne")
  .description("Tekhne CLI - Skill management and quality tooling")
  .version("1.0.0");

program.addCommand(auditCommand);
program.addCommand(tesslCommand);

program.parse();

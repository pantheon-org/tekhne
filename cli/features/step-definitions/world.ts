import {
  type IWorldOptions,
  setWorldConstructor,
  World,
} from "@cucumber/cucumber";
import type { CliResult } from "./run-cli";

export class CliWorld extends World {
  lastResult: CliResult | null = null;
  repoRoot: string;

  constructor(options: IWorldOptions) {
    super(options);
    this.lastResult = null;
    this.repoRoot = process.cwd();
  }
}

setWorldConstructor(CliWorld);

import {
  type IWorldOptions,
  setWorldConstructor,
  World,
} from "@cucumber/cucumber";

export interface CommandResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

export class CliWorld extends World {
  lastResult: CommandResult | null = null;
  repoRoot: string;

  constructor(options: IWorldOptions) {
    super(options);
    this.lastResult = null;
    this.repoRoot = process.cwd();
  }
}

setWorldConstructor(CliWorld);

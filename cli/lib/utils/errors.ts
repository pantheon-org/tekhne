export class CLIError extends Error {
  constructor(
    message: string,
    public readonly exitCode: number = 1,
  ) {
    super(message);
    this.name = this.constructor.name;
  }
}

export class FileNotFoundError extends CLIError {
  constructor(path: string) {
    super(`File not found: ${path}`, 1);
  }
}

export class ValidationError extends CLIError {
  constructor(message: string) {
    super(message, 1);
  }
}

export class ShellCommandError extends CLIError {
  constructor(
    command: string,
    public readonly stderr: string,
    exitCode: number,
  ) {
    super(`Command failed: ${command}\n${stderr}`, exitCode);
  }
}

export class AuditFailedError extends CLIError {
  constructor(
    public readonly skillPath: string,
    public readonly score: number,
  ) {
    super(`Audit failed for ${skillPath} (score: ${score}/120)`, 1);
  }
}

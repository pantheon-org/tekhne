import { $ } from "bun";

export async function exec(
  command: string,
): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  try {
    const proc = await $`sh -c ${command}`.quiet();
    return {
      stdout: proc.stdout.toString(),
      stderr: proc.stderr.toString(),
      exitCode: proc.exitCode,
    };
  } catch (error: unknown) {
    const err = error as {
      stdout?: Buffer;
      stderr?: Buffer;
      exitCode?: number;
    };
    return {
      stdout: err.stdout?.toString() || "",
      stderr: err.stderr?.toString() || "",
      exitCode: err.exitCode || 1,
    };
  }
}

export async function execOrThrow(command: string): Promise<string> {
  const proc = await $`sh -c ${command}`;
  return proc.stdout.toString();
}

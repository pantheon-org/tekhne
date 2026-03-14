import { $ } from "bun";

export const execOrThrow = async (command: string): Promise<string> => {
  const proc = await $`sh -c ${command}`;
  return proc.stdout.toString();
};

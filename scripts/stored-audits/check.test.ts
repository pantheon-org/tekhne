import { describe, expect, test } from "bun:test";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SCRIPT = join(import.meta.dir, "check.sh");

const makeRepo = (skills: string[]): string => {
  const root = mkdtempSync(join(tmpdir(), "stored-check-"));
  for (const s of skills) {
    mkdirSync(join(root, "skills", s), { recursive: true });
    writeFileSync(join(root, "skills", s, "SKILL.md"), "# skill\n");
  }
  return root;
};

// A fake auditor that records its arguments and behaves as told.
const fakeAuditor = (
  root: string,
  exitCode: number,
  stdout = "",
  stderr = "",
): string => {
  const bin = join(root, "fake-auditor");
  writeFileSync(
    bin,
    `#!/usr/bin/env bash\necho "$@" > "${root}/args.txt"\nprintf '%s' '${stdout}'\nprintf '%s' '${stderr}' >&2\nexit ${exitCode}\n`,
  );
  chmodSync(bin, 0o755);
  return bin;
};

// Only the variables the script needs are passed to the child process.
const run = (root: string, auditor: string, args: string[]) => {
  const proc = Bun.spawnSync(["bash", SCRIPT, "--root", root, ...args], {
    env: { PATH: Bun.env.PATH ?? "/usr/bin:/bin", AUDITOR: auditor },
  });
  return {
    code: proc.exitCode ?? -1,
    out: proc.stdout.toString(),
    err: proc.stderr.toString(),
  };
};

const argsSeen = (root: string): string => {
  const file = join(root, "args.txt");
  return existsSync(file) ? readFileSync(file, "utf8").trim() : "";
};

describe("check.sh", () => {
  test("exits 0 when the auditor says everything is current", () => {
    const root = makeRepo(["a/one"]);
    const r = run(root, fakeAuditor(root, 0), ["a/one"]);
    expect(r.code).toBe(0);
    expect(r.out).toContain("current");
    expect(argsSeen(root)).toBe("check-stored a/one");
  });

  test("exits 0 and does not call the auditor when there is nothing to check", () => {
    const root = makeRepo(["a/one"]);
    const r = run(root, fakeAuditor(root, 1), []);
    expect(r.code).toBe(0);
    expect(r.out).toContain("No skills to check");
    expect(argsSeen(root)).toBe("");
  });

  test("on stale, exits 1 and prints the stale lines plus the exact fix command", () => {
    const root = makeRepo(["a/one", "b/two", "c/three"]);
    const stale =
      "STALE a/one: no stored audit\nSTALE c/three: grade changed (stored C+, now B)\n";
    const r = run(root, fakeAuditor(root, 1, stale), [
      "a/one",
      "b/two",
      "c/three",
    ]);
    expect(r.code).toBe(1);
    expect(r.out).toContain("STALE a/one: no stored audit");
    expect(r.out).toContain("fake-auditor batch a/one c/three --store");
    expect(r.out).not.toContain(root);
    expect(r.out).not.toContain("batch a/one b/two");
    expect(r.out).toContain("commit");
  });

  test("on an auditor error, exits 2 and says the check could not run", () => {
    const root = makeRepo(["a/one"]);
    const r = run(
      root,
      fakeAuditor(root, 2, "", "Error: a/one: malformed stored audit"),
      ["a/one"],
    );
    expect(r.code).toBe(2);
    expect(r.out + r.err).toContain("Could not check stored audits");
    expect(r.out + r.err).toContain("malformed stored audit");
  });

  test("any other auditor exit code is treated as an error, not as stale", () => {
    const root = makeRepo(["a/one"]);
    const r = run(root, fakeAuditor(root, 101), ["a/one"]);
    expect(r.code).toBe(2);
  });

  test("a missing auditor binary is an error with the build command", () => {
    const root = makeRepo(["a/one"]);
    const r = run(root, join(root, "no-such-binary"), ["a/one"]);
    expect(r.code).toBe(2);
    expect(r.out + r.err).toContain(
      "cargo build --release -p pantheon-skill-auditor",
    );
  });

  test("--from-file reads one skill per line", () => {
    const root = makeRepo(["a/one", "b/two"]);
    const list = join(root, "skills.txt");
    writeFileSync(list, "a/one\nb/two\n");
    const r = run(root, fakeAuditor(root, 0), ["--from-file", list]);
    expect(r.code).toBe(0);
    expect(argsSeen(root)).toBe("check-stored a/one b/two");
  });

  test("--all checks every skill under skills/", () => {
    const root = makeRepo(["b/two", "a/one"]);
    const r = run(root, fakeAuditor(root, 0), ["--all"]);
    expect(r.code).toBe(0);
    expect(argsSeen(root)).toBe("check-stored a/one b/two");
  });
});

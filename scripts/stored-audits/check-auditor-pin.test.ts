import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const SCRIPT = join(import.meta.dir, "check-auditor-pin.sh");
const TOOL = "github:pantheon-org/pantheon-skill-auditor";
const SHA = "a".repeat(64);

interface Pins {
  toml?: string;
  lock?: string;
  lockSha?: string;
  actionVersion?: string;
  actionSha?: string;
}

const makeRoot = (pins: Pins = {}): string => {
  const root = mkdtempSync(join(tmpdir(), "auditor-pin-"));
  const write = (rel: string, text: string) => {
    mkdirSync(dirname(join(root, rel)), { recursive: true });
    writeFileSync(join(root, rel), text);
  };
  write(
    "mise.toml",
    `[tools]\nbun = "latest"\n"${TOOL}" = "${pins.toml ?? "0.3.1"}"\n`,
  );
  write(
    "mise.lock",
    [
      `[[tools."${TOOL}"]]`,
      `version = "${pins.lock ?? "0.3.1"}"`,
      `backend = "${TOOL}"`,
      "",
      `[tools."${TOOL}"."platforms.linux-x64"]`,
      `checksum = "sha256:${pins.lockSha ?? SHA}"`,
      'url = "https://example.invalid/x.tar.xz"',
      "",
      "[[tools.hk]]",
      'version = "9.9.9"',
      "",
    ].join("\n"),
  );
  write(
    ".github/actions/install-skill-auditor/action.yml",
    `      env:\n        VERSION: "${pins.actionVersion ?? "0.3.1"}"\n        SHA256: "${pins.actionSha ?? SHA}"\n`,
  );
  return root;
};

const run = (root: string) => {
  const proc = Bun.spawnSync(["bash", SCRIPT, "--root", root]);
  return {
    code: proc.exitCode,
    out: proc.stdout.toString(),
    err: proc.stderr.toString(),
  };
};

describe("check-auditor-pin.sh", () => {
  test("passes when all three places agree", () => {
    const r = run(makeRoot());
    expect(r.code).toBe(0);
    expect(r.out).toContain("consistent: 0.3.1");
  });

  test("fails when mise.lock has another version", () => {
    const r = run(makeRoot({ lock: "0.3.0" }));
    expect(r.code).toBe(1);
    expect(r.err).toContain("mise.toml has 0.3.1, mise.lock has 0.3.0");
  });

  test("fails when the install action has another version", () => {
    const r = run(makeRoot({ actionVersion: "0.4.0" }));
    expect(r.code).toBe(1);
    expect(r.err).toContain(
      "mise.toml has 0.3.1, the install action has 0.4.0",
    );
  });

  test("fails when the checksums differ", () => {
    const r = run(makeRoot({ actionSha: "b".repeat(64) }));
    expect(r.code).toBe(1);
    expect(r.err).toContain("checksum differs");
  });

  test("ignores another tool's version in mise.lock", () => {
    // The lock also holds [[tools.hk]] with version 9.9.9; it must not be read.
    expect(run(makeRoot()).code).toBe(0);
  });

  test("exits 2 when a pin cannot be read", () => {
    const root = mkdtempSync(join(tmpdir(), "auditor-pin-empty-"));
    expect(run(root).code).toBe(2);
  });
});

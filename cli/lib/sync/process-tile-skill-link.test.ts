import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { processTileSkillLink } from "./process-tile-skill-link";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "tile-link-test-"));
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

describe("processTileSkillLink", () => {
  test("dry run create returns 'create' without creating link", () => {
    const action = processTileSkillLink("my-skill", "/some/target", tmp, true);
    expect(action).toBe("create");
  });

  test("creates symlink and returns 'create'", () => {
    const action = processTileSkillLink("my-skill", "/some/target", tmp, false);
    expect(action).toBe("create");
    const link = join(tmp, "tessl__my-skill");
    expect(Bun.file(link).size).toBeDefined();
  });

  test("skips when symlink already points to same target", () => {
    const link = join(tmp, "tessl__my-skill");
    symlinkSync("/some/target", link);
    const action = processTileSkillLink("my-skill", "/some/target", tmp, false);
    expect(action).toBe("skip");
  });

  test("dry run update returns 'update' for stale symlink", () => {
    const link = join(tmp, "tessl__my-skill");
    symlinkSync("/old/target", link);
    const action = processTileSkillLink("my-skill", "/new/target", tmp, true);
    expect(action).toBe("update");
  });

  test("updates stale symlink and returns 'update'", () => {
    const link = join(tmp, "tessl__my-skill");
    symlinkSync("/old/target", link);
    const action = processTileSkillLink("my-skill", "/new/target", tmp, false);
    expect(action).toBe("update");
  });

  test("skips and returns 'skip' when path is a regular file not a symlink", () => {
    const link = join(tmp, "tessl__my-skill");
    Bun.write(link, "not a symlink");
    const action = processTileSkillLink("my-skill", "/some/target", tmp, false);
    expect(action).toBe("skip");
  });
});

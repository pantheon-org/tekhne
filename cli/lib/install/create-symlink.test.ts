import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import {
  existsSync,
  lstatSync,
  mkdirSync,
  mkdtempSync,
  readlinkSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createSymlink } from "./create-symlink";

let tmp: string;

beforeEach(() => {
  tmp = realpathSync.native(
    mkdtempSync(join(tmpdir(), "create-symlink-test-")),
  );
});

afterEach(() => {
  rmSync(tmp, { recursive: true, force: true });
});

const makeSource = (): string => {
  const source = join(tmp, "src", "my-skill");
  mkdirSync(source, { recursive: true });
  writeFileSync(join(source, "SKILL.md"), "# skill");
  return source;
};

describe("createSymlink", () => {
  test("creates a link that resolves to the source", () => {
    const source = makeSource();
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");

    expect(createSymlink(source, target, false)).toBe(true);
    expect(existsSync(target)).toBe(true);
    expect(realpathSync.native(target)).toBe(source);
  });

  test("resolves correctly when the target dir is reached via a symlinked parent", () => {
    // Reproduces ~/.claude -> ~/.config/claude: the target's parent is a symlink,
    // so a relative link computed from the logical path would dangle.
    const source = makeSource();
    const realParent = join(tmp, "config", "claude");
    mkdirSync(join(realParent, "skills"), { recursive: true });
    const logicalParent = join(tmp, ".claude");
    symlinkSync(realParent, logicalParent, "dir");

    const target = join(logicalParent, "skills", "my-skill");
    expect(createSymlink(source, target, false)).toBe(true);

    // The link must resolve to the source through the symlinked parent.
    expect(existsSync(target)).toBe(true);
    expect(realpathSync.native(target)).toBe(source);
    // And it must be stored relative (portable), not absolute.
    expect(readlinkSync(target).startsWith("/")).toBe(false);
  });

  test("is idempotent: a second call skips an already-correct link", () => {
    const source = makeSource();
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");

    expect(createSymlink(source, target, false)).toBe(true);
    expect(createSymlink(source, target, false)).toBe(false);
  });

  test("self-heals a broken/dangling symlink", () => {
    const source = makeSource();
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");
    symlinkSync(join(tmp, "does-not-exist"), target, "dir");
    expect(existsSync(target)).toBe(false); // dangling

    expect(createSymlink(source, target, false)).toBe(true);
    expect(realpathSync.native(target)).toBe(source);
  });

  test("replaces a symlink that points somewhere else", () => {
    const source = makeSource();
    const other = join(tmp, "other");
    mkdirSync(other);
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");
    symlinkSync(other, target, "dir");

    expect(createSymlink(source, target, false)).toBe(true);
    expect(realpathSync.native(target)).toBe(source);
  });

  test("never clobbers a real (non-symlink) directory", () => {
    const source = makeSource();
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");
    mkdirSync(target); // a real dir, e.g. a frozen external skill
    writeFileSync(join(target, "SKILL.md"), "# external");

    expect(createSymlink(source, target, false)).toBe(false);
    expect(lstatSync(target).isSymbolicLink()).toBe(false);
    expect(existsSync(join(target, "SKILL.md"))).toBe(true);
  });

  test("dry run reports without writing", () => {
    const source = makeSource();
    const targetDir = join(tmp, "skills");
    mkdirSync(targetDir);
    const target = join(targetDir, "my-skill");

    expect(createSymlink(source, target, true)).toBe(true);
    expect(existsSync(target)).toBe(false);
  });
});

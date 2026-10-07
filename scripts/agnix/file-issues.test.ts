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

const SCRIPT = join(import.meta.dir, "file-issues.sh");

type Diagnostic = {
  level: "error" | "warning" | "info";
  rule: string;
  file: string;
  line: number;
  message: string;
};

const diag = (
  rule: string,
  file: string,
  line: number,
  message = "Unknown tool '*'.",
  level: Diagnostic["level"] = "error",
): Diagnostic => ({ level, rule, file, line, message });

const warn = (
  rule: string,
  file: string,
  line: number,
  message = "Careful.",
): Diagnostic => diag(rule, file, line, message, "warning");

const makeDir = (diagnostics: Diagnostic[], filesChecked = 2146): string => {
  const dir = mkdtempSync(join(tmpdir(), "agnix-issues-"));
  writeFileSync(
    join(dir, "agnix.json"),
    JSON.stringify({
      version: "0.56.6",
      files_checked: filesChecked,
      diagnostics,
      summary: {
        errors: diagnostics.filter((d) => d.level === "error").length,
        warnings: diagnostics.filter((d) => d.level === "warning").length,
        info: 0,
      },
    }),
  );
  return dir;
};

// A fake gh that logs every call. `issue list` answers list.json the first time
// and list2.json afterwards (so a race with another run can be simulated), and
// `issue create` "creates" issue 999. Titles, labels and bodies of created and
// edited issues, and the comments given when closing, are kept for assertions.
const fakeGh = (dir: string, opts: { list?: string; list2?: string } = {}) => {
  const bin = join(dir, "bin");
  mkdirSync(bin, { recursive: true });
  const list = opts.list ?? "[]";
  writeFileSync(join(dir, "list.json"), list);
  writeFileSync(join(dir, "list2.json"), opts.list2 ?? list);
  writeFileSync(
    join(bin, "gh"),
    `#!/usr/bin/env bash
echo "$@" >> "${dir}/calls.log"
case "$1 $2" in
  "issue list")
    n=$(cat "${dir}/list.n" 2>/dev/null || echo 0); n=$((n+1)); echo "$n" > "${dir}/list.n"
    if [ "$n" -ge 2 ]; then cat "${dir}/list2.json"; else cat "${dir}/list.json"; fi ;;
  "issue create")
    while [ $# -gt 0 ]; do
      case "$1" in
        --title) printf '%s\\n' "$2" >> "${dir}/created-titles.txt"; shift ;;
        --label) printf '%s\\n' "$2" >> "${dir}/created-labels.txt"; shift ;;
        --body-file) cat "$2" >> "${dir}/created-bodies.md"; printf '\\n=====\\n' >> "${dir}/created-bodies.md"; cp "$2" "${dir}/body-999.md"; shift ;;
      esac
      shift
    done
    echo "https://github.com/o/r/issues/999" ;;
  "issue edit")
    num="$3"
    while [ $# -gt 0 ]; do
      case "$1" in --body-file) printf '#%s\\n' "$num" >> "${dir}/edited-bodies.md"; cat "$2" >> "${dir}/edited-bodies.md"; cp "$2" "${dir}/body-$num.md"; shift ;; esac
      shift
    done ;;
  "issue view")
    if [ -n "$FAKE_VIEW_CORRUPT" ]; then echo "mangled by github"; else cat "${dir}/body-$3.md" 2>/dev/null; fi ;;
  "issue close")
    num="$3"
    while [ $# -gt 0 ]; do
      case "$1" in --comment) printf '%s\\t%s\\n' "$num" "$2" >> "${dir}/closed.log"; shift ;; esac
      shift
    done ;;
esac
`,
  );
  chmodSync(join(bin, "gh"), 0o755);
};

const run = (dir: string, extra: Record<string, string> = {}) =>
  Bun.spawnSync(["bash", SCRIPT, join(dir, "agnix.json")], {
    env: {
      PATH: `${join(dir, "bin")}:${process.env.PATH ?? ""}`,
      GITHUB_REPOSITORY: "o/r",
      MODE: "pr",
      PR_NUMBER: "42",
      RUN_URL: "https://github.com/o/r/actions/runs/7",
      TRACKED_FILE: join(dir, "tracked.md"),
      ...extra,
    },
  });

const onMain = { MODE: "main", PR_NUMBER: "" };

const read = (dir: string, name: string): string =>
  existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8") : "";

const marker = (rule: string, file: string): string =>
  `<!-- agnix-finding: ${rule} ${file} -->`;

const warningMarker = (rule: string): string =>
  `<!-- agnix-warnings: ${rule} -->`;

const star = "skills/agentic-harness/professional-honesty/SKILL.md";

describe("agnix file-issues.sh: errors", () => {
  test("does nothing when agnix reports nothing on a pull request", () => {
    const dir = makeDir([]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).toBe("");
    expect(read(dir, "tracked.md")).toBe("");
  });

  test("files one issue for an error, labels it and links the pull request", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "created-titles.txt")).toBe(`agnix CC-SK-008: ${star}\n`);
    expect(read(dir, "created-labels.txt")).toBe("bug\n");
    const body = read(dir, "created-bodies.md");
    expect(body).toContain(marker("CC-SK-008", star));
    expect(body).toContain("#42");
    expect(body).toContain("https://github.com/o/r/actions/runs/7");
    expect(body).toContain("line 4");
    expect(read(dir, "tracked.md")).toBe(
      `- \`CC-SK-008\` in \`${star}\`: #999\n`,
    );
  });

  test("turns an absolute runner path into a path under skills/", () => {
    const dir = makeDir([
      diag("CC-SK-008", `/home/runner/work/r/r/${star}`, 4),
    ]);
    fakeGh(dir);
    run(dir);
    expect(read(dir, "created-titles.txt")).toBe(`agnix CC-SK-008: ${star}\n`);
  });

  test("files one issue per rule and file, listing every line", () => {
    const dir = makeDir([
      diag("CC-SK-008", star, 4),
      diag("CC-SK-008", star, 9, "Unknown tool 'x'."),
      diag("CC-SK-002", "skills/a/b/SKILL.md", 7, "Invalid context"),
    ]);
    fakeGh(dir);
    run(dir);
    const titles = read(dir, "created-titles.txt").trim().split("\n");
    expect(titles).toHaveLength(2);
    const bodies = read(dir, "created-bodies.md").split("=====");
    const first = bodies.find((b) => b.includes("CC-SK-008"));
    expect(first).toContain("line 4");
    expect(first).toContain("line 9");
  });

  test("adopts an open issue found by its marker and adds no comment", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "anything", body: marker("CC-SK-008", star) },
      ]),
    });
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).not.toContain("issue create");
    expect(read(dir, "calls.log")).not.toContain("issue comment");
    expect(read(dir, "tracked.md")).toBe(
      `- \`CC-SK-008\` in \`${star}\`: #421\n`,
    );
  });

  test("adopts a hand-filed issue found by its title", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 430, title: `agnix CC-SK-008: ${star}`, body: "by hand" },
      ]),
    });
    run(dir);
    expect(read(dir, "calls.log")).not.toContain("issue create");
    expect(read(dir, "tracked.md")).toContain("#430");
  });

  test("on main, creates a missing issue naming main and leaves an existing one alone", () => {
    const missing = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(missing);
    run(missing, onMain);
    expect(read(missing, "created-titles.txt")).toContain("CC-SK-008");
    expect(read(missing, "created-bodies.md")).toContain("`main`");

    const existing = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(existing, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
      ]),
    });
    run(existing, onMain);
    expect(read(existing, "calls.log")).not.toContain("issue create");
    expect(read(existing, "calls.log")).not.toContain("issue edit");
  });

  test("on main, closes an error issue whose finding is gone and keeps one still present", () => {
    const dir = makeDir([diag("CC-SK-002", "skills/a/b/SKILL.md", 7)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
        {
          number: 422,
          title: "t",
          body: marker("CC-SK-002", "skills/a/b/SKILL.md"),
        },
        { number: 423, title: "unrelated issue", body: "no marker" },
      ]),
    });
    const r = run(dir, onMain);
    expect(r.exitCode).toBe(0);
    const closed = read(dir, "closed.log");
    expect(closed).toContain("421\t");
    expect(closed).toContain("no longer reports");
    expect(closed).not.toContain("422\t");
    expect(closed).not.toContain("423\t");
  });

  test("never closes anything on a pull request run", () => {
    const dir = makeDir([]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
        { number: 424, title: "t", body: warningMarker("CC-SK-017") },
      ]),
    });
    run(dir);
    expect(read(dir, "closed.log")).toBe("");
  });

  test("does not close anything when agnix checked no files", () => {
    const dir = makeDir([], 0);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
      ]),
    });
    const r = run(dir, onMain);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "closed.log")).toBe("");
  });

  test("does nothing with an unusable agnix.json", () => {
    const dir = makeDir([]);
    writeFileSync(join(dir, "agnix.json"), "not json");
    fakeGh(dir);
    const r = run(dir, onMain);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).toBe("");
  });

  test("skips a finding whose rule or path is not plain", () => {
    const dir = makeDir([
      diag("CC-SK-008", "skills/a b/SKILL.md", 1),
      diag("bad rule", "skills/a/SKILL.md", 1),
      diag("CC-SK-008", "skills/x --> y/SKILL.md", 1),
    ]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).not.toContain("issue create");
  });

  test("escapes mentions and markup from a message", () => {
    const dir = makeDir([
      diag("CC-SK-008", star, 4, "see @octocat <b>x</b> `y`"),
    ]);
    fakeGh(dir);
    run(dir);
    const body = read(dir, "created-bodies.md");
    expect(body).not.toContain("@octocat");
    expect(body).not.toContain("<b>");
    expect(body).toContain("&#64;octocat");
  });

  test("closes the duplicate it just created when a lower-numbered issue exists", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    const both = JSON.stringify([
      { number: 500, title: "t", body: marker("CC-SK-008", star) },
      { number: 999, title: "t", body: marker("CC-SK-008", star) },
    ]);
    fakeGh(dir, { list: "[]", list2: both });
    run(dir);
    expect(read(dir, "closed.log")).toContain("999\t");
    expect(read(dir, "closed.log")).toContain("#500");
    expect(read(dir, "tracked.md")).toContain("#500");
  });

  test("reports without changing anything under DRY_RUN=1", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir);
    const r = run(dir, { DRY_RUN: "1" });
    expect(r.exitCode).toBe(0);
    expect(r.stdout.toString()).toContain("would create");
    expect(read(dir, "calls.log")).not.toContain("issue create");
  });
});

describe("agnix file-issues.sh: warnings", () => {
  const a = "skills/a/one/SKILL.md";
  const b = "skills/a/two/SKILL.md";
  const c = "skills/b/three/SKILL.md";

  test("files one issue per rule, listing every file and line", () => {
    const dir = makeDir([
      warn("CC-SK-017", a, 3, "Unknown frontmatter field 'x'"),
      warn("CC-SK-017", a, 8, "Unknown frontmatter field 'y'"),
      warn("CC-SK-017", b, 4, "Unknown frontmatter field 'z'"),
      warn("AS-012", c, 1, "Skill content exceeds 500 lines"),
    ]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    const titles = read(dir, "created-titles.txt").trim().split("\n");
    expect(titles).toEqual([
      "agnix warnings: AS-012",
      "agnix warnings: CC-SK-017",
    ]);
    expect(read(dir, "created-labels.txt")).toBe("enhancement\nenhancement\n");
    const bodies = read(dir, "created-bodies.md").split("=====");
    const multi = bodies.find((x) => x.includes(warningMarker("CC-SK-017")));
    expect(multi).toContain("2 files");
    expect(multi).toContain(
      `- \`${a}\`\n  - line 3: Unknown frontmatter field 'x'\n  - line 8: Unknown frontmatter field 'y'\n- \`${b}\`\n  - line 4: Unknown frontmatter field 'z'`,
    );
    expect(multi).not.toContain("Messages:");
    expect(read(dir, "tracked.md")).toContain(
      "warnings `AS-012` in 1 file: #999",
    );
    expect(read(dir, "tracked.md")).toContain(
      "warnings `CC-SK-017` in 2 files: #999",
    );
  });

  test("files nothing for a rule whose files are all unusable", () => {
    const dir = makeDir([warn("CC-SK-017", "skills/a b/SKILL.md", 3)]);
    fakeGh(dir);
    run(dir);
    expect(read(dir, "calls.log")).not.toContain("issue create");
  });

  test("keeps a warning and an error of the same rule as separate issues", () => {
    const dir = makeDir([warn("CC-SK-008", a, 1), diag("CC-SK-008", b, 2)]);
    fakeGh(dir);
    run(dir);
    const titles = read(dir, "created-titles.txt").trim().split("\n");
    expect(titles).toContain("agnix warnings: CC-SK-008");
    expect(titles).toContain(`agnix CC-SK-008: ${b}`);
  });

  test("adopts an open warnings issue without editing it on a pull request", () => {
    const dir = makeDir([warn("CC-SK-017", a, 3)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 440, title: "t", body: warningMarker("CC-SK-017") },
      ]),
    });
    run(dir);
    expect(read(dir, "calls.log")).not.toContain("issue create");
    expect(read(dir, "calls.log")).not.toContain("issue edit");
    expect(read(dir, "tracked.md")).toContain("#440");
  });

  test("on main, refreshes the file list when it changed and leaves an unchanged one alone", () => {
    const first = makeDir([warn("CC-SK-017", a, 3)]);
    fakeGh(first);
    run(first, onMain);
    const unchangedBody = read(first, "created-bodies.md")
      .split("=====")[0]
      .trim();

    const same = makeDir([warn("CC-SK-017", a, 3)]);
    fakeGh(same, {
      list: JSON.stringify([{ number: 440, title: "t", body: unchangedBody }]),
    });
    run(same, onMain);
    expect(read(same, "calls.log")).not.toContain("issue edit");

    const changed = makeDir([warn("CC-SK-017", a, 3), warn("CC-SK-017", b, 5)]);
    fakeGh(changed, {
      list: JSON.stringify([{ number: 440, title: "t", body: unchangedBody }]),
    });
    run(changed, onMain);
    expect(read(changed, "calls.log")).toContain("issue edit 440");
    expect(read(changed, "edited-bodies.md")).toContain(b);
  });

  test("on main, closes a warnings issue once no file has the rule and keeps a present one", () => {
    const dir = makeDir([warn("AS-012", c, 1)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 440, title: "t", body: warningMarker("CC-SK-017") },
        { number: 441, title: "t", body: warningMarker("AS-012") },
      ]),
    });
    run(dir, onMain);
    const closed = read(dir, "closed.log");
    expect(closed).toContain("440\t");
    expect(closed).toContain("no longer reports");
    expect(closed).not.toContain("441\t");
  });

  test("escapes a warning message and caps the file list at 100", () => {
    const many = Array.from({ length: 120 }, (_, i) =>
      warn(
        "CC-SK-017",
        `skills/s/s${String(i).padStart(3, "0")}/SKILL.md`,
        1,
        "see @octocat <i>x</i>",
      ),
    );
    const dir = makeDir(many);
    fakeGh(dir);
    run(dir);
    const body = read(dir, "created-bodies.md");
    expect(body).toContain("120 files");
    expect(body).toContain("and 20 more");
    expect(body).not.toContain("@octocat");
    expect(body).toContain("&#64;octocat");
    expect(body).not.toContain("s119");
  });

  test("reports a warnings issue under DRY_RUN=1 without creating it", () => {
    const dir = makeDir([warn("CC-SK-017", a, 3)]);
    fakeGh(dir);
    const r = run(dir, { DRY_RUN: "1" });
    expect(r.stdout.toString()).toContain(
      "would create: agnix warnings: CC-SK-017",
    );
    expect(read(dir, "calls.log")).not.toContain("issue create");
  });
  test("keeps a message with its own file when two files share a rule", () => {
    const dir = makeDir([
      warn("AS-012", a, 4, "Skill content exceeds 500 lines (got 518)"),
      warn("AS-012", b, 4, "Skill content exceeds 500 lines (got 631)"),
    ]);
    fakeGh(dir);
    run(dir);
    const body = read(dir, "created-bodies.md");
    expect(body).toContain(
      `- \`${a}\`\n  - line 4: Skill content exceeds 500 lines (got 518)\n- \`${b}\`\n  - line 4: Skill content exceeds 500 lines (got 631)`,
    );
  });

  test("shows at most 5 lines for one file and counts the rest", () => {
    const dir = makeDir(
      Array.from({ length: 8 }, (_, i) =>
        warn("CC-SK-017", a, i + 1, `m${i + 1}`),
      ),
    );
    fakeGh(dir);
    run(dir);
    const body = read(dir, "created-bodies.md");
    expect(body).toContain("line 5: m5");
    expect(body).not.toContain("line 6: m6");
    expect(body).toContain("and 3 more lines");
  });

  test("trims the file list until the body fits GitHub's size limit", () => {
    const long = "x".repeat(290);
    const many = Array.from({ length: 100 }, (_, i) =>
      Array.from({ length: 5 }, (_, j) =>
        warn(
          "CC-SK-017",
          `skills/s/s${String(i).padStart(3, "0")}/SKILL.md`,
          j + 1,
          long,
        ),
      ),
    ).flat();
    const dir = makeDir(many);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    const body = read(dir, "body-999.md");
    expect(Buffer.byteLength(body)).toBeLessThan(60000);
    expect(body).toContain("100 files");
    expect(body).toMatch(/and \d+ more files/);
  });
});

describe("agnix file-issues.sh: read-back check", () => {
  const f = "skills/a/one/SKILL.md";

  test("reads a created warnings issue back and passes when it matches", () => {
    const dir = makeDir([warn("CC-SK-017", f, 3)]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).toContain("issue view 999");
    expect(r.stdout.toString()).toContain("Checked #999");
  });

  test("reads a created error issue back too", () => {
    const dir = makeDir([diag("CC-SK-008", f, 4)]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).toContain("issue view 999");
  });

  test("warns and fails the step when the stored body is not what was sent", () => {
    const dir = makeDir([warn("CC-SK-017", f, 3)]);
    fakeGh(dir);
    const r = run(dir, { FAKE_VIEW_CORRUPT: "1" });
    expect(r.exitCode).toBe(1);
    expect(r.stdout.toString()).toContain("::warning::");
    expect(r.stdout.toString()).toContain("does not match");
  });

  test("names the files missing from the stored body", () => {
    const dir = makeDir([warn("CC-SK-017", f, 3)]);
    fakeGh(dir);
    const r = run(dir, { FAKE_VIEW_CORRUPT: "1" });
    expect(r.stdout.toString()).toContain(f);
  });

  test("reads a refreshed warnings issue back on main", () => {
    const first = makeDir([warn("CC-SK-017", f, 3)]);
    fakeGh(first);
    run(first, onMain);
    const body = read(first, "created-bodies.md").split("=====")[0].trim();
    const dir = makeDir([
      warn("CC-SK-017", f, 3),
      warn("CC-SK-017", "skills/a/two/SKILL.md", 5),
    ]);
    fakeGh(dir, { list: JSON.stringify([{ number: 440, title: "t", body }]) });
    const r = run(dir, onMain);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).toContain("issue view 440");
  });

  test("does not read back an issue it closed as a duplicate", () => {
    const dir = makeDir([diag("CC-SK-008", f, 4)]);
    const both = JSON.stringify([
      { number: 500, title: "t", body: marker("CC-SK-008", f) },
      { number: 999, title: "t", body: marker("CC-SK-008", f) },
    ]);
    fakeGh(dir, { list: "[]", list2: both });
    run(dir);
    expect(read(dir, "calls.log")).not.toContain("issue view");
  });
});

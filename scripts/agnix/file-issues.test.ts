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
// and list2.json afterwards (so a race with another run can be simulated),
// `issue view` answers view.json, and `issue create` "creates" issue 999.
const fakeGh = (
  dir: string,
  opts: { list?: string; list2?: string; view?: string } = {},
) => {
  const bin = join(dir, "bin");
  mkdirSync(bin, { recursive: true });
  const list = opts.list ?? "[]";
  writeFileSync(join(dir, "list.json"), list);
  writeFileSync(join(dir, "list2.json"), opts.list2 ?? list);
  writeFileSync(
    join(dir, "view.json"),
    opts.view ?? '{"body":"","comments":[]}',
  );
  writeFileSync(
    join(bin, "gh"),
    `#!/usr/bin/env bash
echo "$@" >> "${dir}/calls.log"
case "$1 $2" in
  "issue list")
    n=$(cat "${dir}/list.n" 2>/dev/null || echo 0); n=$((n+1)); echo "$n" > "${dir}/list.n"
    if [ "$n" -ge 2 ]; then cat "${dir}/list2.json"; else cat "${dir}/list.json"; fi ;;
  "issue view") cat "${dir}/view.json" ;;
  "issue create")
    while [ $# -gt 0 ]; do
      case "$1" in
        --title) printf '%s\\n' "$2" >> "${dir}/created-titles.txt"; shift ;;
        --label) printf '%s\\n' "$2" >> "${dir}/created-labels.txt"; shift ;;
        --body-file) cat "$2" >> "${dir}/created-bodies.md"; printf '\\n=====\\n' >> "${dir}/created-bodies.md"; shift ;;
      esac
      shift
    done
    echo "https://github.com/o/r/issues/999" ;;
  "issue comment")
    num="$3"
    while [ $# -gt 0 ]; do
      case "$1" in --body) printf '%s\\t%s\\n' "$num" "$2" >> "${dir}/comments.log"; shift ;; esac
      shift
    done ;;
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

const read = (dir: string, name: string): string =>
  existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8") : "";

const marker = (rule: string, file: string): string =>
  `<!-- agnix-finding: ${rule} ${file} -->`;

const star = "skills/agentic-harness/professional-honesty/SKILL.md";

describe("agnix file-issues.sh", () => {
  test("does nothing when agnix reports no errors", () => {
    const dir = makeDir([
      diag("CC-SK-017", star, 3, "Unknown field", "warning"),
    ]);
    fakeGh(dir);
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).not.toContain("issue create");
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

  test("adopts an open issue found by its marker and comments once per pull request", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "anything", body: marker("CC-SK-008", star) },
      ]),
    });
    const r = run(dir);
    expect(r.exitCode).toBe(0);
    expect(read(dir, "calls.log")).not.toContain("issue create");
    expect(read(dir, "comments.log")).toContain("421\tAlso found on #42");
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

  test("does not comment again when the issue already mentions the pull request", () => {
    const dir = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(dir, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
      ]),
      view: JSON.stringify({
        body: marker("CC-SK-008", star),
        comments: [{ body: "Also found on #42: still failing." }],
      }),
    });
    run(dir);
    expect(read(dir, "comments.log")).toBe("");
  });

  test("on main, creates a missing issue and never comments on an existing one", () => {
    const missing = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(missing);
    run(missing, { MODE: "main", PR_NUMBER: "" });
    expect(read(missing, "created-titles.txt")).toContain("CC-SK-008");
    expect(read(missing, "created-bodies.md")).toContain("`main`");

    const existing = makeDir([diag("CC-SK-008", star, 4)]);
    fakeGh(existing, {
      list: JSON.stringify([
        { number: 421, title: "t", body: marker("CC-SK-008", star) },
      ]),
    });
    run(existing, { MODE: "main", PR_NUMBER: "" });
    expect(read(existing, "comments.log")).toBe("");
    expect(read(existing, "calls.log")).not.toContain("issue create");
  });

  test("on main, closes an issue whose finding is gone and keeps one that is still present", () => {
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
    const r = run(dir, { MODE: "main", PR_NUMBER: "" });
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
    const r = run(dir, { MODE: "main", PR_NUMBER: "" });
    expect(r.exitCode).toBe(0);
    expect(read(dir, "closed.log")).toBe("");
  });

  test("does nothing with an unusable agnix.json", () => {
    const dir = makeDir([]);
    writeFileSync(join(dir, "agnix.json"), "not json");
    fakeGh(dir);
    const r = run(dir, { MODE: "main", PR_NUMBER: "" });
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

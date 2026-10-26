import { describe, expect, test } from "bun:test";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  writeFileSync,
} from "node:fs";
import { homedir, tmpdir } from "node:os";
import { dirname, join } from "node:path";

const SKILL_SCRIPTS = join(
  import.meta.dir,
  "../../skills/project-mgmt/context-index/scripts",
);
const GENERATOR = join(SKILL_SCRIPTS, "regenerate-context-index.sh");
const BASELINE = join(
  import.meta.dir,
  "fixtures/regenerate-context-index.baseline.sh",
);

// Real repositories to prove the claims on. Both are read only: the tests copy
// `.context/` into a throwaway repository first. They are skipped when absent
// (CI has neither), and the synthetic repository below always runs.
const JOURNAL_CONTEXT =
  Bun.env.CONTEXT_INDEX_TEST_JOURNAL ??
  join(homedir(), "Documents/Journal/.context");
const SIGNAL_CONTEXT =
  Bun.env.CONTEXT_INDEX_TEST_SIGNAL ??
  join(
    homedir(),
    "Projects/gitlab/plg-tech/plg/sandbox/poc/thoroc-signal/.context",
  );

// Git exports GIT_DIR and friends to hooks; they would point the generator at
// the real repository. Only what the scripts need is passed on.
const childEnv = { PATH: Bun.env.PATH ?? "/usr/bin:/bin" };

const sh = (script: string, root: string, ...args: string[]) => {
  const proc = Bun.spawnSync(["bash", script, ...args], {
    cwd: root,
    env: childEnv,
  });
  return {
    code: proc.exitCode ?? -1,
    out: proc.stdout.toString(),
    err: proc.stderr.toString(),
  };
};

const newRepo = (): string => {
  const root = mkdtempSync(join(tmpdir(), "ctx-index-"));
  Bun.spawnSync(["git", "init", "-q"], { cwd: root, env: childEnv });
  return root;
};

const repoFromContext = (contextDir: string): string => {
  const root = newRepo();
  cpSync(contextDir, join(root, ".context"), { recursive: true });
  return root;
};

const put = (root: string, rel: string, content: string) => {
  const path = join(root, rel);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content);
};

const doc = (
  type: string,
  title: string,
  date: string,
  extra = "",
  status = "active",
) =>
  `---\ntitle: "${title}"\ntype: ${type}\nstatus: ${status}\ndate: ${date}\ntags: []\n${extra}---\n\n# ${title}\n`;

const syntheticRepo = (): string => {
  const root = newRepo();
  put(
    root,
    ".context/plans/2026-01-01-alpha.md",
    doc("plan", "Alpha | plan", "2026-01-01"),
  );
  put(
    root,
    ".context/plans/2026-01-02-beta.md",
    doc(
      "plan",
      "Beta [x]",
      "2026-01-02",
      "blocked-by:\n  - ../follow-ups/2026-01-03-gamma.md\n",
    ),
  );
  put(
    root,
    ".context/follow-ups/2026-01-03-gamma.md",
    doc(
      "follow-up",
      "Gamma",
      "2026-01-03",
      "severity: HIGH\npriority: P1\ngraded: 2026-01-04\n",
    ),
  );
  put(
    root,
    ".context/goals/2026-01-04-delta.md",
    doc("goal", "Delta", "2026-01-04", "", "done"),
  );
  put(
    root,
    ".context/audits/2026-01-05-epsilon.md",
    doc("audit", "Epsilon", "2026-01-05"),
  );
  return root;
};

const read = (root: string, rel: string) =>
  readFileSync(join(root, rel), "utf8");

const entryCount = (text: string) =>
  (text.match(/^\s*- path: /gm) ?? []).length;

// The default index minus its three header lines and blank, against the
// per-type files minus their two header lines and blank, in the default's order.
const body = (text: string, headerLines: number) =>
  text.split("\n").slice(headerLines).join("\n");

const goldenCheck = (root: string) => {
  expect(sh(GENERATOR, root).code).toBe(0);
  expect(sh(GENERATOR, root, "--split").code).toBe(0);
  const aggregate = read(root, ".context/index.yaml");
  const sections = [...aggregate.matchAll(/^([a-z-]+):$/gm)].map((m) => m[1]);
  expect(sections.length).toBeGreaterThan(0);
  let joined = "";
  let total = 0;
  for (const s of sections) {
    const file = read(root, `.context/index-${s}.yaml`);
    expect(file).not.toMatch(/Last updated/);
    expect(file.split("\n")[1]).toMatch(/^# \d+ entries: /);
    joined += body(file, 3);
    total += entryCount(file);
  }
  expect(joined).toBe(body(aggregate, 4));
  expect(total).toBe(entryCount(aggregate));
  expect(total).toBeGreaterThan(0);
  // One yaml and one md per non-empty type, nothing else.
  const split = readdirSync(join(root, ".context")).filter((f) =>
    /^index-/.test(f),
  );
  expect(split.length).toBe(sections.length * 2);
  return total;
};

const defaultIdentical = (root: string) => {
  const baseline = sh(BASELINE, root);
  expect(baseline.code).toBe(0);
  const old = read(root, ".context/index.yaml");
  const oldCheck = sh(BASELINE, root, "--check");
  expect(oldCheck.code).toBe(0);
  const next = sh(GENERATOR, root);
  expect(next.code).toBe(0);
  expect(next.out).toBe(baseline.out);
  expect(read(root, ".context/index.yaml")).toBe(old);
  expect(sh(GENERATOR, root, "--check").out).toBe(oldCheck.out);
  expect(
    readdirSync(join(root, ".context")).some((f) => /^index-/.test(f)),
  ).toBe(false);
};

describe("synthetic repository", () => {
  test("golden: per-type files hold exactly the entries of index.yaml", () => {
    expect(goldenCheck(syntheticRepo())).toBe(5);
  });

  test("default output is byte-identical to the baseline generator", () => {
    defaultIdentical(syntheticRepo());
  });

  test("md view: linked title, status, date, grades only for follow-ups, escaped", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    const plans = read(root, ".context/index-plans.md");
    expect(plans).toContain("| Title | Status | Date |\n");
    expect(plans).toContain(
      "[Alpha \\| plan](plans/2026-01-01-alpha.md) | active | 2026-01-01 |",
    );
    expect(plans).toContain("[Beta \\[x\\]](plans/2026-01-02-beta.md)");
    expect(plans).not.toContain("Severity");
    const followUps = read(root, ".context/index-follow-ups.md");
    expect(followUps).toContain(
      "| Title | Status | Date | Severity | Priority | Graded |",
    );
    expect(followUps).toContain(
      "[Gamma](follow-ups/2026-01-03-gamma.md) | active | 2026-01-03 | HIGH | P1 | 2026-01-04 |",
    );
  });

  test("the generator does not index its own md views", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    const again = sh(GENERATOR, root, "--split");
    expect(again.err).not.toMatch(/index-/);
    expect(read(root, ".context/index.yaml")).not.toContain("index-plans.md");
    expect(sh(GENERATOR, root, "--split", "--check").code).toBe(0);
  });

  test("--only --stdout renders one file and writes nothing", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    const before = readdirSync(join(root, ".context")).sort();
    const yaml = sh(GENERATOR, root, "--only", "plans", "--stdout");
    expect(yaml.out).toBe(read(root, ".context/index-plans.yaml"));
    const md = sh(
      GENERATOR,
      root,
      "--only",
      "plans",
      "--stdout",
      "--format",
      "md",
    );
    expect(md.out).toBe(read(root, ".context/index-plans.md"));
    expect(readdirSync(join(root, ".context")).sort()).toEqual(before);
    const empty = sh(GENERATOR, root, "--only", "tickets", "--stdout");
    expect(empty.code).toBe(0);
    expect(empty.out).toBe("");
    expect(sh(GENERATOR, root, "--only", "plans").code).toBe(2);
    expect(sh(GENERATOR, root, "--only", "nonsense", "--stdout").code).toBe(2);
  });

  test("--split --check fails on stale, orphan and missing files", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    expect(sh(GENERATOR, root, "--split", "--check").code).toBe(0);

    put(root, ".context/index-tickets.yaml", "# stray\n");
    const orphan = sh(GENERATOR, root, "--split", "--check");
    expect(orphan.code).toBe(1);
    expect(orphan.err).toContain("index-tickets.yaml is an orphan");
    // Writing removes the orphan.
    sh(GENERATOR, root, "--split");
    expect(existsSync(join(root, ".context/index-tickets.yaml"))).toBe(false);

    Bun.spawnSync(["rm", join(root, ".context/index-goals.md")]);
    const missing = sh(GENERATOR, root, "--split", "--check");
    expect(missing.code).toBe(1);
    expect(missing.err).toContain("index-goals.md is missing");
    sh(GENERATOR, root, "--split");

    writeFileSync(join(root, ".context/index-plans.yaml"), "changed\n");
    expect(sh(GENERATOR, root, "--split", "--check").err).toContain(
      "index-plans.yaml is stale",
    );
  });

  test("a type that empties loses its pair", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    Bun.spawnSync(["rm", join(root, ".context/goals/2026-01-04-delta.md")]);
    sh(GENERATOR, root, "--split");
    expect(existsSync(join(root, ".context/index-goals.yaml"))).toBe(false);
    expect(existsSync(join(root, ".context/index-goals.md"))).toBe(false);
  });

  test("--no-aggregate leaves index.yaml alone", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split", "--no-aggregate");
    expect(existsSync(join(root, ".context/index.yaml"))).toBe(false);
    expect(
      sh(GENERATOR, root, "--split", "--no-aggregate", "--check").code,
    ).toBe(0);
    expect(sh(GENERATOR, root, "--no-aggregate").code).toBe(2);
  });

  test("the three checks skip index-*.md", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    const md = join(root, ".context/index-plans.md");
    const names = sh(join(SKILL_SCRIPTS, "check-context-filenames.sh"), root);
    expect(names.code).toBe(0);
    expect(
      sh(join(SKILL_SCRIPTS, "check-context-frontmatter.sh"), root, md).code,
    ).toBe(0);
    expect(
      sh(join(SKILL_SCRIPTS, "validate-context-frontmatter.sh"), root, md).code,
    ).toBe(0);
  });

  test("context-ready reads the split set, across types, like index.yaml", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root);
    const fromAggregate = sh(
      join(SKILL_SCRIPTS, "context-ready.sh"),
      root,
      "--blocked",
    );
    expect(fromAggregate.out).toContain("2026-01-03-gamma.md (status: active)");
    sh(GENERATOR, root, "--split", "--no-aggregate");
    Bun.spawnSync(["rm", join(root, ".context/index.yaml")]);
    const fromSplit = sh(
      join(SKILL_SCRIPTS, "context-ready.sh"),
      root,
      "--blocked",
    );
    expect(fromSplit.out).toBe(fromAggregate.out);
    // No count warning: header counts sum over the set and index-*.md is not counted.
    expect(fromSplit.err).toBe("");
    const ready = sh(join(SKILL_SCRIPTS, "context-ready.sh"), root);
    expect(ready.out).toContain("3 ready item(s)");
  });

  test("context-ready warns when split files and index.yaml disagree", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root, "--split");
    put(
      root,
      ".context/notes/2026-01-06-zeta.md",
      doc("note", "Zeta", "2026-01-06"),
    );
    sh(GENERATOR, root); // aggregate only: split files now one entry behind
    const out = sh(join(SKILL_SCRIPTS, "context-ready.sh"), root);
    expect(out.err).toContain("one of them is stale");
  });

  test("plan-staleness reads the split files when present", () => {
    const root = syntheticRepo();
    sh(GENERATOR, root);
    const viaAggregate = sh(
      join(SKILL_SCRIPTS, "check-plan-staleness.sh"),
      root,
    );
    expect(viaAggregate.out).toContain("2026-01-01-alpha.md");
    sh(GENERATOR, root, "--split", "--no-aggregate");
    Bun.spawnSync(["rm", join(root, ".context/index.yaml")]);
    const viaSplit = sh(join(SKILL_SCRIPTS, "check-plan-staleness.sh"), root);
    expect(viaSplit.out).toBe(viaAggregate.out);
  });
});

describe.skipIf(!existsSync(JOURNAL_CONTEXT))(
  "copy of the Journal .context",
  () => {
    test("golden: per-type files hold exactly the entries of index.yaml", () => {
      expect(goldenCheck(repoFromContext(JOURNAL_CONTEXT))).toBeGreaterThan(0);
    });

    test("default output is byte-identical to the baseline generator", () => {
      defaultIdentical(repoFromContext(JOURNAL_CONTEXT));
    });

    test("context-ready gives the same answer from split files as from index.yaml", () => {
      const root = repoFromContext(JOURNAL_CONTEXT);
      sh(GENERATOR, root, "--split");
      const split = sh(
        join(SKILL_SCRIPTS, "context-ready.sh"),
        root,
        "--blocked",
      );
      Bun.spawnSync([
        "rm",
        ...readdirSync(join(root, ".context"))
          .filter((f) => /^index-/.test(f))
          .map((f) => join(root, ".context", f)),
      ]);
      const aggregate = sh(
        join(SKILL_SCRIPTS, "context-ready.sh"),
        root,
        "--blocked",
      );
      expect(split.out).toBe(aggregate.out);
      expect(split.err).toBe(aggregate.err);
    });
  },
);

describe.skipIf(!existsSync(SIGNAL_CONTEXT))(
  "copy of the thoroc-signal .context",
  () => {
    test("default output is byte-identical to the baseline generator", () => {
      defaultIdentical(repoFromContext(SIGNAL_CONTEXT));
    });
  },
);

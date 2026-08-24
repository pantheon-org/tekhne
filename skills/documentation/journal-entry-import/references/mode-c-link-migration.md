# Mode C: External Link Migration (reference)

Historical entries (whether imported by Mode A or written any other way) carry links to systems that have since
moved: a code host migration (Bitbucket to GitLab, GitLab to GitHub, a self-hosted instance to a SaaS one), a
tracker instance change, a documentation site reorganisation. Make those links usable again **without**
rewriting history. Bitbucket to GitLab is one common example used throughout; the workflow applies to any
link-target migration.

## Principle: annotate, do not replace

Keep the original (archived) URL as the historical record and append a pointer to the current home:

```markdown
<https://old-host.example.com/projects/AP/repos/example-service/pull-requests/166/overview>
([now on GitLab: group/subgroup/example-service](https://gitlab.com/group/subgroup/example-service))
```

The original was true at the time; the annotation makes it findable today. Never silently swap the URL.

## Deep links do not translate; point to the repo root

A pull-request number on the old host is not a merge-request number on the new one, and commit/file links may
not survive a migration. Resolve to the **repo root** on the new host and keep the original deep path visible as
the archived link. Do not fabricate a merge-request number or assume a commit SHA survived the move.

## Resolving the new home (per distinct repo)

1. Search the target group/org by the repo's name **and** obvious renames (hyphen/underscore swaps, dropped or
   added prefixes, a rebrand of the project name).
2. Search the **whole target group**, not just the top level - migrated repos are often nested in deep
   subgroups, not sitting at the group root.
3. Check code-holding aggregates and notebook-style repos (a shared scripts/notebooks repo) for a folder or
   notebook matching a one-off script that never had its own repo.
4. A repo-mapping manifest (e.g. a `.meta`-style super-repo, or any file that lists sub-repos by their own git
   URL) that still points at the **old** host's URL means that repo was **not** migrated - it is not a home on
   the new host. Only a real folder/file/project actually present on the target host counts as a resolved home.

## Confidence and scope

Rate each resolution: exact/clear-rename = high; plausible single candidate = medium; uncertain = low; not found
= none. Annotate high/medium/low (mark medium/low "best-effort"); leave "none" as an archived link only, with no
annotation. Get sign-off on the mapping table before editing when the set of links is large.

## Applying safely

- Handle every markdown form: autolink `<url>`, link target `[text](url)`, and bare url. Never insert the
  annotation inside a link's display text `[url]` - annotate the target, or after the autolink's closing `>`.
- Be idempotent: skip a URL that is already followed by its resolved target, so a re-run does not double-annotate.
- After editing, run prettier + markdownlint (long URLs can trip line-length; keep the annotation on its own
  wrapped line) and re-validate every touched entry.

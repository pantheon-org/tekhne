# Question Dimensions

The default question set for a tech evaluation is five dimensions. Each MUST be answerable with cited evidence, not
opinion -- a dimension with no evidence attached is a gap in the evaluation, not a shortcut.

## The default five

| Dimension | Question | Typical evidence |
| --- | --- | --- |
| `correctness` | Does it produce correct output for this specific use case (escaping, encoding, type handling)? | A minimal reproduction run against the actual input shape, or a source read confirming the escaping behaviour. |
| `integration_fit` | Does its API/output shape map cleanly onto what the calling code needs? | The call site the library would replace, cited by file and line. |
| `footprint` | What does adopting it cost in dependencies, bundle size, build/runtime constraints? | `npm ls`/bundle-analyzer output, or the project's own bundling config (e.g. `archive_file`, `bun build`). |
| `maturity` | Is it actively maintained, adopted, and free of known security advisories? | Last release date, open CVE count, download counts -- cited with a URL and a date, since "actively maintained" decays. |
| `practical_fit` | Does the tool's complexity match the actual scale of the problem? | A comparison of the library's feature surface against the actual requirement (e.g. a templating engine's escaping modes vs. this project only ever needing one). |

## Substituting a dimension

The schema requires at least one question, not exactly five. Substitute or add a dimension when the default five
don't fit the technology under review:

- **File format or spec choice** (e.g. YAML vs. TOML vs. JSON5): replace `integration_fit` with `tooling` --
  editor support, linter availability, and schema-validation tooling matter more than an API shape.
- **Protocol or wire-format choice**: add an `interoperability` dimension -- does every consumer already speak
  this format, or does it require a new client library somewhere else in the stack.
- **A hand-rolled alternative is on the table**: keep all five, but add a `maintenance_burden` dimension comparing
  the cost of maintaining bespoke code against the cost of a dependency.

## A gotcha this skill exists to prevent

The single most common failure mode in tech evaluations is accepting a subagent's claim about *this* repository's
own constraints (a dependency pin, an existing convention, a call site) without checking the source. A generic claim
about "how crate X usually behaves" is not evidence about this project's actual `Cargo.toml`/`Cargo.lock` state.
Every dimension whose evidence concerns this repository specifically MUST be checked against the actual file, not
answered from the subagent's training data.

package reporter

import (
	"fmt"
	"sort"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// dimensionAdvice holds generic improvement advice for each dimension when
// it scores below maximum and no specific diagnostic covers the gap.
var dimensionAdvice = map[string]string{
	"knowledgeDelta":          "Add expert-signal keywords: NEVER, ALWAYS, production, gotcha, pitfall, anti-pattern. Remove beginner-oriented patterns (npm install, getting started, hello world).",
	"mindsetProcedures":       "Add a `## Mindset` or `## Philosophy` section. Use numbered procedure lists. Add `## When to Use` and `## When NOT to Use` sections.",
	"antiPatternQuality":      "Add NEVER statements paired with `WHY:` explanations. Include BAD/GOOD contrast examples.",
	"specificationCompliance": "Expand the `description` frontmatter to >100 characters. Ensure no harness-specific paths, agent references, or `../` escapes outside code blocks.",
	"progressiveDisclosure":   "Add a `references/` directory with focused deep-dive `.md` files. Keep `SKILL.md` under 150 lines to maximise the score.",
	"freedomCalibration":      "Balance prescriptive language (NEVER/ALWAYS) with permissive alternatives (consider, optionally, may).",
	"patternRecognition":      "Expand the `description` frontmatter field to more than 15 qualifying words (words longer than 3 characters).",
	"practicalUsability":      "Add more fenced code blocks (aim for >5 pairs). Include `./` or `bun run` commands. Use language-tagged fences (```bash, ```typescript).",
	"evalValidation":          "Create an `evals/` directory with `instructions.json`, `summary.json`, and at least 3 scenario subdirectories each containing `task.md`, `criteria.json` (checklist summing to 100), and `capability.txt`.",
}

type gap struct {
	key   string
	label string
	score int
	max   int
}

// Remediation returns a markdown prioritised action plan derived from a Result.
func Remediation(r *scorer.Result) string {
	var sb strings.Builder

	fmt.Fprintf(&sb, "# Remediation Plan — %s\n\n", r.Skill)
	fmt.Fprintf(&sb, "**Current Grade:** %s (%d/%d)\n\n", r.Grade, r.Total, r.MaxTotal)

	// Build gaps sorted by points available (largest first).
	gaps := make([]gap, 0, len(dimensionOrder))
	for _, d := range dimensionOrder {
		score, ok := r.Dimensions[d.key]
		if !ok {
			continue
		}
		if score < d.max {
			gaps = append(gaps, gap{key: d.key, label: d.label, score: score, max: d.max})
		}
	}
	sort.Slice(gaps, func(i, j int) bool {
		return (gaps[i].max - gaps[i].score) > (gaps[j].max - gaps[j].score)
	})

	if len(gaps) == 0 {
		fmt.Fprintf(&sb, "All dimensions are at maximum score. Nothing to remediate.\n")
		return sb.String()
	}

	fmt.Fprintf(&sb, "## Priority Actions\n\n")

	// Collect diagnostics keyed by dimension label prefix (e.g. "D9").
	diagsByDim := map[string][]scorer.Diagnostic{}
	for _, d := range r.ErrorDetails {
		diagsByDim[d.Dimension] = append(diagsByDim[d.Dimension], d)
	}
	for _, d := range r.WarningDetails {
		diagsByDim[d.Dimension] = append(diagsByDim[d.Dimension], d)
	}

	for _, g := range gaps {
		available := g.max - g.score
		fmt.Fprintf(&sb, "### %s (%d/%d) — %d pt%s available\n\n", g.label, g.score, g.max, available, plural(available))

		// Emit specific diagnostics for this dimension first.
		dimKey := dimLabelToCode(g.label)
		if diags, ok := diagsByDim[dimKey]; ok {
			for _, d := range diags {
				prefix := "⚠️"
				if d.Severity() == "error" {
					prefix = "🔴"
				}
				fmt.Fprintf(&sb, "%s %s\n\n", prefix, d.Message)
			}
		}

		// Fall back to generic advice.
		if advice, ok := dimensionAdvice[g.key]; ok {
			fmt.Fprintf(&sb, "%s\n\n", advice)
		}
	}

	return sb.String()
}

func plural(n int) string {
	if n == 1 {
		return ""
	}
	return "s"
}

// dimLabelToCode maps a dimension display label back to its D-code for diagnostic lookup.
var dimLabelToCode = func() func(string) string {
	m := map[string]string{
		"Knowledge Delta":          "D1",
		"Mindset + Procedures":     "D2",
		"Anti-Pattern Quality":     "D3",
		"Specification Compliance": "D4",
		"Progressive Disclosure":   "D5",
		"Freedom Calibration":      "D6",
		"Pattern Recognition":      "D7",
		"Practical Usability":      "D8",
		"Eval Validation":          "D9",
	}
	return func(label string) string { return m[label] }
}()

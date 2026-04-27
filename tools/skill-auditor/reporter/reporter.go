package reporter

import (
	"fmt"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// dimensionOrder defines the canonical D1–D9 display order with names.
var dimensionOrder = []struct {
	key  string
	name string
}{
	{"D1", "Knowledge Delta"},
	{"D2", "Mindset + Procedures"},
	{"D3", "Anti-Pattern Quality"},
	{"D4", "Specification Compliance"},
	{"D5", "Progressive Disclosure"},
	{"D6", "Freedom Calibration"},
	{"D7", "Pattern Recognition"},
	{"D8", "Practical Usability"},
	{"D9", "Eval Validation"},
}

// Format returns a human-readable representation of a Result.
func Format(r *scorer.Result) string {
	var sb strings.Builder

	fmt.Fprintf(&sb, "Skill: %s\n", r.SkillPath)
	fmt.Fprintf(&sb, "Grade: %s (%d/%d)\n", r.Grade, r.Total, r.MaxTotal)
	fmt.Fprintf(&sb, "\nDimensions:\n")

	for _, d := range dimensionOrder {
		dim, ok := r.Dimensions[d.key]
		if !ok {
			continue
		}
		fmt.Fprintf(&sb, "  %-3s %-28s %2d/%d\n", d.key, d.name, dim.Score, dim.Max)
	}

	fmt.Fprintf(&sb, "\nErrors: %d  Warnings: %d\n", r.Errors, r.Warnings)

	return sb.String()
}

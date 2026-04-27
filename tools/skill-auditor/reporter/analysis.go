package reporter

import (
	"fmt"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// Analysis returns a markdown summary of a Result suitable for writing to Analysis.md.
func Analysis(r *scorer.Result) string {
	var sb strings.Builder

	fmt.Fprintf(&sb, "# Skill Audit — %s\n\n", r.Skill)
	fmt.Fprintf(&sb, "**Grade:** %s (%d/%d)\n\n", r.Grade, r.Total, r.MaxTotal)

	fmt.Fprintf(&sb, "## Dimension Scores\n\n")
	fmt.Fprintf(&sb, "| Dimension | Score | Max |\n")
	fmt.Fprintf(&sb, "|---|---|---|\n")
	for _, d := range dimensionOrder {
		score, ok := r.Dimensions[d.key]
		if !ok {
			continue
		}
		fmt.Fprintf(&sb, "| %s | %d | %d |\n", d.label, score, d.max)
	}

	if len(r.ErrorDetails) > 0 || len(r.WarningDetails) > 0 {
		fmt.Fprintf(&sb, "\n## Diagnostics\n")
		if len(r.ErrorDetails) > 0 {
			fmt.Fprintf(&sb, "\n### Errors\n\n")
			for _, d := range r.ErrorDetails {
				fmt.Fprintf(&sb, "- **%s** %s\n", d.Dimension, d.Message)
			}
		}
		if len(r.WarningDetails) > 0 {
			fmt.Fprintf(&sb, "\n### Warnings\n\n")
			for _, d := range r.WarningDetails {
				fmt.Fprintf(&sb, "- **%s** %s\n", d.Dimension, d.Message)
			}
		}
	} else {
		fmt.Fprintf(&sb, "\n## Diagnostics\n\nNo errors or warnings.\n")
	}

	return sb.String()
}

package reporter

import (
	"fmt"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// dimensionOrder defines the canonical display order with camelCase keys and labels.
var dimensionOrder = []struct {
	key  string
	label string
	max  int
}{
	{"knowledgeDelta", "Knowledge Delta", 20},
	{"mindsetProcedures", "Mindset + Procedures", 15},
	{"antiPatternQuality", "Anti-Pattern Quality", 15},
	{"specificationCompliance", "Specification Compliance", 15},
	{"progressiveDisclosure", "Progressive Disclosure", 15},
	{"freedomCalibration", "Freedom Calibration", 15},
	{"patternRecognition", "Pattern Recognition", 10},
	{"practicalUsability", "Practical Usability", 15},
	{"evalValidation", "Eval Validation", 20},
}

// Format returns a human-readable representation of a Result.
func Format(r *scorer.Result) string {
	var sb strings.Builder

	fmt.Fprintf(&sb, "Skill: %s\n", r.Skill)
	fmt.Fprintf(&sb, "Grade: %s (%d/%d)\n", r.Grade, r.Total, r.MaxTotal)
	fmt.Fprintf(&sb, "\nDimensions:\n")

	for _, d := range dimensionOrder {
		score, ok := r.Dimensions[d.key]
		if !ok {
			continue
		}
		fmt.Fprintf(&sb, "  %-28s %2d/%d\n", d.label, score, d.max)
	}

	fmt.Fprintf(&sb, "\nErrors: %d  Warnings: %d\n", r.Errors, r.Warnings)

	if len(r.ErrorDetails) > 0 || len(r.WarningDetails) > 0 {
		fmt.Fprintf(&sb, "\nDiagnostics:\n")
		for _, d := range r.ErrorDetails {
			fmt.Fprintf(&sb, "  [E] %-3s %s\n", d.Dimension, d.Message)
		}
		for _, d := range r.WarningDetails {
			fmt.Fprintf(&sb, "  [W] %-3s %s\n", d.Dimension, d.Message)
		}
	}

	return sb.String()
}

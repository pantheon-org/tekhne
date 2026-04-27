package reporter

import (
	"strings"
	"testing"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

func makeResult() *scorer.Result {
	return &scorer.Result{
		Skill:    "agentic-harness/skill-quality-auditor",
		Total:    122,
		MaxTotal: 140,
		Grade:    "B+",
		Dimensions: map[string]int{
			"knowledgeDelta":          14,
			"mindsetProcedures":       12,
			"antiPatternQuality":      9,
			"specificationCompliance": 13,
			"progressiveDisclosure":   15,
			"freedomCalibration":      12,
			"patternRecognition":      9,
			"practicalUsability":      13,
			"evalValidation":          15,
		},
		Errors:   0,
		Warnings: 0,
	}
}

func TestFormatBasic(t *testing.T) {
	r := makeResult()
	out := Format(r)

	checks := []string{
		"Skill: agentic-harness/skill-quality-auditor",
		"Grade: B+ (122/140)",
		"Knowledge Delta",
		"Mindset + Procedures",
		"Anti-Pattern Quality",
		"Specification Compliance",
		"Progressive Disclosure",
		"Freedom Calibration",
		"Pattern Recognition",
		"Practical Usability",
		"Eval Validation",
		"Errors: 0  Warnings: 0",
	}

	for _, want := range checks {
		if !strings.Contains(out, want) {
			t.Errorf("Format output missing %q\ngot:\n%s", want, out)
		}
	}
}

func TestFormatDimensionOrder(t *testing.T) {
	r := makeResult()
	out := Format(r)

	labels := []string{
		"Knowledge Delta",
		"Mindset + Procedures",
		"Anti-Pattern Quality",
		"Specification Compliance",
		"Progressive Disclosure",
		"Freedom Calibration",
		"Pattern Recognition",
		"Practical Usability",
		"Eval Validation",
	}

	pos := -1
	for _, label := range labels {
		idx := strings.Index(out, label)
		if idx < 0 {
			t.Errorf("dimension %q not found in output", label)
			continue
		}
		if idx <= pos {
			t.Errorf("dimension %q appears before previous (pos %d <= %d)", label, idx, pos)
		}
		pos = idx
	}
}

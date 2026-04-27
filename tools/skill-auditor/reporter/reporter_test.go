package reporter

import (
	"strings"
	"testing"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

func makeResult() *scorer.Result {
	return &scorer.Result{
		SkillPath: "agentic-harness/skill-quality-auditor",
		Total:     122,
		MaxTotal:  140,
		Grade:     "B+",
		Dimensions: map[string]scorer.DimensionScore{
			"D1": {Score: 14, Max: 20},
			"D2": {Score: 12, Max: 15},
			"D3": {Score: 9, Max: 15},
			"D4": {Score: 13, Max: 15},
			"D5": {Score: 15, Max: 15},
			"D6": {Score: 12, Max: 15},
			"D7": {Score: 9, Max: 10},
			"D8": {Score: 13, Max: 15},
			"D9": {Score: 15, Max: 20},
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
		"D1",
		"D2",
		"D3",
		"D4",
		"D5",
		"D6",
		"D7",
		"D8",
		"D9",
		"Knowledge Delta",
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

	keys := []string{"D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9"}

	pos := -1
	for _, key := range keys {
		idx := strings.Index(out, key)
		if idx < 0 {
			t.Errorf("dimension %q not found in output", key)
			continue
		}
		if idx <= pos {
			t.Errorf("dimension %q appears before the previous dimension (pos %d <= %d)", key, idx, pos)
		}
		pos = idx
	}
}

package scorer

import (
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

// nilBridge is a convenience for unit tests that don't exercise the library path.
func nilBridge() *validatorBridge { return &validatorBridge{} }

func TestD2_WhenToUse(t *testing.T) {
	content := "---\ndescription: x\n---\n## When to Use\nuse it when needed\n\n## When NOT to Use\nnot this time"
	score := scoreD2(content, nilBridge())
	// 0 + 4 (when to use) + 3 (when not to) = 7
	if score != 7 {
		t.Errorf("want 7, got %d", score)
	}
}

func TestD2_MindsetHeading(t *testing.T) {
	content := "---\ndescription: x\n---\n## Mindset\nthink carefully"
	score := scoreD2(content, nilBridge())
	if score != 2 {
		t.Errorf("want 2, got %d", score)
	}
}

func TestD2_PhilosophyHeading(t *testing.T) {
	content := "---\ndescription: x\n---\n## Philosophy\nthink this way"
	score := scoreD2(content, nilBridge())
	if score != 2 {
		t.Errorf("want 2, got %d", score)
	}
}

func TestD2_LibraryImperativeRatio(t *testing.T) {
	b := &validatorBridge{Content: &types.ContentReport{ImperativeRatio: 0.45, ListItemCount: 5}}
	content := "---\ndescription: x\n---\n## When to Use\ndo this"
	score := scoreD2(content, b)
	// 4 (imperative ≥0.4) + 2 (listItems>3) + 4 (when to use) = 10
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

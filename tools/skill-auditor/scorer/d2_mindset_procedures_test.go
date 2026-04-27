package scorer

import "testing"

func TestD2_Full(t *testing.T) {
	content := "---\ndescription: x\n---\n## Mindset\nthink carefully\n\n1. first\n2. second\n\n## When to Use\nuse it when needed\n\n## When NOT to Use\nnot this time"
	score := scoreD2(content)
	// 8 + 2 (mindset) + 2 (numbered) + 2 (when to use) + 1 (when not to) = 15
	if score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD2_Minimal(t *testing.T) {
	content := "---\ndescription: x\n---\n# Skill\nJust a sentence."
	score := scoreD2(content)
	if score != 8 {
		t.Errorf("want 8, got %d", score)
	}
}

func TestD2_PhilosophyHeading(t *testing.T) {
	content := "---\ndescription: x\n---\n## Philosophy\nthink this way"
	score := scoreD2(content)
	// 8 + 2 (philosophy heading) = 10
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

package scorer

import "testing"

func TestD7_LongDescription(t *testing.T) {
	content := "---\ndescription: Validates sanitizes processes transforms normalizes parses serializes deserializes validates enriches computes aggregates filters projects sorts groups\n---\n# x"
	score, _ := scoreD7(content)
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD7_MidDescription(t *testing.T) {
	// 11-15 qualifying words → 9
	content := "---\ndescription: Validates sanitizes processes transforms normalizes parses serializes deserializes validates enriches computes aggregates\n---\n# x"
	score, diags := scoreD7(content)
	if score != 9 {
		t.Errorf("want 9, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
}

func TestD7_ShortDescription(t *testing.T) {
	// 6-10 qualifying words → 8
	content := "---\ndescription: Validates sanitizes processes transforms normalizes parses\n---\n# x"
	score, diags := scoreD7(content)
	if score != 8 {
		t.Errorf("want 8, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics for 6-word description")
	}
}

func TestD7_MissingDescription(t *testing.T) {
	content := "---\nname: test\n---\n# Skill"
	score, diags := scoreD7(content)
	// 0 qualifying words → 6 + warning
	if score != 6 {
		t.Errorf("want 6, got %d", score)
	}
	if len(diags) == 0 {
		t.Error("expected D7 warning for missing/short description")
	}
}

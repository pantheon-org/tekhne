package scorer

import "testing"

func TestD6_HighNeverAlways(t *testing.T) {
	content := "NEVER a. NEVER b. NEVER c. ALWAYS d. ALWAYS e. ALWAYS f. consider this. optionally that."
	score := scoreD6(content)
	// 10 + 3 (>5) + 2 (consider/optionally) = 15
	if score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD6_Minimal(t *testing.T) {
	content := "just some text"
	if score := scoreD6(content); score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD6_MidRangePrescriptive(t *testing.T) {
	// 3 NEVER/ALWAYS → +2; no permissive → 10+2=12
	content := "NEVER a. ALWAYS b. NEVER c."
	if score := scoreD6(content); score != 12 {
		t.Errorf("want 12, got %d", score)
	}
}

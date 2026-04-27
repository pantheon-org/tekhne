package scorer

import (
	"os"
	"path/filepath"
	"testing"
)

func TestD1_Penalties(t *testing.T) {
	content := "---\ndescription: something\n---\n# Skill\ngetting started with npm install"
	score, _ := scoreD1(content, t.TempDir())
	// start=15, -2 for "getting started", -2 for "npm install" = 11
	if score != 11 {
		t.Errorf("want 11, got %d", score)
	}
}

func TestD1_Rewards(t *testing.T) {
	content := "---\ndescription: something\n---\n# Skill\nNEVER do this. ALWAYS validate. anti-pattern here. production gotcha pitfall."
	score, _ := scoreD1(content, t.TempDir())
	// start=15, +1 each for NEVER, ALWAYS, anti-pattern, production, gotcha, pitfall = 21 → capped 20
	if score != 20 {
		t.Errorf("want 20, got %d", score)
	}
}

func TestD1_ExpertRatioBonus(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	// 8 of 10 instructions are "new knowledge" or "preference" → ratio 80% → +2
	instrJSON := `{"instructions":[` +
		`{"why_given":"new knowledge"},{"why_given":"new knowledge"},` +
		`{"why_given":"new knowledge"},{"why_given":"new knowledge"},` +
		`{"why_given":"preference"},{"why_given":"preference"},` +
		`{"why_given":"preference"},{"why_given":"preference"},` +
		`{"why_given":"other"},{"why_given":"other"}` +
		`]}`
	if err := os.WriteFile(filepath.Join(evalsDir, "instructions.json"), []byte(instrJSON), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: x\n---\n# Skill\nsome content"
	score, diags := scoreD1(content, tmpDir)
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
	// base 15 + 2 (expert ratio ≥70%) = 17
	if score != 17 {
		t.Errorf("want 17, got %d", score)
	}
}

func TestD1_ExpertRatioPenalty(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	// 2 of 10 are expert → ratio 20% → -2
	instrJSON := `{"instructions":[` +
		`{"why_given":"new knowledge"},{"why_given":"other"},` +
		`{"why_given":"other"},{"why_given":"other"},{"why_given":"other"},` +
		`{"why_given":"other"},{"why_given":"other"},{"why_given":"other"},` +
		`{"why_given":"other"},{"why_given":"other"}` +
		`]}`
	if err := os.WriteFile(filepath.Join(evalsDir, "instructions.json"), []byte(instrJSON), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: x\n---\n# Skill\nsome content"
	score, _ := scoreD1(content, tmpDir)
	// base 15 - 2 (ratio < 30%) = 13
	if score != 13 {
		t.Errorf("want 13, got %d", score)
	}
}

func TestD1_InstructionsParseError(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(evalsDir, "instructions.json"), []byte("not json"), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: x\n---\n# Skill"
	_, diags := scoreD1(content, tmpDir)
	if len(diags) != 1 || diags[0].severity != "error" || diags[0].Dimension != "D1" {
		t.Errorf("expected D1 error diagnostic, got %v", diags)
	}
}

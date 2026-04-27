package scorer

import (
	"os"
	"path/filepath"
	"testing"
)

func TestD3_NEVERAndWHY(t *testing.T) {
	content := "---\ndescription: x\n---\nNEVER do this. WHY: because reasons."
	score, _ := scoreD3(content, t.TempDir())
	// 8 + 1 (NEVER count=1) + 2 (WHY:) = 11
	if score != 11 {
		t.Errorf("want 11, got %d", score)
	}
}

func TestD3_NEVERHighCount(t *testing.T) {
	content := "---\ndescription: x\n---\nNEVER a. NEVER b. NEVER c. NEVER d. WHY: reasons. BAD example GOOD alternative."
	score, _ := scoreD3(content, t.TempDir())
	// 8 + 3 (>3 NEVERs) + 2 (WHY:) + 2 (BAD.*GOOD) = 15
	if score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD3_AntiPatternInstructionsBonus(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	// 5 instructions with NEVER/ALWAYS in original_snippets → +2
	instrJSON := `{"instructions":[
		{"original_snippets":"NEVER do this","content":"x"},
		{"original_snippets":"ALWAYS validate","content":"x"},
		{"original_snippets":"avoid this pattern","content":"x"},
		{"original_snippets":"do not use","content":"x"},
		{"original_snippets":"anti-pattern here","content":"x"}
	]}`
	if err := os.WriteFile(filepath.Join(evalsDir, "instructions.json"), []byte(instrJSON), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: x\n---\n# Skill"
	score, diags := scoreD3(content, tmpDir)
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
	// 8 + 2 (≥5 anti-pattern instructions) = 10
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD3_InstructionsParseError(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(evalsDir, "instructions.json"), []byte("{bad json"), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: x\n---\n# Skill"
	_, diags := scoreD3(content, tmpDir)
	if len(diags) != 1 || diags[0].severity != "error" || diags[0].Dimension != "D3" {
		t.Errorf("expected D3 error diagnostic, got %v", diags)
	}
}

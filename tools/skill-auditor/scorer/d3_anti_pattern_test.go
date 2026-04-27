package scorer

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

func TestD3_LibraryStrongMarkers(t *testing.T) {
	b := &validatorBridge{Content: &types.ContentReport{StrongMarkers: 10}}
	content := "---\ndescription: x\n---\nNEVER do this. WHY: because reasons."
	score, _ := scoreD3(content, t.TempDir(), b)
	// 0 + 5 (sm>8) + 2 (WHY:) + 2 (BAD/GOOD absent — ok) = library path: 5 + 2 (WHY:) = 7
	if score < 5 {
		t.Errorf("want ≥5 with strong markers, got %d", score)
	}
}

func TestD3_FallbackNEVERAndWHY(t *testing.T) {
	content := "---\ndescription: x\n---\nNEVER do this. WHY: because reasons."
	score, _ := scoreD3(content, t.TempDir(), nilBridge())
	// fallback: 0 + 1 (NEVER=1) + 2 (WHY:) = 3
	if score < 3 {
		t.Errorf("want ≥3 via fallback, got %d", score)
	}
}

func TestD3_BADGOOD(t *testing.T) {
	content := "---\ndescription: x\n---\nBAD: do this GOOD: do that instead. WHY: reasons."
	score, _ := scoreD3(content, t.TempDir(), nilBridge())
	if score < 4 {
		t.Errorf("want ≥4 (WHY: + BAD.*GOOD), got %d", score)
	}
}

func TestD3_AntiPatternInstructionsBonus(t *testing.T) {
	tmpDir := t.TempDir()
	evalsDir := filepath.Join(tmpDir, "evals")
	if err := os.MkdirAll(evalsDir, 0o755); err != nil {
		t.Fatal(err)
	}
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
	score, diags := scoreD3(content, tmpDir, nilBridge())
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
	if score < 2 {
		t.Errorf("want ≥2 (≥5 anti-pattern instructions bonus), got %d", score)
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
	_, diags := scoreD3(content, tmpDir, nilBridge())
	if len(diags) != 1 || diags[0].severity != "error" || diags[0].Dimension != "D3" {
		t.Errorf("expected D3 error diagnostic, got %v", diags)
	}
}

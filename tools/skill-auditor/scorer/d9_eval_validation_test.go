package scorer

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestD9_NoEvalsDir(t *testing.T) {
	score, diags := scoreD9(filepath.Join(t.TempDir(), "nonexistent"))
	if score != 0 {
		t.Errorf("want 0, got %d", score)
	}
	if len(diags) != 1 || diags[0].Dimension != "D9" {
		t.Errorf("expected D9 warning, got %v", diags)
	}
}

func TestD9_FullScore(t *testing.T) {
	evalsDir := t.TempDir()
	writeTestFile(t, filepath.Join(evalsDir, "instructions.json"),
		`{"instructions":[{"type":"a"},{"type":"b"}]}`)
	writeTestFile(t, filepath.Join(evalsDir, "summary.json"),
		`{"instructions_coverage":{"coverage_percentage":85}}`)
	for i := 1; i <= 3; i++ {
		dir := filepath.Join(evalsDir, "scenario-"+string(rune('0'+i)))
		writeTestFile(t, filepath.Join(dir, "task.md"), "# Task")
		writeTestFile(t, filepath.Join(dir, "capability.txt"), "cap")
		writeTestFile(t, filepath.Join(dir, "criteria.json"),
			`{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":40}]}`)
	}
	score, _ := scoreD9(evalsDir)
	// 4 + 3 + 3 + 3 + 4 = 17
	if score != 17 {
		t.Errorf("want 17, got %d", score)
	}
}

func TestD9_LowCoverageWarning(t *testing.T) {
	evalsDir := t.TempDir()
	writeTestFile(t, filepath.Join(evalsDir, "summary.json"),
		`{"instructions_coverage":{"coverage_percentage":72}}`)
	_, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 warning for low coverage")
	}
}

func TestD9_InvalidInstructionsJSON(t *testing.T) {
	evalsDir := t.TempDir()
	writeTestFile(t, filepath.Join(evalsDir, "instructions.json"), "not json")
	_, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "error" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 error for invalid instructions.json")
	}
}

func TestD9_InvalidSummaryJSON(t *testing.T) {
	evalsDir := t.TempDir()
	writeTestFile(t, filepath.Join(evalsDir, "summary.json"), "{bad}")
	_, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "error" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 error for invalid summary.json")
	}
}

func TestD9_ScenarioMissingFiles(t *testing.T) {
	evalsDir := t.TempDir()
	// scenario dir with only task.md — missing criteria.json and capability.txt
	dir := filepath.Join(evalsDir, "scenario-1")
	writeTestFile(t, filepath.Join(dir, "task.md"), "# Task")
	_, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 warning for incomplete scenario")
	}
}

func TestD9_CriteriaNotSummingTo100(t *testing.T) {
	evalsDir := t.TempDir()
	dir := filepath.Join(evalsDir, "scenario-1")
	writeTestFile(t, filepath.Join(dir, "task.md"), "# Task")
	writeTestFile(t, filepath.Join(dir, "capability.txt"), "cap")
	writeTestFile(t, filepath.Join(dir, "criteria.json"),
		`{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":30}]}`)
	score, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 warning for criteria not summing to 100")
	}
	// 0 valid scenarios → +0 scenario points
	if score > 10 {
		t.Errorf("expected no scenario bonus, got total %d", score)
	}
}

func TestD9_InvalidCriteriaJSON(t *testing.T) {
	evalsDir := t.TempDir()
	dir := filepath.Join(evalsDir, "scenario-1")
	writeTestFile(t, filepath.Join(dir, "task.md"), "# Task")
	writeTestFile(t, filepath.Join(dir, "capability.txt"), "cap")
	writeTestFile(t, filepath.Join(dir, "criteria.json"), "{bad json}")
	_, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "error" {
			found = true
		}
	}
	if !found {
		t.Error("expected D9 error for invalid criteria.json")
	}
}

func TestD9_FlatScenarioFormatWarning(t *testing.T) {
	evalsDir := t.TempDir()
	writeTestFile(t, filepath.Join(evalsDir, "scenario-01.md"), "# Scenario 1")
	writeTestFile(t, filepath.Join(evalsDir, "scenario-02.md"), "# Scenario 2")
	score, diags := scoreD9(evalsDir)
	found := false
	for _, d := range diags {
		if d.Dimension == "D9" && d.severity == "warning" && strings.Contains(d.Message, "flat scenario") {
			found = true
		}
	}
	if !found {
		t.Errorf("expected D9 flat-format warning, got diags: %v", diags)
	}
	// evals/ exists → +4, nothing else
	if score != 4 {
		t.Errorf("want score 4, got %d", score)
	}
}

func TestD9_CountValidScenariosWrapper(t *testing.T) {
	evalsDir := t.TempDir()
	for i := 1; i <= 2; i++ {
		dir := filepath.Join(evalsDir, "scenario-"+string(rune('0'+i)))
		writeTestFile(t, filepath.Join(dir, "task.md"), "# Task")
		writeTestFile(t, filepath.Join(dir, "capability.txt"), "cap")
		writeTestFile(t, filepath.Join(dir, "criteria.json"),
			`{"checklist":[{"max_score":100}]}`)
	}
	if n := countValidScenarios(evalsDir); n != 2 {
		t.Errorf("want 2, got %d", n)
	}
}

func TestParseCoveragePercentage(t *testing.T) {
	cases := []struct {
		input interface{}
		want  int
	}{
		{float64(85), 85},
		{int(72), 72},
		{"90%", 90},
		{"75.5%", 75},
		{"", -1},
		{nil, -1},
		{"abc", -1},
		{true, -1},
	}
	for _, tc := range cases {
		got := parseCoveragePercentage(tc.input)
		if got != tc.want {
			t.Errorf("parseCoveragePercentage(%v): want %d, got %d", tc.input, tc.want, got)
		}
	}
}

// writeTestFile creates parent directories and writes content to path.
func writeTestFile(t *testing.T, path, content string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", filepath.Dir(path), err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", path, err)
	}
}

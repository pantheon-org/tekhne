package reporter

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

func makeStoreResult() *scorer.Result {
	return &scorer.Result{
		SkillPath: "agentic-harness/skill-quality-auditor",
		Total:     122,
		MaxTotal:  140,
		Grade:     "B+",
		Dimensions: map[string]scorer.DimensionScore{
			"D1": {Score: 14, Max: 20},
		},
		Errors:   0,
		Warnings: 0,
	}
}

func TestStoreCreatesFile(t *testing.T) {
	root := t.TempDir()
	skillPath := "agentic-harness/skill-quality-auditor"
	r := makeStoreResult()

	if err := Store(root, skillPath, r); err != nil {
		t.Fatalf("Store returned error: %v", err)
	}

	date := time.Now().Format("2006-01-02")
	expected := filepath.Join(root, ".context", "audits", skillPath, date, "audit.json")

	if _, err := os.Stat(expected); os.IsNotExist(err) {
		t.Errorf("expected file not created at %s", expected)
	}
}

func TestStoreContent(t *testing.T) {
	root := t.TempDir()
	skillPath := "agentic-harness/skill-quality-auditor"
	r := makeStoreResult()

	if err := Store(root, skillPath, r); err != nil {
		t.Fatalf("Store returned error: %v", err)
	}

	date := time.Now().Format("2006-01-02")
	dest := filepath.Join(root, ".context", "audits", skillPath, date, "audit.json")

	data, err := os.ReadFile(dest)
	if err != nil {
		t.Fatalf("failed to read audit.json: %v", err)
	}

	// Must be valid JSON.
	var got scorer.Result
	if err := json.Unmarshal(data, &got); err != nil {
		t.Fatalf("audit.json is not valid JSON: %v\ncontent: %s", err, data)
	}

	// Check key fields.
	if got.SkillPath != skillPath {
		t.Errorf("skill_path: got %q, want %q", got.SkillPath, skillPath)
	}
	if got.Total != 122 {
		t.Errorf("total: got %d, want 122", got.Total)
	}
	if got.Grade != "B+" {
		t.Errorf("grade: got %q, want B+", got.Grade)
	}

	// File must use 2-space indent (pretty-printed).
	if !strings.Contains(string(data), "  ") {
		t.Errorf("audit.json does not appear to use indented JSON")
	}
}

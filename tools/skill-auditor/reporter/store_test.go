package reporter

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

const testDate = "2026-01-01"

func makeStoreResult() *scorer.Result {
	return &scorer.Result{
		Skill:    "agentic-harness/skill-quality-auditor",
		Date:     testDate,
		Total:    122,
		MaxTotal: 140,
		Grade:    "B+",
		Dimensions: map[string]int{
			"knowledgeDelta": 14,
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

	expected := filepath.Join(root, ".context", "audits", skillPath, testDate, "audit.json")

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

	dest := filepath.Join(root, ".context", "audits", skillPath, testDate, "audit.json")

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
	if got.Skill != skillPath {
		t.Errorf("skill: got %q, want %q", got.Skill, skillPath)
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

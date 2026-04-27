package scorer

import (
	"os"
	"path/filepath"
	"testing"
)

func TestD4_GoodDescription(t *testing.T) {
	desc := "Validates and sanitizes user inputs before processing to prevent injection attacks and data corruption in production systems"
	content := "---\ndescription: " + desc + "\n---\n# Skill\nSee references/guide.md for details."
	score, _ := scoreD4(content, t.TempDir())
	if score < 10 {
		t.Errorf("want >= 10, got %d", score)
	}
}

func TestD4_HarnessPathWarning(t *testing.T) {
	content := "---\ndescription: does something useful\n---\n# Skill\nSee .claude/settings.json for config."
	_, diags := scoreD4(content, t.TempDir())
	found := false
	for _, d := range diags {
		if d.Dimension == "D4" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D4 warning for harness-specific path")
	}
}

func TestD4_AgentRefWarning(t *testing.T) {
	content := "---\ndescription: does something useful\n---\n# Skill\nThis works with Claude Code and cursor agent."
	_, diags := scoreD4(content, t.TempDir())
	found := false
	for _, d := range diags {
		if d.Dimension == "D4" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D4 warning for agent-specific reference")
	}
}

func TestD4_RelativePathViolation(t *testing.T) {
	content := "---\ndescription: does something useful\n---\n# Skill\nSee ../other-skill/SKILL.md for more."
	score, diags := scoreD4(content, t.TempDir())
	found := false
	for _, d := range diags {
		if d.Dimension == "D4" && d.severity == "warning" {
			found = true
		}
	}
	if !found {
		t.Error("expected D4 warning for ../ outside code blocks")
	}
	// score should be penalised
	if score > 13 {
		t.Errorf("expected score ≤ 13 due to ../ penalty, got %d", score)
	}
}

func TestD4_PenaltyFromDir_AbsPath(t *testing.T) {
	tmpDir := t.TempDir()
	scriptsDir := filepath.Join(tmpDir, "scripts")
	if err := os.MkdirAll(scriptsDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(scriptsDir, "run.sh"), []byte("#!/bin/sh\ncd skills/ci-cd/my-skill && run"), 0o644); err != nil {
		t.Fatal(err)
	}
	content := "---\ndescription: does something useful\n---\n# Skill\ncontent here"
	score1, _ := scoreD4(content, t.TempDir()) // no scripts dir
	score2, _ := scoreD4(content, tmpDir)       // scripts dir with abs path
	if score2 >= score1 {
		t.Errorf("expected penalty from scripts/ abs path: without=%d with=%d", score1, score2)
	}
}

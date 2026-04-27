package scorer

import (
	"strings"
	"testing"
)

func TestCountLines(t *testing.T) {
	content := "line one\n\nline three\nline four\n"
	if n := countLines(content); n != 3 {
		t.Errorf("want 3 non-empty lines, got %d", n)
	}
}

func TestErrDiag(t *testing.T) {
	d := errDiag("D1", "something failed")
	if d.Dimension != "D1" || d.Message != "something failed" || d.severity != "error" {
		t.Errorf("unexpected errDiag: %+v", d)
	}
}

func TestWarnDiag(t *testing.T) {
	d := warnDiag("D4", "missing ref")
	if d.Dimension != "D4" || d.Message != "missing ref" || d.severity != "warning" {
		t.Errorf("unexpected warnDiag: %+v", d)
	}
}

func TestCountPattern(t *testing.T) {
	if n := countPattern("NEVER do this. Never again.", "never"); n != 2 {
		t.Errorf("want 2, got %d", n)
	}
}

func TestRemoveCodeBlocks(t *testing.T) {
	content := "before\n```\ncode line\n```\nafter\n"
	result := removeCodeBlocks(content)
	if strings.Contains(result, "code line") {
		t.Error("removeCodeBlocks should strip code block contents")
	}
	if !strings.Contains(result, "before") || !strings.Contains(result, "after") {
		t.Error("removeCodeBlocks should preserve non-code content")
	}
}

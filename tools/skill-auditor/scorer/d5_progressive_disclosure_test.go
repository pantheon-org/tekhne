package scorer

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestD5_WithRefsShort(t *testing.T) {
	content := makeLines(50)
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	if score := scoreD5(content, tmpDir); score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD5_WithRefsMedium(t *testing.T) {
	content := makeLines(120)
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	// 123 total lines → 100 ≤ x < 150 → 13
	if score := scoreD5(content, tmpDir); score != 13 {
		t.Errorf("want 13, got %d", score)
	}
}

func TestD5_WithRefsLarge(t *testing.T) {
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	cases := []struct {
		lines int
		want  int
	}{
		{195, 11}, // 195+3 frontmatter = 198 → 150 ≤ x < 200 → 11
		{210, 10}, // 210+3 = 213 → ≥200 → 10
	}
	for _, tc := range cases {
		content := makeLines(tc.lines)
		score := scoreD5(content, tmpDir)
		if score != tc.want {
			t.Errorf("lines=%d: want %d, got %d", tc.lines, tc.want, score)
		}
	}
}

func TestD5_NoRefsShort(t *testing.T) {
	content := "---\ndescription: x\n---\n# Short skill"
	if score := scoreD5(content, t.TempDir()); score != 12 {
		t.Errorf("want 12, got %d", score)
	}
}

func TestD5_NoRefsMedium(t *testing.T) {
	// 200-299 lines without refs → 10
	content := makeLines(250)
	if score := scoreD5(content, t.TempDir()); score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD5_NoRefsLarge(t *testing.T) {
	// 300-499 lines → 7
	content := makeLines(350)
	if score := scoreD5(content, t.TempDir()); score != 7 {
		t.Errorf("want 7, got %d", score)
	}
}

func TestD5_NoRefsVeryLarge(t *testing.T) {
	// 500+ lines → 5
	content := makeLines(510)
	if score := scoreD5(content, t.TempDir()); score != 5 {
		t.Errorf("want 5, got %d", score)
	}
}

// makeLines returns content with n body lines plus a 3-line frontmatter header.
func makeLines(n int) string {
	var sb strings.Builder
	sb.WriteString("---\ndescription: x\n---\n")
	for i := 0; i < n; i++ {
		sb.WriteString("line content here\n")
	}
	return sb.String()
}

func writeRefFile(skillDir, name, content string) error {
	dir := filepath.Join(skillDir, "references")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(dir, name), []byte(content), 0o644)
}

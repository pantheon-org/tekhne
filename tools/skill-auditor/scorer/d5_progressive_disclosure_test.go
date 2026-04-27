package scorer

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

func TestD5_WithRefsShort(t *testing.T) {
	content := makeLines(50)
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	// 50+3 = 53 lines < 100 → 15
	if score := scoreD5(content, tmpDir, nilBridge()); score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD5_WithRefsMedium(t *testing.T) {
	content := makeLines(120)
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	// 120+3 = 123 lines: 100 ≤ x < 150 → 13
	if score := scoreD5(content, tmpDir, nilBridge()); score != 13 {
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
		{160, 11}, // 160+3 = 163: 150 ≤ x < 200 → 11
		{210, 10}, // 210+3 = 213: ≥200 → 10
	}
	for _, tc := range cases {
		content := makeLines(tc.lines)
		score := scoreD5(content, tmpDir, nilBridge())
		if score != tc.want {
			t.Errorf("lines=%d: want %d, got %d", tc.lines, tc.want, score)
		}
	}
}

func TestD5_NoRefsShort(t *testing.T) {
	content := "---\ndescription: x\n---\n# Short skill"
	// 4 lines < 200 → 12
	if score := scoreD5(content, t.TempDir(), nilBridge()); score != 12 {
		t.Errorf("want 12, got %d", score)
	}
}

func TestD5_NoRefsMedium(t *testing.T) {
	// 250+3 = 253 lines: 200 ≤ x < 300 → 10
	content := makeLines(250)
	if score := scoreD5(content, t.TempDir(), nilBridge()); score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD5_NoRefsLarge(t *testing.T) {
	// 350+3 = 353 lines: 300 ≤ x < 500 → 7
	content := makeLines(350)
	if score := scoreD5(content, t.TempDir(), nilBridge()); score != 7 {
		t.Errorf("want 7, got %d", score)
	}
}

func TestD5_NoRefsVeryLarge(t *testing.T) {
	// 510+3 = 513 lines: ≥500 → 5
	content := makeLines(510)
	if score := scoreD5(content, t.TempDir(), nilBridge()); score != 5 {
		t.Errorf("want 5, got %d", score)
	}
}

func TestD5_TokenPath_WithRefs(t *testing.T) {
	tmpDir := t.TempDir()
	if err := writeRefFile(tmpDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	b := &validatorBridge{Content: &types.ContentReport{}}
	// Inject token count via a Report that has a TokenCount entry.
	// Since validatorBridge.skillMDTokens() reads b.Structure.TokenCounts,
	// we test the token path indirectly through scoreD5ByTokens directly.
	score, _, _, _ := scoreD5ByTokens(600, 50, 1, true)
	if score != 15 {
		t.Errorf("tokens=600 with refs: want 15, got %d", score)
	}
	score, _, _, _ = scoreD5ByTokens(1000, 120, 1, true)
	if score != 13 {
		t.Errorf("tokens=1000 with refs: want 13, got %d", score)
	}
	score, _, _, _ = scoreD5ByTokens(1400, 160, 1, true)
	if score != 11 {
		t.Errorf("tokens=1400 with refs: want 11, got %d", score)
	}
	score, _, _, _ = scoreD5ByTokens(2000, 200, 1, true)
	if score != 10 {
		t.Errorf("tokens=2000 with refs: want 10, got %d", score)
	}
	_ = b
}

func TestD5_TokenPath_NoRefs(t *testing.T) {
	cases := []struct {
		tokens int
		want   int
	}{
		{800, 12},
		{1500, 10},
		{3000, 7},
		{5000, 5},
	}
	for _, tc := range cases {
		score, _, _, _ := scoreD5ByTokens(tc.tokens, 100, 0, false)
		if score != tc.want {
			t.Errorf("tokens=%d no refs: want %d, got %d", tc.tokens, tc.want, score)
		}
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

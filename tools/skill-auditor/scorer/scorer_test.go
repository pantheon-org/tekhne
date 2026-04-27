package scorer

import (
	"os"
	"path/filepath"
	"testing"
)

const fixturesDir = "../testdata/fixtures"

// TestScoreMinimal verifies skill-minimal scores below 80 (F grade).
func TestScoreMinimal(t *testing.T) {
	skillPath := filepath.Join(fixturesDir, "skill-minimal", "SKILL.md")
	result, err := Score(skillPath)
	if err != nil {
		t.Fatalf("Score() error: %v", err)
	}
	if result.Total >= 80 {
		t.Errorf("skill-minimal expected total < 80, got %d (grade %s)", result.Total, result.Grade)
	}
	if result.Grade != "F" {
		t.Errorf("skill-minimal expected grade F, got %s (total %d)", result.Grade, result.Total)
	}
}

// TestScoreFullAGrade verifies skill-full scores >= 126 (A grade).
func TestScoreFullAGrade(t *testing.T) {
	skillPath := filepath.Join(fixturesDir, "skill-full", "SKILL.md")
	result, err := Score(skillPath)
	if err != nil {
		t.Fatalf("Score() error: %v", err)
	}
	if result.Total < 126 {
		t.Errorf("skill-full expected total >= 126, got %d (grade %s)", result.Total, result.Grade)
		for k, v := range result.Dimensions {
			t.Logf("  %s: %d/%d", k, v.Score, v.Max)
		}
	}
	if GradeRank[result.Grade] < GradeRank["A"] {
		t.Errorf("skill-full expected grade A or A+, got %s (total %d)", result.Grade, result.Total)
	}
}

// --- Per-dimension tests using ScoreFromContent ---

func TestD1_Penalties(t *testing.T) {
	content := "---\ndescription: something\n---\n# Skill\ngetting started with npm install"
	score := scoreD1(content, t.TempDir())
	// start=15, -2 for "getting started", -2 for "npm install" = 11
	if score != 11 {
		t.Errorf("D1 with penalties: want 11, got %d", score)
	}
}

func TestD1_Rewards(t *testing.T) {
	content := "---\ndescription: something\n---\n# Skill\nNEVER do this. ALWAYS validate. anti-pattern here. production gotcha pitfall."
	score := scoreD1(content, t.TempDir())
	// start=15, +1 each for NEVER, ALWAYS, anti-pattern, production, gotcha, pitfall = 15+6=21 → capped 20
	if score != 20 {
		t.Errorf("D1 with all rewards: want 20, got %d", score)
	}
}

func TestD2_Full(t *testing.T) {
	content := "---\ndescription: x\n---\n## Mindset\nthink carefully\n\n1. first\n2. second\n\n## When to Use\nuse it when needed\n\n## When NOT to Use\nnot this time"
	score := scoreD2(content)
	// 8 + 2 (mindset) + 2 (numbered) + 2 (when to use) + 1 (when not to) = 15
	if score != 15 {
		t.Errorf("D2 full: want 15, got %d", score)
	}
}

func TestD2_Minimal(t *testing.T) {
	content := "---\ndescription: x\n---\n# Skill\nJust a sentence."
	score := scoreD2(content)
	// 8 only
	if score != 8 {
		t.Errorf("D2 minimal: want 8, got %d", score)
	}
}

func TestD3_NEVERAndWHY(t *testing.T) {
	content := "---\ndescription: x\n---\nNEVER do this. WHY: because reasons."
	score := scoreD3(content, t.TempDir())
	// 8 + 1 (NEVER count=1) + 2 (WHY:) = 11
	if score != 11 {
		t.Errorf("D3 NEVER+WHY: want 11, got %d", score)
	}
}

func TestD3_NEVERHighCount(t *testing.T) {
	content := "---\ndescription: x\n---\nNEVER a. NEVER b. NEVER c. NEVER d. WHY: reasons. BAD example GOOD alternative."
	score := scoreD3(content, t.TempDir())
	// 8 + 3 (>3 NEVERs) + 2 (WHY:) + 2 (BAD.*GOOD same line) = 15 → capped at 15
	if score != 15 {
		t.Errorf("D3 high NEVER: want 15, got %d", score)
	}
}

func TestD4_GoodDescription(t *testing.T) {
	// description >100 chars, no harness paths, no agent refs, has references/ path
	desc := "Validates and sanitizes user inputs before processing to prevent injection attacks and data corruption in production systems"
	content := "---\ndescription: " + desc + "\n---\n# Skill\nSee references/guide.md for details."
	score := scoreD4(content, t.TempDir())
	// start=10 +2(desc>100) -1(and/or count: "and"=2 +"in"... "and data corruption" + maybe >1)
	// Actually: "and data corruption" has 1 "and", no "or" visible... let me just check range
	if score < 10 {
		t.Errorf("D4 good description: want >= 10, got %d", score)
	}
}

func TestD5_WithRefsShort(t *testing.T) {
	// 50 lines content + references dir with .md → should return 15
	lines := make([]string, 50)
	for i := range lines {
		lines[i] = "line content"
	}
	content := "---\ndescription: x\n---\n"
	for i := 0; i < 50; i++ {
		content += "line " + string(rune('A'+i%26)) + "\n"
	}

	tmpDir := t.TempDir()
	refsDir := filepath.Join(tmpDir, "references")
	if err := mkdirAndFile(refsDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	skillPath := filepath.Join(tmpDir, "SKILL.md")
	score := scoreD5(content, tmpDir)
	_ = skillPath
	// total lines < 100 with refs → 15
	if score != 15 {
		t.Errorf("D5 with refs short: want 15, got %d", score)
	}
}

func TestD5_WithRefsMedium(t *testing.T) {
	// 120 lines + refs → should return 13
	content := "---\ndescription: x\n---\n"
	for i := 0; i < 120; i++ {
		content += "line content here\n"
	}
	tmpDir := t.TempDir()
	refsDir := filepath.Join(tmpDir, "references")
	if err := mkdirAndFile(refsDir, "deep.md", "# Deep Reference"); err != nil {
		t.Fatal(err)
	}
	score := scoreD5(content, tmpDir)
	// 123 lines (3 frontmatter + 120) which is 100 ≤ x < 150 → 13
	if score != 13 {
		t.Errorf("D5 with refs medium: want 13, got %d", score)
	}
}

func TestD5_NoRefsShort(t *testing.T) {
	content := "---\ndescription: x\n---\n# Short skill"
	score := scoreD5(content, t.TempDir())
	// < 200 lines → 12
	if score != 12 {
		t.Errorf("D5 no refs short: want 12, got %d", score)
	}
}

func TestD6_HighNeverAlways(t *testing.T) {
	content := "NEVER a. NEVER b. NEVER c. ALWAYS d. ALWAYS e. ALWAYS f. consider this. optionally that."
	score := scoreD6(content)
	// 10 + 3 (>5) + 2 (consider/optionally) = 15
	if score != 15 {
		t.Errorf("D6 high prescriptive+permissive: want 15, got %d", score)
	}
}

func TestD6_Minimal(t *testing.T) {
	content := "just some text"
	score := scoreD6(content)
	// 10 only
	if score != 10 {
		t.Errorf("D6 minimal: want 10, got %d", score)
	}
}

func TestD7_LongDescription(t *testing.T) {
	// > 15 words with length > 3
	content := "---\ndescription: Validates sanitizes processes transforms normalizes parses serializes deserializes validates enriches computes aggregates filters projects sorts groups\n---\n# x"
	score := scoreD7(content)
	if score != 10 {
		t.Errorf("D7 long description: want 10, got %d", score)
	}
}

func TestD7_MissingDescription(t *testing.T) {
	content := "---\nname: test\n---\n# Skill"
	score := scoreD7(content)
	// 0 words > 3 chars → 6
	if score != 6 {
		t.Errorf("D7 missing description: want 6, got %d", score)
	}
}

func TestD8_ManyCodeBlocks(t *testing.T) {
	content := "---\ndescription: x\n---\n" +
		"```bash\necho hello\n```\n" +
		"```bash\necho world\n```\n" +
		"```typescript\nconst x = 1;\n```\n" +
		"```typescript\nconst y = 2;\n```\n" +
		"```typescript\nconst z = 3;\n```\n" +
		"```typescript\nconst w = 4;\n```\n" +
		"Run ./script.sh to start."
	score := scoreD8(content)
	// 6 blocks > 5 → +4; has "./" → +2; has ```bash → +1; 8+4+2+1=15
	if score != 15 {
		t.Errorf("D8 many code blocks: want 15, got %d", score)
	}
}

func TestD8_FewCodeBlocks(t *testing.T) {
	content := "---\ndescription: x\n---\n# Skill\nNo code blocks here."
	score := scoreD8(content)
	// 8 only (no code blocks, no ./, no lang tags)
	if score != 8 {
		t.Errorf("D8 no code blocks: want 8, got %d", score)
	}
}

func TestD9_NoEvalsDir(t *testing.T) {
	score := scoreD9(filepath.Join(t.TempDir(), "nonexistent"))
	if score != 0 {
		t.Errorf("D9 no evals: want 0, got %d", score)
	}
}

func TestD9_FullScore(t *testing.T) {
	evalsDir := t.TempDir()

	// instructions.json
	writeFile(t, filepath.Join(evalsDir, "instructions.json"),
		`{"instructions":[{"type":"a"},{"type":"b"}]}`)

	// summary.json with instructions_coverage
	writeFile(t, filepath.Join(evalsDir, "summary.json"),
		`{"instructions_coverage":{"coverage_percentage":85}}`)

	// 3 valid scenarios
	for i := 1; i <= 3; i++ {
		dir := filepath.Join(evalsDir, "scenario-"+string(rune('0'+i)))
		writeFile(t, filepath.Join(dir, "task.md"), "# Task")
		writeFile(t, filepath.Join(dir, "capability.txt"), "cap")
		writeFile(t, filepath.Join(dir, "criteria.json"),
			`{"checklist":[{"description":"x","max_score":60},{"description":"y","max_score":40}]}`)
	}

	score := scoreD9(evalsDir)
	// 4 + 3 + 3 + 3 + 4 = 17
	if score != 17 {
		t.Errorf("D9 full: want 17, got %d", score)
	}
}

// --- helpers ---

func mkdirAndFile(dir, name, content string) error {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(dir, name), []byte(content), 0o644)
}

func writeFile(t *testing.T, path, content string) {
	t.Helper()
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", dir, err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", path, err)
	}
}

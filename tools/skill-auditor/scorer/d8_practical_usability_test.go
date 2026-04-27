package scorer

import (
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

func TestD8_ManyCodeBlocks(t *testing.T) {
	// baseline 5 + 4 (>5 blocks via library) + 4 (run cmd ./) = 13; capped at 15
	b := &validatorBridge{Content: &types.ContentReport{
		CodeBlockCount: 6,
		CodeLanguages:  []string{"bash", "typescript"},
	}}
	content := "---\ndescription: x\n---\nRun ./script.sh to start."
	score := scoreD8(content, b)
	// 5 + 4 (>5) + 2 (languages) + 4 (./) = 15
	if score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD8_FewCodeBlocks_Fallback(t *testing.T) {
	// nilBridge → fallback path; no code blocks → baseline 5; no run cmd → 5
	content := "---\ndescription: x\n---\n# Skill\nNo code blocks here."
	if score := scoreD8(content, nilBridge()); score != 5 {
		t.Errorf("want 5, got %d", score)
	}
}

func TestD8_MediumCodeBlocks_Fallback(t *testing.T) {
	// nilBridge → fallback; 3 blocks → +2; no run cmd → 5+2=7
	content := "---\ndescription: x\n---\n" +
		"```\nfoo\n```\n```\nbar\n```\n```\nbaz\n```\n"
	if score := scoreD8(content, nilBridge()); score != 7 {
		t.Errorf("want 7, got %d", score)
	}
}

func TestD8_LibraryCodeBlockCount(t *testing.T) {
	cases := []struct {
		count int
		want  int
	}{
		{0, 5},
		{1, 6},  // 5 + 1
		{3, 7},  // 5 + 2
		{6, 9},  // 5 + 4
	}
	for _, tc := range cases {
		b := &validatorBridge{Content: &types.ContentReport{CodeBlockCount: tc.count}}
		content := "---\ndescription: x\n---\n# Skill"
		score := scoreD8(content, b)
		if score != tc.want {
			t.Errorf("CodeBlockCount=%d: want %d, got %d", tc.count, tc.want, score)
		}
	}
}

func TestD8_LibraryLanguageTags(t *testing.T) {
	// library path: 1 block + languages → 5+1+2 = 8
	b := &validatorBridge{Content: &types.ContentReport{
		CodeBlockCount: 1,
		CodeLanguages:  []string{"bash"},
	}}
	content := "---\ndescription: x\n---\n```bash\necho hi\n```\n"
	if score := scoreD8(content, b); score != 8 {
		t.Errorf("want 8, got %d", score)
	}
}

func TestD8_RunCommands(t *testing.T) {
	cases := []struct {
		name    string
		snippet string
	}{
		{"./", "Run ./build.sh to start."},
		{"npm run", "npm run build"},
		{"yarn", "yarn install"},
		{"pnpm run", "pnpm run test"},
		{"bun run", "bun run dev"},
		{"make", "make install"},
		{"python", "python main.py"},
		{"go run", "go run ./cmd/main.go"},
	}
	for _, tc := range cases {
		content := "---\ndescription: x\n---\n# Skill\n" + tc.snippet
		score := scoreD8(content, nilBridge())
		// 5 + 4 (run cmd) = 9
		if score != 9 {
			t.Errorf("runner=%s: want 9, got %d", tc.name, score)
		}
	}
}

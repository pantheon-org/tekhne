package scorer

import "testing"

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
	// 8 + 4 (>5 blocks) + 2 (./) + 1 (```bash) = 15
	if score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD8_FewCodeBlocks(t *testing.T) {
	content := "---\ndescription: x\n---\n# Skill\nNo code blocks here."
	if score := scoreD8(content); score != 8 {
		t.Errorf("want 8, got %d", score)
	}
}

func TestD8_MediumCodeBlocks(t *testing.T) {
	content := "---\ndescription: x\n---\n" +
		"```\nfoo\n```\n```\nbar\n```\n```\nbaz\n```\n"
	// 8 + 2 (3 blocks) = 10
	if score := scoreD8(content); score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

func TestD8_LanguageTags(t *testing.T) {
	langs := []string{"bash", "sh", "shell", "typescript", "javascript", "python", "go", "rust"}
	for _, lang := range langs {
		content := "---\ndescription: x\n---\n```" + lang + "\nsome code\n```\n"
		score := scoreD8(content)
		// 8 + 0 (1 block, not >2) + 0 (no run cmd) + 1 (lang tag) = 9
		if score != 9 {
			t.Errorf("lang=%s: want 9, got %d", lang, score)
		}
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
		score := scoreD8(content)
		// 8 + 2 (run cmd) = 10
		if score != 10 {
			t.Errorf("runner=%s: want 10, got %d", tc.name, score)
		}
	}
}

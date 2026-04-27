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
	// 3-5 code blocks → +2
	content := "---\ndescription: x\n---\n" +
		"```\nfoo\n```\n```\nbar\n```\n```\nbaz\n```\n"
	score := scoreD8(content)
	// 8 + 2 (3 blocks) = 10
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
}

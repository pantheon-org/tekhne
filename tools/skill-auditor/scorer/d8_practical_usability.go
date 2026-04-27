package scorer

// scoreD8 — Practical Usability (max: 15)
// Code block count and language tags via library; run-command check kept custom.
func scoreD8(content string, b *validatorBridge) int {
	score := 5

	if b.Content != nil {
		switch {
		case b.Content.CodeBlockCount > 5:
			score += 4
		case b.Content.CodeBlockCount > 2:
			score += 2
		case b.Content.CodeBlockCount > 0:
			score++
		}
		if len(b.Content.CodeLanguages) > 0 {
			score += 2
		}
	} else {
		blocks := codeBlockCount(content)
		switch {
		case blocks > 5:
			score += 4
		case blocks > 2:
			score += 2
		}
	}

	// Run-command presence — no library equivalent.
	hasRunCmd := countPattern(content, "./") > 0 ||
		countPattern(content, "npm run") > 0 ||
		countPattern(content, "yarn ") > 0 ||
		countPattern(content, "pnpm run") > 0 ||
		countPattern(content, "bun run") > 0 ||
		countPattern(content, "make ") > 0 ||
		countPattern(content, "python ") > 0 ||
		countPattern(content, "go run") > 0
	if hasRunCmd {
		score += 4
	}

	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}
	return score
}

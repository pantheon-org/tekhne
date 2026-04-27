package scorer

import "regexp"

// scoreD8 — Practical Usability (max: 15)
func scoreD8(content string) int {
	score := 8

	blocks := codeBlockCount(content)
	if blocks > 5 {
		score += 4
	} else if blocks > 2 {
		score += 2
	}

	hasRunCmd := countPattern(content, "./") > 0 ||
		countPattern(content, "npm run") > 0 ||
		countPattern(content, "yarn ") > 0 ||
		countPattern(content, "pnpm run") > 0 ||
		countPattern(content, "bun run") > 0 ||
		countPattern(content, "make ") > 0 ||
		countPattern(content, "python ") > 0 ||
		countPattern(content, "go run") > 0
	if hasRunCmd {
		score += 2
	}

	langTagRe := regexp.MustCompile("```(bash|sh|shell|typescript|javascript|python|go|rust)")
	if langTagRe.MatchString(content) {
		score++
	}

	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}
	return score
}

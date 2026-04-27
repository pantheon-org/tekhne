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

	if countPattern(content, "./") > 0 || countPattern(content, "bun run") > 0 {
		score += 2
	}

	langTagRe := regexp.MustCompile("```(bash|shell|typescript|javascript)")
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

package scorer

// scoreD2 — Mindset + Procedures (max: 15)
func scoreD2(content string) int {
	score := 8

	if matchesRegexCI(content, `(?im)##\s*(mindset|philosophy|principles)`) {
		score += 2
	}
	if matchesRegexCI(content, `(?m)^\s*[0-9]+\.`) {
		score += 2
	}
	if countPattern(content, "when to use") > 0 || countPattern(content, "when to apply") > 0 {
		score += 2
	}
	if countPattern(content, "when not to") > 0 {
		score++
	}

	if score > 15 {
		score = 15
	}
	return score
}

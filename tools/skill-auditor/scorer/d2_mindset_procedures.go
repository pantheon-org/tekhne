package scorer

// scoreD2 — Mindset + Procedures (max: 15)
func scoreD2(content string, b *validatorBridge) int {
	score := 0

	if matchesRegexCI(content, `(?im)##\s*(mindset|philosophy|principles)`) {
		score += 2
	}

	if b.Content != nil {
		// Imperative ratio via library
		switch {
		case b.Content.ImperativeRatio >= 0.4:
			score += 4
		case b.Content.ImperativeRatio >= 0.25:
			score += 3
		case b.Content.ImperativeRatio >= 0.1:
			score += 2
		}
		if b.Content.ListItemCount > 3 {
			score += 2
		} else if b.Content.ListItemCount > 0 {
			score++
		}
	} else {
		if matchesRegexCI(content, `(?m)^\s*[0-9]+\.`) {
			score += 2
		}
	}

	if countPattern(content, "when to use") > 0 || countPattern(content, "when to apply") > 0 {
		score += 4
	}
	if countPattern(content, "when not to") > 0 {
		score += 3
	}

	if score > 15 {
		score = 15
	}
	return score
}

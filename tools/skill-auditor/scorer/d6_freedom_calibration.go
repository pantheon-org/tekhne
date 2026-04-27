package scorer

// scoreD6 — Freedom Calibration (max: 15)
func scoreD6(content string) int {
	score := 10

	neverAlways := countPattern(content, "NEVER") + countPattern(content, "ALWAYS")
	if neverAlways > 5 {
		score += 3
	} else if neverAlways > 2 {
		score += 2
	}

	if countPattern(content, "consider") > 0 || countPattern(content, "optionally") > 0 || countPattern(content, "may") > 0 {
		score += 2
	}

	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}
	return score
}

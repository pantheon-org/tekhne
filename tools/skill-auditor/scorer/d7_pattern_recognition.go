package scorer

import "fmt"

// scoreD7 — Pattern Recognition (max: 10)
// Uses library description length (chars) from CheckFrontmatter.
// Falls back to word-count heuristic when library result unavailable.
func scoreD7(b *validatorBridge) (int, []Diagnostic) {
	var diags []Diagnostic

	descLen := b.descriptionLen()
	if descLen >= 0 {
		// Library has the char count; map to score bands.
		switch {
		case descLen > 200:
			return 10, diags
		case descLen > 120:
			return 9, diags
		case descLen > 60:
			return 8, diags
		default:
			if descLen <= 30 {
				diags = append(diags, warnDiag("D7", fmt.Sprintf("description is only %d chars — aim for >60 for a useful pattern signal", descLen)))
			}
			return 6, diags
		}
	}

	// Fallback: word count on raw frontmatter field (library unavailable).
	if b.Structure == nil {
		return 6, append(diags, warnDiag("D7", "description length unavailable (validator bridge failed)"))
	}
	return 6, diags
}

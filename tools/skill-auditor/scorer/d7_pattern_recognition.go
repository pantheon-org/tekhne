package scorer

import (
	"fmt"
	"regexp"
)

// scoreD7 — Pattern Recognition (max: 10)
func scoreD7(content string) (int, []Diagnostic) {
	var diags []Diagnostic
	description := extractFrontmatterField(content, "description")
	wordRe := regexp.MustCompile(`[[:alnum:]]+`)
	count := 0
	for _, w := range wordRe.FindAllString(description, -1) {
		if len(w) > 3 {
			count++
		}
	}

	if count <= 5 {
		diags = append(diags, warnDiag("D7", fmt.Sprintf("description has %d qualifying words (need >5 for full score)", count)))
	}

	switch {
	case count > 15:
		return 10, diags
	case count > 10:
		return 9, diags
	case count > 5:
		return 8, diags
	default:
		return 6, diags
	}
}

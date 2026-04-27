package scorer

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// scoreD5 — Progressive Disclosure (max: 15)
func scoreD5(content, skillDir string) int {
	score, _, _, _ := scoreD5WithMeta(content, skillDir)
	return score
}

// scoreD5WithMeta returns the D5 score plus metadata used in the Result.
func scoreD5WithMeta(content, skillDir string) (score, lines, refCount int, hasRefs bool) {
	refsDir := filepath.Join(skillDir, "references")
	if info, err := os.Stat(refsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(refsDir)
		for _, e := range entries {
			if !e.IsDir() && strings.HasSuffix(e.Name(), ".md") && !strings.HasPrefix(e.Name(), ".") {
				hasRefs = true
				refCount++
			}
		}
	}

	lines = len(strings.Split(content, "\n"))

	if hasRefs {
		switch {
		case lines < 100:
			return 15, lines, refCount, hasRefs
		case lines < 150:
			return 13, lines, refCount, hasRefs
		case lines < 200:
			return 11, lines, refCount, hasRefs
		default:
			return 10, lines, refCount, hasRefs
		}
	}

	switch {
	case lines < 200:
		return 12, lines, refCount, hasRefs
	case lines < 300:
		return 10, lines, refCount, hasRefs
	case lines < 500:
		return 7, lines, refCount, hasRefs
	default:
		return 5, lines, refCount, hasRefs
	}
}

// isReferenceSectionCompliant checks if ## References is the last H2 with ≥1 bullet link.
func isReferenceSectionCompliant(content string) bool {
	lines := strings.Split(content, "\n")
	lastH2 := ""
	for _, line := range lines {
		if strings.HasPrefix(line, "## ") {
			lastH2 = strings.TrimPrefix(line, "## ")
		}
	}
	bulletLinkRe := regexp.MustCompile(`(?m)^- \[.+\]\(.+\)`)
	return strings.TrimSpace(lastH2) == "References" && bulletLinkRe.MatchString(content)
}

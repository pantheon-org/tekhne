package scorer

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// scoreD4 — Specification Compliance (max: 17)
func scoreD4(content, skillDir string, b *validatorBridge) (int, []Diagnostic) {
	score := 8
	var diags []Diagnostic

	// Description length: prefer library result; fall back to custom parse.
	descLen := b.descriptionLen()
	if descLen < 0 {
		descLen = len(extractFrontmatterField(content, "description"))
	}
	if descLen > 100 {
		score += 2
	}
	if descLen > 200 {
		score++
	}

	// Keyword-stuffing proxy via and/or count (library flags stuffing but no count).
	description := extractFrontmatterField(content, "description")
	andOrRe := regexp.MustCompile(`(?i) and | or `)
	andOrCount := len(andOrRe.FindAllString(description, -1))
	if andOrCount > 3 {
		score -= 2
	} else if andOrCount > 1 {
		score--
	}

	// Harness-specific paths — content scan, no library equivalent.
	if dir := findHarnessPath(content); dir != "" {
		diags = append(diags, warnDiag("D4", "harness-specific path found: "+dir))
	} else {
		score++
	}

	// Agent name references — content scan, no library equivalent.
	if ref := findAgentRef(content); ref != "" {
		diags = append(diags, warnDiag("D4", "agent-specific reference found: "+ref))
	} else {
		score++
	}

	relPathRe := regexp.MustCompile(`(scripts|references|assets)/[a-zA-Z0-9_-]+`)
	if relPathRe.MatchString(content) {
		score++
	}

	nonCode := removeCodeBlocks(content)

	// ../  violations: use library's internal-link check as primary signal.
	if b.hasInternalLinkWarning() || strings.Contains(nonCode, "../") {
		score -= 2
		diags = append(diags, warnDiag("D4", "../ reference outside code blocks (self-containment violation)"))
	}

	absPathRe := regexp.MustCompile(`skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+`)
	if m := absPathRe.FindString(nonCode); m != "" {
		score--
		diags = append(diags, warnDiag("D4", "absolute skill path outside code blocks: "+m))
	}

	ctxAgentsRe := regexp.MustCompile(`\.(context|agents)/`)
	if m := ctxAgentsRe.FindString(nonCode); m != "" {
		score--
		diags = append(diags, warnDiag("D4", ".context/ or .agents/ reference outside code blocks: "+m))
	}

	score -= penaltyFromDir(filepath.Join(skillDir, "scripts"), absPathRe)
	score -= penaltyFromDir(filepath.Join(skillDir, "scripts"), ctxAgentsRe)
	score -= penaltyFromDir(filepath.Join(skillDir, "references"), absPathRe)
	score -= penaltyFromDir(filepath.Join(skillDir, "references"), ctxAgentsRe)

	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}

	bonus := 0
	scriptsDir := filepath.Join(skillDir, "scripts")
	if info, err := os.Stat(scriptsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(scriptsDir)
		for _, e := range entries {
			name := e.Name()
			if strings.HasSuffix(name, ".py") || strings.HasSuffix(name, ".ts") || strings.HasSuffix(name, ".js") {
				bonus++
				break
			}
		}
	}

	lines := strings.Split(content, "\n")
	lastH2 := ""
	for _, line := range lines {
		if strings.HasPrefix(line, "## ") {
			lastH2 = strings.TrimPrefix(line, "## ")
		}
	}
	bulletLinkRe := regexp.MustCompile(`(?m)^- \[.+\]\(.+\)`)
	if strings.TrimSpace(lastH2) == "References" && bulletLinkRe.MatchString(content) {
		bonus++
	}

	score += bonus
	if score > 17 {
		score = 17
	}
	return score, diags
}

// penaltyFromDir returns the number of files in dir matching re, capped at 2.
func penaltyFromDir(dir string, re *regexp.Regexp) int {
	info, err := os.Stat(dir)
	if err != nil || !info.IsDir() {
		return 0
	}
	entries, _ := os.ReadDir(dir)
	penalty := 0
	for _, e := range entries {
		if e.IsDir() {
			continue
		}
		data, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			continue
		}
		if re.MatchString(string(data)) {
			penalty++
			if penalty >= 2 {
				break
			}
		}
	}
	return penalty
}

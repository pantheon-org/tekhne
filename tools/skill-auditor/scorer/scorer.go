package scorer

import (
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// Score evaluates a skill at skillPath and returns a Result.
func Score(skillPath string) (*Result, error) {
	contentBytes, err := os.ReadFile(skillPath)
	if err != nil {
		return nil, err
	}
	content := string(contentBytes)
	evalsDir := filepath.Join(filepath.Dir(skillPath), "evals")
	return ScoreFromContent(skillPath, content, evalsDir)
}

// ScoreFromContent scores a skill from pre-loaded content and an evals directory path.
// This enables unit testing without filesystem dependencies for the content.
func ScoreFromContent(skillPath, content, evalsDir string) (*Result, error) {
	skillDir := filepath.Dir(skillPath)

	d1 := scoreD1(content, skillDir)
	d2 := scoreD2(content)
	d3 := scoreD3(content, skillDir)
	d4 := scoreD4(content, skillDir)
	d5, lines, refCount, hasRefs := scoreD5WithMeta(content, skillDir)
	d6 := scoreD6(content)
	d7 := scoreD7(content)
	d8 := scoreD8(content)
	d9 := scoreD9(evalsDir)

	total := d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9

	refSectionCompliant := isReferenceSectionCompliant(content)

	return &Result{
		Skill:                    skillPath,
		Total:                    total,
		MaxTotal:                 140,
		Grade:                    Grade(total),
		Lines:                    lines,
		HasReferences:            hasRefs,
		ReferenceCount:           refCount,
		ReferenceSectionCompliant: refSectionCompliant,
		Dimensions: map[string]int{
			"knowledgeDelta":          d1,
			"mindsetProcedures":       d2,
			"antiPatternQuality":      d3,
			"specificationCompliance": d4,
			"progressiveDisclosure":   d5,
			"freedomCalibration":      d6,
			"patternRecognition":      d7,
			"practicalUsability":      d8,
			"evalValidation":          d9,
		},
	}, nil
}

// scoreD1 — Knowledge Delta (max: 20)
// Replicates evaluate_knowledge_delta from evaluate.sh.
func scoreD1(content, skillDir string) int {
	score := 15

	// Penalties: -2 each for beginner-oriented patterns
	for _, pat := range []string{"npm install", "yarn add", "pip install", "getting started", "introduction", "basic syntax", "hello world"} {
		if countPattern(content, pat) > 0 {
			score -= 2
		}
	}

	// Rewards: +1 each for expert signals
	for _, pat := range []string{"anti-pattern", "NEVER", "ALWAYS", "production", "gotcha", "pitfall"} {
		if countPattern(content, pat) > 0 {
			score++
		}
	}

	// Eval bonus: check instructions.json why_given field distribution
	instrFile := filepath.Join(skillDir, "evals", "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []struct {
				WhyGiven string `json:"why_given"`
			} `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) == nil {
			total := len(instrData.Instructions)
			if total > 0 {
				newKnow := 0
				pref := 0
				for _, instr := range instrData.Instructions {
					switch instr.WhyGiven {
					case "new knowledge":
						newKnow++
					case "preference":
						pref++
					}
				}
				expertRatio := (newKnow + pref) * 100 / total
				if expertRatio >= 70 {
					score += 2
				} else if expertRatio < 30 {
					score -= 2
				}
			}
		}
	}

	if score < 0 {
		score = 0
	}
	if score > 20 {
		score = 20
	}
	return score
}

// scoreD2 — Mindset + Procedures (max: 15)
// Replicates evaluate_mindset_procedures.
func scoreD2(content string) int {
	score := 8

	// ## Mindset or ## Philosophy or ## Principles (case-insensitive)
	if matchesRegexCI(content, `(?im)##\s*(mindset|philosophy|principles)`) {
		score += 2
	}
	// Numbered list
	if matchesRegexCI(content, `(?m)^\s*[0-9]+\.`) {
		score += 2
	}
	// when to use/apply
	if countPattern(content, "when to use") > 0 || countPattern(content, "when to apply") > 0 {
		score += 2
	}
	// when not to
	if countPattern(content, "when not to") > 0 {
		score++
	}

	if score > 15 {
		score = 15
	}
	return score
}

// scoreD3 — Anti-Pattern Quality (max: 15)
// Replicates evaluate_anti_pattern_quality.
func scoreD3(content, skillDir string) int {
	score := 8

	// NEVER count
	neverCount := countPattern(content, "NEVER")
	if neverCount > 3 {
		score += 3
	} else if neverCount > 0 {
		score += neverCount
	}

	// BAD.*GOOD on same line (case-insensitive)
	if matchesRegexCI(content, `(?i)BAD.*GOOD`) {
		score += 2
	}

	// WHY:
	if countPattern(content, "WHY:") > 0 {
		score += 2
	}

	// Eval anti-pattern instructions
	instrFile := filepath.Join(skillDir, "evals", "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []struct {
				OriginalSnippets interface{} `json:"original_snippets"`
				Content          string      `json:"content"`
			} `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) == nil {
			antiPat := regexp.MustCompile(`(?i)NEVER|ALWAYS|anti-pattern|avoid|do not`)
			antiInstr := 0
			for _, instr := range instrData.Instructions {
				snippetStr := ""
				switch v := instr.OriginalSnippets.(type) {
				case string:
					snippetStr = v
				case []interface{}:
					parts := make([]string, 0, len(v))
					for _, item := range v {
						if s, ok := item.(string); ok {
							parts = append(parts, s)
						}
					}
					snippetStr = strings.Join(parts, " ")
				}
				if antiPat.MatchString(snippetStr) {
					antiInstr++
				}
			}
			if antiInstr >= 5 {
				score += 2
			} else if antiInstr >= 3 {
				score++
			}
		}
	}

	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}
	return score
}

// scoreD4 — Specification Compliance (max: 17)
// Replicates evaluate_specification_compliance.
func scoreD4(content, skillDir string) int {
	score := 10

	description := extractFrontmatterField(content, "description")
	descLen := len(description)

	// Description length bonuses
	if descLen > 100 {
		score += 2
	}
	if descLen > 200 {
		score++
	}

	// Task focus: penalize multi-purpose descriptions
	andOrRe := regexp.MustCompile(`(?i) and | or `)
	andOrMatches := andOrRe.FindAllString(description, -1)
	andOrCount := len(andOrMatches)
	if andOrCount > 3 {
		score -= 2
	} else if andOrCount > 1 {
		score--
	}

	// Portability: no harness-specific paths
	harnessPathRe := regexp.MustCompile(`\.(opencode|claude|cursor|aider|continue)/`)
	if !harnessPathRe.MatchString(content) {
		score++
	}

	// No agent-specific references in instructions
	agentRefRe := regexp.MustCompile(`(?i)claude code|cursor agent|github copilot|aider|continue\.dev`)
	if !agentRefRe.MatchString(content) {
		score++
	}

	// Has relative path references (scripts/, references/, assets/)
	relPathRe := regexp.MustCompile(`(scripts|references|assets)/[a-zA-Z0-9_-]+`)
	if relPathRe.MatchString(content) {
		score++
	}

	// Self-containment checks on non-code content
	nonCode := removeCodeBlocks(content)

	// ../ references escape skill directory
	if strings.Contains(nonCode, "../") {
		score -= 2
	}

	// Absolute skills/X/Y paths
	absPathRe := regexp.MustCompile(`skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+`)
	if absPathRe.MatchString(nonCode) {
		score--
	}

	// .context/ or .agents/ references
	ctxAgentsRe := regexp.MustCompile(`\.(context|agents)/`)
	if ctxAgentsRe.MatchString(nonCode) {
		score--
	}

	// Penalty: absolute repo paths in scripts/ files (-1 per file, cap -2)
	scriptsDir := filepath.Join(skillDir, "scripts")
	if info, err := os.Stat(scriptsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(scriptsDir)
		penalty := 0
		for _, e := range entries {
			if e.IsDir() {
				continue
			}
			data, err := os.ReadFile(filepath.Join(scriptsDir, e.Name()))
			if err != nil {
				continue
			}
			if absPathRe.MatchString(string(data)) {
				penalty++
				if penalty >= 2 {
					break
				}
			}
		}
		score -= penalty
	}

	// Penalty: .context/.agents/ in scripts/ files (-1 per file, cap -2)
	if info, err := os.Stat(scriptsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(scriptsDir)
		penalty := 0
		for _, e := range entries {
			if e.IsDir() {
				continue
			}
			data, err := os.ReadFile(filepath.Join(scriptsDir, e.Name()))
			if err != nil {
				continue
			}
			if ctxAgentsRe.MatchString(string(data)) {
				penalty++
				if penalty >= 2 {
					break
				}
			}
		}
		score -= penalty
	}

	// Penalty: absolute repo paths in references/ files (-1 per file, cap -2)
	refsDir := filepath.Join(skillDir, "references")
	if info, err := os.Stat(refsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(refsDir)
		penalty := 0
		for _, e := range entries {
			if e.IsDir() {
				continue
			}
			data, err := os.ReadFile(filepath.Join(refsDir, e.Name()))
			if err != nil {
				continue
			}
			if absPathRe.MatchString(string(data)) {
				penalty++
				if penalty >= 2 {
					break
				}
			}
		}
		score -= penalty
	}

	// Penalty: .context/.agents/ in references/ files (-1 per file, cap -2)
	if info, err := os.Stat(refsDir); err == nil && info.IsDir() {
		entries, _ := os.ReadDir(refsDir)
		penalty := 0
		for _, e := range entries {
			if e.IsDir() {
				continue
			}
			data, err := os.ReadFile(filepath.Join(refsDir, e.Name()))
			if err != nil {
				continue
			}
			if ctxAgentsRe.MatchString(string(data)) {
				penalty++
				if penalty >= 2 {
					break
				}
			}
		}
		score -= penalty
	}

	// Cap base score
	if score > 15 {
		score = 15
	}
	if score < 0 {
		score = 0
	}

	// Bonuses (applied after cap, max combined +2)
	bonus := 0

	// Script portability bonus
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

	// References section format bonus: ## References is last H2, has bullet markdown link
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
	return score
}

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

	// evaluate.sh uses wc -l which counts total lines (including empty ones)
	lines = len(strings.Split(content, "\n"))

	if hasRefs {
		if lines < 100 {
			return 15, lines, refCount, hasRefs
		}
		if lines < 150 {
			return 13, lines, refCount, hasRefs
		}
		if lines < 200 {
			return 11, lines, refCount, hasRefs
		}
		return 10, lines, refCount, hasRefs
	}

	if lines < 200 {
		return 12, lines, refCount, hasRefs
	}
	if lines < 300 {
		return 10, lines, refCount, hasRefs
	}
	if lines < 500 {
		return 7, lines, refCount, hasRefs
	}
	return 5, lines, refCount, hasRefs
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

// scoreD6 — Freedom Calibration (max: 15)
// Replicates evaluate_freedom_calibration.
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

// scoreD7 — Pattern Recognition (max: 10)
// Replicates evaluate_pattern_recognition.
func scoreD7(content string) int {
	description := extractFrontmatterField(content, "description")
	// Count words with length > 3 (tr -cs '[:alnum:]' '\n' | awk 'length > 3')
	wordRe := regexp.MustCompile(`[[:alnum:]]+`)
	words := wordRe.FindAllString(description, -1)
	count := 0
	for _, w := range words {
		if len(w) > 3 {
			count++
		}
	}

	if count > 15 {
		return 10
	}
	if count > 10 {
		return 9
	}
	if count > 5 {
		return 8
	}
	return 6
}

// scoreD8 — Practical Usability (max: 15)
// Replicates evaluate_practical_usability.
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

	// Language-tagged fence: ```bash, ```shell, ```typescript, ```javascript
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

// scoreD9 — Eval Validation (max: 20)
// Replicates evaluate_eval_validation.
func scoreD9(evalsDir string) int {
	score := 0

	if info, err := os.Stat(evalsDir); err != nil || !info.IsDir() {
		return score
	}
	score += 4

	// instructions.json
	instrFile := filepath.Join(evalsDir, "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []json.RawMessage `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) == nil && len(instrData.Instructions) > 0 {
			score += 3
		} else if len(data) > 0 {
			// fallback: non-empty file
			score += 3
		}
	}

	// summary.json
	summaryFile := filepath.Join(evalsDir, "summary.json")
	if data, err := os.ReadFile(summaryFile); err == nil {
		var summaryData struct {
			InstructionsCoverage struct {
				CoveragePercentage interface{} `json:"coverage_percentage"`
			} `json:"instructions_coverage"`
		}
		if json.Unmarshal(data, &summaryData) == nil {
			coverage := parseCoveragePercentage(summaryData.InstructionsCoverage.CoveragePercentage)
			if coverage >= 0 {
				score += 3
				if coverage >= 80 {
					score += 3
				}
			} else if len(data) > 0 {
				// jq not available fallback: non-empty
				score += 3
			}
		} else if len(data) > 0 {
			score += 3
		}
	}

	// Valid scenarios
	validScenarios := countValidScenarios(evalsDir)
	if validScenarios >= 3 {
		score += 4
	} else if validScenarios >= 1 {
		score += 2
	}

	if score > 20 {
		score = 20
	}
	return score
}

// parseCoveragePercentage parses a coverage percentage value (int, float64, or string) to int.
// Returns -1 if unparseable.
func parseCoveragePercentage(v interface{}) int {
	if v == nil {
		return -1
	}
	switch val := v.(type) {
	case float64:
		return int(val)
	case int:
		return val
	case string:
		s := strings.TrimRight(strings.TrimSpace(val), "%")
		if s == "" {
			return -1
		}
		// parse integer part only
		dotIdx := strings.Index(s, ".")
		if dotIdx >= 0 {
			s = s[:dotIdx]
		}
		n := 0
		for _, ch := range s {
			if ch < '0' || ch > '9' {
				return -1
			}
			n = n*10 + int(ch-'0')
		}
		return n
	}
	return -1
}

// countValidScenarios counts scenario-N/ dirs with task.md, criteria.json (checklist sum=100), capability.txt.
func countValidScenarios(evalsDir string) int {
	entries, err := os.ReadDir(evalsDir)
	if err != nil {
		return 0
	}

	valid := 0
	for _, e := range entries {
		if !e.IsDir() || !strings.HasPrefix(e.Name(), "scenario-") {
			continue
		}
		scenarioDir := filepath.Join(evalsDir, e.Name())
		hasTask := fileExists(filepath.Join(scenarioDir, "task.md"))
		hasCriteria := fileExists(filepath.Join(scenarioDir, "criteria.json"))
		hasCapability := fileExists(filepath.Join(scenarioDir, "capability.txt"))
		if !hasTask || !hasCriteria || !hasCapability {
			continue
		}

		// Validate criteria.json: checklist[].max_score must sum to 100
		data, err := os.ReadFile(filepath.Join(scenarioDir, "criteria.json"))
		if err != nil {
			// If we can't read it, count it as valid (jq-not-available fallback)
			valid++
			continue
		}
		var criteriaData struct {
			Checklist []struct {
				MaxScore int `json:"max_score"`
			} `json:"checklist"`
		}
		if json.Unmarshal(data, &criteriaData) != nil {
			valid++
			continue
		}
		sum := 0
		for _, item := range criteriaData.Checklist {
			sum += item.MaxScore
		}
		if sum == 100 {
			valid++
		}
	}
	return valid
}

// fileExists returns true if the path exists and is a regular file.
func fileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && !info.IsDir()
}

// matchesRegexCI matches content against a regex pattern (case-insensitive).
func matchesRegexCI(content, pattern string) bool {
	re, err := regexp.Compile(pattern)
	if err != nil {
		return false
	}
	return re.MatchString(content)
}

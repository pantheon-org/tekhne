package scorer

import (
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// scoreD3 — Anti-Pattern Quality (max: 15)
func scoreD3(content, skillDir string) (int, []Diagnostic) {
	score := 8
	var diags []Diagnostic

	neverCount := countPattern(content, "NEVER")
	if neverCount > 3 {
		score += 3
	} else if neverCount > 0 {
		score += neverCount
	}

	if matchesRegexCI(content, `(?i)BAD.*GOOD`) {
		score += 2
	}
	if countPattern(content, "WHY:") > 0 {
		score += 2
	}

	instrFile := filepath.Join(skillDir, "evals", "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []struct {
				OriginalSnippets interface{} `json:"original_snippets"`
				Content          string      `json:"content"`
			} `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) != nil {
			diags = append(diags, errDiag("D3", "instructions.json exists but cannot be parsed"))
		} else {
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
	return score, diags
}

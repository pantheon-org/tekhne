package scorer

import (
	"encoding/json"
	"os"
	"path/filepath"
)

// scoreD1 — Knowledge Delta (max: 20)
func scoreD1(content, skillDir string) (int, []Diagnostic) {
	score := 15
	var diags []Diagnostic

	for _, pat := range []string{"npm install", "yarn add", "pip install", "getting started", "introduction", "basic syntax", "hello world"} {
		if countPattern(content, pat) > 0 {
			score -= 2
		}
	}

	for _, pat := range []string{"anti-pattern", "NEVER", "ALWAYS", "production", "gotcha", "pitfall"} {
		if countPattern(content, pat) > 0 {
			score++
		}
	}

	instrFile := filepath.Join(skillDir, "evals", "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []struct {
				WhyGiven string `json:"why_given"`
			} `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) != nil {
			diags = append(diags, errDiag("D1", "instructions.json exists but cannot be parsed"))
		} else {
			total := len(instrData.Instructions)
			if total > 0 {
				newKnow, pref := 0, 0
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
	return score, diags
}

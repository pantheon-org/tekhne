package scorer

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// scoreD9 — Eval Validation (max: 20)
func scoreD9(evalsDir string) (int, []Diagnostic) {
	score := 0
	var diags []Diagnostic

	if info, err := os.Stat(evalsDir); err != nil || !info.IsDir() {
		diags = append(diags, warnDiag("D9", "evals/ directory missing entirely"))
		return score, diags
	}
	score += 4

	instrFile := filepath.Join(evalsDir, "instructions.json")
	if data, err := os.ReadFile(instrFile); err == nil {
		var instrData struct {
			Instructions []json.RawMessage `json:"instructions"`
		}
		if json.Unmarshal(data, &instrData) != nil {
			diags = append(diags, errDiag("D9", "instructions.json exists but is not valid JSON"))
		} else if len(instrData.Instructions) > 0 || len(data) > 0 {
			score += 3
		}
	}

	summaryFile := filepath.Join(evalsDir, "summary.json")
	if data, err := os.ReadFile(summaryFile); err == nil {
		var summaryData struct {
			InstructionsCoverage struct {
				CoveragePercentage interface{} `json:"coverage_percentage"`
			} `json:"instructions_coverage"`
		}
		if json.Unmarshal(data, &summaryData) != nil {
			diags = append(diags, errDiag("D9", "summary.json exists but is not valid JSON"))
		} else {
			coverage := parseCoveragePercentage(summaryData.InstructionsCoverage.CoveragePercentage)
			if coverage >= 0 {
				score += 3
				if coverage >= 80 {
					score += 3
				} else {
					diags = append(diags, warnDiag("D9", fmt.Sprintf("summary.json coverage is %d%% (below 80%% threshold)", coverage)))
				}
			} else if len(data) > 0 {
				score += 3
			}
		}
	}

	validScenarios, scenarioDiags := countValidScenariosWithDiags(evalsDir)
	diags = append(diags, scenarioDiags...)
	if validScenarios >= 3 {
		score += 4
	} else if validScenarios >= 1 {
		score += 2
	}

	if score > 20 {
		score = 20
	}
	return score, diags
}

// parseCoveragePercentage parses a coverage percentage value to int. Returns -1 if unparseable.
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
		if dotIdx := strings.Index(s, "."); dotIdx >= 0 {
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

// countValidScenarios is a thin wrapper used by tests.
func countValidScenarios(evalsDir string) int {
	count, _ := countValidScenariosWithDiags(evalsDir)
	return count
}

// countValidScenariosWithDiags counts valid scenario dirs and emits diagnostics for problems.
func countValidScenariosWithDiags(evalsDir string) (int, []Diagnostic) {
	var diags []Diagnostic
	entries, err := os.ReadDir(evalsDir)
	if err != nil {
		return 0, diags
	}

	// Detect flat scenario-NN.md files (legacy format) when no subdirectory scenarios exist.
	flatCount := 0
	for _, e := range entries {
		if !e.IsDir() && strings.HasPrefix(e.Name(), "scenario-") && strings.HasSuffix(e.Name(), ".md") {
			flatCount++
		}
	}

	valid := 0
	for _, e := range entries {
		if !e.IsDir() || !strings.HasPrefix(e.Name(), "scenario-") {
			continue
		}
		name := e.Name()
		scenarioDir := filepath.Join(evalsDir, name)
		hasTask := fileExists(filepath.Join(scenarioDir, "task.md"))
		hasCriteria := fileExists(filepath.Join(scenarioDir, "criteria.json"))
		hasCapability := fileExists(filepath.Join(scenarioDir, "capability.txt"))

		if !hasTask || !hasCriteria || !hasCapability {
			var missing []string
			if !hasTask {
				missing = append(missing, "task.md")
			}
			if !hasCriteria {
				missing = append(missing, "criteria.json")
			}
			if !hasCapability {
				missing = append(missing, "capability.txt")
			}
			diags = append(diags, warnDiag("D9", fmt.Sprintf("%s missing: %s", name, strings.Join(missing, ", "))))
			continue
		}

		data, err := os.ReadFile(filepath.Join(scenarioDir, "criteria.json"))
		if err != nil {
			valid++
			continue
		}
		var criteriaData struct {
			Checklist []struct {
				MaxScore int `json:"max_score"`
			} `json:"checklist"`
		}
		if json.Unmarshal(data, &criteriaData) != nil {
			diags = append(diags, errDiag("D9", fmt.Sprintf("%s/criteria.json is not valid JSON", name)))
			valid++
			continue
		}
		sum := 0
		for _, item := range criteriaData.Checklist {
			sum += item.MaxScore
		}
		if sum == 100 {
			valid++
		} else {
			diags = append(diags, warnDiag("D9", fmt.Sprintf("%s/criteria.json checklist does not sum to 100 (got %d)", name, sum)))
		}
	}

	if valid == 0 && flatCount > 0 {
		diags = append(diags, warnDiag("D9", fmt.Sprintf("%d flat scenario-NN.md file(s) found; migrate to scenario-N/ subdirectory format to score on D9", flatCount)))
	}

	return valid, diags
}

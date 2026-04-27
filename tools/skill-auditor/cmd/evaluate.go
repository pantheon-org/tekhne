package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/reporter"
	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
	"github.com/spf13/cobra"
)

var (
	evaluateJSON      bool
	evaluateStore     bool
	evaluateRepoRoot  string
)

var evaluateCmd = &cobra.Command{
	Use:   "evaluate <skill>",
	Short: "Evaluate a single skill",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		skillArg := args[0]

		repoRoot, err := resolveRepoRoot(evaluateRepoRoot)
		if err != nil {
			return fmt.Errorf("cannot determine repo root: %w", err)
		}

		skillPath := resolveSkillPath(skillArg, repoRoot)
		skillKey := canonicalSkillKey(skillPath, repoRoot)

		result, err := scorer.Score(skillPath)
		if err != nil {
			return fmt.Errorf("scoring failed: %w", err)
		}
		result.Skill = skillKey

		if evaluateJSON {
			data, err := json.MarshalIndent(result, "", "  ")
			if err != nil {
				return fmt.Errorf("marshal result: %w", err)
			}
			fmt.Println(string(data))
		} else {
			fmt.Print(reporter.Format(result))
		}

		if evaluateStore {
			if err := reporter.Store(repoRoot, skillKey, result); err != nil {
				return fmt.Errorf("store result: %w", err)
			}
		}

		return nil
	},
}

func init() {
	evaluateCmd.Flags().BoolVar(&evaluateJSON, "json", false, "emit JSON output")
	evaluateCmd.Flags().BoolVar(&evaluateStore, "store", false, "persist result to .context/audits/")
	evaluateCmd.Flags().StringVar(&evaluateRepoRoot, "repo-root", "", "repo root (auto-detected if empty)")
	rootCmd.AddCommand(evaluateCmd)
}

// canonicalSkillKey derives the domain/skill-name storage key from an absolute
// SKILL.md path by stripping <repoRoot>/skills/ and the trailing /SKILL.md.
func canonicalSkillKey(skillPath, repoRoot string) string {
	prefix := filepath.Join(repoRoot, "skills") + string(filepath.Separator)
	key := strings.TrimPrefix(skillPath, prefix)
	key = strings.TrimSuffix(key, string(filepath.Separator)+"SKILL.md")
	return key
}

// resolveSkillPath converts a skill arg to an absolute filesystem path to SKILL.md.
// Absolute paths and relative paths starting with ./ or ../ are used as-is.
// Bare domain/skill-name args are resolved under <repoRoot>/skills/.
func resolveSkillPath(skillArg, repoRoot string) string {
	isFilesystemPath := filepath.IsAbs(skillArg) ||
		strings.HasPrefix(skillArg, "./") ||
		strings.HasPrefix(skillArg, "../")

	var base string
	if isFilesystemPath {
		abs, err := filepath.Abs(skillArg)
		if err != nil {
			abs = skillArg
		}
		base = abs
	} else {
		base = filepath.Join(repoRoot, "skills", skillArg)
	}

	if strings.HasSuffix(base, "SKILL.md") {
		return base
	}
	return filepath.Join(base, "SKILL.md")
}

// resolveRepoRoot returns the repo root from flag or auto-detection.
// Auto-detection walks up from cwd looking for go.mod or .git.
func resolveRepoRoot(flagValue string) (string, error) {
	if flagValue != "" {
		return flagValue, nil
	}
	cwd, err := os.Getwd()
	if err != nil {
		return "", err
	}
	return findRepoRoot(cwd)
}

// findRepoRoot walks up from dir until it finds go.mod or .git.
func findRepoRoot(dir string) (string, error) {
	current := dir
	for {
		if fileExists(filepath.Join(current, ".git")) || fileExists(filepath.Join(current, "go.mod")) {
			return current, nil
		}
		parent := filepath.Dir(current)
		if parent == current {
			return "", fmt.Errorf("no .git or go.mod found above %s", dir)
		}
		current = parent
	}
}

// fileExists reports whether the path exists.
func fileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

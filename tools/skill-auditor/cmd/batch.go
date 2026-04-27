package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"sort"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/reporter"
	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
	"github.com/spf13/cobra"
)

var (
	batchJSON      bool
	batchStore     bool
	batchFailBelow string
	batchRepoRoot  string
)

var batchCmd = &cobra.Command{
	Use:   "batch <skill1> [skill2 ...]",
	Short: "Evaluate multiple skills",
	Args:  cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		repoRoot, err := resolveRepoRoot(batchRepoRoot)
		if err != nil {
			return fmt.Errorf("cannot determine repo root: %w", err)
		}

		type entry struct {
			arg    string
			result *scorer.Result
			err    error
		}

		entries := make([]entry, len(args))
		for i, arg := range args {
			skillPath := resolveSkillPath(arg, repoRoot)
			result, err := scorer.Score(skillPath)
			if err != nil {
				// Non-fatal: record error inline
				entries[i] = entry{arg: arg, err: err}
				continue
			}
			entries[i] = entry{arg: arg, result: result}

			if batchStore {
				if storeErr := reporter.Store(repoRoot, arg, result); storeErr != nil {
					fmt.Fprintf(os.Stderr, "warning: store %s: %v\n", arg, storeErr)
				}
			}
		}

		if batchJSON {
			results := make([]*scorer.Result, 0, len(entries))
			for _, e := range entries {
				if e.result != nil {
					results = append(results, e.result)
				}
			}
			data, err := json.MarshalIndent(results, "", "  ")
			if err != nil {
				return fmt.Errorf("marshal results: %w", err)
			}
			fmt.Println(string(data))
		} else {
			// Sort by score descending (errors go last)
			sort.Slice(entries, func(i, j int) bool {
				if entries[i].result == nil {
					return false
				}
				if entries[j].result == nil {
					return true
				}
				return entries[i].result.Total > entries[j].result.Total
			})

			// Determine column width for skill name
			maxLen := 0
			for _, e := range entries {
				if len(e.arg) > maxLen {
					maxLen = len(e.arg)
				}
			}
			if maxLen < 40 {
				maxLen = 40
			}

			totalScore := 0
			successCount := 0
			for _, e := range entries {
				if e.err != nil {
					fmt.Printf("%-*s  ERROR: %v\n", maxLen, e.arg, e.err)
					continue
				}
				fmt.Printf("%-*s  %-2s (%d/%d)\n", maxLen, e.arg, e.result.Grade, e.result.Total, e.result.MaxTotal)
				totalScore += e.result.Total
				successCount++
			}

			// Totals row
			sep := strings.Repeat("─", maxLen+20)
			fmt.Println(sep)
			avg := 0
			if successCount > 0 {
				avg = totalScore / successCount
			}
			fmt.Printf("Total: %d skill(s)  Average: %d/140\n", len(entries), avg)
		}

		// --fail-below check
		if batchFailBelow != "" {
			threshold, ok := scorer.GradeRank[batchFailBelow]
			if !ok {
				return fmt.Errorf("unknown grade %q for --fail-below", batchFailBelow)
			}
			for _, e := range entries {
				if e.result == nil {
					continue
				}
				if scorer.GradeRank[e.result.Grade] < threshold {
					return fmt.Errorf("skill %s scored %s, below threshold %s", e.arg, e.result.Grade, batchFailBelow)
				}
			}
		}

		return nil
	},
}

func init() {
	batchCmd.Flags().BoolVar(&batchJSON, "json", false, "emit JSON array output")
	batchCmd.Flags().BoolVar(&batchStore, "store", false, "persist each result to .context/audits/")
	batchCmd.Flags().StringVar(&batchFailBelow, "fail-below", "", "exit 1 if any skill scores below this grade (e.g. B+)")
	batchCmd.Flags().StringVar(&batchRepoRoot, "repo-root", "", "repo root (auto-detected if empty)")
	rootCmd.AddCommand(batchCmd)
}

// Golden-corpus parity harness: Go reference generator (A4a chunk 2).
//
// Invokes the SAME two entry points the skill-auditor couples to through
// tools/skill-auditor/scorer/validator_bridge.go:
//
//  1. orchestrate.RunContentAnalysis(dir), keeping .ContentReport.
//  2. structure.Validate(dir, {SkipOrphans, AllowFlatLayouts,
//     AllowExtraFrontmatter} all true).
//
// For each corpus entry it emits the bridge-consumed surface as canonical
// JSON: the ContentReport, plus the structure Report's Results, TokenCounts,
// OtherTokenCounts, and Errors/Warnings tallies. Absolute paths (SkillDir) are
// deliberately NOT emitted so the goldens are machine-independent; every File
// field is already relative to the skill dir.
//
// Usage:
//
//	go run . <repo-root> < corpus.txt > goldens.json
//
// corpus.txt holds one repo-root-relative skill directory per line (blank
// lines and lines beginning with '#' are ignored). This is the SAME validator
// the auditor depends on (github.com/agent-ecosystem/skill-validator v1.5.6).
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/agent-ecosystem/skill-validator/orchestrate"
	"github.com/agent-ecosystem/skill-validator/structure"
	"github.com/agent-ecosystem/skill-validator/types"
)

// structOut is the subset of the structure Report the auditor bridge reads.
type structOut struct {
	Results          []types.Result     `json:"results"`
	TokenCounts      []types.TokenCount `json:"token_counts"`
	OtherTokenCounts []types.TokenCount `json:"other_token_counts"`
	Errors           int                `json:"errors"`
	Warnings         int                `json:"warnings"`
}

// record is one corpus entry's canonical parity surface.
type record struct {
	Dir       string               `json:"dir"`
	Content   *types.ContentReport `json:"content"`
	Structure structOut            `json:"structure"`
}

func analyze(repoRoot, rel string) record {
	dir := filepath.Join(repoRoot, rel)

	// Bridge call 1: content analysis (auditor keeps .ContentReport only).
	contentRpt := orchestrate.RunContentAnalysis(dir)
	var content *types.ContentReport
	if contentRpt != nil {
		content = contentRpt.ContentReport
	}

	// Bridge call 2: structure validation with the exact auditor options.
	structRpt := structure.Validate(dir, structure.Options{
		SkipOrphans:           true,
		AllowFlatLayouts:      true,
		AllowExtraFrontmatter: true,
	})

	out := structOut{}
	if structRpt != nil {
		out.Results = structRpt.Results
		out.TokenCounts = structRpt.TokenCounts
		out.OtherTokenCounts = structRpt.OtherTokenCounts
		out.Errors = structRpt.Errors
		out.Warnings = structRpt.Warnings
	}
	if out.Results == nil {
		out.Results = []types.Result{}
	}
	if out.TokenCounts == nil {
		out.TokenCounts = []types.TokenCount{}
	}
	if out.OtherTokenCounts == nil {
		out.OtherTokenCounts = []types.TokenCount{}
	}

	return record{Dir: rel, Content: content, Structure: out}
}

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: go run . <repo-root> < corpus.txt")
		os.Exit(2)
	}
	repoRoot := os.Args[1]

	records := []record{}
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 0, 64*1024), 1024*1024)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		records = append(records, analyze(repoRoot, line))
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintf(os.Stderr, "read error: %v\n", err)
		os.Exit(1)
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(records); err != nil {
		fmt.Fprintf(os.Stderr, "marshal error: %v\n", err)
		os.Exit(1)
	}
}

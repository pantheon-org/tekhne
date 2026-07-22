// Golden-corpus grade-parity harness: Go reference generator (A4b chunk 2).
//
// Runs the SAME deterministic scorer the auditor's `evaluate` subcommand uses
// (scorer.Score, see tools/skill-auditor/scorer/scorer.go) over every corpus
// entry and emits, per skill, the full auditor Result as canonical JSON.
//
// The Date field (Go uses time.Now()) is the only non-deterministic input; it
// is zeroed here so the goldens are reproducible, and it is excluded from the
// Rust parity comparison. The Skill field is set to the repo-root-relative
// corpus directory so the goldens are machine-independent (no absolute paths).
//
// Usage:
//
//	go run . <repo-root> < corpus.txt > goldens.json
//
// corpus.txt holds one repo-root-relative skill directory per line (blank lines
// and lines beginning with '#' are ignored). Each directory must contain a
// SKILL.md; the auditor scores that file plus any sibling evals/ directory.
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// record is one corpus entry's canonical parity surface: the repo-root-relative
// directory plus the full auditor Result (with Date zeroed).
type record struct {
	Dir    string         `json:"dir"`
	Result *scorer.Result `json:"result"`
}

func score(repoRoot, rel string) (record, error) {
	skillPath := filepath.Join(repoRoot, rel, "SKILL.md")
	result, err := scorer.Score(skillPath)
	if err != nil {
		return record{}, err
	}
	// Date is time.Now() (excluded from parity); zero it for reproducibility.
	result.Date = ""
	// Skill is the absolute path from scorer.Score; replace with the relative
	// corpus dir so goldens do not embed a machine-specific path.
	result.Skill = rel
	return record{Dir: rel, Result: result}, nil
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
		rec, err := score(repoRoot, line)
		if err != nil {
			fmt.Fprintf(os.Stderr, "score %s: %v\n", line, err)
			os.Exit(1)
		}
		records = append(records, rec)
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

// S1 token-parity gate: Go reference generator.
//
// Reads corpus.jsonl (one JSON-encoded string per line), tokenizes each item
// with tiktoken-go's O200kBase codec, and emits an index-aligned JSON array of
// token counts to stdout. Count == number of token ids returned by Encode,
// which is exactly what the Rust auditor must reproduce.
//
// This is the SAME codec the repo's skill-auditor depends on
// (github.com/tiktoken-go/tokenizer, via github.com/agent-ecosystem/skill-validator).
//
// Usage:
//   go run . < corpus.jsonl > expected_counts.json
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"

	"github.com/tiktoken-go/tokenizer"
)

func main() {
	enc, err := tokenizer.Get(tokenizer.O200kBase)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to load O200kBase: %v\n", err)
		os.Exit(1)
	}

	counts := []int{}
	scanner := bufio.NewScanner(os.Stdin)
	// SKILL.md files JSON-encode to long single lines; raise the buffer cap.
	scanner.Buffer(make([]byte, 0, 1024*1024), 16*1024*1024)

	line := 0
	for scanner.Scan() {
		line++
		raw := scanner.Bytes()
		if len(raw) == 0 {
			continue
		}
		var text string
		if err := json.Unmarshal(raw, &text); err != nil {
			fmt.Fprintf(os.Stderr, "line %d: invalid JSON string: %v\n", line, err)
			os.Exit(1)
		}
		ids, _, err := enc.Encode(text)
		if err != nil {
			fmt.Fprintf(os.Stderr, "line %d: encode error: %v\n", line, err)
			os.Exit(1)
		}
		counts = append(counts, len(ids))
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintf(os.Stderr, "read error: %v\n", err)
		os.Exit(1)
	}

	out, err := json.MarshalIndent(counts, "", "  ")
	if err != nil {
		fmt.Fprintf(os.Stderr, "marshal error: %v\n", err)
		os.Exit(1)
	}
	fmt.Println(string(out))
}

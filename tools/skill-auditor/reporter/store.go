package reporter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// Store writes a Result as JSON to .context/audits/<skillPath>/<date>/audit.json
// relative to repoRoot. Creates directories with os.MkdirAll.
func Store(repoRoot, skillPath string, r *scorer.Result) error {
	date := time.Now().Format("2006-01-02")
	dir := filepath.Join(repoRoot, ".context", "audits", skillPath, date)

	if err := os.MkdirAll(dir, 0o755); err != nil {
		return fmt.Errorf("store: create directories: %w", err)
	}

	data, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return fmt.Errorf("store: marshal result: %w", err)
	}

	dest := filepath.Join(dir, "audit.json")
	tmp := dest + ".tmp"

	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return fmt.Errorf("store: write temp file: %w", err)
	}

	if err := os.Rename(tmp, dest); err != nil {
		_ = os.Remove(tmp)
		return fmt.Errorf("store: rename to final path: %w", err)
	}

	return nil
}

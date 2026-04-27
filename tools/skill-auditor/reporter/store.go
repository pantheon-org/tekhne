package reporter

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/pantheon-org/tekhne/tools/skill-auditor/scorer"
)

// Store writes audit.json, Analysis.md, and Remediation.md to
// .context/audits/<skillPath>/<date>/ relative to repoRoot.
func Store(repoRoot, skillPath string, r *scorer.Result) error {
	dir := filepath.Join(repoRoot, ".context", "audits", skillPath, r.Date)

	if err := os.MkdirAll(dir, 0o755); err != nil {
		return fmt.Errorf("store: create directories: %w", err)
	}

	auditJSON, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return fmt.Errorf("store: marshal result: %w", err)
	}

	files := map[string][]byte{
		"audit.json":     auditJSON,
		"Analysis.md":    []byte(Analysis(r)),
		"Remediation.md": []byte(Remediation(r)),
	}

	for name, data := range files {
		dest := filepath.Join(dir, name)
		tmp := dest + ".tmp"
		if err := os.WriteFile(tmp, data, 0o644); err != nil {
			return fmt.Errorf("store: write %s: %w", name, err)
		}
		if err := os.Rename(tmp, dest); err != nil {
			_ = os.Remove(tmp)
			return fmt.Errorf("store: rename %s: %w", name, err)
		}
	}

	return nil
}

package scorer

import (
	"fmt"
	"strings"

	"github.com/agent-ecosystem/skill-validator/orchestrate"
	"github.com/agent-ecosystem/skill-validator/structure"
	"github.com/agent-ecosystem/skill-validator/types"
)

// validatorBridge holds cached results from skill-validator for a single skill.
// Constructed once per Score call and passed into dimension functions that use it.
type validatorBridge struct {
	Content   *types.ContentReport
	Structure *types.Report
}

func newValidatorBridge(skillDir string) *validatorBridge {
	contentRpt := orchestrate.RunContentAnalysis(skillDir)
	structRpt := structure.Validate(skillDir, structure.Options{
		SkipOrphans:           true,
		AllowFlatLayouts:      true,
		AllowExtraFrontmatter: true,
	})

	b := &validatorBridge{Structure: structRpt}
	if contentRpt != nil {
		b.Content = contentRpt.ContentReport
	}
	return b
}

// skillMDTokens returns the token count for SKILL.md, or 0 if unavailable.
func (b *validatorBridge) skillMDTokens() int {
	if b.Structure == nil {
		return 0
	}
	for _, tc := range b.Structure.TokenCounts {
		if tc.File == "SKILL.md" || strings.HasPrefix(tc.File, "SKILL.md ") {
			return tc.Tokens
		}
	}
	return 0
}

// hasStructureWarning reports whether structure.Validate emitted a Warning
// result whose message contains the given substring.
func (b *validatorBridge) hasStructureWarning(substr string) bool {
	if b.Structure == nil {
		return false
	}
	for _, r := range b.Structure.Results {
		if r.Level == types.Warning && containsString(r.Message, substr) {
			return true
		}
	}
	return false
}

// descriptionLen returns the byte length of the description field as reported
// by CheckFrontmatter ("description: (N chars)"), or -1 if not found.
func (b *validatorBridge) descriptionLen() int {
	if b.Structure == nil {
		return -1
	}
	for _, r := range b.Structure.Results {
		if r.Category != "Frontmatter" {
			continue
		}
		var n int
		if _, err := fmt.Sscanf(r.Message, "description: (%d chars)", &n); err == nil {
			return n
		}
	}
	return -1
}

// hasInternalLinkWarning reports whether CheckInternalLinks emitted a warning,
// which covers `../` outside-code-block violations.
func (b *validatorBridge) hasInternalLinkWarning() bool {
	if b.Structure == nil {
		return false
	}
	for _, r := range b.Structure.Results {
		if r.Category == "Links" && r.Level == types.Warning {
			return true
		}
	}
	return false
}

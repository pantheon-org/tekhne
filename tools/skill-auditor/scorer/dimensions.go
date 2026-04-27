package scorer

import (
	"strings"
)

// DimensionScore holds the score and max for a single rubric dimension.
type DimensionScore struct {
	Score int `json:"score"`
	Max   int `json:"max"`
}

// Result is the output of scoring a skill.
type Result struct {
	SkillPath  string                    `json:"skill_path"`
	Total      int                       `json:"total"`
	MaxTotal   int                       `json:"max_total"`
	Grade      string                    `json:"grade"`
	Dimensions map[string]DimensionScore `json:"dimensions"`
	Errors     int                       `json:"errors"`
	Warnings   int                       `json:"warnings"`
}

// countPattern counts case-insensitive substring occurrences.
func countPattern(content, pattern string) int {
	lower := strings.ToLower(content)
	pat := strings.ToLower(pattern)
	count := 0
	start := 0
	for {
		idx := strings.Index(lower[start:], pat)
		if idx < 0 {
			break
		}
		count++
		start += idx + len(pat)
	}
	return count
}

// countLines counts non-empty lines in content.
func countLines(content string) int {
	count := 0
	for _, line := range strings.Split(content, "\n") {
		if strings.TrimSpace(line) != "" {
			count++
		}
	}
	return count
}

// extractFrontmatterField parses a YAML frontmatter field between --- delimiters.
func extractFrontmatterField(content, field string) string {
	lines := strings.Split(content, "\n")
	inFrontmatter := false
	fmStarted := false
	for _, line := range lines {
		trimmed := strings.TrimRight(line, "\r")
		if trimmed == "---" {
			if !fmStarted {
				fmStarted = true
				inFrontmatter = true
				continue
			}
			break
		}
		if inFrontmatter {
			prefix := field + ":"
			if strings.HasPrefix(trimmed, prefix) {
				val := strings.TrimPrefix(trimmed, prefix)
				val = strings.TrimSpace(val)
				val = strings.Trim(val, `"'`)
				return val
			}
		}
	}
	return ""
}

// codeBlockCount counts the number of triple-backtick fence pairs.
func codeBlockCount(content string) int {
	count := 0
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "```") {
			count++
		}
	}
	return count / 2
}

// removeCodeBlocks strips fenced code blocks from content (replicates awk '/^```/{skip=!skip;next} !skip').
func removeCodeBlocks(content string) string {
	var result strings.Builder
	skip := false
	for _, line := range strings.Split(content, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "```") {
			skip = !skip
			continue
		}
		if !skip {
			result.WriteString(line)
			result.WriteString("\n")
		}
	}
	return result.String()
}

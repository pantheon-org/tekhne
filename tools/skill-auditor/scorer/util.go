package scorer

import (
	"os"
	"regexp"
)

// fileExists returns true if the path exists and is a regular file.
func fileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && !info.IsDir()
}

// matchesRegexCI matches content against a regex pattern (case-insensitive).
func matchesRegexCI(content, pattern string) bool {
	re, err := regexp.Compile(pattern)
	if err != nil {
		return false
	}
	return re.MatchString(content)
}

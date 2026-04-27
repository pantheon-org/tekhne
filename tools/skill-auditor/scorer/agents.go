package scorer

import "strings"

// harnessDirs are the config/data directory names used by each supported agent
// (from https://github.com/vercel-labs/skills#supported-agents).
var harnessDirs = []string{
	".agents",
	".aider",
	".claude",
	".codeium",
	".codex",
	".continue",
	".copilot",
	".cursor",
	".deepagents",
	".firebender",
	".gemini",
	".goose",
	".pi",
	".windsurf",
}

// agentNames are the human-readable agent identifiers whose presence in skill
// content signals a non-portable, agent-specific instruction.
var agentNames = []string{
	"aider",
	"amp agent",
	"claude code",
	"cline",
	"codex",
	"continue.dev",
	"cursor agent",
	"deep agents",
	"firebender",
	"gemini cli",
	"github copilot",
	"goose",
	"kimi code",
	"opencode",
	"replit agent",
	"warp agent",
	"windsurf",
}

// findHarnessPath returns the first harness directory reference found in content,
// or an empty string if none is present.
func findHarnessPath(content string) string {
	for _, dir := range harnessDirs {
		if strings.Contains(content, dir+"/") {
			return dir + "/"
		}
	}
	return ""
}

// findAgentRef returns the first agent name reference found in content (case-insensitive),
// or an empty string if none is present.
func findAgentRef(content string) string {
	lower := strings.ToLower(content)
	for _, name := range agentNames {
		if strings.Contains(lower, name) {
			return name
		}
	}
	return ""
}

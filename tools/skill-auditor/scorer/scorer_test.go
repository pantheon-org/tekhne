package scorer

import (
	"path/filepath"
	"testing"
)

const fixturesDir = "../testdata/fixtures"

func TestScoreMinimal(t *testing.T) {
	skillPath := filepath.Join(fixturesDir, "skill-minimal", "SKILL.md")
	result, err := Score(skillPath)
	if err != nil {
		t.Fatalf("Score() error: %v", err)
	}
	if result.Total >= 80 {
		t.Errorf("skill-minimal expected total < 80, got %d (grade %s)", result.Total, result.Grade)
	}
	if result.Grade != "F" {
		t.Errorf("skill-minimal expected grade F, got %s (total %d)", result.Grade, result.Total)
	}
}

func TestScoreFullAGrade(t *testing.T) {
	skillPath := filepath.Join(fixturesDir, "skill-full", "SKILL.md")
	result, err := Score(skillPath)
	if err != nil {
		t.Fatalf("Score() error: %v", err)
	}
	if result.Total < 126 {
		t.Errorf("skill-full expected total >= 126, got %d (grade %s)", result.Total, result.Grade)
		for k, v := range result.Dimensions {
			t.Logf("  %s: %d", k, v)
		}
	}
	if GradeRank[result.Grade] < GradeRank["A"] {
		t.Errorf("skill-full expected grade A or A+, got %s (total %d)", result.Grade, result.Total)
	}
}

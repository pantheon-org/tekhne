package scorer

import (
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

func TestD7_VeryLongDescription(t *testing.T) {
	// descLen > 200 → 10
	b := &validatorBridge{Structure: &types.Report{
		Results: []types.Result{
			{Category: "Frontmatter", Level: types.Pass, Message: "description: (210 chars)"},
		},
	}}
	score, diags := scoreD7(b)
	if score != 10 {
		t.Errorf("want 10, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
}

func TestD7_MediumDescription(t *testing.T) {
	// 120 < descLen ≤ 200 → 9
	b := &validatorBridge{Structure: &types.Report{
		Results: []types.Result{
			{Category: "Frontmatter", Level: types.Pass, Message: "description: (150 chars)"},
		},
	}}
	score, diags := scoreD7(b)
	if score != 9 {
		t.Errorf("want 9, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics, got %v", diags)
	}
}

func TestD7_ShortDescription(t *testing.T) {
	// 60 < descLen ≤ 120 → 8
	b := &validatorBridge{Structure: &types.Report{
		Results: []types.Result{
			{Category: "Frontmatter", Level: types.Pass, Message: "description: (80 chars)"},
		},
	}}
	score, diags := scoreD7(b)
	if score != 8 {
		t.Errorf("want 8, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no diagnostics for 80-char description")
	}
}

func TestD7_VeryShortDescription(t *testing.T) {
	// descLen ≤ 30 → 6 + warning
	b := &validatorBridge{Structure: &types.Report{
		Results: []types.Result{
			{Category: "Frontmatter", Level: types.Pass, Message: "description: (20 chars)"},
		},
	}}
	score, diags := scoreD7(b)
	if score != 6 {
		t.Errorf("want 6, got %d", score)
	}
	if len(diags) == 0 {
		t.Error("expected D7 warning for very short description")
	}
}

func TestD7_FallbackNoBridge(t *testing.T) {
	// nilBridge() has no Structure → fallback 6 + warning
	score, diags := scoreD7(nilBridge())
	if score != 6 {
		t.Errorf("want 6, got %d", score)
	}
	if len(diags) == 0 {
		t.Error("expected D7 warning when bridge unavailable")
	}
}

func TestD7_DescLen31to60(t *testing.T) {
	// 30 < descLen ≤ 60 → 6, no warning
	b := &validatorBridge{Structure: &types.Report{
		Results: []types.Result{
			{Category: "Frontmatter", Level: types.Pass, Message: "description: (45 chars)"},
		},
	}}
	score, diags := scoreD7(b)
	if score != 6 {
		t.Errorf("want 6, got %d", score)
	}
	if len(diags) != 0 {
		t.Errorf("expected no warning for 45-char description, got %v", diags)
	}
}

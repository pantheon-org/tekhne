package scorer

import (
	"math"
	"testing"

	"github.com/agent-ecosystem/skill-validator/types"
)

func TestD6_HighSpecificity(t *testing.T) {
	// InstructionSpecificity=1.0 (all strong) → round(1.0*15) = 15
	b := &validatorBridge{Content: &types.ContentReport{
		StrongMarkers:          6,
		WeakMarkers:            0,
		InstructionSpecificity: 1.0,
	}}
	if score := scoreD6(b); score != 15 {
		t.Errorf("want 15, got %d", score)
	}
}

func TestD6_MixedSpecificity(t *testing.T) {
	// InstructionSpecificity=0.6 → round(0.6*15) = round(9.0) = 9
	b := &validatorBridge{Content: &types.ContentReport{
		StrongMarkers:          3,
		WeakMarkers:            2,
		InstructionSpecificity: 0.6,
	}}
	want := int(math.Round(0.6 * 15))
	if score := scoreD6(b); score != want {
		t.Errorf("want %d, got %d", want, score)
	}
}

func TestD6_ZeroMarkers(t *testing.T) {
	// No strong or weak markers → 0 (no directive language)
	b := &validatorBridge{Content: &types.ContentReport{
		StrongMarkers:          0,
		WeakMarkers:            0,
		InstructionSpecificity: 1.0, // library returns 1.0 when both zero — we override to 0
	}}
	if score := scoreD6(b); score != 0 {
		t.Errorf("want 0 when no markers, got %d", score)
	}
}

func TestD6_NilContent(t *testing.T) {
	if score := scoreD6(nilBridge()); score != 0 {
		t.Errorf("want 0 when bridge has no content, got %d", score)
	}
}

func TestD6_LowSpecificity(t *testing.T) {
	// InstructionSpecificity=0.2 → round(0.2*15) = round(3.0) = 3
	b := &validatorBridge{Content: &types.ContentReport{
		StrongMarkers:          1,
		WeakMarkers:            4,
		InstructionSpecificity: 0.2,
	}}
	want := int(math.Round(0.2 * 15))
	if score := scoreD6(b); score != want {
		t.Errorf("want %d, got %d", want, score)
	}
}

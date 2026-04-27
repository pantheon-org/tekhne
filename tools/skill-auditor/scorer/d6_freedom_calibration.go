package scorer

import "math"

// scoreD6 — Freedom Calibration (max: 15)
// score = round(InstructionSpecificity * 15); no baseline.
// Zero markers → 0 (no directive language is a genuine quality gap).
func scoreD6(b *validatorBridge) int {
	if b.Content == nil {
		return 0
	}
	if b.Content.StrongMarkers+b.Content.WeakMarkers == 0 {
		return 0
	}
	score := int(math.Round(b.Content.InstructionSpecificity * 15))
	if score > 15 {
		return 15
	}
	if score < 0 {
		return 0
	}
	return score
}

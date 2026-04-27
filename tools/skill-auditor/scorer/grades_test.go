package scorer

import "testing"

func TestGrade(t *testing.T) {
	tests := []struct {
		total    int
		expected string
	}{
		{140, "A+"},
		{133, "A+"},
		{132, "A"},
		{126, "A"},
		{125, "B+"},
		{119, "B+"},
		{118, "B"},
		{112, "B"},
		{111, "C+"},
		{105, "C+"},
		{104, "C"},
		{98, "C"},
		{97, "D"},
		{91, "D"},
		{90, "F"},
		{0, "F"},
	}

	for _, tt := range tests {
		got := Grade(tt.total)
		if got != tt.expected {
			t.Errorf("Grade(%d) = %q, want %q", tt.total, got, tt.expected)
		}
	}
}

package scorer

// GradeRank maps grade letters to numeric rank (higher = better).
var GradeRank = map[string]int{
	"A+": 8, "A": 7, "B+": 6, "B": 5, "C+": 4, "C": 3, "D": 2, "F": 1,
}

// Grade returns the letter grade for a total score.
func Grade(total int) string {
	switch {
	case total >= 133:
		return "A+"
	case total >= 126:
		return "A"
	case total >= 119:
		return "B+"
	case total >= 112:
		return "B"
	case total >= 105:
		return "C+"
	case total >= 98:
		return "C"
	case total >= 91:
		return "D"
	default:
		return "F"
	}
}

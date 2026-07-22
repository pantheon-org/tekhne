//! Grade bands and grade ranking, ported verbatim from `grades.go`.

/// Numeric rank for each grade letter (higher is better). Mirrors Go
/// `GradeRank`.
pub fn grade_rank(grade: &str) -> Option<i32> {
    match grade {
        "A+" => Some(8),
        "A" => Some(7),
        "B+" => Some(6),
        "B" => Some(5),
        "C+" => Some(4),
        "C" => Some(3),
        "D" => Some(2),
        "F" => Some(1),
        _ => None,
    }
}

/// Return the letter grade for a total score (Go `Grade`). `MaxTotal` is 140.
pub fn grade(total: i32) -> String {
    let letter = if total >= 133 {
        "A+"
    } else if total >= 126 {
        "A"
    } else if total >= 119 {
        "B+"
    } else if total >= 112 {
        "B"
    } else if total >= 105 {
        "C+"
    } else if total >= 98 {
        "C"
    } else if total >= 91 {
        "D"
    } else {
        "F"
    };
    letter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_bands() {
        let cases = [
            (140, "A+"),
            (133, "A+"),
            (132, "A"),
            (126, "A"),
            (125, "B+"),
            (119, "B+"),
            (118, "B"),
            (112, "B"),
            (111, "C+"),
            (105, "C+"),
            (104, "C"),
            (98, "C"),
            (97, "D"),
            (91, "D"),
            (90, "F"),
            (0, "F"),
        ];
        for (total, expected) in cases {
            assert_eq!(grade(total), expected, "grade({total})");
        }
    }
}

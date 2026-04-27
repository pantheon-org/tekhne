package scorer

import (
	"os"
	"path/filepath"
)

// Score evaluates a skill at skillPath and returns a Result.
func Score(skillPath string) (*Result, error) {
	contentBytes, err := os.ReadFile(skillPath)
	if err != nil {
		return nil, err
	}
	evalsDir := filepath.Join(filepath.Dir(skillPath), "evals")
	return ScoreFromContent(skillPath, string(contentBytes), evalsDir)
}

// ScoreFromContent scores a skill from pre-loaded content and an evals directory path.
func ScoreFromContent(skillPath, content, evalsDir string) (*Result, error) {
	skillDir := filepath.Dir(skillPath)

	d1, diag1 := scoreD1(content, skillDir)
	d2 := scoreD2(content)
	d3, diag3 := scoreD3(content, skillDir)
	d4, diag4 := scoreD4(content, skillDir)
	d5, lines, refCount, hasRefs := scoreD5WithMeta(content, skillDir)
	d6 := scoreD6(content)
	d7, diag7 := scoreD7(content)
	d8 := scoreD8(content)
	d9, diag9 := scoreD9(evalsDir)

	total := d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9

	var errorDetails, warningDetails []Diagnostic
	for _, d := range append(append(append(append(diag1, diag3...), diag4...), diag7...), diag9...) {
		if d.severity == "error" {
			errorDetails = append(errorDetails, d)
		} else {
			warningDetails = append(warningDetails, d)
		}
	}
	if !hasRefs {
		warningDetails = append(warningDetails, warnDiag("D5", "no references/ directory (progressive disclosure missing)"))
	}

	return &Result{
		Skill:                     skillPath,
		Total:                     total,
		MaxTotal:                  140,
		Grade:                     Grade(total),
		Lines:                     lines,
		HasReferences:             hasRefs,
		ReferenceCount:            refCount,
		ReferenceSectionCompliant: isReferenceSectionCompliant(content),
		Errors:                    len(errorDetails),
		Warnings:                  len(warningDetails),
		ErrorDetails:              errorDetails,
		WarningDetails:            warningDetails,
		Dimensions: map[string]int{
			"knowledgeDelta":          d1,
			"mindsetProcedures":       d2,
			"antiPatternQuality":      d3,
			"specificationCompliance": d4,
			"progressiveDisclosure":   d5,
			"freedomCalibration":      d6,
			"patternRecognition":      d7,
			"practicalUsability":      d8,
			"evalValidation":          d9,
		},
	}, nil
}

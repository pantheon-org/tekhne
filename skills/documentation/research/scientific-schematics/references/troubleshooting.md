# Troubleshooting and Quality Checklist

## AI Generation Issues

**Problem**: Overlapping text or elements
- **Solution**: AI generation automatically handles spacing
- **Solution**: Increase iterations: `--iterations 2` for better refinement

**Problem**: Elements not connecting properly
- **Solution**: Make your prompt more specific about connections and layout
- **Solution**: Increase iterations for better refinement

## Image Quality Issues

**Problem**: Export quality poor
- **Solution**: AI generation produces high-quality images automatically
- **Solution**: Increase iterations for better results: `--iterations 2`

**Problem**: Elements overlap after generation
- **Solution**: AI generation automatically handles spacing
- **Solution**: Increase iterations: `--iterations 2` for better refinement
- **Solution**: Make your prompt more specific about layout and spacing requirements

## Quality Check Issues

**Problem**: False positive overlap detection
- **Solution**: Adjust threshold: `detect_overlaps(image_path, threshold=0.98)`
- **Solution**: Manually review flagged regions in visual report

**Problem**: Generated image quality is low
- **Solution**: AI generation produces high-quality images by default
- **Solution**: Increase iterations for better results: `--iterations 2`

**Problem**: Colorblind simulation shows poor contrast
- **Solution**: Switch to Okabe-Ito palette explicitly in code
- **Solution**: Add redundant encoding (shapes, patterns, line styles)
- **Solution**: Increase color saturation and lightness differences

**Problem**: High-severity overlaps detected
- **Solution**: Review overlap_report.json for exact positions
- **Solution**: Increase spacing in those specific regions
- **Solution**: Re-run with adjusted parameters and verify again

**Problem**: Visual report generation fails
- **Solution**: Check Pillow and matplotlib installations
- **Solution**: Ensure image file is readable: `Image.open(path).verify()`
- **Solution**: Check sufficient disk space for report generation

## Accessibility Problems

**Problem**: Colors indistinguishable in grayscale
- **Solution**: Run accessibility checker: `verify_accessibility(image_path)`
- **Solution**: Add patterns, shapes, or line styles for redundancy
- **Solution**: Increase contrast between adjacent elements

**Problem**: Text too small when printed
- **Solution**: Run resolution validator: `validate_resolution(image_path)`
- **Solution**: Design at final size, use minimum 7-8 pt fonts
- **Solution**: Check physical dimensions in resolution report

**Problem**: Accessibility checks consistently fail
- **Solution**: Review accessibility_report.json for specific failures
- **Solution**: Increase color contrast by at least 20%
- **Solution**: Test with actual grayscale conversion before finalizing

Before submitting diagrams, verify:

## Visual Quality
- [ ] High-quality image format (PNG from AI generation)
- [ ] No overlapping elements (AI handles automatically)
- [ ] Adequate spacing between all components (AI optimizes)
- [ ] Clean, professional alignment
- [ ] All arrows connect properly to intended targets

## Accessibility
- [ ] Colorblind-safe palette (Okabe-Ito) used
- [ ] Works in grayscale (tested with accessibility checker)
- [ ] Sufficient contrast between elements (verified)
- [ ] Redundant encoding where appropriate (shapes + colors)
- [ ] Colorblind simulation passes all checks

## Typography and Readability
- [ ] Text minimum 7-8 pt at final size
- [ ] All elements labeled clearly and completely
- [ ] Consistent font family and sizing
- [ ] No text overlaps or cutoffs
- [ ] Units included where applicable

## Publication Standards
- [ ] Consistent styling with other figures in manuscript
- [ ] Comprehensive caption written with all abbreviations defined
- [ ] Referenced appropriately in manuscript text
- [ ] Meets journal-specific dimension requirements
- [ ] Exported in required format for journal (PDF/EPS/TIFF)

## Quality Verification (Required)
- [ ] Ran `run_quality_checks()` and achieved PASS status
- [ ] Reviewed overlap detection report (zero high-severity overlaps)
- [ ] Passed accessibility verification (grayscale and colorblind)
- [ ] Resolution validated at target DPI (300+ for print)
- [ ] Visual quality report generated and reviewed
- [ ] All quality reports saved with figure files

## Documentation and Version Control
- [ ] Source files (.tex, .py) saved for future revision
- [ ] Quality reports archived in `quality_reports/` directory
- [ ] Configuration parameters documented (colors, spacing, sizes)
- [ ] Git commit includes source, output, and quality reports
- [ ] README or comments explain how to regenerate figure

## Final Integration Check
- [ ] Figure displays correctly in compiled manuscript
- [ ] Cross-references work (`\ref{}` points to correct figure)
- [ ] Figure number matches text citations
- [ ] Caption appears on correct page relative to figure
- [ ] No compilation warnings or errors related to figure

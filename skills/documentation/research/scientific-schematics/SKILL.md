---
name: scientific-schematics
description: "Create publication-quality scientific diagrams using Claude AI with smart iterative refinement. Uses Claude for quality review. Only regenerates if quality is below threshold for your document type. Specialized in neural network architectures, system diagrams, flowcharts, biological pathways, and complex scientific visualizations."
allowed-tools: [Read, Write, Edit, Bash]
---

# Scientific Schematics and Diagrams

## Overview

Scientific schematics and diagrams transform complex concepts into clear visual representations for publication. **This skill uses Claude AI for diagram generation with Claude quality review.**

**How it works:**
- Describe your diagram in natural language
- Claude generates publication-quality images automatically
- **Claude reviews quality** against document-type thresholds
- **Smart iteration**: Only regenerates if quality is below threshold
- Publication-ready output in minutes
- No coding, templates, or manual drawing required

**Quality Thresholds by Document Type:**

| Document Type | Threshold | Description |
|---------------|-----------|-------------|
| journal | 8.5/10 | Nature, Science, peer-reviewed journals |
| conference | 8.0/10 | Conference papers |
| thesis | 8.0/10 | Dissertations, theses |
| grant | 8.0/10 | Grant proposals |
| preprint | 7.5/10 | arXiv, bioRxiv, etc. |
| report | 7.5/10 | Technical reports |
| poster | 7.0/10 | Academic posters |
| presentation | 6.5/10 | Slides, talks |
| default | 7.5/10 | General purpose |

**Simply describe what you want, and Claude creates it.** All diagrams are stored in the figures/ subfolder and referenced in papers/posters.

## Quick Start: Generate Any Diagram

Create any scientific diagram by simply describing it. Claude handles everything automatically with **smart iteration**:

```bash
# Generate for journal paper (highest quality threshold: 8.5/10)
python scripts/generate_schematic.py "CONSORT participant flow diagram with 500 screened, 150 excluded, 350 randomized" -o figures/consort.png --doc-type journal

# Generate for presentation (lower threshold: 6.5/10 - faster)
python scripts/generate_schematic.py "Transformer encoder-decoder architecture showing multi-head attention" -o figures/transformer.png --doc-type presentation

# Generate for poster (moderate threshold: 7.0/10)
python scripts/generate_schematic.py "MAPK signaling pathway from EGFR to gene transcription" -o figures/mapk_pathway.png --doc-type poster

# Custom max iterations (max 2)
python scripts/generate_schematic.py "Complex circuit diagram with op-amp, resistors, and capacitors" -o figures/circuit.png --iterations 2 --doc-type journal
```

**What happens behind the scenes:**
1. **Generation 1**: Claude creates initial image following scientific diagram best practices
2. **Review 1**: **Claude** evaluates quality against document-type threshold
3. **Decision**: If quality >= threshold → **DONE** (no more iterations needed!)
4. **If below threshold**: Improved prompt based on critique, regenerate
5. **Repeat**: Until quality meets threshold OR max iterations reached

**Smart Iteration Benefits:**
- ✅ Saves API calls if first generation is good enough
- ✅ Higher quality standards for journal papers
- ✅ Faster turnaround for presentations/posters
- ✅ Appropriate quality for each use case

**Output**: Versioned images plus a detailed review log with quality scores, critiques, and early-stop information.

### Configuration

Set your Anthropic Claude API API key:
```bash
export ANTHROPIC_API_KEY='your_api_key_here'
```

Get an API key at: <https://console.anthropic.com/>

### AI Generation Best Practices

**Effective Prompts for Scientific Diagrams:**

✓ **Good prompts** (specific, detailed):
- "CONSORT flowchart showing participant flow from screening (n=500) through randomization to final analysis"
- "Transformer neural network architecture with encoder stack on left, decoder stack on right, showing multi-head attention and cross-attention connections"
- "Biological signaling cascade: EGFR receptor → RAS → RAF → MEK → ERK → nucleus, with phosphorylation steps labeled"
- "Block diagram of IoT system: sensors → microcontroller → WiFi module → cloud server → mobile app"

✗ **Avoid vague prompts**:
- "Make a flowchart" (too generic)
- "Neural network" (which type? what components?)
- "Pathway diagram" (which pathway? what molecules?)

**Key elements to include:**
- **Type**: Flowchart, architecture diagram, pathway, circuit, etc.
- **Components**: Specific elements to include
- **Flow/Direction**: How elements connect (left-to-right, top-to-bottom)
- **Labels**: Key annotations or text to include
- **Style**: Any specific visual requirements

**Scientific Quality Guidelines** (automatically applied):
- Clean white/light background
- High contrast for readability
- Clear, readable labels (minimum 10pt)
- Professional typography (sans-serif fonts)
- Colorblind-friendly colors (Okabe-Ito palette)
- Proper spacing to prevent crowding
- Scale bars, legends, axes where appropriate

## Mindset

A diagram communicates faster than a paragraph. Three principles guide every generation:

1. **Precision in the prompt** — vague natural-language descriptions produce generic output that fails quality review. ALWAYS include element names, relationships, and notation conventions in the description.
2. **Document-type awareness** — quality thresholds differ by venue. A poster (7.0/10) and a Nature submission (8.5/10) require different levels of fidelity. ALWAYS pass the correct `--doc-type` flag.
3. **Smart iteration** — the system stops when quality is met, not when iterations are exhausted. NEVER set `--iterations` higher than 2; diminishing returns set in fast.

```bash
# Confirm your API key is set before running any generation
echo "ANTHROPIC_API_KEY is ${ANTHROPIC_API_KEY:+set}"
```

## When to Use This Skill

This skill should be used when:
- Creating neural network architecture diagrams (Transformers, CNNs, RNNs, etc.)
- Illustrating system architectures and data flow diagrams
- Drawing methodology flowcharts for study design (CONSORT, PRISMA)
- Visualizing algorithm workflows and processing pipelines
- Creating circuit diagrams and electrical schematics
- Depicting biological pathways and molecular interactions
- Generating network topologies and hierarchical structures
- Illustrating conceptual frameworks and theoretical models
- Designing block diagrams for technical papers

## When Not to Use This Skill

- The diagram is a data visualisation (bar chart, scatter plot, heatmap) — use a plotting library (matplotlib, seaborn, R) instead
- The figure must be reproduced exactly from a prior publication — use the original source or a vector redraw tool
- You need an interactive or animated diagram — this skill produces static publication images only

## How to Use This Skill

**Simply describe your diagram in natural language.** Claude generates it automatically:

```bash
python scripts/generate_schematic.py "your diagram description" -o output.png
```

**That's it!** The AI handles:
- ✓ Layout and composition
- ✓ Labels and annotations
- ✓ Colors and styling
- ✓ Quality review and refinement
- ✓ Publication-ready output

**Works for all diagram types:**
- Flowcharts (CONSORT, PRISMA, etc.)
- Neural network architectures
- Biological pathways
- Circuit diagrams
- System architectures
- Block diagrams
- Any scientific visualization

**No coding, no templates, no manual drawing required.**

---

## AI Generation Mode (Claude)

### Smart Iterative Refinement Workflow

The AI generation system uses **smart iteration** - it only regenerates if quality is below the threshold for your document type:

### How Smart Iteration Works

```
┌─────────────────────────────────────────────────────┐
│  1. Generate image with Claude             │
│                    ↓                                │
│  2. Review quality with Claude                │
│                    ↓                                │
│  3. Score >= threshold?                             │
│       YES → DONE! (early stop)                      │
│       NO  → Improve prompt, go to step 1            │
│                    ↓                                │
│  4. Repeat until quality met OR max iterations      │
└─────────────────────────────────────────────────────┘
```

### Iteration 1: Initial Generation
**Prompt Construction:**
```
Scientific diagram guidelines + User request
```

**Output:** `diagram_v1.png`

### Quality Review by Claude

Claude evaluates the diagram on:
1. **Scientific Accuracy** (0-2 points) - Correct concepts, notation, relationships
2. **Clarity and Readability** (0-2 points) - Easy to understand, clear hierarchy
3. **Label Quality** (0-2 points) - Complete, readable, consistent labels
4. **Layout and Composition** (0-2 points) - Logical flow, balanced, no overlaps
5. **Professional Appearance** (0-2 points) - Publication-ready quality

**Example Review Output:**
```
SCORE: 8.0

STRENGTHS:
- Clear flow from top to bottom
- All phases properly labeled
- Professional typography

ISSUES:
- Participant counts slightly small
- Minor overlap on exclusion box

VERDICT: ACCEPTABLE (for poster, threshold 7.0)
```

### Decision Point: Continue or Stop?

| If Score... | Action |
|-------------|--------|
| >= threshold | **STOP** - Quality is good enough for this document type |
| < threshold | Continue to next iteration with improved prompt |

**Example:**
- For a **poster** (threshold 7.0): Score of 7.5 → **DONE after 1 iteration!**
- For a **journal** (threshold 8.5): Score of 7.5 → Continue improving

### Subsequent Iterations (Only If Needed)

If quality is below threshold, the system:
1. Extracts specific issues from Claude's review
2. Enhances the prompt with improvement instructions
3. Regenerates with Claude
4. Reviews again with Claude
5. Repeats until threshold met or max iterations reached

### Review Log
All iterations are saved with a JSON review log that includes early-stop information:
```json
{
  "user_prompt": "CONSORT participant flow diagram...",
  "doc_type": "poster",
  "quality_threshold": 7.0,
  "iterations": [
    {
      "iteration": 1,
      "image_path": "figures/consort_v1.png",
      "score": 7.5,
      "needs_improvement": false,
      "critique": "SCORE: 7.5\nSTRENGTHS:..."
    }
  ],
  "final_score": 7.5,
  "early_stop": true,
  "early_stop_reason": "Quality score 7.5 meets threshold 7.0 for poster"
}
```

**Note:** With smart iteration, you may see only 1 iteration instead of the full 2 if quality is achieved early!

## Advanced AI Generation Usage

See [references/advanced-usage.md](references/advanced-usage.md) for:
- Python API (`ScientificSchematicGenerator` class)
- Full CLI option reference
- Prompt engineering tips

## AI Generation Examples

See [references/generation-examples.md](references/generation-examples.md) for complete examples:
CONSORT flowchart, neural network architecture, biological pathway, system architecture.

## Command-Line Usage

The main entry point for generating scientific schematics:

```bash
# Basic usage
python scripts/generate_schematic.py "diagram description" -o output.png

# Custom iterations (max 2)
python scripts/generate_schematic.py "complex diagram" -o diagram.png --iterations 2

# Verbose mode
python scripts/generate_schematic.py "diagram" -o out.png -v
```

**Note:** The Claude AI generation system includes automatic quality review in its iterative refinement process. Each iteration is evaluated for scientific accuracy, clarity, and accessibility.

## Best Practices Summary

### Design Principles

1. **Clarity over complexity** - Simplify, remove unnecessary elements
2. **Consistent styling** - Use templates and style files
3. **Colorblind accessibility** - Use Okabe-Ito palette, redundant encoding
4. **Appropriate typography** - Sans-serif fonts, minimum 7-8 pt
5. **Vector format** - Always use PDF/SVG for publication

### Technical Requirements

1. **Resolution** - Vector preferred, or 300+ DPI for raster
2. **File format** - PDF for LaTeX, SVG for web, PNG as fallback
3. **Color space** - RGB for digital, CMYK for print (convert if needed)
4. **Line weights** - Minimum 0.5 pt, typical 1-2 pt
5. **Text size** - 7-8 pt minimum at final size

### Integration Guidelines

1. **Include in LaTeX** - Use `\includegraphics{}` for generated images
2. **Caption thoroughly** - Describe all elements and abbreviations
3. **Reference in text** - Explain diagram in narrative flow
4. **Maintain consistency** - Same style across all figures in paper
5. **Version control** - Keep prompts and generated images in repository

## Troubleshooting Common Issues

See [references/troubleshooting.md](references/troubleshooting.md) for:
- AI generation issues (API errors, quality failures)
- Image quality issues (resolution, format)
- Accessibility problems (colour contrast, font size)

## Resources and References

### Detailed References

Load these files for comprehensive information on specific topics:

- **`references/diagram_types.md`** - Catalog of scientific diagram types with examples
- **`references/best_practices.md`** - Publication standards and accessibility guidelines

### External Resources

#### Python Libraries

- Schemdraw Documentation: <https://schemdraw.readthedocs.io/>
- NetworkX Documentation: <https://networkx.org/documentation/>
- Matplotlib Documentation: <https://matplotlib.org/>

#### Publication Standards

- Nature Figure Guidelines: <https://www.nature.com/nature/for-authors/final-submission>
- Science Figure Guidelines: <https://www.science.org/content/page/instructions-preparing-initial-manuscript>
- CONSORT Diagram: <http://www.consort-statement.org/consort-statement/flow-diagram>

## Integration with Other Skills

This skill works synergistically with:

- **Scientific Writing** - Diagrams follow figure best practices
- **Scientific Visualization** - Shares color palettes and styling
- **LaTeX Posters** - Generate diagrams for poster presentations
- **Research Grants** - Methodology diagrams for proposals
- **Peer Review** - Evaluate diagram clarity and accessibility

## Quick Reference Checklist

See [references/troubleshooting.md](references/troubleshooting.md) for the full quality,
accessibility, typography, and publication-standards checklist.

## Environment Setup

```bash
# Required
export ANTHROPIC_API_KEY='your_api_key_here'

# Get key at: https://console.anthropic.com/
```

## Quick Setup

**Simplest possible usage:**

```bash
export ANTHROPIC_API_KEY='your_api_key_here'
python ./scripts/generate_schematic.py "your diagram description" -o figures/output.png
```

---

## Anti-Patterns

### NEVER hardcode the API key

**WHY:** Committed credentials are immediately exposed in version history and will be rotated or revoked.

**BAD** — key hardcoded in source:

```python
api_key = "sk-ant-abc123"
```

**GOOD** — key from environment:

```bash
export ANTHROPIC_API_KEY='your_api_key_here'
python ./scripts/generate_schematic.py "diagram" -o figures/output.png
```

### NEVER use --iterations 3 or higher

**WHY:** The maximum is 2. Exceeding it either raises a validation error or produces no additional quality gain; smart iteration already stops early when the threshold is met.

**BAD** `--iterations 5` to "ensure maximum quality". → **GOOD** Use `--iterations 2` at most; the system stops early if quality is achieved on iteration 1.

### NEVER skip the --doc-type flag for formal submissions

**WHY:** The default threshold (7.5/10) is lower than journal (8.5/10). Skipping the flag for journal submissions means the diagram may pass a lower bar and fail editorial review.

**BAD** Run without `--doc-type` for a Nature submission. → **GOOD** ALWAYS pass `--doc-type journal` for peer-reviewed journals.

### NEVER use this skill for data plots

**WHY:** Matplotlib, seaborn, or R produce reproducible, data-bound figures with proper axis labels and error bars. The AI generation script is designed for conceptual scientific diagrams, not data visualisation.

**BAD** Generate a bar chart or scatter plot with this script. → **GOOD** Use a plotting library for data figures; reserve this skill for architecture diagrams, flowcharts, and pathway diagrams.

## References

- [Diagram Types Catalog](references/diagram_types.md) — prompt templates and quality notes for every supported diagram type
- [Publication Standards & Accessibility](references/best_practices.md) — colour, typography, layout, and file-format guidelines for publication-quality output

# Scientific Diagram Types Catalog

Reference catalog of diagram types supported by the scientific-schematics skill, with generation prompt templates and quality notes.

## Flowcharts & Process Diagrams

**CONSORT Flowchart** (clinical trials)
- Prompt template: `"CONSORT flowchart showing [N] participants recruited, [n] allocated to [arm1], [n] allocated to [arm2], [n] lost to follow-up, [n] analysed"`
- Required elements: recruitment, randomisation, allocation, follow-up, analysis boxes
- Recommended doc type: `journal` (8.5 threshold)

### Algorithm Flowchart
- Prompt template: `"Flowchart of [algorithm name] showing decision nodes at [step1], [step2], and termination conditions"`
- Required elements: start/end terminators, decision diamonds, process rectangles

**PRISMA Flowchart** (systematic reviews)
- Prompt template: `"PRISMA 2020 flowchart: [N] records identified, [n] screened, [n] assessed for eligibility, [n] included"`
- Recommended doc type: `journal`

## Network & Architecture Diagrams

### Neural Network Architecture
- Prompt template: `"[Architecture name] neural network: input layer → [encoder layers] → [bottleneck/attention] → [decoder layers] → output, label all layer types and dimensions"`
- Common variants: transformer encoder-decoder, CNN, RNN/LSTM, GAN
- Recommended doc type: `conference` or `journal`

### System Architecture
- Prompt template: `"System architecture diagram: [Component A] sends [data type] to [Component B] via [protocol], [Component C] reads from [storage], arrows labelled with data flow"`

## Biological & Chemical Diagrams

### Signalling Pathway
- Prompt template: `"[Pathway name] signalling pathway: [Receptor] → [intermediate1 with phosphorylation site] → [intermediate2] → [transcription factor] → gene expression, use standard biochemical notation"`
- Required elements: phosphorylation arrows (→P), activation/inhibition, membrane boundary

### Molecular Structure
- Prompt template: `"[Molecule name] molecular structure showing [key functional groups], label all bonds and atoms"`

## Statistical & Data Visualisation

### Conceptual Framework Diagram
- Use for theoretical models, not data plots
- Prompt template: `"Conceptual framework: [construct1] influences [construct2] (positive), moderated by [construct3], outcome is [construct4]"`

## Quality Notes by Diagram Type

| Type | Recommended doc-type | Common pitfalls |
|------|---------------------|-----------------|
| CONSORT | journal | Missing exclusion reason boxes |
| Neural network | conference | Unlabelled layers, missing dimensions |
| Signalling pathway | journal | Non-standard arrows, missing membrane |
| System architecture | conference | Missing data-flow labels |
| PRISMA | journal | Outdated 2009 vs 2020 format |

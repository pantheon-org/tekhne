# Phase 2: Redistribute Reference Documents

**Status:** Pending  
**Effort:** ~30 minutes  
**Dependencies:** Phase 1

## Description

Move all 44 reference documents from the original hub skill to appropriate focused skills using `git mv` to preserve history.

## File Distribution

### clean-architecture/references/ (37 docs)

**Dependency Management (6):**
- dep-acyclic-dependencies.md
- dep-data-crossing-boundaries.md
- dep-interface-ownership.md
- dep-inward-only.md
- dep-no-framework-imports.md
- dep-stable-abstractions.md

**Component Design (5):**
- comp-common-closure.md
- comp-common-reuse.md
- comp-reuse-release-equivalence.md
- comp-screaming-architecture.md
- comp-stable-dependencies.md

**Boundary Management (6):**
- bound-boundary-cost-awareness.md
- bound-defer-decisions.md
- bound-humble-object.md
- bound-main-component.md
- bound-partial-boundaries.md
- bound-service-internal-architecture.md

**Framework Integration (5):**
- frame-di-container-edge.md
- frame-domain-purity.md
- frame-logging-abstraction.md
- frame-orm-in-infrastructure.md
- frame-web-in-infrastructure.md

**Entity Layer (5):**
- entity-encapsulate-invariants.md
- entity-no-persistence-awareness.md
- entity-pure-business-rules.md
- entity-rich-not-anemic.md
- entity-value-objects.md

**Use Case Layer (6):**
- usecase-explicit-dependencies.md
- usecase-input-output-ports.md
- usecase-no-presentation-logic.md
- usecase-orchestrates-not-implements.md
- usecase-single-responsibility.md
- usecase-transaction-boundary.md

**Adapter Layer (5):**
- adapt-anti-corruption-layer.md
- adapt-controller-thin.md
- adapt-gateway-abstraction.md
- adapt-mapper-translation.md
- adapt-presenter-formats.md

### design-patterns/references/ (3 docs)
- anti-patterns-and-frameworks.md
- detailed-examples.md
- pattern-selection-workflow.md (to be created in Phase 3)

### testable-design/references/ (4 docs)
- test-boundary-verification.md
- test-layer-isolation.md
- test-testable-design.md
- test-tests-are-architecture.md

## Commands

```bash
# Set base paths
SRC="skills/software-engineering/software-design-principles/references"
DEST_CA="skills/software-engineering/design-principles/clean-architecture/references"
DEST_DP="skills/software-engineering/design-principles/design-patterns/references"
DEST_TD="skills/software-engineering/design-principles/testable-design/references"

# Move to clean-architecture
git mv $SRC/dep-*.md $DEST_CA/
git mv $SRC/comp-*.md $DEST_CA/
git mv $SRC/bound-*.md $DEST_CA/
git mv $SRC/frame-*.md $DEST_CA/
git mv $SRC/entity-*.md $DEST_CA/
git mv $SRC/usecase-*.md $DEST_CA/
git mv $SRC/adapt-*.md $DEST_CA/

# Move to design-patterns
git mv $SRC/anti-patterns-and-frameworks.md $DEST_DP/
git mv $SRC/detailed-examples.md $DEST_DP/

# Move to testable-design
git mv $SRC/test-*.md $DEST_TD/
```

## Additional Reference Doc Creation

### solid-principles/references/ (2-3 new docs)

Create practical refactoring examples:
- `srp-refactoring-examples.md` - God class splits, focused responsibility patterns
- `ocp-extension-patterns.md` - Strategy, Template Method, Plugin architecture examples

### design-patterns/references/ (3-5 new docs)

Move 3-5 implementation-focused docs from clean-architecture:
- `comp-screaming-architecture.md` → Relates to pattern selection
- `comp-reuse-release-equivalence.md` → Package/module design patterns
- `detailed-implementation-guides.md` (create new) - Common pattern implementations

**Rationale:** Redistribute clean-architecture docs (from 37 to 30-32) to balance reference distribution and prevent recreating monolithic structure.

## Acceptance Criteria

- [ ] All 44 original files moved (37 + 2 + 4 = 43, +1 to create)
- [ ] Git history preserved (used `git mv`)
- [ ] 2-3 new reference docs created for solid-principles
- [ ] 3-5 docs redistributed from clean-architecture to design-patterns
- [ ] No files left in original `software-design-principles/references/`
- [ ] File count verified in each destination (balanced distribution)

## Verification

```bash
# Count files in each destination
echo "clean-architecture: $(find $DEST_CA -type f -name "*.md" | wc -l) (expected: 37)"
echo "design-patterns: $(find $DEST_DP -type f -name "*.md" | wc -l) (expected: 2)"
echo "testable-design: $(find $DEST_TD -type f -name "*.md" | wc -l) (expected: 4)"

# Verify original is empty
echo "Original remaining: $(find $SRC -type f -name "*.md" 2>/dev/null | wc -l) (expected: 0)"

# Total check
echo "Total: $(find skills/software-engineering/design-principles -type f -path "*/references/*.md" | wc -l) (expected: 43)"
```

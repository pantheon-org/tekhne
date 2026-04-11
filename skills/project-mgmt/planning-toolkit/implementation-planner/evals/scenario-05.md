# Scenario 05: Create an Implementation Plan for an E-commerce Checkout Redesign

## User Prompt

Create a complete implementation plan for the following requirements.

## Project: E-commerce Checkout Redesign

**Summary**
Redesign the checkout flow of an existing e-commerce web app to improve conversion rates. The current checkout is a single-page form; the new design uses a three-step wizard (cart review → shipping → payment).

### Requirements

- Step 1 — Cart review: display line items, quantities, subtotal; allow quantity changes
- Step 2 — Shipping: address form with validation, delivery method selection, estimated delivery date
- Step 3 — Payment: credit card form (Stripe Elements), order summary, place order button
- Progress indicator showing current step
- Back navigation between steps without losing data
- Mobile-responsive layout (supports screens 320px wide and up)
- Client-side form validation with accessible error messages
- Order confirmation email sent via existing `sendgrid` integration
- Loading and error states for all async operations
- Unit tests for all business logic
- E2E tests with Playwright for the full checkout flow

**Tech stack**: React, TypeScript, Tailwind CSS, React Hook Form, Stripe Elements, existing Node.js/Express backend

Create a complete implementation plan under `.context/plans/` for this project.

## Expected Behavior

1. Use a lowercase kebab-case plan root directory slug (e.g. `plan-ecommerce-checkout-redesign`)
2. Use lowercase kebab-case phase directory slugs that describe the phase outcome (e.g. `phase-01-cart-review-component`)
3. Every task file uses the `P{NN}T{NN}` format with zero-padded 1-based numbers for both phase and task (e.g. `task-P01T01-*.md`, `task-P02T03-*.md`)
4. Task numbering within each phase starts at `T01` (not `T00`); phase numbering starts at `P01` (not `P00`)
5. Task numbers within each phase are sequential with no gaps
6. Phase numbers are sequential with no gaps across the plan
7. No directory or file name contains spaces, underscores, or mixed case — all paths use hyphens
8. All phase and task numbers use two-digit zero-padding (01, 02, … 10, 11 — not 1, 2, 10)
9. The slug portion of each task filename is descriptive and reflects the task content (e.g. `task-P01T02-cart-item-quantity-component.md`)
10. Root `README.md` links to phases but contains no task-level implementation detail
11. Every phase `README.md` includes a goal statement and a concrete, runnable gate
12. Run `validate-plan.sh <plan-slug>` after creating all files

## Success Criteria

- **plan-slug-is-kebab-case**: The plan root directory slug is lowercase kebab-case
- **phase-slugs-are-kebab-case**: All phase directory slugs are lowercase kebab-case and describe the phase outcome; no numeric-only slugs
- **task-identifier-format-all-files**: Every task file uses the `P{NN}T{NN}` format with zero-padded 1-based numbers for both phase and task
- **task-numbering-starts-at-01**: Task numbering starts at `T01` (not `T00`); phase numbering starts at `P01` (not `P00`)
- **task-numbering-sequential-no-gaps**: Task numbers within each phase are sequential with no gaps
- **phase-numbering-sequential-no-gaps**: Phase numbers are sequential with no gaps across the plan
- **no-spaces-or-underscores-in-slugs**: No directory or file name contains spaces, underscores, or mixed case; all paths use hyphens
- **phase-and-task-zero-padding-two-digits**: All phase and task numbers use two-digit zero-padding (01, 02, … 10, 11)
- **task-slug-after-identifier-is-descriptive**: The slug portion of each task filename is descriptive and reflects the task content
- **root-readme-is-index**: Root `README.md` links to phases but contains no task-level implementation detail
- **phase-readmes-include-goal-gate**: Every phase `README.md` includes a goal statement and a concrete, runnable gate
- **validate-plan-run**: Agent ran `validate-plan.sh <plan-slug>` after creating all files
- **completion-summary-includes-paths**: Completion summary lists all file paths including directory-separated structure

## Failure Conditions

- Plan root directory slug uses uppercase, underscores, or spaces
- Phase directory slugs use uppercase, underscores, spaces, or are numeric-only
- Task files use incorrect format (e.g. `task-1-1.md`, `task-p1t1.md`, `task-01-03.md`)
- Task or phase numbering starts at 0 instead of 1
- Task or phase numbering has gaps (e.g. T01, T03, skipping T02)
- Directory or file names contain spaces, underscores, or uppercase letters
- Phase or task numbers use single-digit format without zero-padding
- Task slug is non-descriptive (e.g. `task-P01T02-task.md` or `task-P01T02-step.md`)
- Root `README.md` contains implementation details instead of navigation links
- Phase `README.md` files lack goal statements or runnable gates
- Does not run `validate-plan.sh`

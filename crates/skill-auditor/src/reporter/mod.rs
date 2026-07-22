//! Human-readable and stored report rendering, ported from the Go
//! `tools/skill-auditor/reporter` package.

mod analysis;
mod remediation;
mod report;
mod store;

pub use analysis::analysis;
pub use remediation::remediation;
pub use report::format;
pub use store::store;

/// Canonical display order for the nine dimensions: `(key, label, max)`.
/// Shared by [`format`], [`analysis`] and [`remediation`] (Go
/// `dimensionOrder`).
const DIMENSION_ORDER: &[(&str, &str, i32)] = &[
    ("knowledgeDelta", "Knowledge Delta", 20),
    ("mindsetProcedures", "Mindset + Procedures", 15),
    ("antiPatternQuality", "Anti-Pattern Quality", 15),
    ("specificationCompliance", "Specification Compliance", 15),
    ("progressiveDisclosure", "Progressive Disclosure", 15),
    ("freedomCalibration", "Freedom Calibration", 15),
    ("patternRecognition", "Pattern Recognition", 10),
    ("practicalUsability", "Practical Usability", 15),
    ("evalValidation", "Eval Validation", 20),
];

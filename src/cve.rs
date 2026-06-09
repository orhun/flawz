use nvd_cve::cve::Cve as NvdCve;

/// Flat, UI-friendly view of an NVD CVE record.
///
/// Built from [`nvd_cve::cve::Cve`] so the rest of the TUI does not need to
/// know about the API's nested shape (descriptions per language, references
/// with source/tags, multiple CVSS scoring systems).
#[derive(Clone, Debug)]
pub struct Cve {
    /// CVE ID.
    pub id: String,
    /// English description, when available.
    pub description: Option<String>,
    /// Identifier of the CNA / source that submitted the record.
    pub assigner: String,
    /// External reference URLs, deduplicated upstream by NVD.
    pub references: Vec<String>,
    /// Highest-precedence CVSS base score (`v4.0 → v3.1 → v3.0 → v2.0`).
    pub base_score: Option<f32>,
    /// Severity label matching `base_score` (`LOW`, `MEDIUM`, `HIGH`, `CRITICAL`).
    pub severity: Option<String>,
    /// `Analyzed`, `Modified`, `Rejected`, … (from NVD's `vulnStatus`).
    pub status: String,
}

impl From<NvdCve> for Cve {
    fn from(cve: NvdCve) -> Self {
        let description = cve.description_en().map(str::to_string);
        let base_score = cve.base_score();
        let severity = cve.severity().map(str::to_string);
        Self {
            id: cve.id,
            description,
            assigner: cve.source_identifier,
            references: cve.references.into_iter().map(|r| r.url).collect(),
            base_score,
            severity,
            status: cve.vuln_status,
        }
    }
}

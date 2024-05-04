use nvd_cve::cve::Cve as NvdCve;

/// CVE.
#[derive(Clone, Debug)]
pub struct Cve {
    /// ID.
    pub id: String,
    /// Description.
    pub description: Option<String>,
    /// Assigner.
    pub assigner: String,
    /// References.
    pub references: Vec<String>,
}

impl From<NvdCve> for Cve {
    fn from(cve: NvdCve) -> Self {
        Self {
            id: cve.cve_data_meta.id.to_string(),
            description: cve
                .description
                .description_data
                .iter()
                .find(|desc| desc.lang == *"en")
                .map(|v| v.value.to_string()),
            assigner: cve.cve_data_meta.assigner.to_string(),
            references: cve
                .references
                .reference_data
                .iter()
                .map(|v| v.url.to_string())
                .collect(),
        }
    }
}

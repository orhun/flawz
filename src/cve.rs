use nvd_cve::cve::Cve as NvdCve;

#[derive(Clone, Debug)]
pub struct Cve {
    pub id: String,
    pub description: Option<String>,
    pub assigner: String,
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

use indy_api_types::validation::Validatable;

pub use indy_vdr::ledger::requests::rev_reg::{RevocationRegistryDeltaV1};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum RevocationRegistryDelta {
    #[serde(rename = "1.0")]
    RevocationRegistryDeltaV1(RevocationRegistryDeltaV1)
}

impl From<RevocationRegistryDelta> for RevocationRegistryDeltaV1 {
    fn from(rev_reg_delta: RevocationRegistryDelta) -> Self {
        match rev_reg_delta {
            RevocationRegistryDelta::RevocationRegistryDeltaV1(rev_reg_delta) => rev_reg_delta,
        }
    }
}

impl Validatable for RevocationRegistryDelta {}
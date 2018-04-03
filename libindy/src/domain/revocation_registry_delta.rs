extern crate indy_crypto;
extern crate serde;
extern crate serde_json;

use self::indy_crypto::utils::json::JsonDecodable;
use self::indy_crypto::cl::{RevocationRegistryDelta as RegistryDelta, RevocationRegistry};

use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevocationRegistryDeltaFromLedger {
    pub value: RevocationRegistryDeltaValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevocationRegistryDeltaValue {
    pub accum_from: Option<AccumulatorState>,
    pub accum_to: AccumulatorState,
    pub issued: HashSet<u32>,
    pub revoked: HashSet<u32>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccumulatorState {
    pub value: RevocationRegistry
}

pub type RevocationRegistryDeltaLocal = RegistryDelta;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RevocationRegistryDelta {
    DeltaLocal(RevocationRegistryDeltaLocal),
    DeltaFromLedger(RevocationRegistryDeltaFromLedger)
}

impl<'a> JsonDecodable<'a> for RevocationRegistryDelta {}

impl From<RevocationRegistryDelta> for RevocationRegistryDeltaLocal {
    fn from(rev_reg_delta: RevocationRegistryDelta) -> Self {
        match rev_reg_delta {
            RevocationRegistryDelta::DeltaLocal(rev_reg_delta) => rev_reg_delta,
            RevocationRegistryDelta::DeltaFromLedger(rev_reg_delta) =>
                RegistryDelta::from_parts(&rev_reg_delta.value.accum_from.map(|accum| accum.value).unwrap(),
                                          &rev_reg_delta.value.issued,
                                          &rev_reg_delta.value.revoked).unwrap()
        }
    }
}
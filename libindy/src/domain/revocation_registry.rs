extern crate indy_crypto;

use self::indy_crypto::cl::RevocationRegistry as CryptoRevocationRegistry;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct RevocationRegistryData {
    pub value: CryptoRevocationRegistry,
}

pub type RevocationRegistryLocal = CryptoRevocationRegistry;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RevocationRegistry {
    RevocationRegistryLocal(RevocationRegistryLocal),
    RevocationRegistryFromLedger(RevocationRegistryData)
}

impl<'a> JsonDecodable<'a> for RevocationRegistry {}

impl From<RevocationRegistry> for RevocationRegistryLocal {
    fn from(rev_reg: RevocationRegistry) -> Self {
        match rev_reg {
            RevocationRegistry::RevocationRegistryLocal(rev_reg) => rev_reg,
            RevocationRegistry::RevocationRegistryFromLedger(rev_reg) => rev_reg.value
        }
    }
}

pub fn rev_regs_map_to_rev_regs_local_map(rev_regs: HashMap<String, HashMap<u64, RevocationRegistry>>) -> HashMap<String, HashMap<u64, RevocationRegistryLocal>> {
    let mut rev_regs_local: HashMap<String, HashMap<u64, RevocationRegistryLocal>> = HashMap::new();

    for (rev_reg_id, rev_reg_to_timespams) in rev_regs {
        let mut rev_regs_for_id: HashMap<u64, RevocationRegistryLocal> = HashMap::new();

        for (timestamp, rev_reg) in rev_reg_to_timespams {
            rev_regs_for_id.insert(timestamp, RevocationRegistryLocal::from(rev_reg));
        }
        rev_regs_local.insert(rev_reg_id, rev_regs_for_id);
    }
    rev_regs_local
}
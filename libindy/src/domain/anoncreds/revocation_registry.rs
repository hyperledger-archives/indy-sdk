use ursa::cl::RevocationRegistry as CryptoRevocationRegistry;
use named_type::NamedType;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationRegistryV1 {
    pub value: CryptoRevocationRegistry
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum RevocationRegistry {
    #[serde(rename = "1.0")]
    RevocationRegistryV1(RevocationRegistryV1),
}

impl From<RevocationRegistry> for RevocationRegistryV1 {
    fn from(rev_reg: RevocationRegistry) -> Self {
        match rev_reg {
            RevocationRegistry::RevocationRegistryV1(rev_reg) => rev_reg,
        }
    }
}

pub fn rev_regs_map_to_rev_regs_local_map(rev_regs: HashMap<String, HashMap<u64, RevocationRegistry>>) -> HashMap<String, HashMap<u64, RevocationRegistryV1>> {
    let mut rev_regs_local: HashMap<String, HashMap<u64, RevocationRegistryV1>> = HashMap::new();

    for (rev_reg_id, rev_reg_to_timespams) in rev_regs {
        let mut rev_regs_for_id: HashMap<u64, RevocationRegistryV1> = HashMap::new();

        for (timestamp, rev_reg) in rev_reg_to_timespams {
            rev_regs_for_id.insert(timestamp, RevocationRegistryV1::from(rev_reg));
        }
        rev_regs_local.insert(rev_reg_id, rev_regs_for_id);
    }
    rev_regs_local
}

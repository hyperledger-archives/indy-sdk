pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pairwise;
pub mod pool;
pub mod wallet;
pub mod cache;

use regex::Regex;
use utils::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndyConfig {
    pub crypto_thread_pool_size : Option<usize>,
    pub collect_backtrace: Option<bool>,
    pub freshness_threshold: Option<u64>,
    pub did_default_method_name: Option<String>,
    pub did_protocol_version: Option<usize>
}

impl Validatable for IndyConfig {
    fn validate(&self) -> Result<(), String> {
        if let Some(version) = self.did_protocol_version {
            if version != 0 && version != 1 {
                return Err(format!("Invalid DID protocol version: {}. Should be 0 or 1.", version));
            }
        }

        if let Some(ref name) = self.did_default_method_name {
            lazy_static! {
                static ref REGEX_METHOD_NAME: Regex = Regex::new("^[a-z0-9]+$").unwrap();
            }
            if !REGEX_METHOD_NAME.is_match(name) {
                return Err(format!("Invalid default name: {}. It does not match the DID method name format.", name))
            }
        }

        Ok(())
    }
}
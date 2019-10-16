pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pairwise;
pub mod pool;
pub mod wallet;
pub mod cache;

use crate::utils::validation::Validatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndyConfig {
    pub crypto_thread_pool_size: Option<usize>,
    pub collect_backtrace: Option<bool>,
    pub freshness_threshold: Option<u64>
}

impl Validatable for IndyConfig {}
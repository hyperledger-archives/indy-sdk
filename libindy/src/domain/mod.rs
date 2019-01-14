pub mod anoncreds;
pub mod crypto;
pub mod ledger;
pub mod pairwise;
pub mod pool;
pub mod wallet;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndyConfig {
    pub crypto_thread_pool_size : usize
}
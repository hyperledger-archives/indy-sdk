pub const POOL_CON_ACTIVE_TO: i64 = 5;
pub const POOL_ACK_TIMEOUT: i64 = 10;
pub const POOL_REPLY_TIMEOUT: i64 = 50;
pub const MAX_REQ_PER_POOL_CON: usize = 5;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolOpenConfig {
    #[serde(default="PoolOpenConfig::default_timeout")]
    pub timeout: i64,
    #[serde(default="PoolOpenConfig::default_extended_timeout")]
    pub extended_timeout: i64,
    #[serde(default="PoolOpenConfig::default_conn_limit")]
    pub conn_limit: usize,
    #[serde(default="PoolOpenConfig::default_conn_active_timeout")]
    pub conn_active_timeout: i64,
}

impl Default for PoolOpenConfig {
    fn default() -> Self {
        PoolOpenConfig {
            timeout: PoolOpenConfig::default_timeout(),
            extended_timeout: PoolOpenConfig::default_extended_timeout(),
            conn_limit: PoolOpenConfig::default_conn_limit(),
            conn_active_timeout: PoolOpenConfig::default_conn_active_timeout(),
        }
    }
}

impl PoolOpenConfig {
    fn default_timeout() -> i64 {
        POOL_ACK_TIMEOUT
    }

    fn default_extended_timeout() -> i64 {
        POOL_REPLY_TIMEOUT
    }

    fn default_conn_limit() -> usize {
        MAX_REQ_PER_POOL_CON
    }

    fn default_conn_active_timeout() -> i64 {
        POOL_CON_ACTIVE_TO
    }
}

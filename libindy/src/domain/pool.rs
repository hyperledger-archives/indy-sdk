use utils::validation::Validatable;

pub const POOL_CON_ACTIVE_TO: i64 = 5;
pub const POOL_ACK_TIMEOUT: i64 = 20;
pub const POOL_REPLY_TIMEOUT: i64 = 60;
pub const MAX_REQ_PER_POOL_CON: usize = 5;
pub const NUMBER_READ_NODES: u8 = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolConfig {
    pub genesis_txn: String
}

impl PoolConfig {
    pub fn default_for_name(name: &str) -> PoolConfig {
        let mut txn = name.to_string();
        txn += ".txn";
        PoolConfig { genesis_txn: txn }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolOpenConfig {
    #[serde(default = "PoolOpenConfig::default_timeout")]
    pub timeout: i64,
    #[serde(default = "PoolOpenConfig::default_extended_timeout")]
    pub extended_timeout: i64,
    #[serde(default = "PoolOpenConfig::default_conn_limit")]
    pub conn_limit: usize,
    #[serde(default = "PoolOpenConfig::default_conn_active_timeout")]
    pub conn_active_timeout: i64,
    #[serde(default = "PoolOpenConfig::default_preordered_nodes")]
    pub preordered_nodes: Vec<String>,
    #[serde(default = "PoolOpenConfig::default_number_read_nodes")]
    pub number_read_nodes: u8,
}

impl Validatable for PoolOpenConfig {
    fn validate(&self) -> Result<(), String> {
        if self.timeout <= 0 {
            return Err(String::from("`timeout` must be greater than 0"));
        }
        if self.extended_timeout <= 0 {
            return Err(String::from("`extended_timeout` must be greater than 0"));
        }
        if self.conn_limit == 0 {
            return Err(String::from("`conn_limit` must be greater than 0"));
        }
        if self.conn_active_timeout <= 0 {
            return Err(String::from("`conn_active_timeout` must be greater than 0"));
        }
        if self.number_read_nodes == 0 {
            return Err(String::from("`number_read_nodes` must be greater than 0"));
        }
        Ok(())
    }
}

impl Default for PoolOpenConfig {
    fn default() -> Self {
        PoolOpenConfig {
            timeout: PoolOpenConfig::default_timeout(),
            extended_timeout: PoolOpenConfig::default_extended_timeout(),
            conn_limit: PoolOpenConfig::default_conn_limit(),
            conn_active_timeout: PoolOpenConfig::default_conn_active_timeout(),
            preordered_nodes: PoolOpenConfig::default_preordered_nodes(),
            number_read_nodes: PoolOpenConfig::default_number_read_nodes(),
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

    fn default_preordered_nodes() -> Vec<String> {
        Vec::new()
    }

    fn default_number_read_nodes() -> u8 { NUMBER_READ_NODES }
}

pub const NODE: &str = "0";
pub const NYM: &str = "1";
pub const GET_TXN: &str = "3";
pub const ATTRIB: &str = "100";
pub const SCHEMA: &str = "101";
pub const CRED_DEF: &str = "102";
pub const GET_ATTR: &str = "104";
pub const GET_NYM: &str = "105";
pub const GET_SCHEMA: &str = "107";
pub const GET_CRED_DEF: &str = "108";
pub const POOL_UPGRADE: &str = "109";
pub const POOL_RESTART: &str = "118";
pub const POOL_CONFIG: &str = "111";
pub const REVOC_REG_DEF: &str = "113";
pub const REVOC_REG_ENTRY: &str = "114";
pub const GET_REVOC_REG_DEF: &str = "115";
pub const GET_REVOC_REG: &str = "116";
pub const GET_REVOC_REG_DELTA: &str = "117";
pub const GET_VALIDATOR_INFO: &str = "119";
pub const AUTH_RULE: &str = "120";
pub const GET_DDO: &str = "120";//TODO change number

pub const WRITE_REQUESTS: [&str; 9] = [NODE, NYM, ATTRIB, SCHEMA, CRED_DEF, POOL_UPGRADE, POOL_CONFIG, REVOC_REG_DEF, REVOC_REG_ENTRY];

pub const TRUSTEE: &str = "0";
pub const STEWARD: &str = "2";
pub const TRUST_ANCHOR: &str = "101";
pub const NETWORK_MONITOR: &str = "201";
pub const ROLE_REMOVE: &str = "";


pub fn txn_name_to_code(txn: &str) -> Option<&str> {
    if WRITE_REQUESTS.contains(&txn) {
        return Some(txn)
    }

    match txn {
        "NODE" => Some(NODE),
        "NYM" => Some(NYM),
        "ATTRIB" => Some(ATTRIB),
        "SCHEMA" => Some(SCHEMA),
        "CRED_DEF" | "CLAIM_DEF" => Some(CRED_DEF),
        "POOL_UPGRADE" => Some(POOL_UPGRADE),
        "POOL_CONFIG" => Some(POOL_CONFIG),
        "REVOC_REG_DEF" => Some(REVOC_REG_DEF),
        "REVOC_REG_ENTRY" => Some(REVOC_REG_ENTRY),
        _ => None
    }
}
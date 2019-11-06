pub const NODE: &str = "0";
pub const NYM: &str = "1";
pub const GET_TXN: &str = "3";
pub const TXN_AUTHR_AGRMT: &str = "4";  // TODO Use nonabbreviated names as in updated design
pub const TXN_AUTHR_AGRMT_AML: &str = "5";
pub const GET_TXN_AUTHR_AGRMT: &str = "6";
pub const GET_TXN_AUTHR_AGRMT_AML: &str = "7";
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
pub const GET_AUTH_RULE: &str = "121";
pub const AUTH_RULES: &str = "122";
pub const GET_DDO: &str = "120";//TODO change number

pub const REQUESTS: [&str; 25] = [NODE, NYM, GET_TXN, ATTRIB, SCHEMA, CRED_DEF, GET_ATTR, GET_NYM, GET_SCHEMA,
    GET_CRED_DEF, POOL_UPGRADE, POOL_RESTART, POOL_CONFIG, REVOC_REG_DEF, REVOC_REG_ENTRY, GET_REVOC_REG_DEF,
    GET_REVOC_REG, GET_REVOC_REG_DELTA, GET_VALIDATOR_INFO, AUTH_RULE, GET_DDO, TXN_AUTHR_AGRMT, TXN_AUTHR_AGRMT_AML,
    GET_TXN_AUTHR_AGRMT, GET_TXN_AUTHR_AGRMT_AML];

pub const TRUSTEE: &str = "0";
pub const STEWARD: &str = "2";
pub const ENDORSER: &str = "101";
pub const NETWORK_MONITOR: &str = "201";
pub const ROLE_REMOVE: &str = "";

pub const ROLES: [&str; 4] = [TRUSTEE, STEWARD, ENDORSER, NETWORK_MONITOR];

pub fn txn_name_to_code(txn: &str) -> Option<&str> {
    if REQUESTS.contains(&txn) {
        return Some(txn)
    }

    match txn {
        "NODE" => Some(NODE),
        "NYM" => Some(NYM),
        "GET_TXN" => Some(GET_TXN),
        "ATTRIB" => Some(ATTRIB),
        "SCHEMA" => Some(SCHEMA),
        "CRED_DEF" | "CLAIM_DEF" => Some(CRED_DEF),
        "GET_ATTR" => Some(GET_ATTR),
        "GET_NYM" => Some(GET_NYM),
        "GET_SCHEMA" => Some(GET_SCHEMA),
        "GET_CRED_DEF" => Some(GET_CRED_DEF),
        "POOL_UPGRADE" => Some(POOL_UPGRADE),
        "POOL_RESTART" => Some(POOL_RESTART),
        "POOL_CONFIG" => Some(POOL_CONFIG),
        "REVOC_REG_DEF" => Some(REVOC_REG_DEF),
        "REVOC_REG_ENTRY" => Some(REVOC_REG_ENTRY),
        "GET_REVOC_REG_DEF" => Some(GET_REVOC_REG_DEF),
        "GET_REVOC_REG" => Some(GET_REVOC_REG),
        "GET_REVOC_REG_DELTA" => Some(GET_REVOC_REG_DELTA),
        "GET_VALIDATOR_INFO" => Some(GET_VALIDATOR_INFO),
        "AUTH_RULE" => Some(AUTH_RULE),
        "GET_DDO" => Some(GET_DDO),
        "TXN_AUTHR_AGRMT" => Some(TXN_AUTHR_AGRMT),
        "TXN_AUTHR_AGRMT_AML" => Some(TXN_AUTHR_AGRMT_AML),
        "GET_TXN_AUTHR_AGRMT" => Some(GET_TXN_AUTHR_AGRMT),
        "GET_TXN_AUTHR_AGRMT_AML" => Some(GET_TXN_AUTHR_AGRMT_AML),
        val => Some(val)
    }
}
extern crate byteorder;
extern crate futures;
extern crate rmp_serde;

use self::byteorder::{LittleEndian, WriteBytesExt};

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use indy::{ErrorCode, IndyError};
use indy::pool;
use self::futures::Future;
use serde_json;
use time;

use utils::types::{Response, ResponseType};
use utils::constants::PROTOCOL_VERSION;
use utils::{environment, test};

#[derive(Serialize, Deserialize)]
struct PoolConfig {
    pub genesis_txn: String
}

pub fn create_genesis_txn_file(pool_name: &str,
                               txn_file_data: &str,
                               txn_file_path: Option<&Path>) -> PathBuf {
    let txn_file_path = txn_file_path.map_or(
        environment::tmp_file_path(format!("{}.txn", pool_name).as_str()),
        |path| path.to_path_buf());

    if !txn_file_path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(txn_file_path.parent().unwrap()).unwrap();
    }

    let mut f = fs::File::create(txn_file_path.as_path()).unwrap();
    f.write_all(txn_file_data.as_bytes()).unwrap();
    f.flush().unwrap();
    f.sync_all().unwrap();

    txn_file_path
}

pub fn create_genesis_txn_file_for_test_pool(pool_name: &str,
                                             nodes_count: Option<u8>,
                                             txn_file_path: Option<&Path>) -> PathBuf {
    let nodes_count = nodes_count.unwrap_or(4);

    let node_txns = test::gen_txns();

    let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");

    create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

pub fn create_genesis_txn_file_for_test_pool_with_invalid_nodes(pool_name: &str,
                                                                txn_file_path: Option<&Path>) -> PathBuf {
    let test_pool_ip = environment::test_pool_ip();
    let node_txns = test::gen_txns();

    let node_txns = node_txns.iter().map(|txn|
        txn.replace(format!(r#""client_ip":"{0}","client_port":9702,"node_ip":"{0}","node_port":9701"#, test_pool_ip).as_str(), r#""node_port":9701"#))
        .collect::<Vec<String>>();

    let txn_file_data = node_txns.join("\n");
    create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

pub fn create_genesis_txn_file_for_empty_lines(pool_name: &str,
                                               txn_file_path: Option<&Path>) -> PathBuf {
    let mut node_txns = test::gen_txns();
    node_txns.insert(0, "      \n".to_string());
    node_txns.insert(2, "\n".to_string());
    node_txns.insert(5, "      \n".to_string());
    node_txns.push("      \n".to_string());

    let txn_file_data = node_txns.join("\n");
    create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

pub fn create_genesis_txn_file_for_test_pool_with_wrong_alias(pool_name: &str,
                                                              txn_file_path: Option<&Path>) -> PathBuf {
    let mut node_txns = test::gen_txns();
    node_txns[0] = node_txns[0].replace("Node1", "ALIAS_NODE");

    let txn_file_data = node_txns.join("\n");
    create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

pub fn create_genesis_txn_file_for_test_pool_with_wrong_ips(pool_name: &str,
                                                            txn_file_path: Option<&Path>) -> PathBuf {
    let node_txns = test::gen_txns();
    let node_txns = node_txns.iter().map(|txn|
        txn.replace(environment::test_pool_ip().as_str(), "aa")).collect::<Vec<String>>();

    let txn_file_data = node_txns.join("\n");

    create_genesis_txn_file(pool_name, txn_file_data.as_str(), txn_file_path)
}

// Note that to be config valid it assumes genesis txt file is already exists
pub fn pool_config_json(txn_file_path: &Path) -> String {
    let config = PoolConfig {
        genesis_txn: txn_file_path.to_string_lossy().to_string()
    };

    serde_json::to_string(&config).unwrap()
}

pub fn create_pool_ledger_config(pool_name: &str, pool_config: Option<&str>) -> Result<(), IndyError> {
    pool::create_pool_ledger_config(pool_name, pool_config).wait()
}

#[cfg(feature = "local_nodes_pool")]
pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, IndyError> {
    pool::open_pool_ledger(pool_name, config).wait()
}

pub fn dump_correct_genesis_txns_to_cache(pool_name: &str) -> Result<(), ErrorCode> {
    _dump_genesis_txns_to_cache(pool_name, &test::gen_txns())
}

pub fn dump_incorrect_genesis_txns_to_cache(pool_name: &str) -> Result<(), ErrorCode> {
    let mut node_txns = test::gen_txns();
    node_txns[0] = node_txns[0].replace("Node1", "ALIAS_NODE");

    _dump_genesis_txns_to_cache(pool_name, &node_txns)
}

fn _dump_genesis_txns_to_cache(pool_name: &str, node_txns: &Vec<String>) -> Result<(), ErrorCode> {
    let mut txn_file_path = environment::pool_path(pool_name);
    txn_file_path.push("stored");
    txn_file_path.set_extension("btxn");

    if !txn_file_path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(txn_file_path.parent().unwrap()).unwrap();
    }

    let txns = node_txns.iter().map(|txn| {
        let txn_json = serde_json::from_str::<serde_json::Value>(txn).map_err(|_| ErrorCode::CommonInvalidStructure)?;
        rmp_serde::to_vec_named(&txn_json).map_err(|_| ErrorCode::CommonInvalidStructure)
    }).fold(Ok(vec![]), |acc, next| {
        match (acc, next) {
            (Err(e), _) | (_, Err(e)) => Err(e),
            (Ok(mut acc), Ok(next)) => {
                acc.push(next);
                Ok(acc)
            }
        }
    })?;

    let mut f = fs::File::create(&txn_file_path).map_err(|_| ErrorCode::CommonIOError)?;
    txns.iter().for_each(|vec| {
        f.write_u64::<LittleEndian>(vec.len() as u64).unwrap();
        f.write_all(vec).unwrap();
    });

    Ok(())
}

pub fn create_and_open_pool_ledger(pool_name: &str) -> Result<i32, IndyError> {
    set_protocol_version(PROTOCOL_VERSION).unwrap();
    let txn_file_path = create_genesis_txn_file_for_test_pool(pool_name, None, None);
    let pool_config = pool_config_json(txn_file_path.as_path());
    create_pool_ledger_config(pool_name, Some(pool_config.as_str()))?;
    open_pool_ledger(pool_name, None)
}

pub fn refresh(pool_handle: i32) -> Result<(), IndyError> {
    pool::refresh_pool_ledger(pool_handle).wait()
}

pub fn close(pool_handle: i32) -> Result<(), IndyError> {
    pool::close_pool_ledger(pool_handle).wait()
}

pub fn delete(pool_name: &str) -> Result<(), IndyError> {
    pool::delete_pool_ledger(pool_name).wait()
}

pub fn set_protocol_version(protocol_version: usize) -> Result<(), IndyError> {
    pool::set_protocol_version(protocol_version).wait()
}

pub fn get_req_id() -> u64 {
    time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
}

pub fn check_response_type(response: &str, _type: ResponseType) {
    let response: Response = serde_json::from_str(&response).unwrap();
    assert_eq!(response.op, _type);
}
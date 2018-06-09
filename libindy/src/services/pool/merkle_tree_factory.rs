extern crate byteorder;
extern crate rmp_serde;

use errors::pool::PoolError;
use std::path::PathBuf;
use services::ledger::merkletree::merkletree::MerkleTree;
use std::fs;
use self::byteorder::{ByteOrder, LittleEndian, WriteBytesExt, ReadBytesExt};
use errors::common::CommonError;
use std::io;
use std::io::{Read, BufRead};
use utils::environment::EnvironmentUtils;
use serde_json;
use serde_json::Value as SJsonValue;

pub fn create(pool_name: &str) -> Result<MerkleTree, PoolError> {
    let mut p = EnvironmentUtils::pool_path(pool_name);

    let mut p_stored = p.clone();
    p_stored.push("stored");
    p_stored.set_extension("btxn");

    if !p_stored.exists() {
        p.push(pool_name);
        p.set_extension("txn");

        if !p.exists() {
            return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
        }

        _from_genesis(&p)
    } else {
        _from_cache(&p_stored)
    }
}

pub fn drop_cache(pool_name: &str) -> Result<(), PoolError> {
    warn!("Cache is invalid -- dropping it!");
    let mut p = EnvironmentUtils::pool_path(pool_name);

    p.push("stored");
    p.set_extension("btxn");
    if p.exists() {
        fs::remove_file(p).map_err(CommonError::IOError).map_err(PoolError::from)?;
        Ok(())
    } else {
        Err(PoolError::CommonError(CommonError::InvalidState("Can't recover to genesis -- no txns stored. Possible problems in genesis txns.".to_string())))
    }
}

fn _from_cache(file_name: &PathBuf) -> Result<MerkleTree, PoolError> {
    let mut mt = MerkleTree::from_vec(Vec::new()).map_err(map_err_trace!())?;

    let mut f = fs::File::open(file_name).map_err(map_err_trace!())?;

    while let Ok(bytes) = f.read_u64::<LittleEndian>().map_err(CommonError::IOError).map_err(PoolError::from) {
        let mut buf = vec![0; bytes as usize];
        f.read(buf.as_mut()).map_err(map_err_trace!())?;
        mt.append(buf.to_vec()).map_err(map_err_trace!())?;
    }
    Ok(mt)
}

fn _from_genesis(file_name: &PathBuf) -> Result<MerkleTree, PoolError> {
    let mut mt = MerkleTree::from_vec(Vec::new()).map_err(map_err_trace!())?;

    let f = fs::File::open(file_name).map_err(map_err_trace!())?;

    let reader = io::BufReader::new(&f);
    for line in reader.lines() {
        let line: String = line.map_err(map_err_trace!())?.trim().to_string();
        if line.is_empty() { continue };
        let genesis_txn: SJsonValue = serde_json::from_str(line.as_str())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
        let bytes = rmp_serde::encode::to_vec_named(&genesis_txn)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
        mt.append(bytes).map_err(map_err_trace!())?;
    }
    Ok(mt)
}
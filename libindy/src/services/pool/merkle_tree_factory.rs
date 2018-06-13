extern crate byteorder;
extern crate rmp_serde;

use errors::pool::PoolError;
use std::path::PathBuf;
use services::ledger::merkletree::merkletree::MerkleTree;
use std::fs;
use std::str::from_utf8;
use self::byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use errors::common::CommonError;
use std::io;
use std::io::{Read, BufRead, Write};
use utils::environment::EnvironmentUtils;
use serde_json;
use serde_json::Value as SJsonValue;
use std::collections::HashMap;
use services::pool::types::NodeTransactionV1;
use services::pool::types::NodeTransaction;

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

    trace!("start recover from cache");
    while let Ok(bytes) = f.read_u64::<LittleEndian>().map_err(CommonError::IOError).map_err(PoolError::from) {
        if bytes == 0 {
            continue;
        }
        trace!("bytes: {:?}", bytes);
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

pub fn dump_new_txns(pool_name: &str, txns: &Vec<Vec<u8>>) -> Result<(), PoolError>{
    let mut p = EnvironmentUtils::pool_path(pool_name);

    p.push("stored");
    p.set_extension("btxn");
    if !p.exists() {
        _dump_genesis_to_stored(&p, pool_name)?;
    }

    let mut file = fs::OpenOptions::new().append(true).open(p)
        .map_err(|e| CommonError::IOError(e))
        .map_err(map_err_err!())?;

    _dump_vec_to_file(txns, &mut file)
}

fn _dump_genesis_to_stored(p: &PathBuf, pool_name: &str) -> Result<(), PoolError> {
    let mut file = fs::File::create(p)
        .map_err(|e| CommonError::IOError(e))
        .map_err(map_err_err!())?;

    let mut p_genesis = EnvironmentUtils::pool_path(pool_name);
    p_genesis.push(pool_name);
    p_genesis.set_extension("txn");

    if !p_genesis.exists() {
        return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
    }

    let genesis_vec = _genesis_to_binary(&p_genesis)?;
    _dump_vec_to_file(&genesis_vec, &mut file)
}

fn _dump_vec_to_file(v: &Vec<Vec<u8>>, file : &mut fs::File) -> Result<(), PoolError> {
    v.into_iter().map(|vec| {
        file.write_u64::<LittleEndian>(vec.len() as u64).map_err(map_err_trace!())?;
        file.write_all(vec).map_err(map_err_trace!())
    }).fold(Ok(()), |acc, next| {
        match (acc, next) {
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(PoolError::CommonError(CommonError::IOError(e))),
            _ => Ok(()),
        }
    })
}

fn _genesis_to_binary(p: &PathBuf) -> Result<Vec<Vec<u8>>, PoolError> {
    let f = fs::File::open(p).map_err(map_err_trace!())?;
    let reader = io::BufReader::new(&f);
    reader
        .lines()
        .into_iter()
        .map(|res| {
            let line = res.map_err(map_err_trace!())?;
            _parse_txn_from_json(line.trim().as_bytes()).map_err(PoolError::from).map_err(map_err_err!())
        })
        .fold(Ok(Vec::new()), |acc, next| {
            match (acc, next) {
                (Err(e), _) | (_, Err(e)) => Err(e),
                (Ok(mut acc), Ok(res)) => {
                    let mut vec = vec![];
                    vec.append(&mut acc);
                    vec.push(res);
                    Ok(vec)
                }
            }
        })
}

fn _parse_txn_from_json(txn: &[u8]) -> Result<Vec<u8>, CommonError> {
    let txn_str = from_utf8(txn).map_err(|_| CommonError::InvalidStructure(format!("Can't parse valid UTF-8 string from this array: {:?}", txn)))?;

    if txn_str.trim().is_empty() {
        return Ok(vec![]);
    }

    let genesis_txn: SJsonValue = serde_json::from_str(txn_str.trim())
        .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))?;
    rmp_serde::encode::to_vec_named(&genesis_txn)
        .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize Genesis Transaction file: {:?}", err)))
}

pub fn build_node_state(merkle_tree: &MerkleTree) -> Result<HashMap<String, NodeTransactionV1>, CommonError> {
    let mut gen_tnxs: HashMap<String, NodeTransactionV1> = HashMap::new();

    for gen_txn in merkle_tree {
        let gen_txn: NodeTransaction =
            rmp_serde::decode::from_slice(gen_txn.as_slice())
                .map_err(|e|
                    CommonError::InvalidState(format!("MerkleTree contains invalid data {:?}", e)))?;

        let mut gen_txn: NodeTransactionV1 = NodeTransactionV1::from(gen_txn);

        if gen_tnxs.contains_key(&gen_txn.txn.data.dest) {
            gen_tnxs.get_mut(&gen_txn.txn.data.dest).unwrap().update(&mut gen_txn)?;
        } else {
            gen_tnxs.insert(gen_txn.txn.data.dest.clone(), gen_txn);
        }
    }
    Ok(gen_tnxs)
}

pub fn from_file(txn_file: &str) -> Result<MerkleTree, PoolError> {
    _from_genesis(&PathBuf::from(txn_file))
}
extern crate byteorder;
extern crate rmp_serde;

use domain::ledger::request::ProtocolVersion;
use errors::common::CommonError;
use errors::pool::PoolError;
use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde_json;
use serde_json::Value as SJsonValue;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::pool::types::{NodeTransaction, NodeTransactionV0, NodeTransactionV1};
use std::{fs, io};
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::str::from_utf8;
use utils::environment::EnvironmentUtils;

pub fn create(pool_name: &str) -> Result<MerkleTree, PoolError> {
    let mut p = EnvironmentUtils::pool_path(pool_name);

    let mut p_stored = p.clone();
    p_stored.push("stored");
    p_stored.set_extension("btxn");

    if !p_stored.exists() {
        trace!("Restoring merkle tree from genesis");
        p.push(pool_name);
        p.set_extension("txn");

        if !p.exists() {
            trace!("here");
            return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
        }

        _from_genesis(&p)
    } else {
        trace!("Restoring merkle tree from cache");
        _from_cache(&p_stored)
    }
}

pub fn drop_cache(pool_name: &str) -> Result<(), PoolError> {
    let mut p = EnvironmentUtils::pool_path(pool_name);

    p.push("stored");
    p.set_extension("btxn");
    if p.exists() {
        warn!("Cache is invalid -- dropping it!");
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

pub fn dump_new_txns(pool_name: &str, txns: &Vec<Vec<u8>>) -> Result<(), PoolError> {
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
        trace!("here");
        return Err(PoolError::NotCreated(format!("Pool is not created for name: {:?}", pool_name)));
    }

    let genesis_vec = _genesis_to_binary(&p_genesis)?;
    _dump_vec_to_file(&genesis_vec, &mut file)
}

fn _dump_vec_to_file(v: &Vec<Vec<u8>>, file: &mut fs::File) -> Result<(), PoolError> {
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

pub fn build_node_state(merkle_tree: &MerkleTree) -> Result<HashMap<String, NodeTransactionV1>, PoolError> {
    let mut gen_tnxs: HashMap<String, NodeTransactionV1> = HashMap::new();

    for gen_txn in merkle_tree {
        let gen_txn: NodeTransaction =
            rmp_serde::decode::from_slice(gen_txn.as_slice())
                .map_err(|e|
                    CommonError::InvalidState(format!("MerkleTree contains invalid data {:?}", e)))?;

        let protocol_version = ProtocolVersion::get();

        let mut gen_txn = match gen_txn {
            NodeTransaction::NodeTransactionV0(txn) => {
                if protocol_version != 1 {
                    return Err(PoolError::PoolIncompatibleProtocolVersion(
                        format!("Libindy PROTOCOL_VERSION is {} but Pool Genesis Transactions are of version {}.\
                             Call indy_set_protocol_version(1) to set correct PROTOCOL_VERSION", protocol_version, NodeTransactionV0::VERSION)));
                }
                NodeTransactionV1::from(txn)
            }
            NodeTransaction::NodeTransactionV1(txn) => {
                if protocol_version != 2 {
                    return Err(PoolError::PoolIncompatibleProtocolVersion(
                        format!("Libindy PROTOCOL_VERSION is {} but Pool Genesis Transactions are of version {}.\
                             Call indy_set_protocol_version(2) to set correct PROTOCOL_VERSION", protocol_version, NodeTransactionV1::VERSION)));
                }
                txn
            }
        };

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


#[cfg(test)]
mod tests {
    use byteorder::LittleEndian;
    use domain::ledger::request::ProtocolVersion;
    use std::fs;
    use super::*;
    use utils::test::TestUtils;

    fn _set_protocol_version(version: usize) {
        ProtocolVersion::set(version);
    }

    const TEST_PROTOCOL_VERSION: usize = 2;
    pub const NODE1_OLD: &'static str = r#"{"data":{"alias":"Node1","client_ip":"192.168.1.35","client_port":9702,"node_ip":"192.168.1.35","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#;
    pub const NODE2_OLD: &'static str = r#"{"data":{"alias":"Node2","client_ip":"192.168.1.35","client_port":9704,"node_ip":"192.168.1.35","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#;

    pub const NODE1: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"10.0.0.2","client_port":9702,"node_ip":"10.0.0.2","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"},"metadata":{"from":"Th7MpTaRZVRYnPiabds81Y"},"type":"0"},"txnMetadata":{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"},"ver":"1"}"#;
    pub const NODE2: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"10.0.0.2","client_port":9704,"node_ip":"10.0.0.2","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"},"metadata":{"from":"EbP4aYNeTHL6q385GuVpRV"},"type":"0"},"txnMetadata":{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"},"ver":"1"}"#;
    pub const NODE3: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"10.0.0.2","client_port":9706,"node_ip":"10.0.0.2","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"},"metadata":{"from":"4cU41vWW82ArfxJxHkzXPG"},"type":"0"},"txnMetadata":{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"},"ver":"1"}"#;
    pub const NODE4: &'static str = r#"{"reqSignature":{},"txn":{"data":{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"10.0.0.2","client_port":9708,"node_ip":"10.0.0.2","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"},"metadata":{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"},"type":"0"},"txnMetadata":{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"},"ver":"1"}"#;


    fn _write_genesis_txns(txns: &str) {
        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push(pool_name);
        path.set_extension("txn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        f.write(txns.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();
    }

    #[test]
    fn pool_worker_build_node_state_works_for_new_txns_format_and_1_protocol_version() {
        TestUtils::cleanup_storage();

        _set_protocol_version(1);

        let txns_src = format!("{}\n{}\n", NODE1, NODE2);

        _write_genesis_txns(&txns_src);

        let merkle_tree = super::create("test").unwrap();
        let res = super::build_node_state(&merkle_tree);
        assert_match!(Err(PoolError::PoolIncompatibleProtocolVersion(_)), res);
    }

    #[test]
    pub fn pool_worker_works_for_deserialize_cache() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let txn1_json: serde_json::Value = serde_json::from_str(NODE1).unwrap();
        let txn2_json: serde_json::Value = serde_json::from_str(NODE2).unwrap();
        let txn3_json: serde_json::Value = serde_json::from_str(NODE3).unwrap();
        let txn4_json: serde_json::Value = serde_json::from_str(NODE4).unwrap();

        let pool_cache = vec![rmp_serde::to_vec_named(&txn1_json).unwrap(),
                              rmp_serde::to_vec_named(&txn2_json).unwrap(),
                              rmp_serde::to_vec_named(&txn3_json).unwrap(),
                              rmp_serde::to_vec_named(&txn4_json).unwrap()];

        let pool_name = "test";
        let mut path = EnvironmentUtils::pool_path(pool_name);
        fs::create_dir_all(path.as_path()).unwrap();
        path.push("stored");
        path.set_extension("btxn");
        let mut f = fs::File::create(path.as_path()).unwrap();
        pool_cache.iter().for_each(|vec| {
            f.write_u64::<LittleEndian>(vec.len() as u64).unwrap();
            f.write_all(vec).unwrap();
        });

        let merkle_tree = super::create("test").unwrap();
        let _node_state = super::build_node_state(&merkle_tree).unwrap();
    }

    #[test]
    fn pool_worker_restore_merkle_tree_works_from_genesis_txns() {
        TestUtils::cleanup_storage();

        let txns_src = format!("{}\n{}", NODE1, NODE2);
        _write_genesis_txns(&txns_src);

        let merkle_tree = super::create("test").unwrap();

        assert_eq!(merkle_tree.count(), 2, "test restored MT size");
        assert_eq!(merkle_tree.root_hash_hex(), "3768ef5b25a01d19c0fda687f2354b29e004821bce8557e70085379f536907ed", "test restored MT root hash");
    }

    #[test]
    fn pool_worker_build_node_state_works_for_old_format() {
        TestUtils::cleanup_storage();

        _set_protocol_version(1);

        let node1: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransactionV0>(NODE1_OLD).unwrap());
        let node2: NodeTransactionV1 = NodeTransactionV1::from(serde_json::from_str::<NodeTransactionV0>(NODE2_OLD).unwrap());

        let txns_src = format!("{}\n{}\n", NODE1_OLD, NODE2_OLD);

        _write_genesis_txns(&txns_src);

        let merkle_tree = super::create("test").unwrap();
        let node_state = super::build_node_state(&merkle_tree).unwrap();

        assert_eq!(1, ProtocolVersion::get());

        assert_eq!(2, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], node1);
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], node2);
    }

    #[test]
    fn pool_worker_build_node_state_works_for_new_format() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let node1: NodeTransactionV1 = serde_json::from_str(NODE1).unwrap();
        let node2: NodeTransactionV1 = serde_json::from_str(NODE2).unwrap();

        let txns_src = format!("{}\n{}\n{}\n{}\n", NODE1, NODE2, NODE3, NODE4);

        _write_genesis_txns(&txns_src);

        let merkle_tree = super::create("test").unwrap();
        let node_state = super::build_node_state(&merkle_tree).unwrap();

        assert_eq!(2, ProtocolVersion::get());

        assert_eq!(4, node_state.len());
        assert!(node_state.contains_key("Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"));
        assert!(node_state.contains_key("8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"));

        assert_eq!(node_state["Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"], node1);
        assert_eq!(node_state["8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"], node2);
    }

    #[test]
    fn pool_worker_build_node_state_works_for_old_txns_format_and_2_protocol_version() {
        TestUtils::cleanup_storage();

        _set_protocol_version(TEST_PROTOCOL_VERSION);

        let txns_src = format!("{}\n{}\n", NODE1_OLD, NODE2_OLD);

        _write_genesis_txns(&txns_src);

        let merkle_tree = super::create("test").unwrap();
        let res = super::build_node_state(&merkle_tree);
        assert_match!(Err(PoolError::PoolIncompatibleProtocolVersion(_)), res);
    }
}
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;

pub fn usize_to_byte_array(n: usize) -> Vec<u8> {
    let mut wtr: Vec<u8> = Vec::new();
    wtr.write_u64::<LittleEndian>(n as u64).unwrap();
    wtr
}

pub fn byte_array_to_usize(v: Vec<u8>) -> usize {
    let mut rdr = Cursor::new(v);
    rdr.read_u64::<LittleEndian>().unwrap() as usize
}

pub mod tests {
    use super::*;
    use utils::environment::EnvironmentUtils;
    use services::microledger::constants::*;
    use std::collections::HashMap;
    use services::microledger::microledger::Microledger;
    use services::microledger::did_microledger::DidMicroledger;

    pub fn valid_storage_options() -> HashMap<String, String>{
        let mut options: HashMap<String, String> = HashMap::new();
        let mut path = EnvironmentUtils::tmp_path();
        path.push("did_ml_path");
        let storage_path = path.to_str().unwrap().to_owned();
        options.insert("storage_type".to_string(), "sqlite".to_string());
        options.insert("storage_path".to_string(), storage_path);
        options
    }

    pub fn get_new_microledger(did: &str) -> DidMicroledger{
        let options = valid_storage_options();
        DidMicroledger::new(did, options).unwrap()
    }

    pub fn get_4_txns() -> Vec<String> {
        let txn = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1"}}"#;
        let txn_2 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"dest":"75KUW8tPUQNBS4W7ibFeY8","type":"1","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_3 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        let txn_4 = r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#;
        vec![txn.to_string(), txn_2.to_string(), txn_3.to_string(), txn_4.to_string()]
    }

    pub fn get_10_txns() -> Vec<String> {
        let txns = vec![
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":[],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"authorizations":["all","add_key","rem_key"],"type":"2","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"https://agent1.example.com:9080","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":1,"txnVersion":1,"operation":{"address":"tcp://123.88.912.091:9876","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":2,"txnVersion":2,"operation":{"address":"https://agent1.example.com","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#,
            r#"{"protocolVersion":2,"txnVersion":1,"operation":{"address":"http://agent2.example.org","type":"3","verkey":"6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1"}}"#
        ];
        let mut txns: Vec<String> = txns.iter().map(|s|s.to_string()).collect();
        for txn in get_4_txns() {
            txns.push(txn)
        }
        txns
    }
}
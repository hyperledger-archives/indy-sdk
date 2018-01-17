use serde_json;
use serde_json::Value;
extern crate rand;

use settings;
use rand::Rng;
use std::fmt;
use std::sync::Mutex;
use std::string::ToString;
use std::collections::HashMap;
use utils::error;
use utils::constants::{ SCHEMA_REQ, CREATE_SCHEMA_RESULT };
use utils::libindy::pool::{ get_pool_handle };
use utils::wallet::{ get_wallet_handle };
use utils::libindy::ledger::{
    libindy_build_get_txn_request,
    libindy_build_schema_request,
    libindy_submit_request,
    libindy_sign_and_submit_request
};

lazy_static! {
    static ref SCHEMA_MAP: Mutex<HashMap<u32, Box<CreateSchema>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaTransaction {
    #[serde(rename = "seqNo")]
    sequence_num: Option<usize>,
    #[serde(rename = "identifier")]
    sponsor: Option<String>,
    #[serde(rename = "txnTime")]
    txn_timestamp: Option<usize>,
    #[serde(rename = "type")]
    txn_type: Option<String>,
    data: Option<SchemaData>

}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaData {
    name: String,
    version: String,
    attr_names: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerSchema {
    sequence_num: i32,
    pub data: Option<SchemaTransaction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchema {
    data: SchemaTransaction,
    handle: u32,
    name: String,
    source_id: String,
    sequence_num: u32,
}

pub trait Schema: ToString {
    type SchemaType;
    fn retrieve_schema(sequence_num: i32) -> Result<SchemaTransaction, u32>
    {
        let txn = Self::retrieve_from_ledger(sequence_num)?;
        match Self::process_ledger_txn(txn){
            Ok(data) => Ok(data),
            Err(code) => return Err(error::INVALID_SCHEMA_SEQ_NO.code_num)
        }
    }

    fn process_ledger_txn(txn: String) -> Result<SchemaTransaction, u32>
    {
        let result = Self::extract_result_from_txn(&txn)?;
        match result.get("data") {
            Some(d) => {
                let schema: SchemaTransaction = match serde_json::from_value(d.clone()) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        warn!("{}: {:?}","Parse from value error", e);
                        return Err(error::INVALID_JSON.code_num)
                    }
                };
                Ok(schema)
            },
            None => {
                warn!("{}","'data' not found in json");
                Err(error::INVALID_JSON.code_num)
            }
        }
    }

    fn extract_result_from_txn(txn:&str) -> Result<serde_json::Value, u32> {
        let txn_struct: Value = match serde_json::from_str(txn) {
            Ok(stc) => stc,
            Err(e) => {
                warn!("{}: {:?}","Parse from json error", e);
                return Err(error::INVALID_JSON.code_num)
            }
        };
        match txn_struct.get("result"){
            Some(result) => Ok(result.clone()),
            None => {
                warn!("{}","'result' not found in json");
                return Err(error::INVALID_JSON.code_num)
            }
        }
    }

    fn retrieve_from_ledger(sequence_num: i32) -> Result<String, u32>
    {
        let txn = Self::build_get_txn(sequence_num)?;
        let pool_handle = get_pool_handle()?;

        libindy_submit_request(pool_handle, txn)
    }

    fn build_get_txn(sequence_num: i32) -> Result<String, u32>
    {
        let submitter_did = "GGBDg1j8bsKmr4h5T9XqYf".to_string();

        libindy_build_get_txn_request(submitter_did, sequence_num)
    }
}

impl Schema for LedgerSchema {
    type SchemaType = LedgerSchema;
}

impl Schema for CreateSchema {
    type SchemaType = CreateSchema;
}

impl fmt::Display for LedgerSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let data = &self.data;
        if data.is_some() {
            match serde_json::to_string(data){
                Ok(s) => {
                    write!(f, "{}", s)
                },
                Err(e) => {
                    error!("{}: {:?}",error::INVALID_SCHEMA.message, e);
                    write!(f, "null")
                }

            }
        }
            else {
                write!(f, "null")
            }
    }
}

impl fmt::Display for CreateSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match serde_json::to_string(&self){
            Ok(s) => {
                write!(f, "{}", s)
            },
            Err(e) => {
                error!("{}: {:?}",error::INVALID_SCHEMA.message, e);
                write!(f, "null")
            }
        }
    }
}

impl LedgerSchema {
    pub fn new_from_ledger(sequence_num: i32) -> Result<LedgerSchema, u32>
    {
        Ok(LedgerSchema{
            sequence_num: sequence_num,
            data: Some(LedgerSchema::retrieve_schema(sequence_num)?)

        })
    }
}

impl CreateSchema {
    pub fn create_schema_req(submitter_did: &str, data: String) -> Result<String, u32> {
        if settings::test_indy_mode_enabled() { return Ok(SCHEMA_REQ.to_string()); }
        libindy_build_schema_request(submitter_did.to_string(), data)
            .or(Err(error::INVALID_SCHEMA_CREATION.code_num))
    }

    pub fn sign_and_send_request(submitter_did: &str, request: &str) ->  Result<String, u32> {
        if settings::test_indy_mode_enabled() { return Ok(CREATE_SCHEMA_RESULT.to_string()); }
        let pool_handle = get_pool_handle()?;
        let wallet_handle = get_wallet_handle();
        libindy_sign_and_submit_request(pool_handle,
                                        wallet_handle,
                                        submitter_did.to_string(),
                                        request.to_string())
            .or(Err(error::INVALID_SCHEMA_CREATION.code_num))
    }

    pub fn parse_schema_data(data: &str) -> Result<SchemaTransaction, u32> {
        let result = CreateSchema::extract_result_from_txn(data)?;
        match serde_json::from_str(&result.to_string()) {
            Ok(x) => Ok(x),
            Err(x) => Err(error::INVALID_SCHEMA_CREATION.code_num),
        }
    }

    pub fn set_sequence_num(&mut self, sequence_num: u32) {self.sequence_num = sequence_num;}

    pub fn get_sequence_num(&self) -> u32 {let sequence_num = self.sequence_num as u32; sequence_num}

}

pub fn create_new_schema(source_id: String,
                         schema_name: String,
                         issuer_did: String,
                         data: String) -> Result<u32, u32> {
    let req = CreateSchema::create_schema_req(&issuer_did, data)?;
    let sign_response = CreateSchema::sign_and_send_request(&issuer_did, &req)?;
    info!("created schema on ledger");

    let new_handle = rand::thread_rng().gen::<u32>();
    let mut new_schema = Box::new(CreateSchema {
        source_id,
        handle: new_handle,
        name: schema_name,
        data: CreateSchema::parse_schema_data(&sign_response)?,
        sequence_num: 0,
    });

    match new_schema.data.sequence_num {
        Some(x) => {
            new_schema.set_sequence_num(x as u32);
            info!("created schema object with sequence_num: {}", new_schema.sequence_num);
        },
        None => return Err(error::INVALID_SCHEMA_CREATION.code_num),
    };

    {
        let mut m = SCHEMA_MAP.lock().unwrap();
        info!("inserting handle {} into schema table", new_handle);
        m.insert(new_handle, new_schema);
    }

    Ok(new_handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn get_sequence_num(handle: u32) -> Result<u32, u32> {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(x) => Ok(x.get_sequence_num()),
        None => Err(error::INVALID_SCHEMA_HANDLE.code_num)
    }
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(p.to_string().to_owned()),
        None => Err(error::INVALID_SCHEMA_HANDLE.code_num)
    }
}

pub fn from_string(schema_data: &str) -> Result<u32, u32> {
    let derived_schema: CreateSchema = match serde_json::from_str(schema_data) {
        Ok(x) => x,
        Err(y) => return Err(error::INVALID_JSON.code_num),
    };
    let new_handle = derived_schema.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let schema = Box::from(derived_schema);

    {
        let mut m = SCHEMA_MAP.lock().unwrap();
        info!("inserting handle {} into schema table", new_handle);
        m.insert(new_handle, schema);
    }
    Ok(new_handle)
}

pub fn release(handle: u32) -> u32 {
    match SCHEMA_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_SCHEMA_HANDLE.code_num,
    }
}

#[cfg(test)]
mod tests {
    use settings;
    use super::*;
    use utils::libindy::pool;
    use utils::signus::SignusUtils;
    use utils::wallet::{ delete_wallet, init_wallet };
    use utils::constants::{ MY1_SEED };
    use std::path::Path;
    use std::str::FromStr;

    static  EXAMPLE: &str = r#"{
    "seqNo": 15,
    "dest": "4fUDR9R7fjwELRvH9JT6HH",
    "identifier":"4fUDR9R7fjwELRvH9JT6HH",
    "txnTime": 1510246647,
    "type": "107",
    "data": {
       "version": "0.1",
       "name": "Home Address",
       "attr_names": [
         "address1",
         "address2",
         "city",
         "state",
         "zip"
       ]
    }
}"#;

    static DIRTY_EXAMPLE: &str = r#"
{
  "auditPath":[
    "ERHXC95c5GkeGN1Cn8AsFL8ruU65Mmc5948ey4FybZMk",
    "8RPu6xcwmSaEgVohv83GtZu2hjJm5ghWQ6UEvSdjYCg4",
    "FUUbzChmnGjrGChBv3LZoKunodBPrVuMcg2vUrhkndmz"
  ],
  "data":{
    "attr_names":[
      "address1",
      "address2",
      "city",
      "state",
      "zip"
    ],
    "name":"Home Address",
    "version":"0.1"
  },
  "identifier":"4fUDR9R7fjwELRvH9JT6HH",
  "reqId":1510246647859168767,
  "rootHash":"Gnrip4cJgwJ3HE1fbrTBAPcuJ9RejAhX12PAUaF5HMij",
  "seqNo":15,
  "signature":"2paGvrWEfsCAYFAD47Qh7hedinymLy8VsbfatUrjWW7tpcryFtTsikJjWhKkD5QA3PLr7dLTmBFteNr4LWRHhrEn",
  "txnTime":1510246647,
  "type":"101"
}"#;
    static LEDGER_SAMPLE: &str = r#"
        {
          "result":{
            "data":{
              "rootHash":"Gnrip4cJgwJ3HE1fbrTBAPcuJ9RejAhX12PAUaF5HMij",
              "data":{
                "version":"0.1",
                "name":"Home Address",
                "attr_names":[
                  "address1",
                  "address2",
                  "city",
                  "state",
                  "zip"
                ]
              },
              "reqId":1510246647859168767,
              "seqNo":15,
              "txnTime":1510246647,
              "signature":"2paGvrWEfsCAYFAD47Qh7hedinymLy8VsbfatUrjWW7tpcryFtTsikJjWhKkD5QA3PLr7dLTmBFteNr4LWRHhrEn",
              "type":"101",
              "identifier":"4fUDR9R7fjwELRvH9JT6HH",
              "auditPath":[
                "ERHXC95c5GkeGN1Cn8AsFL8ruU65Mmc5948ey4FybZMk",
                "8RPu6xcwmSaEgVohv83GtZu2hjJm5ghWQ6UEvSdjYCg4",
                "FUUbzChmnGjrGChBv3LZoKunodBPrVuMcg2vUrhkndmz"
              ]
            },
            "type":"3",
            "identifier":"GGBDg1j8bsKmr4h5T9XqYf",
            "reqId":1513364428103873981,
            "seqNo":15
          },
          "op":"REPLY"
        }
        "#;

    static  EXAMPLE_OPTIONAL: &str = r#"{
}"#;

    #[test]
    fn test_schema_transaction(){
        let data: SchemaTransaction = serde_json::from_str(EXAMPLE).unwrap();

        assert_eq!(15, data.sequence_num.unwrap());
        assert_eq!("4fUDR9R7fjwELRvH9JT6HH", data.sponsor.unwrap().as_str());
        assert_eq!(1510246647, data.txn_timestamp.unwrap());
        assert_eq!("107", data.txn_type.unwrap().as_str());


        let data: SchemaTransaction = serde_json::from_str(DIRTY_EXAMPLE).unwrap();

        println!("{:?}", data);

        assert_eq!(15, data.sequence_num.unwrap());
        assert_eq!("4fUDR9R7fjwELRvH9JT6HH", data.sponsor.unwrap().as_str());
        assert_eq!(1510246647, data.txn_timestamp.unwrap());
        assert_eq!("101", data.txn_type.unwrap().as_str());

    }

    #[test]
    fn test_optional_schema_data(){
        let data: SchemaTransaction = serde_json::from_str(EXAMPLE_OPTIONAL).unwrap();

        assert!(data.sequence_num.is_none());
        assert!(data.sponsor.is_none());
    }

    #[test]
    fn test_txn_build(){
        let test = LedgerSchema::build_get_txn(15).unwrap();
        let txn: Value = serde_json::from_str(test.as_str()).unwrap();
        assert_eq!(15, txn.get("operation").unwrap().get("data").unwrap().as_i64().unwrap());
    }

    #[test]
    fn test_process_ledger_txn(){
        ::utils::logger::LoggerUtils::init();
        let test = LedgerSchema::process_ledger_txn(String::from_str(LEDGER_SAMPLE).unwrap()).unwrap();
    }

    #[test]
    fn test_schema_request(){
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        let test = CreateSchema::create_schema_req("4fUDR9R7fjwELRvH9JT6HH", data).unwrap();
        assert!(test.contains("{\"type\":\"101\",\"data\":{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}"));
    }

    #[test]
    fn test_extract_result_from_txn(){
        let test = CreateSchema::extract_result_from_txn(CREATE_SCHEMA_RESULT).unwrap();
        assert_eq!(test.get("type").unwrap(), "101");
        assert_eq!(test.get("reqId").unwrap().to_string(), "1515795761424583710".to_string());
    }

    #[test]
    fn test_ledger_schema_to_string(){
        ::utils::logger::LoggerUtils::init();
        let test = LedgerSchema::process_ledger_txn(String::from_str(LEDGER_SAMPLE).unwrap()).unwrap();

        let schema = LedgerSchema {sequence_num:15, data:Some(test)};

        println!("{}", schema.to_string())
    }

    #[test]
    fn test_parse_schema_data() {
        let schema_txn = CreateSchema::parse_schema_data(CREATE_SCHEMA_RESULT).unwrap();
        assert_eq!(schema_txn.sequence_num, Some(299));
        assert_eq!(schema_txn.txn_type, Some("101".to_string()));
        assert_eq!(schema_txn.sponsor, Some("VsKV7grR1BUE29mG2Fm2kX".to_string()));
    }

    #[test]
    fn test_create_schema_to_string(){
        ::utils::logger::LoggerUtils::init();
        let create_schema = CreateSchema {
            data: serde_json::from_str(DIRTY_EXAMPLE).unwrap(),
            source_id: "testId".to_string(),
            handle: 1,
            name: "schema_name".to_string(),
            sequence_num: 306,
        };
        let create_schema_str = r#"{"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"handle":1,"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        assert_eq!(create_schema.to_string(), create_schema_str.to_string());
    }

    #[test]
    fn test_create_schema_success(){
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        ::utils::logger::LoggerUtils::init();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert!(create_new_schema("1".to_string(), "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data).is_ok());
    }

    #[test]
    fn test_create_schema_fails(){
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(create_new_schema("1".to_string(), "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), "".to_string()),
        Err(error::INVALID_SCHEMA_CREATION.code_num));
    }

    #[test]
    fn test_from_ledger_without_pool(){
        let test = LedgerSchema::new_from_ledger(15);
        assert!(test.is_err());
        assert_eq!(error::NO_POOL_OPEN.code_num, test.unwrap_err())
    }

    #[ignore]
    #[test]
    fn test_sign_and_submit_create_schema(){
        settings::set_defaults();
        open_sandbox_pool();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        let wallet_handle = init_wallet("wallet1").unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
        let req = CreateSchema::create_schema_req(&my_did, data).unwrap();
        let sign_response = CreateSchema::sign_and_send_request(&my_did, &req).unwrap();
        assert!(sign_response.contains("\"data\":{\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"],\"name\":\"name\"}"));
        delete_wallet("wallet1").unwrap();
    }

    #[ignore]
    #[test]
    fn test_create_schema(){
        settings::set_defaults();
        open_sandbox_pool();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        let wallet_handle = init_wallet("wallet1").unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
        let rc = create_new_schema("id".to_string(), "name".to_string(), my_did, data).unwrap();
        delete_wallet("wallet1").unwrap();
        assert_eq!(rc, 0);
}

    #[ignore]
    #[test]
    fn from_ledger(){
        open_sandbox_pool();
        let test: LedgerSchema = LedgerSchema::new_from_ledger(15).unwrap();
        print!("{}", test.to_string());
    }

    #[ignore]
    #[test]
    fn create_schema(){
        open_sandbox_pool();
        let test: LedgerSchema = LedgerSchema::new_from_ledger(15).unwrap();
        print!("{}", test.to_string());
    }

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
            r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
            r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
            r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
        let pool_name = "pool1";
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 4;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool1.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        assert_eq!(pool::pool_config_json(Path::new(txn_file_path)),config_string);
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));

    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = "pool1".to_string();
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }
}

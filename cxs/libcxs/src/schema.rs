extern crate libc;

use serde_json;
use serde_json::Value;

use self::libc::c_char;
use std::ffi::CString;
use settings;
use utils::error;
use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use std::sync::mpsc::channel;
use utils::pool::get_pool_handle;


extern {

    fn indy_build_get_txn_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  data: i32,
                                  cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         request_json: *const c_char)>) -> i32;

    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                request_result_json: *const c_char)>) -> i32;
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaTransaction {
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

struct LedgerSchema {
    sequence_num: i32,
    data: Option<SchemaTransaction>
}

trait Schema {

}

impl LedgerSchema {
    fn new_from_ledger(sequence_num: i32) -> Result<LedgerSchema, u32>
    {
        let txn = LedgerSchema::retrieve_from_ledger(sequence_num)?;
        let data: SchemaTransaction = match LedgerSchema::process_ledger_txn(txn){
            Ok(data) => data,
            Err(code) => return Err(code)
        };
        Ok(LedgerSchema{
            sequence_num: sequence_num,
            data: Some(data)

        })
    }

    fn process_ledger_txn(txn: String) -> Result<SchemaTransaction, u32>
    {
        let txn_struct: Value = match serde_json::from_str(txn.as_str()) {
            Ok(stc) => stc,
            Err(e) => return Err(1)
        };
        let result = match txn_struct.get("result"){
            Some(result) => result,
            None => return Err(2)
        };
        match result.get("data") {
            Some(d) => {
                print!("{}",serde_json::to_string(d).unwrap());
                match serde_json::from_value(d.clone()) {
                    Ok(parsed) => parsed,
                    Err(e) => Err(4)
            }},
            None => Err(5)
        }
    }

    fn retrieve_from_ledger(sequence_num: i32) -> Result<String, u32>
    {
        let txn = LedgerSchema::build_get_txn(sequence_num)?;
        let pool_handle = get_pool_handle();

        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid | {
            sender.send((err, valid)).unwrap();
        });
        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        unsafe {
            let indy_err = indy_submit_request(command_handle,
                                               pool_handle as i32,
                                               CString::new(txn.as_str()).unwrap().as_ptr(),
                                               cb);
            if indy_err != 0 {
                return Err(error::UNKNOWN_ERROR.code_num)
            }
        }

        let (err, txn) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != 0 {
            return Err(error::UNKNOWN_ERROR.code_num)
        }

        Ok(txn)
    }

    fn build_get_txn(sequence_num: i32) -> Result<String, u32>
    {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, valid | {
            sender.send((err, valid)).unwrap();
        });
        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        unsafe {
            let indy_err = indy_build_get_txn_request(command_handle,
                                               CString::new("GGBDg1j8bsKmr4h5T9XqYf").unwrap().as_ptr(),
                                               sequence_num,
                                               cb);
            if indy_err != 0 {
                return Err(error::UNKNOWN_ERROR.code_num)
            }
        }

        let (err, txn) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
        if err != 0 {
            return Err(error::UNKNOWN_ERROR.code_num)
        }

        Ok(txn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::pool;
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

static  EXAMPLE_OPTIONAL: &str = r#"{
}"#;

    #[ignore]
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
        assert_eq!("107", data.txn_type.unwrap().as_str());


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

    #[ignore]
    #[test]
    fn test_process_ledger_txn(){
        let sample = r#"
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
        let test = LedgerSchema::process_ledger_txn(String::from_str(sample).unwrap()).unwrap();

        println!("{}", serde_json::to_string(&test).unwrap());
        eprintln!("****************************");
        eprintln!("{}", serde_json::to_string(&test).unwrap());
    }

    #[test]
    fn retrieve_from_ledger(){
        open_sandbox_pool();
        let test = LedgerSchema::retrieve_from_ledger(15).unwrap();
        print!("{}", test);
    }

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
            r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
            r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
            r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
        let pool_name = settings::CONFIG_POOL_NAME;
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 4;
        let pool_name = settings::CONFIG_POOL_NAME;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool_name.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        assert_eq!(pool::pool_config_json(Path::new(txn_file_path)),config_string);
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));

    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = settings::CONFIG_POOL_NAME;
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }
}
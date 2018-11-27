use serde_json;
use serde_json::Value;
extern crate rand;

use settings;
use std::fmt;
use std::string::ToString;
use utils::error;
use utils::libindy::ledger;
use utils::libindy::payments::PaymentTxn;
use error::schema::SchemaError;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use object_cache::ObjectCache;

lazy_static! {
    static ref SCHEMA_MAP: ObjectCache<CreateSchema> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SchemaData {
    name: String,
    version: String,
    #[serde(rename = "attrNames")]
    attr_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateSchema {
    data: Vec<String>,
    version: String,
    schema_id: String,
    name: String,
    source_id: String,
    sequence_num: u32,
    payment_txn: Option<PaymentTxn>,
}

impl Default for CreateSchema {
    fn default() -> CreateSchema {
        CreateSchema {
            data: Vec::new(),
            version: String::new(),
            schema_id: String::new(),
            name: String::new(),
            source_id: String::new(),
            sequence_num: 0,
            payment_txn: None,
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

impl CreateSchema {

    pub fn set_sequence_num(&mut self, sequence_num: u32) {self.sequence_num = sequence_num;}

    pub fn get_sequence_num(&self) -> u32 {let sequence_num = self.sequence_num as u32; sequence_num}

    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_schema_id(&self) -> &String { &self.schema_id }

    fn get_payment_txn(&self) -> Result<PaymentTxn, u32> {
        trace!("CreateSchema::get_payment_txn >>>");
        Ok(self.payment_txn.clone().ok_or(error::NOT_READY.code_num)?)
    }

    fn to_string_with_version(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }

    fn from_str(data: &str) -> Result<CreateSchema, SchemaError> {
        let data:Value = serde_json::from_str(&data)
            .or(Err(SchemaError::InvalidSchemaCreation()))?;
        let schema: CreateSchema = serde_json::from_value(data["data"].clone())
            .or(Err(SchemaError::InvalidSchemaCreation()))?;
        Ok(schema)
    }
}

pub fn create_new_schema(source_id: &str,
                         issuer_did: String,
                         name: String,
                         version: String,
                         data: String) -> Result<u32, SchemaError> {
    trace!("create_new_schema >>> source_id: {}, issuer_did: {}, name: {}, version: {}, data: {}",
           source_id, issuer_did, name, version, data);

    debug!("creating schema with source_id: {}, name: {}, issuer_did: {}", source_id, name, issuer_did);

    let (schema_id, payment_txn) = ledger::create_schema(&name, &version, &data)
        .map_err(|e| {
            if e == error::UNKNOWN_SCHEMA_REJECTION.code_num {SchemaError::UnknownRejection()}
            else if e == error::DUPLICATE_SCHEMA.code_num {SchemaError::DuplicateSchema()}
            else {SchemaError::CommonError(e)}
        })?;

    debug!("created schema on ledger with id: {}", schema_id);

    let new_schema = CreateSchema {
        source_id: source_id.to_string(),
        name,
        data: serde_json::from_str(&data).unwrap_or_default(),
        version,
        schema_id,
        //Todo: Take sequence number out. Id will be used instead
        sequence_num: 0,
        payment_txn,
    };

    let new_handle = SCHEMA_MAP.add(new_schema).map_err(|key|SchemaError::InvalidSchemaCreation())?;

    Ok(new_handle)
}


pub fn get_schema_attrs(source_id: String, schema_id: String) -> Result<(u32, String), SchemaError> {
    trace!("get_schema_attrs >>> source_id: {}, schema_id: {}", source_id, schema_id);

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)
        .map_err(|e| SchemaError::CommonError(e))?;

    let (schema_id, schema_json) = ledger::get_schema_json(&schema_id)
        .or(Err(SchemaError::InvalidSchemaSeqNo()))?;

    let schema_data: SchemaData = serde_json::from_str(&schema_json)
        .or(Err(SchemaError::CommonError(error::INVALID_JSON.code_num)))?;

    let new_schema = CreateSchema {
        source_id,
        schema_id,
        sequence_num: 0,
        name: schema_data.name,
        version: schema_data.version,
        data: schema_data.attr_names,
        payment_txn: None,
    };

    let new_handle = SCHEMA_MAP.add(new_schema).map_err(|key|SchemaError::InvalidSchemaCreation())?;

    Ok((new_handle, to_string(new_handle)?))
}

pub fn is_valid_handle(handle: u32) -> bool {
    SCHEMA_MAP.has_handle(handle)
}

pub fn get_sequence_num(handle: u32) -> Result<u32, SchemaError> {
    SCHEMA_MAP.get(handle,|s|{
        Ok(s.get_sequence_num())
    }).map_err(|ec|SchemaError::CommonError(ec))
}

pub fn to_string(handle: u32) -> Result<String, SchemaError> {
    SCHEMA_MAP.get(handle,|s|{
        Ok(s.to_string_with_version().to_owned())
    }).map_err(|ec|SchemaError::CommonError(ec))
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    SCHEMA_MAP.get(handle,|s|{
        Ok(s.get_source_id().clone())
    })
}

pub fn get_schema_id(handle: u32) -> Result<String, SchemaError> {
    SCHEMA_MAP.get(handle,|s|{
        Ok(s.get_schema_id().clone())
    }).map_err(|ec|SchemaError::CommonError(ec))
}

pub fn get_payment_txn(handle: u32) -> Result<PaymentTxn, SchemaError> {
    SCHEMA_MAP.get(handle,|s|{
        s.get_payment_txn()
    }).or(Err(SchemaError::NoPaymentInformation()))
}

pub fn from_string(schema_data: &str) -> Result<u32, SchemaError> {
    let derived_schema: CreateSchema = CreateSchema::from_str(schema_data)
        .map_err(|_| {
            error!("Invalid Json format for CreateSchema string");
            SchemaError::CommonError(error::INVALID_JSON.code_num)
        })?;

    let source_id = derived_schema.source_id.clone();
    let new_handle = SCHEMA_MAP.add(derived_schema).map_err(|ec|SchemaError::CommonError(ec))?;

    Ok(new_handle)
}

pub fn release(handle: u32) -> Result<(), SchemaError> {
    match SCHEMA_MAP.release(handle) {
        Ok(_) => Ok(()),
        Err(_) => Err(SchemaError::InvalidHandle()),
    }
}

pub fn release_all() {
    match SCHEMA_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}

#[cfg(test)]
pub mod tests {
    extern crate rand;

    use super::*;
    #[allow(unused_imports)]
    use rand::Rng;
    use utils::constants::{ SCHEMA_ID, SCHEMA_JSON };

    pub fn create_schema_real() -> u32 {
        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}",rand::thread_rng().gen::<u32>().to_string(),
                                                 rand::thread_rng().gen::<u32>().to_string());
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        create_new_schema("id", did, schema_name, schema_version, data).unwrap()
    }

    #[test]
    fn test_create_schema_to_string(){
        let source_id = "testId";
        let create_schema = CreateSchema {
            data: vec!["name".to_string(), "age".to_string(), "sex".to_string(), "height".to_string()],
            version: "1.0".to_string(),
            schema_id: SCHEMA_ID.to_string(),
            source_id: "testId".to_string(),
            name: "schema_name".to_string(),
            sequence_num: 306,
            payment_txn: None,
        };
        let create_schema_str = r#"{"data":["name","age","sex","height"],"version":"1.0","schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","name":"schema_name","source_id":"testId","sequence_num":306,"payment_txn":null}"#;
        assert_eq!(create_schema.to_string(), create_schema_str.to_string());
        let value: serde_json::Value = serde_json::from_str(&create_schema.to_string_with_version()).unwrap();
        assert_eq!(value["version"], "1.0");
        let create_schema:CreateSchema = serde_json::from_str(&value["data"].to_string()).unwrap();
        assert_eq!(create_schema.source_id, source_id);
        use utils::constants::SCHEMA_WITH_VERSION;
        let handle = from_string(SCHEMA_WITH_VERSION).unwrap();
        let schema_str = to_string(handle).unwrap();
        let value: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
        assert_eq!(value["version"], "1.0");
        let data = value["data"].clone();
        let schema:CreateSchema = serde_json::from_str(&data.to_string()).unwrap();
    }

    #[test]
    fn test_create_schema_success(){
        init!("true");
        let data = r#"["name","male"]"#;
        assert!(create_new_schema("1",
                                  "VsKV7grR1BUE29mG2Fm2kX".to_string(),
                                  "name".to_string(),
                                  "1.0".to_string(),
                                  data.to_string()).is_ok());
    }

    #[test]
    fn test_get_schema_attrs_success(){
        init!("true");
        let (handle, schema_attrs ) = get_schema_attrs("Check For Success".to_string(), SCHEMA_ID.to_string()).unwrap();
        assert!(schema_attrs.contains(r#""schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4""#));
        assert!(schema_attrs.contains(r#""data":["height","name","sex","age"]"#));
        assert!(handle > 0);
    }

    #[test]
    fn test_create_schema_fails(){
        init!("false");
        let schema = create_new_schema("1", "VsKV7grR1BUE29mG2Fm2kX".to_string(),
                                       "name".to_string(),
                                       "1.0".to_string(),
                                       "".to_string());
        assert_eq!(schema, Err(SchemaError::CommonError(error::INVALID_LIBINDY_PARAM.code_num)))
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_schema_attrs_from_ledger(){
        init!("ledger");

        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let (_, schema_attrs ) = get_schema_attrs("id".to_string(), schema_id.clone()).unwrap();
        assert!(schema_attrs.contains(&schema_id));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_schema_with_pool(){
        init!("ledger");
        let handle = create_schema_real();
        let payment = serde_json::to_string(&get_payment_txn(handle).unwrap()).unwrap();
        assert!(payment.len() > 50);

        assert!(handle > 0);
        let schema_id = get_schema_id(handle).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_schema_no_fees_with_pool(){
        init!("ledger");
        ::utils::libindy::payments::mint_tokens_and_set_fees(Some(0),Some(0),Some(r#"{"101":0, "102":0}"#.to_string()), None).unwrap();

        let handle = create_schema_real();
        assert!(handle > 0);
        let schema_id = get_schema_id(handle).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_duplicate_fails_no_fees(){
        use settings;
        init!("ledger");
        ::utils::libindy::payments::mint_tokens_and_set_fees(Some(0),Some(0),Some(r#"{"101":0, "102":0}"#.to_string()), None).unwrap();

        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}",rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());
        let rc = create_new_schema("id", did.clone(), schema_name.clone(), schema_version.clone(), data.clone());
        assert!(rc.is_ok());
        let rc = create_new_schema("id", did.clone(), schema_name.clone(), schema_version.clone(), data.clone());

        assert_eq!(rc, Err(SchemaError::DuplicateSchema()));
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let data = r#"["address1","address2","zip","city","state"]"#;
        let version = r#"0.0.0"#;
        let did = r#"2hoqvcwupRTUNkXn6ArYzs"#;
        let h1 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(),data.to_string()).unwrap();
        let h2 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(),data.to_string()).unwrap();
        let h3 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(),data.to_string()).unwrap();
        let h4 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(),data.to_string()).unwrap();
        let h5 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(),data.to_string()).unwrap();
        release_all();
        assert_eq!(release(h1).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h2).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h3).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h4).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h5).err(),Some(SchemaError::InvalidHandle()));
    }

    #[test]
    fn test_errors(){
        init!("false");
        assert_eq!(get_sequence_num(145661).err(), Some(SchemaError::CommonError(error::INVALID_OBJ_HANDLE.code_num)));
        assert_eq!(to_string(13435178).err(), Some(SchemaError::CommonError(error::INVALID_OBJ_HANDLE.code_num)));
    }

    #[test]
    fn test_extract_data_from_schema_json() {
        let data: SchemaData = serde_json::from_str(SCHEMA_JSON).unwrap();
        assert_eq!(data.name, "test-licence".to_string());
    }
}

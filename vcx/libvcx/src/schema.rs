use serde_json;

use std::string::ToString;

use settings;
use utils::libindy::anoncreds;
use utils::libindy::payments::PaymentTxn;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use object_cache::ObjectCache;
use messages::ObjectWithVersion;
use error::prelude::*;

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateSchema {
    data: Vec<String>,
    version: String,
    schema_id: String,
    name: String,
    source_id: String,
    payment_txn: Option<PaymentTxn>,
}

impl CreateSchema {
    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_schema_id(&self) -> &String { &self.schema_id }

    fn get_payment_txn(&self) -> VcxResult<PaymentTxn> {
        trace!("CreateSchema::get_payment_txn >>>");
        self.payment_txn.clone()
            .ok_or(VcxError::from(VcxErrorKind::NoPaymentInformation))
    }

    fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize Schema"))
    }

    fn from_str(data: &str) -> VcxResult<CreateSchema> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<CreateSchema>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Schema"))
    }
}

pub fn create_new_schema(source_id: &str,
                         issuer_did: String,
                         name: String,
                         version: String,
                         data: String) -> VcxResult<u32> {
    trace!("create_new_schema >>> source_id: {}, issuer_did: {}, name: {}, version: {}, data: {}", source_id, issuer_did, name, version, data);
    debug!("creating schema with source_id: {}, name: {}, issuer_did: {}", source_id, name, issuer_did);

    let (schema_id, payment_txn) = anoncreds::create_schema(&name, &version, &data)?;

    debug!("created schema on ledger with id: {}", schema_id);

    let schema = CreateSchema {
        source_id: source_id.to_string(),
        name,
        data: serde_json::from_str(&data).unwrap_or_default(),
        version,
        schema_id,
        payment_txn,
    };

    SCHEMA_MAP.add(schema)
        .or(Err(VcxError::from(VcxErrorKind::CreateSchema)))
}


pub fn get_schema_attrs(source_id: String, schema_id: String) -> VcxResult<(u32, String)> {
    trace!("get_schema_attrs >>> source_id: {}, schema_id: {}", source_id, schema_id);

    let submitter_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID)?;

    let (schema_id, schema_data_json) = anoncreds::get_schema_json(&schema_id)
        .map_err(|err| err.map(VcxErrorKind::InvalidSchemaSeqNo, "Schema not found"))?;

    let schema_data: SchemaData = serde_json::from_str(&schema_data_json)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize schema: {}", err)))?;

    let schema = CreateSchema {
        source_id,
        schema_id,
        name: schema_data.name,
        version: schema_data.version,
        data: schema_data.attr_names,
        payment_txn: None,
    };

    let schema_json = schema.to_string()?;

    let handle = SCHEMA_MAP.add(schema)
        .or(Err(VcxError::from(VcxErrorKind::CreateSchema)))?;

    Ok((handle, schema_json))
}

pub fn is_valid_handle(handle: u32) -> bool {
    SCHEMA_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    SCHEMA_MAP.get(handle, |s| {
        s.to_string()
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    SCHEMA_MAP.get(handle, |s| {
        Ok(s.get_source_id().to_string())
    })
}

pub fn get_schema_id(handle: u32) -> VcxResult<String> {
    SCHEMA_MAP.get(handle, |s| {
        Ok(s.get_schema_id().to_string())
    })
}

pub fn get_payment_txn(handle: u32) -> VcxResult<PaymentTxn> {
    SCHEMA_MAP.get(handle, |s| {
        s.get_payment_txn()
    })
}

pub fn from_string(schema_data: &str) -> VcxResult<u32> {
    let schema: CreateSchema = CreateSchema::from_str(schema_data)?;
    SCHEMA_MAP.add(schema)
}

pub fn release(handle: u32) -> VcxResult<()> {
    SCHEMA_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidSchemaHandle)))
}

pub fn release_all() {
    SCHEMA_MAP.drain().ok();
}

#[cfg(test)]
pub mod tests {
    extern crate rand;

    use super::*;
    #[allow(unused_imports)]
    use rand::Rng;
    use utils::constants::{SCHEMA_ID, SCHEMA_JSON};

    pub fn create_schema_real() -> u32 {
        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}", rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        create_new_schema("id", did, schema_name, schema_version, data).unwrap()
    }

    #[test]
    fn test_create_schema_to_string() {
        let source_id = "testId";
        let create_schema = CreateSchema {
            data: vec!["name".to_string(), "age".to_string(), "sex".to_string(), "height".to_string()],
            version: "1.0".to_string(),
            schema_id: SCHEMA_ID.to_string(),
            source_id: "testId".to_string(),
            name: "schema_name".to_string(),
            payment_txn: None,
        };
        let value: serde_json::Value = serde_json::from_str(&create_schema.to_string().unwrap()).unwrap();
        assert_eq!(value["version"], "1.0");
        let create_schema: CreateSchema = serde_json::from_str(&value["data"].to_string()).unwrap();
        assert_eq!(create_schema.source_id, source_id);
        use utils::constants::SCHEMA_WITH_VERSION;
        let handle = from_string(SCHEMA_WITH_VERSION).unwrap();
        let schema_str = to_string(handle).unwrap();
        let value: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
        assert_eq!(value["version"], "1.0");
        let data = value["data"].clone();
        let schema: CreateSchema = serde_json::from_str(&data.to_string()).unwrap();
    }

    #[test]
    fn test_create_schema_success() {
        init!("true");
        let data = r#"["name","male"]"#;
        assert!(create_new_schema("1",
                                  "VsKV7grR1BUE29mG2Fm2kX".to_string(),
                                  "name".to_string(),
                                  "1.0".to_string(),
                                  data.to_string()).is_ok());
    }

    #[test]
    fn test_get_schema_attrs_success() {
        init!("true");
        let (handle, schema_attrs) = get_schema_attrs("Check For Success".to_string(), SCHEMA_ID.to_string()).unwrap();
        assert!(schema_attrs.contains(r#""schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4""#));
        assert!(schema_attrs.contains(r#""data":["height","name","sex","age"]"#));
        assert!(handle > 0);
    }

    #[test]
    fn test_create_schema_fails() {
        init!("false");
        let schema = create_new_schema("1", "VsKV7grR1BUE29mG2Fm2kX".to_string(),
                                       "name".to_string(),
                                       "1.0".to_string(),
                                       "".to_string());
        assert_eq!(schema.unwrap_err().kind(), VcxErrorKind::InvalidLibindyParam)
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_schema_attrs_from_ledger() {
        init!("ledger");

        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let (_, schema_attrs) = get_schema_attrs("id".to_string(), schema_id.clone()).unwrap();
        assert!(schema_attrs.contains(&schema_id));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_schema_with_pool() {
        init!("ledger");
        let handle = create_schema_real();
        let payment = serde_json::to_string(&get_payment_txn(handle).unwrap()).unwrap();
        assert!(payment.len() > 50);

        assert!(handle > 0);
        let schema_id = get_schema_id(handle).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_schema_no_fees_with_pool() {
        init!("ledger");
        ::utils::libindy::payments::mint_tokens_and_set_fees(Some(0), Some(0), Some(r#"{"101":0, "102":0}"#.to_string()), None).unwrap();

        let handle = create_schema_real();
        assert!(handle > 0);
        let schema_id = get_schema_id(handle).unwrap();
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_duplicate_fails_no_fees() {
        use settings;
        init!("ledger");
        ::utils::libindy::payments::mint_tokens_and_set_fees(Some(0), Some(0), Some(r#"{"101":0, "102":0}"#.to_string()), None).unwrap();

        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}", rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());
        let rc = create_new_schema("id", did.clone(), schema_name.clone(), schema_version.clone(), data.clone());
        assert!(rc.is_ok());
        let rc = create_new_schema("id", did.clone(), schema_name.clone(), schema_version.clone(), data.clone());

        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::DuplicationSchema)
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let data = r#"["address1","address2","zip","city","state"]"#;
        let version = r#"0.0.0"#;
        let did = r#"2hoqvcwupRTUNkXn6ArYzs"#;
        let h1 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(), data.to_string()).unwrap();
        let h2 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(), data.to_string()).unwrap();
        let h3 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(), data.to_string()).unwrap();
        let h4 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(), data.to_string()).unwrap();
        let h5 = create_new_schema("1", did.to_string(), "name".to_string(), version.to_string(), data.to_string()).unwrap();
        release_all();
        assert_eq!(release(h1).unwrap_err().kind(), VcxErrorKind::InvalidSchemaHandle);
        assert_eq!(release(h2).unwrap_err().kind(), VcxErrorKind::InvalidSchemaHandle);
        assert_eq!(release(h3).unwrap_err().kind(), VcxErrorKind::InvalidSchemaHandle);
        assert_eq!(release(h4).unwrap_err().kind(), VcxErrorKind::InvalidSchemaHandle);
        assert_eq!(release(h5).unwrap_err().kind(), VcxErrorKind::InvalidSchemaHandle);
    }

    #[test]
    fn test_errors() {
        init!("false");
        assert_eq!(to_string(13435178).unwrap_err().kind(), VcxErrorKind::InvalidHandle);
    }

    #[test]
    fn test_extract_data_from_schema_json() {
        let data: SchemaData = serde_json::from_str(SCHEMA_JSON).unwrap();
        assert_eq!(data.name, "test-licence".to_string());
    }
}

extern crate rust_base58;
extern crate serde_json;

use std::collections::HashMap;
use utils::error;
use messages::validation;

static PROOF_REQUEST: &str = "PROOF_REQUEST";
static PROOF_DATA: &str = "proof_request_data";
static REQUESTED_ATTRS: &str = "requested_attrs";
static REQUESTED_PREDICATES: &str = "requested_predicates";

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ProofType {
    name: String,
    #[serde(rename = "version")]
    type_version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ProofTopic {
    mid: u32,
    tid: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Attr {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_seq_no: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_did: Option<String>,
}

//Todo: Move Predicate to a common place for both proof_req and proof msg
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Predicate {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_seq_no: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_did: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofAttrs {
    attrs: Vec<Attr>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofPredicates {
    predicates: Vec<Predicate>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofRequestData{
    nonce: String,
    name: String,
    #[serde(rename = "version")]
    data_version: String,
    pub requested_attrs: HashMap<String, Attr>,
    pub requested_predicates: HashMap<String, Predicate>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofRequestMessage{
    #[serde(rename = "@type")]
    type_header: ProofType,
    #[serde(rename = "@topic")]
    topic: ProofTopic,
    pub proof_request_data: ProofRequestData,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    pub msg_ref_id: Option<String>,
}

impl ProofAttrs {
    pub fn create() -> ProofAttrs {
        ProofAttrs {
            attrs: Vec::new()
        }
    }
}

impl ProofPredicates {
    pub fn create() -> ProofPredicates {
        ProofPredicates {
            predicates: Vec::new()
        }
    }
}

impl ProofRequestMessage {
    pub fn create() -> ProofRequestMessage {
        ProofRequestMessage {
            type_header: ProofType {
                name: String::from(PROOF_REQUEST),
                type_version: String::new(),
            },
            topic: ProofTopic {
                tid: 0,
                mid: 0,
            },
            proof_request_data: ProofRequestData {
                nonce: String::new(),
                name: String::new(),
                data_version: String::new(),
                requested_attrs:HashMap::new(),
                requested_predicates: HashMap::new(),
            },
            validate_rc: 0,
            msg_ref_id: None,
        }
    }

    pub fn type_version(&mut self, version: &str) -> &mut Self {
        self.type_header.type_version = String::from(version);
        self
    }

    pub fn tid(&mut self, tid: u32) -> &mut Self {
        self.topic.tid = tid;
        self
    }

    pub fn mid(&mut self, mid: u32) -> &mut Self {
        self.topic.mid = mid;
        self
    }

    pub fn nonce(&mut self, nonce: &str) -> &mut Self {
        match validation::validate_nonce(nonce) {
            Ok(x) => {
                self.proof_request_data.nonce = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn proof_name(&mut self, name: &str) -> &mut Self {
        self.proof_request_data.name = String::from(name);
        self
    }

    pub fn proof_data_version(&mut self, version: &str) -> &mut Self {
        self.proof_request_data.data_version = String::from(version);
        self
    }


    pub fn requested_attrs(&mut self, attrs: &str) -> &mut Self {
        let mut proof_attrs = ProofAttrs::create();
        proof_attrs.attrs = match serde_json::from_str(attrs) {
            Ok(x) => x,
            Err(x) => {
                self.validate_rc = error::INVALID_JSON.code_num;
                return self;
            }
        };
        for (i, attr) in proof_attrs.attrs.iter().enumerate() {
            self.proof_request_data.requested_attrs.insert(format!("{}_{}", attr.name, i), attr.clone());
        }
        self
    }

    pub fn requested_predicates(&mut self, predicates: &str) -> &mut Self {
        let mut proof_predicates = ProofPredicates::create();
        proof_predicates.predicates = match serde_json::from_str(predicates) {
            Ok(x) => x,
            Err(x) => {
                warn!("Invalid predicate JSON");
                self.validate_rc = error::INVALID_JSON.code_num;
                return self;
            }
        };
        for (i, attr) in proof_predicates.predicates.iter().enumerate() {
            self.proof_request_data.requested_predicates.insert(
                format!("{}_{}", attr.attr_name, i), attr.clone());
        }
        self
    }

    pub fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        match serde_json::to_string(self) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_JSON.code_num)
        }
    }

    pub fn get_proof_request_data(&mut self) -> String {
        json!(self)[PROOF_DATA].to_string()
    }

    pub fn to_string(&self) -> Result<String, u32> {
        match serde_json::to_string(&self){
            Ok(s) => Ok(s),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use messages::{proof_request};

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}]";

    #[test]
    fn test_create_proof_request_data() {
        let request = proof_request();
        let proof_data = ProofRequestData {
            nonce: String::new(),
            name: String::new(),
            data_version: String::new(),
            requested_attrs: HashMap::new(),
            requested_predicates: HashMap::new(),
        };
        assert_eq!(request.proof_request_data, proof_data);
    }

    #[test]
    fn test_proof_request_msg() {
        //proof data
        let data_name = "Test";
        let nonce = "123432421212";
        let data_version = "3.75";
        let attrs = "";
        let version = "1.3";
        let tid = 89;
        let mid = 98;

        let mut request = proof_request()
            .type_version(version)
            .tid(tid)
            .mid(mid)
            .nonce(nonce)
            .proof_name(data_name)
            .proof_data_version(data_version)
            .requested_attrs(REQUESTED_ATTRS)
            .requested_predicates(REQUESTED_PREDICATES)
            .clone();

        let proof_request_test: serde_json::Value = json!({
            "@type": { "name": "PROOF_REQUEST", "version": "1.3" },
            "@topic": { "tid": 89, "mid": 98 },
            "proof_request_data": {
                "nonce": "123432421212",
                "name": "Test",
                "version": "3.75",
                "requested_attrs": {
                    "person name_0": { "name": "person name" },
                    "address_1_1": { "schema_seq_no": 1, "name": "address_1" },
                    "address_2_2": {
                        "schema_seq_no": 2,
                        "issuer_did": "8XFh8yBzrpJQmNyZzgoTqB",
                        "name": "address_2",
                    },
                    "city_3": {
                        "schema_seq_no": 1,
                        "name": "city",
                    },
                    "state_4": {
                        "schema_seq_no": 1,
                        "name": "state",
                    },
                    "zip_5": {
                        "schema_seq_no": 1,
                        "name": "zip",
                    },
                },
                "requested_predicates": {
                        "age_0": {
                            "schema_seq_no": 2,
                            "issuer_did": "8XFh8yBzrpJQmNyZzgoTqB",
                            "attr_name": "age",
                            "p_type": "GE",
                            "value": 18
                        }
                },
            },
        });
        let serialized_msg = request.serialize_message().unwrap();
        assert!(serialized_msg.contains(r#""@type":{"name":"PROOF_REQUEST","version":"1.3"}"#));
        assert!(serialized_msg.contains(r#"@topic":{"mid":98,"tid":89}"#));
        assert!(serialized_msg.contains(r#"proof_request_data":{"nonce":"123432421212","name":"Test","version":"3.75","requested_attrs""#));
        assert!(serialized_msg.contains(r#""zip_5":{"name":"zip","schema_seq_no":1}"#));
        assert!(serialized_msg.contains(r#""age_0":{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":1,"issuer_did":"DID1"}"#));
    }
}
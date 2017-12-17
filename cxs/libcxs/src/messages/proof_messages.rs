extern crate rust_base58;
extern crate serde_json;

use std::collections::HashMap;
use utils::error;
use messages::validation;

static PROOF_REQUEST: &str = "PROOF_REQUEST";
static PROOF_DATA: &str = "proof_request_data";
static REQUESTED_ATTRS: &str = "requested_attrs";
static REQUESTED_PREDICATES: &str = "requested_predicates";
static DEFAULT_ATTR: &str = "ATTR";

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
struct ProofType {
    name: String,
    #[serde(rename = "version")]
    type_version: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
struct ProofTopic {
    mid: u32,
    tid: u32,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub struct ProofRequestData{
    nonce: String,
    name: String,
    #[serde(rename = "version")]
    data_version: String,
    requested_attrs: String,
    #[serde(skip_serializing, default)]
    requested_predicates: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub struct ProofRequest{
    prover_did: String,
    requester_did: String,
    #[serde(rename = "@type")]
    type_header: ProofType,
    #[serde(rename = "@topic")]
    topic: ProofTopic,
    #[serde(skip_serializing, default)]
    intended_use: String,
    proof_request_data: ProofRequestData,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
}

impl ProofRequest {
    pub fn create() -> ProofRequest {
        ProofRequest {
            prover_did: String::new(),
            requester_did: String::new(),
            type_header: ProofType {
                name: String::from(PROOF_REQUEST),
                type_version: String::new(),
            },
            topic: ProofTopic {
                tid: 0,
                mid: 0,
            },
            intended_use: String::new(),
            proof_request_data: ProofRequestData {
                nonce: String::new(),
                name: String::new(),
                data_version: String::new(),
                requested_attrs: String::new(),
                requested_predicates: String::new(),
            },
            validate_rc: 0,
        }
    }

    pub fn prover_did(&mut self, did: &str) ->&mut Self{
        match validation::validate_did(did){
            Ok(x) => {
                self.prover_did = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn requester_did(&mut self, did: &str) ->&mut Self{
        match validation::validate_did(did){
            Ok(x) => {
                self.requester_did = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    //Todo: find out difference between outter version and inner version
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

    pub fn intended_use(&mut self, intended_use: &str) -> &mut Self {
        self.intended_use = String::from(intended_use);
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
        self.proof_request_data.requested_attrs = attrs.to_string();
        self
    }

    pub fn requested_predicates(&mut self, predicates: &str) -> &mut Self {
        self.proof_request_data.requested_predicates = predicates.to_string();
        self
    }

    pub fn serialize_message(&mut self) -> Result<String, u32> {
        let attrs = self.proof_request_data.requested_attrs.clone();
        let predicates = self.proof_request_data.requested_predicates.clone();
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        let mut proof = json!(self);
        proof[PROOF_DATA][REQUESTED_ATTRS] = combine_request_attributes(
            &self.proof_request_data.name,
            &self.proof_request_data.requested_attrs)?;
//        proof[PROOF_DATA][REQUESTED_PREDICATES] =combine_request_attributes(
//            &self.proof_request_data.name,
//            &self.proof_request_data.requested_predicates)?;
        Ok(proof.to_string())
    }

//    pub fn get_proof_request_data(&mut self) -> ProofRequestData {
//        let attrs = self.proof_request_data.requested_attrs.clone();
//        let predicates = self.proof_request_data.requested_predicates.clone();
//        combine_request_attributes(
//            &self.proof_request_data.name,
//            &self.proof_request_data.requested_attrs)?;
//    }
}

pub fn combine_request_attributes(name: &str, requested_attrs: &str) -> Result<serde_json::Value, u32> {

    let attrs_json: serde_json::Value = match serde_json::from_str(requested_attrs) {
        Ok(x) => x,
        Err(y) => {
            warn!("wrong json format for proof request attributes");
            return Err(error::INVALID_JSON.code_num)
        },
    };
    match attrs_json.as_array() {
        Some(x) => {

            let mut all_attrs: HashMap<String, serde_json::Value> = HashMap::new();
            for i in 0..x.len() {

                let name:String = match serde_json::from_value(x[i]["name"].clone()) {
                    Ok(x) => x,
                    Err(_) => {
                        let attr_name: String = match serde_json::from_value(x[i]["attr_name"].clone()) {
                            Ok(y) => y,
                            Err(_) => return Err(error::INVALID_JSON.code_num),
                        };
                        attr_name
                    }
                };
                all_attrs.insert(format!("{}_{}", name, i), x[i].clone());
            };
            Ok(json!(all_attrs))

        },
        None => Err(error::UNKNOWN_ERROR.code_num)
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
            requested_attrs: String::new(),
            requested_predicates: String::new(),
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
//            .requested_predicates(REQUESTED_PREDICATES)
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
            },
            "prover_did": "",
            "requester_did": "",
        });
        assert_eq!(request.serialize_message().unwrap(), proof_request_test.to_string());
    }
}
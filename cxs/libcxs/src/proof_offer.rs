extern crate serde_json;

use utils::error;
use serde_json::Value;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static PROVER_DID: &'static str = "prover_did";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofOffer{
    pub requester_did: String,
    pub schema_seq_no: i32,
    optional_data: Option<serde_json::Map<String, Value>>,
    tid: String,
    to_did: String,
    from_did: String,
    version: String,
}

impl ProofOffer {
    pub fn new(did: &str) -> ProofOffer {
        ProofOffer {
            requester_did: String::from(did),
            schema_seq_no: 0,
            to_did: String::new(),
            from_did: String::new(),
            tid: String::new(),
            version: String::new(),
            optional_data: None,
        }
    }

    pub fn from_str(payload:&str) -> Result<ProofOffer, u32> {
        match serde_json::from_str(payload) {
            Ok(p) => Ok(p),
            Err(_) => {
                warn!("{}",error::INVALID_PROOF_OFFER.message);
                Err(error::INVALID_PROOF_OFFER.code_num)},
        }
    }

    pub fn get_attrs(&self) -> Result<String, u32> {
        warn!("Invalid json for proof offer");
        Err(error::INVALID_JSON.code_num)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEMP_REQUESTER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";

    fn create_proof_offer() -> ProofOffer {
        let requester_did = String::from(TEMP_REQUESTER_DID);
        ProofOffer::new(&requester_did)
    }
    #[test]
    fn test_proof_offer_struct(){
        let offer = create_proof_offer();
        assert_eq!(offer.requester_did, TEMP_REQUESTER_DID);
    }

    #[test]
    fn test_serialize(){
        let offer = create_proof_offer();
        let serialized = serde_json::to_string(&offer);
        let string_serialized = match serialized {
            Ok(i) => i,
            Err(e) => {assert_eq!(0,1); // this will always fail
                String::from("Err")},
        };

        let output = r#"{"requester_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":0,"optional_data":null,"tid":"","to_did":"","from_did":"","version":""}"#;

        assert_eq!(string_serialized, output)
    }

    #[test]
    fn test_deserialize() {
        let requester_did = String::from("4reqXeZVm7JZAffAoaNLsb");
        let input = r#"{"requester_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":1,"optional_data":null,"tid":"","to_did":"","from_did":"","version":""}"#;
        let offer: ProofOffer = match serde_json::from_str(&input) {
            Ok(i) => i,
            Err(_) => ProofOffer::new("BAD_DID"),
        };
        assert_eq!(offer.requester_did, requester_did);

    }

    #[test]
    fn test_proof_offer_is_parsed_correctly(){
        let response = r#"{\"requester_did\":\"V4SGRU86Z58d6TV7PBUe6f\",\"schema_seq_no\":103,\"msg_type\":\"proofOffer\",\"version\":\"0.1\",\"to_did\":\"H35Tam37aefr7o9wJGvjM7\",\"from_did\":\"G16EUDB5dfd73w3oaywFzh\",\"tid\":\"1\"}"#;
        let v = String::from(response).replace("\\\"", "\"");
        let proof_offer:ProofOffer = ProofOffer::from_str(&v).unwrap();
        assert_eq!(proof_offer.requester_did,"V4SGRU86Z58d6TV7PBUe6f");
    }
}

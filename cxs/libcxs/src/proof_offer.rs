extern crate serde_json;

use utils::error;
use serde_json::Value;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static PROVER_DID: &'static str = "prover_did";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofOffer{
    version: String,
    to_did: String,
    from_did: String,
    proof_request_id: String,
    proofs: Option<serde_json::Map<String, Value>>,
    aggregated_proof: Option<serde_json::Map<String, Value>>,
    requested_proof: Option<serde_json::Map<String, Value>>,
    unrevealed_attrs: Option<serde_json::Map<String, Value>>,
    self_attested_attrs: Option<serde_json::Map<String, Value>>,
    predicates: Option<serde_json::Map<String, Value>>,

}

//#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//pub struct Proofs{
//
//}
//
//#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//pub struct AggregatedProof{
//
//}
//
//#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//pub struct RequestedProof{
//    revealed_attrs: serde_json::Map<String, Vec<serde_json::Value>>,
//
//}

impl ProofOffer {
    pub fn new(did: &str) -> ProofOffer {
        ProofOffer {
            version: String::new(),
            to_did: String::new(),
            from_did: String::from(did),
            proof_request_id: String::new(),
            proofs: None,
            aggregated_proof: None,
            requested_proof: None,
            unrevealed_attrs: None,
            self_attested_attrs: None,
            predicates: None,
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
    static EXAMPLE_PROOF: &'static str = "{\"msg_type\":\"proof\",\"version\":\"0.1\",\"to_did\":\"BnRXf8yDMUwGyZVDkSENeq\",\"from_did\":\"GxtnGN6ypZYgEqcftSQFnC\",\"proof_request_id\":\"cCanHnpFAD\",\"proofs\":{\"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b\":{\"proof\":{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"state\":\"96473275571522321025213415717206189191162\"},\"a_prime\":\"921....546\",\"e\":\"158....756\",\"v\":\"114....069\",\"m\":{\"address1\":\"111...738\",\"zip\":\"149....066\",\"city\":\"209....294\",\"address2\":\"140....691\"},\"m1\":\"777....518\",\"m2\":\"515....229\"},\"ge_proofs\":[]},\"non_revoc_proof\":null},\"schema_seq_no\":15,\"issuer_did\":\"4fUDR9R7fjwELRvH9JT6HH\"}},\"aggregated_proof\":{\"c_hash\":\"25105671496406009212798488318112715144459298495509265715919744143493847046467\",\"c_list\":[[72,245,38,\"....\",46,195,18]]},\"requested_proof\":{\"revealed_attrs\":{\"attr_key_id\":[\"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b\",\"UT\",\"96473275571522321025213415717206189191162\"]},\"unrevealed_attrs\":{},\"self_attested_attrs\":{},\"predicates\":{}}}";

    fn create_proof_offer() -> ProofOffer {
        let requester_did = String::from(TEMP_REQUESTER_DID);
        ProofOffer::new(&requester_did)
    }
    #[test]
    fn test_proof_offer_struct(){
        let offer = create_proof_offer();
        assert_eq!(offer.from_did, TEMP_REQUESTER_DID);
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

        let output = r#"{"version":"","to_did":"","from_did":"4reqXeZVm7JZAffAoaNLsb","proof_request_id":"","proofs":null,"aggregated_proof":null,"requested_proof":null,"unrevealed_attrs":null,"self_attested_attrs":null,"predicates":null}"#;

        assert_eq!(string_serialized, output)
    }

    #[test]
    fn test_deserialize() {
        let requester_did = String::from("GxtnGN6ypZYgEqcftSQFnC");
        let offer: ProofOffer = match serde_json::from_str(EXAMPLE_PROOF) {
            Ok(i) => i,
            Err(_) => ProofOffer::new("BAD_DID"),
        };
        let issuer_did = serde_json::to_value("4fUDR9R7fjwELRvH9JT6HH").unwrap();
        assert_eq!(offer.from_did, requester_did);
        assert_eq!(offer.proofs.unwrap()
                       .get("claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b").unwrap()
                       .get("issuer_did").unwrap(), &issuer_did);

    }

    #[test]
    fn test_proof_offer_is_parsed_correctly(){
        let response = r#"{"version":"","to_did":"","from_did":"V4SGRU86Z58d6TV7PBUe6f","proof_request_id":"","proofs":null,"aggregated_proof":null,"requested_proof":null,"unrevealed_attrs":null,"self_attested_attrs":null,"predicates":null}"#;
        let v = String::from(response).replace("\\\"", "\"");
        let proof_offer:ProofOffer = ProofOffer::from_str(&v).unwrap();
        assert_eq!(proof_offer.from_did,"V4SGRU86Z58d6TV7PBUe6f");
    }
}

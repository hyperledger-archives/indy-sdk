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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClaimData{
    schema_seq_no: u32,
    issuer_did: String,
}
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

    pub fn to_string(&self) -> Result<String, u32> {
        match serde_json::to_string(&self){
            Ok(s) => Ok(s),
            Err(_) => Err(error::INVALID_PROOF_OFFER.code_num),
        }
    }

    pub fn from_string(s: &str) -> Result<ProofOffer, u32> {
        match serde_json::from_str(s){
            Ok(po) => Ok(po),
            Err(_) => {
                warn!("{}",error::INVALID_PROOF_OFFER.message);
                Err(error::INVALID_PROOF_OFFER.code_num)},
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

    pub fn get_proof(&self) -> Option<serde_json::Map<String, Value>>{
        self.proofs.to_owned()
    }

    pub fn get_aggregated_proof(&self) -> Result<String, u32> {
        let aggregated_proof = self.aggregated_proof.to_owned();
        match aggregated_proof {
            // TODO FIX THIS UNWRAP
            Some(a) => Ok(serde_json::to_string(&a).unwrap()),
            None => Err(error::INVALID_PROOF_OFFER.code_num)
        }
    }
    pub fn get_proof_as_json(&self) -> Result<String, u32> {
        let proofs = self.get_proof();
        match proofs {
            // TODO FIX THIS
            Some(p) => match serde_json::to_string(&p) {
                Ok(s) => Ok(s),
                Err(_) => Err(error::INVALID_PROOF_OFFER.code_num),
            },
            None => Err(error::INVALID_PROOF_OFFER.code_num),
        }
    }
    fn get_claim_schema_info (&self) -> Result<Vec<ClaimData>, u32> {
        let proofs = match self.proofs {
            Some(ref x) => x,
            None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
        };

        let mut claims: Vec<ClaimData> = Vec::new();
        for (attr, vec) in proofs.iter() {
            claims.push(ClaimData {
                issuer_did: match vec.get("issuer_did") {
                    Some(d) => {
                        match d.as_str() {
                            Some(n) => n,
                            None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
                        }
                    }
                    None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
                }.to_string(),
                schema_seq_no: match vec.get("schema_seq_no") {
                    Some(x) => {
                        match x.as_u64() {
                            Some(x) => x as u32,
                            None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
                        }
                    }
                    None => return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
                },
            });
        }
        Ok(claims)
    }
}

fn create_from_message(s: &str) -> Result<ProofOffer, u32>{
   match serde_json::from_str(s) {
       Ok(p) => Ok(p),
       Err(_) => {
           warn!("{}",error::INVALID_PROOF_OFFER.message);
           Err(error::INVALID_PROOF_OFFER.code_num)},
   }
}



#[cfg(test)]
pub mod tests {
    use super::*;
//    static DEFAULT_PROOF_OFFER: &str = r#"{version:"0.1",to_did:"BnRXf8yDMUwGyZVDkSENeq",from_did:"GxtnGN6ypZYgEqcftSQFnC",proof_request_id:"cCanHnpFAD",proofs:Some({"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":Object({"issuer_did":String("4fUDR9R7fjwELRvH9JT6HH"),"proof":Object({"non_revoc_proof":Null,"primary_proof":Object({"eq_proof":Object({"a_prime":String("921....546"),"e":String("158....756"),"m":Object({"address1":String("111...738"),"address2":String("140....691"),"city":String("209....294"),"zip":String("149....066")}),"m1":String("777....518"),"m2":String("515....229"),"revealed_attrs":Object({"state":String("96473275571522321025213415717206189191162")}),"v":String("114....069")}),"ge_proofs":Array([])})}),"schema_seq_no":Number(PosInt(15))})}),aggregated_proof:Some({"c_hash":String("25105671496406009212798488318112715144459298495509265715919744143493847046467"),"c_list":Array([Array([Number(PosInt(72)),Number(PosInt(245)),Number(PosInt(38)),String("...."),Number(PosInt(46)),Number(PosInt(195)),Number(PosInt(18))])])}),requested_proof:Some({"predicates":Object({}),"revealed_attrs":Object({"attr_key_id":Array([String("claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b"),String("UT"),String("96473275571522321025213415717206189191162")])}),"self_attested_attrs":Object({}),"unrevealed_attrs":Object({})}),unrevealed_attrs:None,self_attested_attrs:None,predicates:None}"#;
    static DEFAULT_SERIALIZED_PROOF_OFFER: &str = r#"{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":{"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","proof":{"non_revoc_proof":null,"primary_proof":{"eq_proof":{"a_prime":"921....546","e":"158....756","m":{"address1":"111...738","address2":"140....691","city":"209....294","zip":"149....066"},"m1":"777....518","m2":"515....229","revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"v":"114....069"},"ge_proofs":[]}},"schema_seq_no":15}},"aggregated_proof":{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]},"requested_proof":{"predicates":{},"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"self_attested_attrs":{},"unrevealed_attrs":{}},"unrevealed_attrs":null,"self_attested_attrs":null,"predicates":null}"#;

    static TEMP_REQUESTER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";
    static EXAMPLE_PROOF: &'static str = "{\"msg_type\":\"proof\",\"version\":\"0.1\",\"to_did\":\"BnRXf8yDMUwGyZVDkSENeq\",\"from_did\":\"GxtnGN6ypZYgEqcftSQFnC\",\"proof_request_id\":\"cCanHnpFAD\",\"proofs\":{\"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b\":{\"proof\":{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"state\":\"96473275571522321025213415717206189191162\"},\"a_prime\":\"921....546\",\"e\":\"158....756\",\"v\":\"114....069\",\"m\":{\"address1\":\"111...738\",\"zip\":\"149....066\",\"city\":\"209....294\",\"address2\":\"140....691\"},\"m1\":\"777....518\",\"m2\":\"515....229\"},\"ge_proofs\":[]},\"non_revoc_proof\":null},\"schema_seq_no\":15,\"issuer_did\":\"4fUDR9R7fjwELRvH9JT6HH\"}},\"aggregated_proof\":{\"c_hash\":\"25105671496406009212798488318112715144459298495509265715919744143493847046467\",\"c_list\":[[72,245,38,\"....\",46,195,18]]},\"requested_proof\":{\"revealed_attrs\":{\"attr_key_id\":[\"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b\",\"UT\",\"96473275571522321025213415717206189191162\"]},\"unrevealed_attrs\":{},\"self_attested_attrs\":{},\"predicates\":{}}}";
    static MSG_FROM_API: &str = r#"{"msg_type":"proof","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH"},"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]},"requested_proof":{"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}"#;
    pub fn create_default_proof_offer()-> ProofOffer {
        ProofOffer::from_string(DEFAULT_SERIALIZED_PROOF_OFFER).unwrap()
    }

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
        let proof_offer: ProofOffer = create_from_message(MSG_FROM_API).unwrap();
        assert!(proof_offer.get_aggregated_proof().is_ok());
        assert_eq!(proof_offer.from_did,"GxtnGN6ypZYgEqcftSQFnC");
        let serialized = proof_offer.to_string().unwrap();
        let new_proof_offer:ProofOffer = ProofOffer::from_string(&serialized).unwrap();
        assert_eq!(proof_offer.from_did, new_proof_offer.from_did);
        assert!(proof_offer.get_proof_as_json().is_ok());
        let proof_json = proof_offer.get_proof_as_json();
        let stuff =  r#"{
            "proof":{
                "primary_proof":{
                    "eq_proof":{
                        "revealed_attrs":{"state":"96473275571522321025213415717206189191162"},
                        "a_prime":"921....546",
                        "e":"158....756",
                        "v":"114....069",
                        "m":{
                            "address1":"111...738",
                            "zip":"149....066",
                            "city":"209....294",
                            "address2":"140....691"
                        },
                        "m1":"777....518",
                        "m2":"515....229"
                    },
                    "ge_proofs":[]
                },
                "non_revoc_proof":null
            },
            "schema_seq_no":15,
            "issuer_did":"4fUDR9R7fjwELRvH9JT6HH"
        }"#;
        let claim_data: ClaimData = serde_json::from_str(stuff).unwrap();
        assert_eq!(claim_data.issuer_did, "4fUDR9R7fjwELRvH9JT6HH");
        assert_eq!(proof_offer.get_claim_schema_info().unwrap()[0].issuer_did, "4fUDR9R7fjwELRvH9JT6HH");
        assert_eq!(proof_offer.get_claim_schema_info().unwrap()[1].issuer_did, "33UDR9R7fjwELRvH9JT6HH");
        let mut proof_offer_bad: ProofOffer = create_from_message(MSG_FROM_API).unwrap();
        proof_offer_bad.proofs = None;
        assert!(proof_offer_bad.get_claim_schema_info().is_err());

    }

}

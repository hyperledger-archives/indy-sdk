extern crate serde_json;

use utils::error;
use serde_json::Value;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static BLINDED_MS: &'static str ="blinded_ms";
static PROVER_DID: &'static str = "prover_did";

#[allow(non_snake_case)]
static U: &'static str = "u";
static UR: &'static str = "ur";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClaimRequest{
    pub blinded_ms: Option<BlindedMasterSecret> ,
    pub issuer_did: String,
    // TODO: Either change this to u32 or convert other things to i32
    pub schema_seq_no: i32,
    optional_data: Option<serde_json::Map<String, Value>>,
    tid: String,
    to_did: String,
    from_did: String,
    version: String,
    mid: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlindedMasterSecret {
    pub prover_did: String,
    pub u: String,
    pub ur: Option<String>,
}

impl ClaimRequest {
    pub fn new(secret: Option<BlindedMasterSecret>, did: &str, seq_no: i32) -> ClaimRequest {
       ClaimRequest {
           blinded_ms: secret,
           issuer_did: String::from(did),
           schema_seq_no: seq_no,
           to_did: String::new(),
           from_did: String::new(),
           mid: String::new(),
           tid: String::new(),
           version: String::new(),
           optional_data: None,
       }
    }

    pub fn from_str(payload:&str) -> Result<ClaimRequest, u32> {
        match serde_json::from_str(payload) {
            Ok(p) => Ok(p),
            Err(_) => {warn!("{}",error::INVALID_CLAIM_REQUEST.message);
                        Err(error::INVALID_CLAIM_REQUEST.code_num)},
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::CLAIM_REQ_STRING;

    static TEMP_ISSUER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";

    fn create_claim_req() -> ClaimRequest {
        let master_secret:Option<BlindedMasterSecret> = None;
        let issuer_did = String::from(TEMP_ISSUER_DID);
        let seq_no = 1;
        ClaimRequest::new(master_secret, &issuer_did, seq_no)
    }
    #[test]
    fn test_claim_request_struct(){
        let req = create_claim_req();
        assert_eq!(req.issuer_did, TEMP_ISSUER_DID);
    }

    #[test]
    fn test_serialize(){
        let req = create_claim_req();
        let serialized = serde_json::to_string(&req);
        let string_serialized = match serialized {
            Ok(i) => i,
            Err(e) => {assert_eq!(0,1); // this will always fail
                        String::from("Err")},
        };

        let output = r#"{"blinded_ms":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":1,"optional_data":null,"tid":"","to_did":"","from_did":"","version":"","mid":""}"#;

        assert_eq!(string_serialized, output)
    }

    #[test]
    fn test_deserialize() {
        let issuer_did = String::from("4reqXeZVm7JZAffAoaNLsb");
        let input = r#"{"blinded_ms":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":1,"optional_data":null,"tid":"","to_did":"","from_did":"","version":"","mid":""}"#;
        let req: ClaimRequest = match serde_json::from_str(&input) {
            Ok(i) => i,
            Err(_) => ClaimRequest::new(None, "BAD_DID", 0),
        };
        assert_eq!(req.issuer_did, issuer_did);

    }

    #[test]
    fn test_create_claim_request_from_raw_message() {
        let claim_req:ClaimRequest = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();

        let bms: BlindedMasterSecret = claim_req.blinded_ms.unwrap().clone();
        assert_eq!(bms.prover_did,"FQ7wPBUgSPnDGJnS1EYjTK");
        assert_eq!(bms.u, "923...607");
        assert_eq!(bms.ur,None);
        assert_eq!(claim_req.issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(claim_req.schema_seq_no, 15);
        assert_eq!(claim_req.tid, "cCanHnpFAD");
        assert_eq!(claim_req.to_did, "BnRXf8yDMUwGyZVDkSENeq");
        assert_eq!(claim_req.from_did, "GxtnGN6ypZYgEqcftSQFnC");
        assert_eq!(claim_req.version, "0.1");
        assert_eq!(claim_req.mid, "");
    }
    #[test]
    fn test_create_claim_request_from_api_msg(){
        let claim_req = ClaimRequest::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_did = claim_req.issuer_did;
        let seq_no = claim_req.schema_seq_no;
        let master_secret = claim_req.blinded_ms.unwrap();
        assert_eq!(issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(seq_no, 15);
        assert_eq!(master_secret.prover_did, "FQ7wPBUgSPnDGJnS1EYjTK");
    }

    #[test]
    fn test_claim_request_comes_from_response_is_parsed_correctly(){
        let response = r#"{\"blinded_ms\":{\"prover_did\":\"G16EUDB5dfd73w3oaywFzh\",\"u\":\"1094279170675034245598640432411538327296225119909229995298347779566763746113690246882694662546933377604478614968217678170666658221203557584882866694715078438080578431109840811329571205051783663795978327553718149359059190363720526574101091846654186337257673447790576085524883306362644745797582272885216971090442678422538163610615661054859030505209936739031695333842224696940648077254927477742303105203568012636507245452412619868034546441069561591690553997098638575873471077241345907454279267668483734714369746789196752913402370749399762398167860924315079234679025142186086404286563578667397864594981884627430046515556\",\"ur\":null},\"issuer_did\":\"V4SGRU86Z58d6TV7PBUe6f\",\"schema_seq_no\":103,\"msg_type\":\"claimReq\",\"version\":\"0.1\",\"to_did\":\"H35Tam37aefr7o9wJGvjM7\",\"from_did\":\"G16EUDB5dfd73w3oaywFzh\",\"tid\":\"1\",\"mid\":\"1\"}"#;
        let v = String::from(response).replace("\\\"", "\"");
        let claim_req:ClaimRequest = ClaimRequest::from_str(&v).unwrap();
        assert_eq!(claim_req.issuer_did,"V4SGRU86Z58d6TV7PBUe6f");
    }
}

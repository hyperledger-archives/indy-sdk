extern crate serde_json;

use object_cache::ObjectCache;
use api::VcxStateType;
use utils::error;
use connection;
use messages;
use messages::GeneralMessage;
use messages::proofs::proof_message::{ProofMessage };
use messages::proofs::proof_request::{ ProofRequestMessage };
use messages::extract_json_payload;
use messages::to_u8;

use claim_def::{ RetrieveClaimDef, ClaimDefCommon };
use schema::LedgerSchema;

use utils::libindy::anoncreds;
use utils::libindy::wallet;
use utils::libindy::SigTypes;
use utils::libindy::crypto;

use serde_json::Value;
use serde_json::Map;

use settings;
use utils::httpclient;
use utils::constants::SEND_MESSAGE_RESPONSE;

use error::ToErrorCode;
use error::proof::ProofError;

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<DisclosedProof>  = Default::default();
}

impl Default for DisclosedProof {
    fn default() -> DisclosedProof
    {
        DisclosedProof {
            source_id: String::new(),
            my_did: None,
            my_vk: None,
            state: VcxStateType::VcxStateNone,
            proof_request: None,
            link_secret_alias: settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap(),
            their_did: None,
            their_vk: None,
            agent_did: None,
            agent_vk: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DisclosedProof {
    source_id: String,
    my_did: Option<String>,
    my_vk: Option<String>,
    state: VcxStateType,
    proof_request: Option<ProofRequestMessage>,
    link_secret_alias: String,
    their_did: Option<String>,
    their_vk: Option<String>,
    agent_did: Option<String>,
    agent_vk: Option<String>,
}

fn _match_claim(claims: &Value, id: &str) -> Option<(String, String, u32)> {
    let claims = match claims {
        &Value::Array(ref list) => list,
        _ => return None
    };
    for claim in claims.iter() {
        let claim_id = &claim["claim_uuid"];
        if let &Value::String(ref str) = claim_id {
            if str.eq(id) {

                fn get_val(val: Option<&Value>) -> Option<&String> {
                    match val {
                        Some(did) => {
                            match did {
                                &Value::String(ref s) => Some(s),
                                _ => None
                            }
                        },
                        None => None
                    }
                }
                let issuer_did = get_val(claim.get("issuer_did"));
                let issuer_did = match issuer_did {
                    Some(v) => v,
                    None => continue
                };

                let schema_seq_no = get_val(claim.get("schema_seq_no"));
                let schema_seq_no = match schema_seq_no {
                    Some(v) => v,
                    None => continue
                };
                let schema_seq_no = match schema_seq_no.parse::<u32>(){
                    Ok(i) => i,
                    Err(_) => continue
                };

                return Some((String::from(id), issuer_did.to_owned(), schema_seq_no))

            }
        }
    }
    None
}

fn claim_def_identifiers(claims: &str) -> Result<Vec<(String, String, String, u64)>, ProofError>{
    let mut rtn = Vec::new();

    let claims: Value = serde_json::from_str(&claims)
        .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

    if let Value::Object(ref map) = claims["attrs"] {
        for (key, value) in map {
            if let Value::Object(ref attr_obj) = value[0] {
                let claim_uuid = match attr_obj.get("claim_uuid") {
                    Some(i) => if i.is_string() { i.as_str().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                let issuer_did = match attr_obj.get("issuer_did") {
                    Some(i) => if i.is_string() { i.as_str().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                let schema_seq_no = match attr_obj.get("schema_seq_no") {
                    Some(i) => if i.is_number() { i.as_u64().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                rtn.push((key.to_owned(),
                          claim_uuid.to_owned(),
                          issuer_did.to_owned(),
                          schema_seq_no))
            }
        }
    }
    else {
        return Err(ProofError::CommonError(error::INVALID_JSON.code_num))
    }

    Ok(rtn)
}


impl DisclosedProof {

    fn set_proof_request(&mut self, req: ProofRequestMessage) {self.proof_request = Some(req)}

    fn get_state(&self) -> u32 {self.state as u32}
    fn set_state(&mut self, state: VcxStateType) {self.state = state}

    fn _find_schemas(&self, claims_identifers: &Vec<(String, String, String, u64)>) -> Result<String, ProofError> {
        let mut rtn = Map::new();

        for &(ref attr_id, ref claim_uuid, ref issuer_did, schema_seq_num) in claims_identifers {
            let schema = LedgerSchema::new_from_ledger(schema_seq_num as i32).map_err(|_| ProofError::InvalidSchema())?;
            let schema = schema.data.ok_or(ProofError::CommonError(error::INVALID_SCHEMA.code_num))?;

            let schema: Value = serde_json::to_value(schema)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

            rtn.insert(claim_uuid.to_owned(), schema);
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&Value::Object(rtn))
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }
    }

    fn _find_claim_def(&self, claims_identifers: &Vec<(String, String, String, u64)>) -> Result<String, ProofError> {

        let mut rtn = Map::new();

        for &(ref attr_id, ref claim_uuid, ref issuer_did, schema_seq_num) in claims_identifers {
            let claim_def = RetrieveClaimDef::new()
                .retrieve_claim_def("GGBDg1j8bsKmr4h5T9XqYf",
                                    schema_seq_num as u32,
                                    Some(SigTypes::CL),
                                    &issuer_did).map_err(|_| ProofError::InvalidCredData())?;

            let claim_def: Value = serde_json::from_str(&claim_def).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

            rtn.insert(claim_uuid.to_owned(), claim_def);
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&Value::Object(rtn)).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }
    }

    fn _build_requested_claims(&self, claims_identifiers: &Vec<(String, String, String, u64)>) -> Result<String, ProofError> {
        let mut rtn: Value = json!({
              "self_attested_attributes":{},
              "requested_attrs":{},
              "requested_predicates":{}
        });
        if let Value::Object(ref mut map) = rtn["requested_attrs"] {
            for &(ref attr_id, ref claim_uuid, ref issuer_did, schema_seq_num) in claims_identifiers {
                let insert_val = json!([claim_uuid, true]);
                map.insert(attr_id.to_owned(), insert_val);
            }
        }

        let rtn = serde_json::to_string_pretty(&rtn).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;
        Ok(rtn)

    }

    fn _build_proof(&self) -> Result<ProofMessage, ProofError> {

        let wallet_h = wallet::get_wallet_handle();

        let proof_req = self.proof_request.as_ref()
            .ok_or(ProofError::CreateProofError())?;
        let proof_req_data_json = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        let claims = anoncreds::libindy_prover_get_claims(wallet_h,
                                                          &proof_req_data_json)
            .map_err(|ec| ProofError::CommonError(ec))?;

        debug!("claims: {}", claims);
        let claims_identifiers = claim_def_identifiers(&claims)?;
        let requested_claims = self._build_requested_claims(&claims_identifiers)?;

        let schemas = self._find_schemas(&claims_identifiers)?;
        debug!("schemas: {}", schemas);
        let claim_defs_json = self._find_claim_def(&claims_identifiers)?;
        debug!("claim_defs: {}", claim_defs_json);
        let revoc_regs_json = Some("{}");

        let proof = anoncreds::libindy_prover_create_proof(wallet_h,
                                                          &proof_req_data_json,
                                                           &requested_claims,
                                                          &schemas,
                                                          &self.link_secret_alias,
                                                          &claim_defs_json,
                                                          revoc_regs_json).map_err(|ec| ProofError::CommonError(ec))?;

        let proof: ProofMessage = serde_json::from_str(&proof)
            .or(Err(ProofError::CommonError(error::UNKNOWN_LIBINDY_ERROR.code_num)))?;

        Ok(proof)
    }

    fn send_proof(&mut self, connection_handle: u32) -> Result<u32, ProofError> {
        debug!("sending proof via connection connection: {}", connection_handle);
        // There feels like there's a much more rusty way to do the below.
        self.my_did = Some(connection::get_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_did = Some(connection::get_agent_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);


        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let local_their_did = self.their_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_their_vk = self.their_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_did = self.agent_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_did = self.my_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_vk = self.my_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;

        let proof_req = self.proof_request.as_ref().ok_or(ProofError::CreateProofError())?;
        let ref_msg_uid = proof_req.msg_ref_id.as_ref().ok_or(ProofError::CreateProofError())?;

        let proof = match settings::test_indy_mode_enabled() {
            false => {
                let proof: ProofMessage = self._build_proof()?;
                serde_json::to_string(&proof).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?
            },
            true => String::from("dummytestmodedata")
        };

        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &proof, "PROOF")
            .or(Err(ProofError::ProofConnectionError()))?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("proof")
            .agent_did(local_agent_did)
            .agent_vk(local_agent_vk)
            .edge_agent_payload(&data)
            .ref_msg_id(ref_msg_uid)
            .send_secure() {
            Ok(response) => {
                self.state = VcxStateType::VcxStateAccepted;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proof: {}", x);
                return Err(ProofError::CommonError(x));
            }
        }
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }
    fn get_source_id(&self) -> &String { &self.source_id }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(code_num: u32) -> u32 {
    if code_num == error::INVALID_OBJ_HANDLE.code_num {
        error::INVALID_DISCLOSED_PROOF_HANDLE.code_num
    }
    else {
        code_num
    }
}

pub fn create_proof(source_id: String, proof_req: String) -> Result<u32, ProofError> {
    debug!("creating disclosed proof with id: {}", source_id);

    let mut new_proof: DisclosedProof = Default::default();

    new_proof.set_source_id(&source_id);
    new_proof.set_proof_request(serde_json::from_str(&proof_req)
        .map_err(|_| ProofError::CommonError(error::INVALID_JSON.code_num))?);

    new_proof.set_state(VcxStateType::VcxStateRequestReceived);

    Ok(HANDLE_MAP.add(new_proof).map_err(|ec| ProofError::CommonError(ec))?)
}

pub fn get_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

// update_state is just the same as get_state for disclosed_proof
pub fn update_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj|{
        Ok(obj.get_state())
    })
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj|{
        serde_json::to_string(&obj).map_err(|e|{
            warn!("Unable to serialize: {:?}", e);
            error::SERIALIZATION_ERROR.code_num
        })
    })
}

pub fn from_string(proof_data: &str) -> Result<u32, ProofError> {
    let derived_proof: DisclosedProof = match serde_json::from_str(proof_data) {
        Ok(x) => x,
        Err(y) => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
    };

    let new_handle = HANDLE_MAP.add(derived_proof).map_err(|ec| ProofError::CommonError(ec))?;

    info!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}

pub fn release(handle: u32) -> Result<(), u32> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn send_proof(handle: u32, connection_handle: u32) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.send_proof(connection_handle).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

//TODO one function with claim
pub fn get_proof_request_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, u32> {
    let my_did = connection::get_pw_did(connection_handle)?;
    let my_vk = connection::get_pw_verkey(connection_handle)?;
    let agent_did = connection::get_agent_did(connection_handle)?;
    let agent_vk = connection::get_agent_verkey(connection_handle)?;

    let payload = messages::get_message::get_all_message(&my_did,
                                                         &my_vk,
                                                         &agent_did,
                                                         &agent_vk)?;

    let mut messages: Vec<ProofRequestMessage> = Default::default();

    for msg in payload {
        if msg.sender_did.eq(&my_did){ continue; }

        if msg.msg_type.eq("proofReq") {
            let msg_data = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice())?
                },
                None => return Err(error::INVALID_HTTP_RESPONSE.code_num)
            };

            let req = extract_json_payload(&msg_data)?;

            let mut req: ProofRequestMessage = serde_json::from_str(&req)
                .or(Err(error::INVALID_HTTP_RESPONSE.code_num))?;

            req.msg_ref_id = Some(msg.uid.to_owned());
            messages.push(req);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).unwrap())
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use utils::httpclient;
    use issuer_claim;
    use proof;
    use claim;
    use std::thread;
    use std::time::Duration;
    use api::ProofStateType;

    const CLAIMS: &str = r#"{"attrs":{"address1_0":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"zip_4":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"address2_1":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"city_2":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"state_3":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}]},"predicates":{}}"#;
    const PROOF_OBJECT_JSON: &str = r#"{"source_id":"","my_did":null,"my_vk":null,"state":3,"proof_request":{"@type":{"name":"PROOF_REQUEST","version":"1.0"},"@topic":{"mid":9,"tid":1},"proof_request_data":{"nonce":"838186471541979035208225","name":"Account Certificate","version":"0.1","requested_attrs":{"name_0":{"name":"name","schema_seq_no":52},"business_2":{"name":"business","schema_seq_no":52},"email_1":{"name":"email","schema_seq_no":52}},"requested_predicates":{}},"msg_ref_id":"ymy5nth"},"link_secret_alias":"main","their_did":null,"their_vk":null,"agent_did":null,"agent_vk":null}"#;
    const DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";

    #[test]
    fn test_create_proof() {
        settings::set_defaults();
        assert!(create_proof("1".to_string(), ::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap() > 0);
    }

    #[test]
    fn test_create_fails() {
        settings::set_defaults();
        assert_eq!(create_proof("1".to_string(),"{}".to_string()).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_proof_cycle() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_h = connection::build_connection("test_send_claim_offer").unwrap();

        httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec());

        let requests = get_proof_request_messages(connection_h, None).unwrap();
        let requests:Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();

        let handle = create_proof("TEST_CLAIM".to_owned(), requests).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap());
        send_proof(handle, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(handle).unwrap());
    }

    #[test]
    fn get_state_test(){
        settings::set_defaults();
        let proof: DisclosedProof =  Default::default();
        assert_eq!(VcxStateType::VcxStateNone as u32, proof.get_state());
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap())
    }

    #[test]
    fn to_string_test() {
        settings::set_defaults();
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        let serialized = to_string(handle).unwrap();
        println!("serizlied: {}", serialized);
        from_string(&serialized).unwrap();
    }

    #[test]
    fn test_deserialize_fails() {
        assert_eq!(from_string("{}").err(),
        Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_claim_def_identifiers() {
        let claims_identifiers = claim_def_identifiers(CLAIMS).unwrap();

        assert_eq!(claims_identifiers.len(), 5);
        assert_eq!(claims_identifiers[1],("address2_1".to_string(),"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a".to_string(),"2hoqvcwupRTUNkXn6ArYzs".to_string(), 22));
        assert_eq!(claims_identifiers[2],("city_2".to_string(),"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a".to_string(),"2hoqvcwupRTUNkXn6ArYzs".to_string(), 22));
    }

    #[test]
    fn test_claim_def_identifiers_failures() {
        assert_eq!(claim_def_identifiers("{}").err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));

        let claims = r#"{"attrs":{"state_3":[{"claim_uuid":"uuid","issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}]}}"#;
        assert_eq!(claim_def_identifiers(claims).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));

        let claims = r#"{"attrs":{"state_3":[{"claim_uuid":"uuid","attrs":{"state":"UT","zip":"84000"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}]}}"#;
        let claims_identifiers = claim_def_identifiers(&claims).unwrap();
    }

    #[test]
    fn test_find_schemas() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let claim_ids = claim_def_identifiers(CLAIMS).unwrap();
        let proof: DisclosedProof = Default::default();
        let schemas = proof._find_schemas(&claim_ids).unwrap();
        assert!(schemas.len() > 0);
    }

    #[test]
    fn test_find_schemas_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let claim_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_schemas(&claim_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_find_claim_def() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let claim_ids = claim_def_identifiers(CLAIMS).unwrap();
        let proof: DisclosedProof = Default::default();
        let claim_def = proof._find_claim_def(&claim_ids).unwrap();
        assert!(claim_def.len() > 0);
    }

    #[test]
    fn test_find_claim_def_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let claim_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_claim_def(&claim_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_build_requested_claims() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let claim_ids = claim_def_identifiers(CLAIMS).unwrap();
        let proof: DisclosedProof = Default::default();
        let requested_claim = proof._build_requested_claims(&claim_ids).unwrap();
        assert!(requested_claim.len() > 0);
    }

    #[ignore]
    #[test]
    fn test_real_proof() {
        ::utils::logger::LoggerUtils::init();
        settings::set_to_defaults();
        //BE INSTITUTION AND GENERATE INVITE FOR CONSUMER
        ::utils::devsetup::setup_dev_env("test_real_proof");
        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(wallet::get_wallet_handle(), ::settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        let alice = connection::build_connection("alice").unwrap();
        connection::connect(alice, Some("{}".to_string())).unwrap();
        let details = connection::get_invite_details(alice,true).unwrap();
        //BE CONSUMER AND ACCEPT INVITE FROM INSTITUTION
        ::utils::devsetup::be_consumer();
        let faber = connection::build_connection_with_invite("faber", &details).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, connection::get_state(faber));
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, connection::get_state(alice));
        connection::connect(faber, Some("{}".to_string())).unwrap();
        //BE INSTITUTION AND CHECK THAT INVITE WAS ACCEPTED
        ::utils::devsetup::be_institution();
        thread::sleep(Duration::from_millis(2000));
        connection::update_state(alice).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, connection::get_state(alice));
        // AS INSTITUTION SEND CLAIM OFFER
        let claim_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let schema_seq_no = 22;
        let claim_offer = issuer_claim::issuer_claim_create(schema_seq_no,
                                                            "1".to_string(),
                                                            settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                            "claim_name".to_string(),
                                                            claim_data.to_owned()).unwrap();
        issuer_claim::send_claim_offer(claim_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND CLAIM REQUEST
        ::utils::devsetup::be_consumer();
        let claim_offers = claim::get_claim_offer_messages(faber, None).unwrap();
        let offers:Value = serde_json::from_str(&claim_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        println!("claim_offer: {}", offers);
        let claim = claim::claim_create_with_offer("TEST_CLAIM", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, claim::get_state(claim).unwrap());
        claim::send_claim_request(claim, faber).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS INSTITUTION SEND CLAIM
        ::utils::devsetup::be_institution();
        issuer_claim::update_state(claim_offer);
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_claim::get_state(claim_offer));
        issuer_claim::send_claim(claim_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CLAIM
        ::utils::devsetup::be_consumer();
        claim::update_state(claim).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, claim::get_state(claim).unwrap());
        // AS INSTITUTION SEND PROOF REQUEST
        ::utils::devsetup::be_institution();
        let requested_attrs = json!([
           {
              "schema_seq_no":schema_seq_no,
              "name":"address1",
              "issuer_did": ::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"address2",
              "issuer_did": ::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"city",
              "issuer_did": ::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"state",
              "issuer_did": ::utils::devsetup::INSTITUTION_DID
           },
           {
              "schema_seq_no":schema_seq_no,
              "name":"zip",
              "issuer_did": ::utils::devsetup::INSTITUTION_DID
           }
        ]).to_string();

        let proof_req_handle = proof::create_proof("1".to_string(),requested_attrs,"[]".to_string(),"name".to_string()).unwrap();
        proof::send_proof_request(proof_req_handle,alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND PROOF
        ::utils::devsetup::be_consumer();
        let requests = get_proof_request_messages(faber, None).unwrap();
        let requests:Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();
        let proof_handle = create_proof(DEFAULT_PROOF_NAME.to_string(), requests).unwrap();
        send_proof(proof_handle, faber).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(proof_handle).unwrap());
        thread::sleep(Duration::from_millis(5000));
        // AS INSTITUTION VALIDATE PROOF
        ::utils::devsetup::be_institution();
        proof::update_state(proof_req_handle);
        assert_eq!(proof::get_proof_state(proof_req_handle), ProofStateType::ProofValidated as u32);
        ::utils::devsetup::cleanup_dev_env("test_real_proof");
    }
}

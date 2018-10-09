extern crate serde_json;

use std::collections::HashMap;
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

use credential_def::{ retrieve_credential_def };
use schema::{ LedgerSchema };

use utils::libindy::anoncreds;
use utils::libindy::crypto;
use utils::serde_utils;

use settings;
use utils::httpclient;
use utils::constants::{ DEFAULT_SERIALIZE_VERSION, CREDS_FROM_PROOF_REQ, DEFAULT_GENERATED_PROOF };

use serde_json::{Value};

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
            proof: None,
            link_secret_alias: settings::DEFAULT_LINK_SECRET_ALIAS.to_string(),
            their_did: None,
            their_vk: None,
            agent_did: None,
            agent_vk: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisclosedProof {
    source_id: String,
    my_did: Option<String>,
    my_vk: Option<String>,
    state: VcxStateType,
    proof_request: Option<ProofRequestMessage>,
    proof: Option<ProofMessage>,
    link_secret_alias: String,
    their_did: Option<String>,
    their_vk: Option<String>,
    agent_did: Option<String>,
    agent_vk: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCreds {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CredInfo {
    pub referent: String,
    pub schema_id: String,
    pub cred_def_id: String,
}

fn credential_def_identifiers(credentials: &str) -> Result<Vec<(String, String, String, String)>, ProofError> {
    let mut rtn = Vec::new();

    let credentials: Value = serde_json::from_str(credentials)
        .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

    if let Value::Object(ref attrs) = credentials["attrs"] {
        for (requested_attr, value) in attrs {
            if let Some(ref attr_obj) = value.get("cred_info") {
                rtn.push((
                    requested_attr.to_string(),
                    serde_utils::get_value_to_string("referent", attr_obj)
                        .map_err(|e| ProofError::CommonError(e))?,
                    serde_utils::get_value_to_string("schema_id", attr_obj)
                             .map_err(|e| ProofError::CommonError(e))?,
                    serde_utils::get_value_to_string("cred_def_id", attr_obj)
                             .map_err(|e| ProofError::CommonError(e))?
                 ));
            }
        }
    }
    Ok(rtn)
}

impl DisclosedProof {

    fn set_proof_request(&mut self, req: ProofRequestMessage) {self.proof_request = Some(req)}

    fn get_state(&self) -> u32 {self.state as u32}
    fn set_state(&mut self, state: VcxStateType) {self.state = state}

    fn retrieve_credentials(&self) -> Result<String, ProofError> {
        if settings::test_indy_mode_enabled() {return Ok(CREDS_FROM_PROOF_REQ.to_string())}

        let proof_req = self.proof_request.as_ref().ok_or(ProofError::ProofNotReadyError())?;
        let indy_proof_req = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        anoncreds::libindy_prover_get_credentials_for_proof_req(&indy_proof_req)
            .map_err(|err| ProofError::CommonError(err))
    }

    fn _find_schemas(&self, credentials_identifiers: &Vec<(String, String, String, String)>) -> Result<String, ProofError> {
        if credentials_identifiers.len() == 0 { return Ok("{}".to_string()); }

        let mut rtn: HashMap<String, Value> = HashMap::new();

        for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {
            if !rtn.contains_key(schema_id) {
                let schema = LedgerSchema::new_from_ledger(schema_id).or( Err(ProofError::InvalidSchema()))?;
                let schema_json = serde_json::from_str(&schema.schema_json).or(Err(ProofError::InvalidSchema()))?;
                rtn.insert(schema_id.to_owned(), schema_json);
            }
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&rtn)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }
    }

    fn _find_credential_def(&self, credentials_identifiers: &Vec<(String, String, String, String)>) -> Result<String, ProofError> {
        if credentials_identifiers.len() == 0 { return Ok("{}".to_string()); }

        let mut rtn: HashMap<String, Value> = HashMap::new();

        for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {
            if !rtn.contains_key(cred_def_id) {
                let (_, credential_def) = retrieve_credential_def(cred_def_id)
                    .or(Err(ProofError::InvalidCredData()))?;
                let credential_def = serde_json::from_str(&credential_def)
                    .or(Err(ProofError::InvalidCredData()))?;
                rtn.insert(cred_def_id.to_owned(), credential_def);
            }
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&rtn)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }

    }

    fn _build_requested_credentials(&self,
                                    credentials_identifiers: &Vec<(String, String, String, String)>,
                                    self_attested_attrs: &str) -> Result<String, ProofError> {
        let mut rtn: Value = json!({
              "self_attested_attributes":{},
              "requested_attributes":{},
              "requested_predicates":{}
        });
        //Todo: need to do same for predicates and self_attested
        //Todo: need to handle if the attribute is not revealed
        if let Value::Object(ref mut map) = rtn["requested_attributes"] {
            for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {

                let insert_val = json!({"cred_id": cred_uuid, "revealed": true});
                map.insert(attr_id.to_owned(), insert_val);
            }
        }

        let self_attested_attrs: Value = serde_json::from_str(self_attested_attrs)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;
        rtn["self_attested_attributes"] = self_attested_attrs;

        let rtn = serde_json::to_string(&rtn)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        Ok(rtn)
    }

    fn generate_proof(&mut self, credentials: &str, self_attested_attrs: &str) -> Result<u32, ProofError> {
        debug!("generating proof {}", self.source_id);
        if settings::test_indy_mode_enabled() {return Ok(error::SUCCESS.code_num)}

        let proof_req = self.proof_request.as_ref()
            .ok_or(ProofError::CreateProofError())?;
        let proof_req_data_json = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;


        let credentials_identifiers = credential_def_identifiers(credentials)?;
        let requested_credentials = self._build_requested_credentials(&credentials_identifiers,
                                                                      self_attested_attrs)?;
        let schemas = self._find_schemas(&credentials_identifiers)?;
        let credential_defs_json = self._find_credential_def(&credentials_identifiers)?;
        let revoc_regs_json = Some("{}");
        let proof = anoncreds::libindy_prover_create_proof(&proof_req_data_json,
                                                           &requested_credentials,
                                                          &self.link_secret_alias,
                                                           &schemas,
                                                          &credential_defs_json,
                                                          revoc_regs_json).map_err(|ec| ProofError::CommonError(ec))?;
        let mut proof_msg = ProofMessage::new();
        proof_msg.libindy_proof = proof;
        self.proof = Some(proof_msg);

        Ok(error::SUCCESS.code_num)
    }

    fn send_proof(&mut self, connection_handle: u32) -> Result<u32, ProofError> {
        debug!("sending proof {} via connection: {}", self.source_id, connection::get_source_id(connection_handle).unwrap_or_default());
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
                let proof: &ProofMessage = self.proof.as_ref().ok_or(ProofError::CreateProofError())?;
                serde_json::to_string(&proof).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?
            },
            true => DEFAULT_GENERATED_PROOF.to_string(),
        };

        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &proof, "PROOF")
            .or(Err(ProofError::ProofConnectionError()))?;

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
    fn to_string(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }
    fn from_str(s: &str) -> Result<DisclosedProof, ProofError> {
        let s:Value = serde_json::from_str(&s)
            .or(Err(ProofError::InvalidJson()))?;
        let proof: DisclosedProof= serde_json::from_value(s["data"].clone())
            .or(Err(ProofError::InvalidJson()))?;
        Ok(proof)
    }
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

pub fn create_proof(source_id: &str, proof_req: &str) -> Result<u32, ProofError> {
    debug!("creating disclosed proof with id: {}", source_id);

    let mut new_proof: DisclosedProof = Default::default();

    new_proof.set_source_id(source_id);
    new_proof.set_proof_request(serde_json::from_str(proof_req)
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
        Ok(DisclosedProof::to_string(&obj))
    })
}

pub fn from_string(proof_data: &str) -> Result<u32, ProofError> {
    let derived_proof: DisclosedProof = match DisclosedProof::from_str(proof_data) {
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

pub fn release_all() {
    match HANDLE_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}

pub fn send_proof(handle: u32, connection_handle: u32) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.send_proof(connection_handle).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn generate_proof(handle: u32, credentials: String, self_attested_attrs: String) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.generate_proof(&credentials, &self_attested_attrs).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn retrieve_credentials(handle: u32) -> Result<String, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.retrieve_credentials().map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

//TODO one function with credential
pub fn get_proof_request(connection_handle: u32, msg_id: &str) -> Result<String, ProofError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec()); }

    let message = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 Some(vec![msg_id.to_string()])).map_err(|ec| ProofError::CommonError(ec))?;

    if message[0].msg_type.eq("proofReq") {
        let (_, msg_data) = match message[0].payload {
            Some(ref data) => {
                let data = to_u8(data);
                crypto::parse_msg(&my_vk, data.as_slice()).map_err(|ec| ProofError::CommonError(ec))?
            },
            None => return Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num))
        };

        let request = extract_json_payload(&msg_data).map_err(|ec| ProofError::CommonError(ec))?;
        let mut request: ProofRequestMessage = serde_json::from_str(&request)
           .or(Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num)))?;

        request.msg_ref_id = Some(message[0].uid.to_owned());
        Ok(serde_json::to_string_pretty(&request).or(Err(ProofError::InvalidJson()))?)
    } else {
        Err(ProofError::CommonError(error::INVALID_MESSAGES.code_num))
    }
}

//TODO one function with credential
pub fn get_proof_request_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, ProofError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec()); }

    let payload = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 None).map_err(|ec| ProofError::CommonError(ec))?;

    let mut messages: Vec<ProofRequestMessage> = Default::default();

    for msg in payload {
        if msg.sender_did.eq(&my_did){ continue; }

        if msg.msg_type.eq("proofReq") {
            let (_, msg_data) = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(&my_vk, data.as_slice())
                        .map_err(|ec| ProofError::CommonError(ec))?
                },
                None => return Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num))
            };

            let req = extract_json_payload(&msg_data).map_err(|ec| ProofError::CommonError(ec))?;

            let mut req: ProofRequestMessage = serde_json::from_str(&req)
                .or(Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num)))?;

            req.msg_ref_id = Some(msg.uid.to_owned());
            messages.push(req);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).or(Err(ProofError::InvalidJson()))?)
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
    use utils::constants::{ ADDRESS_CRED_ID, LICENCE_CRED_ID, ADDRESS_SCHEMA_ID, ADDRESS_CRED_DEF_ID, CRED_DEF_ID, SCHEMA_ID };
    use serde_json::Value;

    #[test]
    fn test_create_proof() {
        init!("true");
        assert!(create_proof("1", ::utils::constants::PROOF_REQUEST_JSON).unwrap() > 0);
    }

    #[test]
    fn test_create_fails() {
        init!("true");
        assert_eq!(create_proof("1","{}").err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_proof_cycle() {
        init!("true");

        let connection_h = connection::build_connection("test_send_credential_offer").unwrap();

        let requests = get_proof_request_messages(connection_h, None).unwrap();
        let requests:Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();

        let handle = create_proof("TEST_CREDENTIAL", &requests).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap());
        send_proof(handle, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(handle).unwrap());
    }

    #[test]
    fn get_state_test(){
        init!("true");
        let proof: DisclosedProof =  Default::default();
        assert_eq!(VcxStateType::VcxStateNone as u32, proof.get_state());
        let handle = create_proof("id",::utils::constants::PROOF_REQUEST_JSON).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap())
    }

    #[test]
    fn to_string_test() {
        init!("true");
        let handle = create_proof("id",::utils::constants::PROOF_REQUEST_JSON).unwrap();
        let serialized = to_string(handle).unwrap();
        let j:Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(j["version"], "1.0");
        DisclosedProof::from_str(&serialized).unwrap();
    }

    #[test]
    fn test_deserialize_fails() {
        assert_eq!(from_string("{}").err(),
        Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_find_schemas() {
        init!("true");
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];

        let proof: DisclosedProof = Default::default();
        let schemas = proof._find_schemas(&creds).unwrap();
        assert!(schemas.len() > 0);
        assert!(schemas.contains(r#""id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","name":"test-licence""#));
    }

    #[test]
    fn test_find_schemas_fails() {
        init!("false");

        let mut credential_ids = Vec::new();
        credential_ids.push(("1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()));
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_schemas(&credential_ids).err(),
                   Some(ProofError::InvalidSchema()));
    }

    #[test]
    fn test_find_credential_def() {
        init!("true");
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];

        let proof: DisclosedProof = Default::default();
        let credential_def = proof._find_credential_def(&creds).unwrap();
        assert!(credential_def.len() > 0);
        assert!(credential_def.contains(r#""id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471","schemaId":"2471""#));
    }

    #[test]
    fn test_find_credential_def_fails() {
        init!("false");

        let mut credential_ids = Vec::new();
        credential_ids.push(("1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()));
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_credential_def(&credential_ids).err(),
                   Some(ProofError::InvalidCredData()));
    }

    #[test]
    fn test_build_requested_credentials() {
        init!("true");
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];
        let self_attested_attrs = json!({
            "self_attested_attr_3": "my self attested 1",
            "self_attested_attr_4": "my self attested 2",
        }).to_string();

        let test: Value = json!({
              "self_attested_attributes":{
                  "self_attested_attr_3": "my self attested 1",
                  "self_attested_attr_4": "my self attested 2",
              },
              "requested_attributes":{
                  "height_1": {"cred_id": LICENCE_CRED_ID, "revealed": true },
                  "zip_2": {"cred_id": ADDRESS_CRED_ID, "revealed": true },
              },
              "requested_predicates":{}
        });

        let proof: DisclosedProof = Default::default();
        let requested_credential = proof._build_requested_credentials(&creds, &self_attested_attrs).unwrap();
        assert_eq!(test.to_string(), requested_credential);
    }

    #[test]
    fn test_get_proof_request() {
        init!("true");

        let connection_h = connection::build_connection("test_get_proof_request").unwrap();

        let request = get_proof_request(connection_h, "123").unwrap();
        assert!(request.len() > 50);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieve_credentials() {
        init!("ledger");
        ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let (_, _, req, _) = ::utils::libindy::anoncreds::tests::create_proof();

        let mut proof_req = ProofRequestMessage::create();
        let mut proof: DisclosedProof = Default::default();
        proof_req.proof_request_data = serde_json::from_str(&req).unwrap();
        proof.proof_request = Some(proof_req);

        let retrieved_creds = proof.retrieve_credentials().unwrap();
        assert!(retrieved_creds.len() > 500);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_case_for_proof_req_doesnt_matter_for_retrieve_creds() {
        init!("ledger");
        ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let mut req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "zip_1": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": did })]
               })
           }),
           "requested_predicates": json!({}),
        });

        let mut proof_req = ProofRequestMessage::create();
        let mut proof: DisclosedProof = Default::default();
        proof_req.proof_request_data = serde_json::from_str(&req.to_string()).unwrap();
        proof.proof_request = Some(proof_req.clone());

        // All lower case
        let retrieved_creds = proof.retrieve_credentials().unwrap();
        assert!(retrieved_creds.contains(r#""zip":"84000""#));
        let ret_creds_as_value:Value = serde_json::from_str(&retrieved_creds).unwrap();
        assert_eq!(ret_creds_as_value["attrs"]["zip_1"][0]["cred_info"]["attrs"]["zip"], "84000");
        // First letter upper
        req["requested_attributes"]["zip_1"]["name"] = json!("Zip");
        proof_req.proof_request_data = serde_json::from_str(&req.to_string()).unwrap();
        proof.proof_request = Some(proof_req.clone());
        let retrieved_creds2 = proof.retrieve_credentials().unwrap();
        assert!(retrieved_creds2.contains(r#""zip":"84000""#));

        //entire word upper
        req["requested_attributes"]["zip_1"]["name"] = json!("ZIP");
        proof_req.proof_request_data = serde_json::from_str(&req.to_string()).unwrap();
        proof.proof_request = Some(proof_req.clone());
        let retrieved_creds3 = proof.retrieve_credentials().unwrap();
        assert!(retrieved_creds3.contains(r#""zip":"84000""#));
    }

    #[test]
    fn test_retrieve_credentials_fails_with_no_proof_req() {
        init!("false");

        let proof: DisclosedProof = Default::default();
        assert_eq!(proof.retrieve_credentials(), Err(ProofError::ProofNotReadyError()));
    }

    #[test]
    fn test_credential_def_identifiers() {
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let selected_credentials : Value = json!({
           "attrs":{
              "height_1":{
                "cred_info":{
                   "referent":LICENCE_CRED_ID,
                   "attrs":{
                      "sex":"male",
                      "age":"111",
                      "name":"Bob",
                      "height":"4'11"
                   },
                   "schema_id": SCHEMA_ID,
                   "cred_def_id": CRED_DEF_ID,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
              },
              "zip_2":{
                "cred_info":{
                   "referent":ADDRESS_CRED_ID,
                   "attrs":{
                      "address1":"101 Tela Lane",
                      "address2":"101 Wilson Lane",
                      "zip":"87121",
                      "state":"UT",
                      "city":"SLC"
                   },
                   "schema_id":ADDRESS_SCHEMA_ID,
                   "cred_def_id":ADDRESS_CRED_DEF_ID,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
             }
           },
           "predicates":{

           }
        });
        let creds = credential_def_identifiers(&selected_credentials.to_string()).unwrap();
        assert_eq!(creds, vec![cred1, cred2]);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_generate_proof() {
        init!("ledger");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, _, cred_def_id, _, _, _, _, cred_id) = ::utils::libindy::anoncreds::tests::create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS);

        let mut proof_req = ProofRequestMessage::create();
        let indy_proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "self_attested_attr_3": json!({
                   "name":"self_attested_attr",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        proof_req.proof_request_data = serde_json::from_str(&indy_proof_req).unwrap();

        let selected_credentials : Value = json!({
           "attrs":{
              "address1_1":{
                "cred_info":{
                   "referent":cred_id,
                   "attrs":{
                      "sex":"male",
                      "age":"111",
                      "name":"Bob",
                      "height":"4'11"
                   },
                   "schema_id": schema_id,
                   "cred_def_id": cred_def_id,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
              },
              "zip_2":{
                "cred_info":{
                   "referent":cred_id,
                   "attrs":{
                      "address1":"101 Tela Lane",
                      "address2":"101 Wilson Lane",
                      "zip":"87121",
                      "state":"UT",
                      "city":"SLC"
                   },
                   "schema_id":schema_id,
                   "cred_def_id":cred_def_id,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
             }
           },
           "predicates":{ }
        });

        let self_attested: Value = json!({
              "self_attested_attr_3":"attested_val"
        });

        let mut proof: DisclosedProof = Default::default();
        proof.proof_request = Some(proof_req);
        proof.link_secret_alias = "main".to_string();
        let generated_proof = proof.generate_proof(&selected_credentials.to_string(), &self_attested.to_string());

        assert!(generated_proof.is_ok());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_generate_self_attested_proof() {
        init!("ledger");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let mut proof_req = ProofRequestMessage::create();
        let indy_proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
               }),
               "zip_2": json!({
                   "name":"zip",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        proof_req.proof_request_data = serde_json::from_str(&indy_proof_req).unwrap();

        let selected_credentials : Value = json!({});

        let self_attested: Value = json!({
              "address1_1":"attested_address",
              "zip_2": "attested_zip"
        });

        let mut proof: DisclosedProof = Default::default();
        proof.proof_request = Some(proof_req);
        proof.link_secret_alias = "main".to_string();
        let generated_proof = proof.generate_proof(&selected_credentials.to_string(), &self_attested.to_string());

        assert!(generated_proof.is_ok());
    }
}

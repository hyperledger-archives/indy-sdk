extern crate rand;
extern crate serde_json;
extern crate libc;
extern crate openssl;

use self::openssl::bn::{ BigNum, BigNumRef };
use settings;
use connection;
use rand::Rng;
use api::{ VcxStateType, ProofStateType };
use std::sync::Mutex;
use std::collections::HashMap;
use messages::proofs::proof_message::{ProofMessage, ClaimData };
use messages;
use messages::proofs::proof_request::{ ProofRequestMessage };
use messages::GeneralMessage;
use utils::httpclient;
use utils::error;
use utils::constants::*;
use utils::libindy::SigTypes;
use utils::libindy::anoncreds::libindy_verifier_verify_proof;
use claim_def::{ RetrieveClaimDef, ClaimDefCommon, ClaimDefinition };
use schema::{ LedgerSchema, SchemaTransaction };
use proof_compliance::{ proof_compliance };

lazy_static! {
    static ref PROOF_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Proof {
    source_id: String,
    handle: u32,
    requested_attrs: String,
    requested_predicates: String,
    msg_uid: String,
    ref_msg_id: String,
    prover_did: String,
    prover_vk: String,
    state: VcxStateType,
    proof_state: ProofStateType,
    name: String,
    version: String,
    nonce: String,
    proof: Option<ProofMessage>,
    proof_request: Option<ProofRequestMessage>,
    remote_did: String,
    remote_vk: String,
    agent_did: String,
    agent_vk: String,
}

impl Proof {
    fn validate_proof_request(&self) -> Result<u32, u32> {
        //TODO: validate proof request
        debug!("successfully validated proof request {}", self.handle);
        Ok(error::SUCCESS.code_num)
    }

    fn validate_proof_indy(&mut self,
                           proof_req_json: &str,
                           proof_json: &str,
                           schemas_json: &str,
                           claim_defs_json: &str,
                           revoc_regs_json: &str) -> Result<u32, u32> {
        if settings::test_indy_mode_enabled() {return Ok(error::SUCCESS.code_num);}

        debug!("starting libindy proof verification");
        let valid = libindy_verifier_verify_proof(proof_req_json,
                                                  proof_json,
                                                  schemas_json,
                                                  claim_defs_json,
                                                  revoc_regs_json).map_err(|err| {
                error!("Error: {}, Proof {} wasn't valid", err, self.handle);
                self.proof_state = ProofStateType::ProofInvalid;
                error::INVALID_PROOF.code_num
        })?;

        if !valid {
            warn!("indy returned false when validating proof");
            self.proof_state = ProofStateType::ProofInvalid;
            return Ok(error::SUCCESS.code_num)
        }
        debug!("Indy validated Proof: {:?}", self.handle);
        self.proof_state = ProofStateType::ProofValidated;
        Ok(error::SUCCESS.code_num)
    }

    fn build_claim_defs_json(&mut self, claim_data:&Vec<ClaimData>) -> Result<String, u32> {
        debug!("building claimdef json for proof validation");
        let mut claim_json: HashMap<String, ClaimDefinition> = HashMap::new();
        for claim in claim_data.iter() {
            let schema_seq_no = match claim.schema_seq_no {
                Some(x) => x,
                None => return Err(error::INVALID_CLAIM_DEF_JSON.code_num)
            };
            let issuer_did = match claim.issuer_did {
                Some(ref x) => x,
                None => return Err(error::INVALID_CLAIM_DEF_JSON.code_num)
            };
            let claim_uuid = match claim.claim_uuid {
                Some(ref x) => x,
                None => return Err(error::INVALID_CLAIM_DEF_JSON.code_num)
            };
            let claim_def = RetrieveClaimDef::new()
                .retrieve_claim_def("GGBDg1j8bsKmr4h5T9XqYf", schema_seq_no, Some(SigTypes::CL), issuer_did)?;
            let claim_obj: ClaimDefinition = serde_json::from_str(&claim_def)
                .map_err(|err| {
                    error!("Invalid json format for ClaimDefinition.");
                    error::INVALID_JSON.code_num
                })?;
            claim_json.insert(claim_uuid.to_string(), claim_obj);
        }

        serde_json::to_string(&claim_json).map_err(|err| {
            warn!("{} with serde error: {}",error::INVALID_CLAIM_DEF_JSON.message, err);
            error::INVALID_CLAIM_DEF_JSON.code_num
        })
    }

    fn build_proof_json(&mut self) -> Result<String, u32> {
        debug!("building proof json for proof validation");
        match self.proof {
            Some(ref x) => x.to_string(),
            None => Err(error::INVALID_PROOF.code_num),
        }
    }

    fn build_schemas_json(&mut self, claim_data:&Vec<ClaimData>) -> Result<String, u32> {
        debug!("building schemas json for proof validation");

        let mut schema_json: HashMap<String, SchemaTransaction> = HashMap::new();
        for schema in claim_data.iter() {
            let schema_seq_no = match schema.schema_seq_no {
                Some(x) => x,
                None => return Err(error::INVALID_SCHEMA.code_num)
            };
            let claim_uuid = match schema.claim_uuid {
                Some(ref x) => x,
                None => return Err(error::INVALID_SCHEMA.code_num)
            };
            let schema_obj = LedgerSchema::new_from_ledger(schema_seq_no as i32)?;
            let data = match schema_obj.data {
                Some(x) => x,
                None => return Err(error::INVALID_PROOF.code_num)
            };
            schema_json.insert(claim_uuid.to_string(), data);
        }

        serde_json::to_string(&schema_json).map_err(|err| {
            warn!("{} with serde error: {}",error::INVALID_SCHEMA.message, err);
            error::INVALID_SCHEMA.code_num
        })
    }

    fn build_proof_req_json(&mut self) -> Result<String, u32> {
        debug!("building proof request json for proof validation");
        match self.proof_request {
            Some(ref mut x) => {
                Ok(x.get_proof_request_data())
            },
            None => Err(error::INVALID_PROOF.code_num)
        }
    }

    fn proof_validation(&mut self) -> Result<u32, u32> {
        let proof_req_msg = match self.proof_request.clone() {
            Some(x) => x,
            None => return Err(error::INVALID_PROOF.code_num),
        };
        let proof_msg = match self.proof.clone() {
            Some(x) => x,
            None => return Err(error::INVALID_PROOF.code_num),
        };
        let claim_data = proof_msg.get_claim_info()?;

        if claim_data.len() == 0 {
            return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
        }
        let claim_def_msg = self.build_claim_defs_json(&claim_data)?;
        let schemas_json = self.build_schemas_json(&claim_data)?;
        let proof_json = self.build_proof_json()?;
        let proof_req_json = self.build_proof_req_json()?;
        debug!("*******\n{}\n********", claim_def_msg);
        debug!("*******\n{}\n********", schemas_json);
        debug!("*******\n{}\n********", proof_json);
        debug!("*******\n{}\n********", proof_req_json);
        proof_compliance(&proof_req_msg.proof_request_data, &proof_msg)?;
        self.validate_proof_indy(&proof_req_json, &proof_json, &schemas_json, &claim_def_msg, INDY_REVOC_REGS_JSON)
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("proof {} has invalid state {} for sending proofRequest", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }
        debug!("sending proof request with proof: {}, and connection {}", self.handle, connection_handle);
        self.prover_did = connection::get_pw_did(connection_handle)?;
        self.agent_did = connection::get_agent_did(connection_handle)?;
        self.agent_vk = connection::get_agent_verkey(connection_handle)?;
        self.remote_vk = connection::get_their_pw_verkey(connection_handle)?;
        self.prover_vk = connection::get_pw_verkey(connection_handle)?;

        debug!("prover_did: {} -- agent_did: {} -- agent_vk: {} -- remote_vk: {} -- prover_vk: {}",
               self.prover_did,
               self.agent_did,
               self.agent_vk,
               self.remote_vk,
               self.prover_vk);

        let data_version = "0.1";
        let mut proof_obj = messages::proof_request();
        let proof_request = proof_obj
            .type_version(&self.version)
            .tid(1)
            .mid(9)
            .nonce(&self.nonce)
            .proof_name(&self.name)
            .proof_data_version(data_version)
            .requested_attrs(&self.requested_attrs)
            .requested_predicates(&self.requested_predicates)
            .serialize_message()?;

        self.proof_request = Some(proof_obj);
        let data = connection::generate_encrypted_payload(&self.prover_vk, &self.remote_vk, &proof_request, "PROOF_REQUEST")?;
        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_CLAIM_OFFER_RESPONSE.to_vec()); }
        let title = format!("{} wants you to share {}", settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME).unwrap(), self.name);

        match messages::send_message().to(&self.prover_did)
            .to_vk(&self.prover_vk)
            .msg_type("proofReq")
            .agent_did(&self.agent_did)
            .set_title(&title)
            .set_detail(&title)
            .agent_vk(&self.agent_vk)
            .edge_agent_payload(&data)
            .send_secure() {
            Ok(response) => {
                self.msg_uid = get_proof_details(&response[0])?;
                self.state = VcxStateType::VcxStateOfferSent;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proofReq: {}", x);
                return Err(x);
            }
        }
    }

    fn get_proof(&self) -> Result<String, u32> {
        let proof = match self.proof {
            Some(ref x) => x,
            None => return Err(error::INVALID_PROOF.code_num),
        };
        proof.get_proof_attributes()
    }

    fn get_proof_request_status(&mut self) -> Result<u32, u32> {
        debug!("updating state for proof {}", self.handle);
        if self.state == VcxStateType::VcxStateAccepted {
            return Ok(error::SUCCESS.code_num);
        }
        else if self.state != VcxStateType::VcxStateOfferSent || self.msg_uid.is_empty() || self.prover_did.is_empty() {
            return Ok(error::SUCCESS.code_num);
        }

        let payload = messages::get_message::get_ref_msg(&self.msg_uid, &self.prover_did, &self.prover_vk, &self.agent_did, &self.agent_vk)?;

        self.proof = match parse_proof_payload(&payload) {
            Err(_) => return Ok(error::SUCCESS.code_num),
            Ok(x) => Some(x),
        };

        self.state = VcxStateType::VcxStateAccepted;

        match self.proof_validation() {
            Ok(x) => {
                if self.proof_state != ProofStateType::ProofInvalid {
                    debug!("Proof format was validated for proof {}", self.handle);
                    self.proof_state = ProofStateType::ProofValidated;
                }
            }
            Err(x) => {
                if x == error::TIMEOUT_LIBINDY_ERROR.code_num {
                    warn!("Proof {} unable to be validated", self.handle);
                    self.proof_state = ProofStateType::ProofUndefined;
                } else {
                    warn!("Proof {} had invalid format with err {}", self.handle, x);
                    self.proof_state = ProofStateType::ProofInvalid;
                }
            }
        };

        Ok(error::SUCCESS.code_num)
    }

    fn update_state(&mut self) {
        self.get_proof_request_status().unwrap_or(error::SUCCESS.code_num);
    }

    fn get_state(&self) -> u32 {let state = self.state as u32; state}

    fn get_proof_state(&self) -> u32 {let state = self.proof_state as u32; state}

    fn get_proof_uuid(&self) -> String { self.msg_uid.clone() }

}

pub fn create_proof(source_id: Option<String>,
                    requested_attrs: String,
                    requested_predicates: String,
                    name: String) -> Result<u32, u32> {

    let new_handle = rand::thread_rng().gen::<u32>();
    debug!("creating proof with name: {}, requested_attrs: {}, requested_predicates: {}", name, requested_attrs, requested_predicates);

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        ref_msg_id: String::new(),
        requested_attrs,
        requested_predicates,
        prover_did: String::new(),
        prover_vk: String::new(),
        state: VcxStateType::VcxStateNone,
        proof_state: ProofStateType::ProofUndefined,
        name,
        version: String::from("1.0"),
        nonce: generate_nonce()?,
        proof: None,
        proof_request: None,
        remote_did: String::new(),
        remote_vk: String::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
    });

    new_proof.validate_proof_request()?;

    new_proof.state = VcxStateType::VcxStateInitialized;

    {
        let mut m = PROOF_MAP.lock().unwrap();
        debug!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, new_proof);
    }

    Ok(new_handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn update_state(handle: u32) {
    match PROOF_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => VcxStateType::VcxStateNone as u32,
    }
}

pub fn get_proof_state(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_proof_state(),
        None => VcxStateType::VcxStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_PROOF_HANDLE.code_num,
    }
}

pub fn release_all() {
    let mut map = PROOF_MAP.lock().unwrap();

    map.drain();
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num)
    }
}

pub fn from_string(proof_data: &str) -> Result<u32, u32> {
    let derived_proof: Proof = serde_json::from_str(proof_data).map_err(|err| {
        warn!("{} with serde error: {}",error::INVALID_JSON.message, err);
        error::INVALID_JSON.code_num
    })?;
    let new_handle = derived_proof.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let proof = Box::from(derived_proof);

    {
        let mut m = PROOF_MAP.lock().unwrap();
        debug!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, proof);
    }
    Ok(new_handle)
}

pub fn send_proof_request(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match PROOF_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => Ok(c.send_proof_request(connection_handle)?),
        None => Err(error::INVALID_PROOF_HANDLE.code_num),
    }
}

fn get_proof_details(response: &str) -> Result<String, u32> {
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = match json["uid"].as_str() {
                Some(x) => x,
                None => {
                    warn!("response had no uid");
                    return Err(error::INVALID_JSON.code_num)
                },
            };
            Ok(String::from(detail))
        },
        Err(_) => {
            warn!("Proof called without a valid response from server");
            Err(error::INVALID_JSON.code_num)
        },
    }
}

pub fn get_proof_uuid(handle: u32) -> Result<String,u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(proof) => Ok(proof.get_proof_uuid()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num),
    }
}

fn parse_proof_payload(payload: &Vec<u8>) -> Result<ProofMessage, u32> {
    debug!("parsing proof payload: {:?}", payload);
    let data = messages::extract_json_payload(payload)?;

    let my_claim_req = ProofMessage::from_str(&data).map_err(|err| {
        warn!("invalid json {}", err);
        error::INVALID_JSON.code_num
    })?;
    Ok(my_claim_req)
}

pub fn get_proof(handle: u32) -> Result<String,u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(proof) => Ok(proof.get_proof()?),
        None => Err(error::INVALID_PROOF.code_num),
    }
}

pub fn generate_nonce() -> Result<String, u32> {
    let mut bn = BigNum::new().map_err(|err| error::BIG_NUMBER_ERROR.code_num)?;

    BigNumRef::rand(&mut bn, LARGE_NONCE as i32, openssl::bn::MSB_MAYBE_ZERO, false)
        .map_err(|_| error::BIG_NUMBER_ERROR.code_num)?;
    Ok(bn.to_dec_str().map_err(|err| error::BIG_NUMBER_ERROR.code_num)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use connection::build_connection;
    use utils::libindy::pool;
    use messages::proofs::proof_message::{Attr};

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = r#"[{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":1,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB"},{"attr_name":"num","p_type":"LE","value":99,"schema_seq_no":1,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB"}]"#;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_create_proof_succeeds() {
        set_default_and_enable_test_mode();

        create_proof(None,
                     REQUESTED_ATTRS.to_owned(),
                     REQUESTED_PREDICATES.to_owned(),
                     "Optional".to_owned()).unwrap();
    }

    #[test]
    fn test_nonce() {
        let nonce = generate_nonce().unwrap();
        assert!(BigNum::from_dec_str(&nonce).unwrap().num_bits() < 81)
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = create_proof(None,
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        let proof_string = to_string(handle).unwrap();
        assert!(!proof_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = create_proof(None,
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        let proof_data = to_string(handle).unwrap();
        assert!(!proof_data.is_empty());
        release(handle);
        let new_handle = from_string(&proof_data).unwrap();
        let new_proof_data = to_string(new_handle).unwrap();
        assert_eq!(new_handle, handle);
        assert_eq!(new_proof_data, proof_data);
    }

    #[test]
    fn test_release_proof() {
        set_default_and_enable_test_mode();
        let handle = create_proof(Some("1".to_string()),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert_eq!(release(handle), 0);
        assert!(!is_valid_handle(handle));
    }

    #[test]
    fn test_send_proof_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_proof_request".to_owned()).unwrap();
        connection::set_agent_verkey(connection_handle, VERKEY);
        connection::set_agent_did(connection_handle, DID);
        connection::set_their_pw_verkey(connection_handle, VERKEY);

        let handle = create_proof(Some("1".to_string()),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert_eq!(send_proof_request(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_proof_uuid(handle).unwrap(), "ntc2ytb");
    }


    #[test]
    fn test_send_proof_request_fails_with_no_pw() {
        //This test has 2 purposes:
        //1. when send_proof_request fails, Ok(c.send_proof_request(connection_handle)?) returns error instead of Ok(_)
        //2. Test that when no PW connection exists, send message fails on invalid did
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_proof_request".to_owned()).unwrap();
        connection::set_pw_did(connection_handle, "");

        let handle = create_proof(Some("1".to_string()),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        match send_proof_request(handle, connection_handle) {
            Ok(x) => panic!("Should have failed in send_proof_request"),
            Err(y) => assert_eq!(y, error::INVALID_DID.code_num)
        }
    }

    #[test]
    fn test_get_proof_fails_with_no_proof() {
        set_default_and_enable_test_mode();
        let handle = create_proof(Some("1".to_string()),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert!(is_valid_handle(handle));

        match get_proof(handle) {
            Ok(x) => {
                warn!("Should have failed with no proof");
                assert_eq!(0, 1)
            },
            Err(x) => assert_eq!(x, error::INVALID_PROOF.code_num),
        }
    }

    #[test]
    fn test_update_state_with_pending_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_proof_request".to_owned()).unwrap();

        let new_handle = 1;
        let mut proof = Box::new(Proof {
            handle: new_handle,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: String::from("[]"),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            proof_state: ProofStateType::ProofUndefined,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: None,
            proof_request: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        });

        httpclient::set_next_u8_response(PROOF_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_PROOF_RESPONSE.to_vec());

        proof.update_state();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_get_proof_returns_proof_when_proof_state_invalid() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_proof_request".to_owned()).unwrap();

        let proof_msg = r#"{"msg_type":"proof","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::e5fec91f-d03d-4513-813c-ab6db5715d55":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"22605045280481376895214546474258256134055560453004805058368015338423404000586901936329279496160366852115900235316791489357953785379851822281248296428005020302405076144264617943389810572564188437603815231794326272302243703078443007359698858400857606408856314183672828086906560155576666631125808137726233827430076624897399072853872527464581329767287002222137559918765406079546649258389065217669558333867707240780369514832185660287640444094973804045885379406641474693993903268791773620198293469768106363470543892730424494655747935463337367735239405840517696064464669905860189004121807576749786474060694597244797343224031","e":"70192089123105616042684481760592174224585053817450673797400202710878562748001698340846985261463026529360990669802293480312441048965520897","v":"1148619141217957986496757711054111791862691178309410923416837802801708689012670430650138736456223586898110113348220116209094530854607083005898964558239710027534227973983322542548800291320747321452329327824406430787211689678096549398458892087551551587767498991043777397791000822007896620414888602588897806008609113730393639807814070738699614969916095861363383223421727858670289337712185089527052065958362840287749622133424503902085247641830693297082507827948006947829401008622239294382186995101394791468192083810475776455445579931271665980788474331866572497866962452476638881287668931141052552771328556458489781734943404258692308937784221642452132005267809852656378394530342203469943982066011466088478895643800295937901139711103301249691253510784029114718919483272055970725860849610885050165709968510696738864528287788491998027072378656038991754015693216663830793243584350961586874315757599094357535856429087122365865868729","m":{"address2":"11774234640096848605908744857306447015748098256395922562149769943967941106193320512788344020652220849708117081570187385467979956319507248530701654682748372348387275979419669108338","city":"4853213962270369118453000522408430296589146124488849630769837449684434138367659379663124155088827069418193027370932024893343033367076071757003149452226758383807126385017161888440","address1":"12970590675851114145396120869959510754345567924518524026685086869487243290925032320159287997675756075512889990901552679591155319959039145119122576164798225386578339739435869622811","zip":"8333721522340131864419931745588776943042067606218561135102011966361165456174036379901390244538991611895455576519950813910672825465382312504250936740379785802177629077591444977329"},"m1":"92853615502250003546205004470333326341901175168428906399291824325990659330595200000112546157141090642053863739870044907457400076448073272490169488870502566172795456430489790324815765612798273406119873266684053517977802902202155082987833343670942161987285661291655743810590661447300059024966135828466539810035","m2":"14442362430453309930284822850357071315613831915865367971974791350454381198894252834180803515368579729220423713315556807632571621646127926114010380486713602821529657583905131582938"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"68430476900085482958838239880418115228681348197588159723604944078288347793331","c_list":[[179,17,2,242,194,227,92,203,28,32,255,113,112,20,5,243,9,111,220,111,21,210,116,12,167,119,253,181,37,40,143,215,140,42,179,97,75,229,96,94,54,248,206,3,48,14,61,219,160,122,139,227,166,183,37,43,197,200,28,220,217,10,65,42,6,195,124,44,164,65,114,206,51,231,254,156,170,141,21,153,50,251,237,65,147,97,243,17,157,116,213,201,80,119,106,70,88,60,55,36,33,160,135,106,60,212,191,235,116,57,78,177,61,86,44,226,205,100,134,118,93,6,26,58,220,66,232,166,202,62,90,174,231,207,19,239,233,223,70,191,199,100,157,62,139,176,28,184,9,70,116,199,142,237,198,183,12,32,53,84,207,202,77,56,97,177,154,169,223,201,212,163,212,101,184,255,215,167,16,163,136,44,25,123,49,15,229,41,149,133,159,86,106,208,234,73,207,154,194,162,141,63,159,145,94,47,174,51,225,91,243,2,221,202,59,11,212,243,197,208,116,42,242,131,221,137,16,169,203,215,239,78,254,150,42,169,202,132,172,106,179,130,178,130,147,24,173,213,151,251,242,44,54,47,208,223]]},"requested_proof":{"revealed_attrs":{"sdf":["claim::e5fec91f-d03d-4513-813c-ab6db5715d55","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}"#;
        let new_handle = 1;
        let mut proof = Box::new(Proof {
            handle: new_handle,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: String::from("[]"),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            proof_state: ProofStateType::ProofInvalid,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: Some(ProofMessage::from_str(&proof_msg).unwrap()),
            proof_request: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        });

        httpclient::set_next_u8_response(PROOF_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_PROOF_RESPONSE.to_vec());
        //httpclient::set_next_u8_response(GET_PROOF_OR_CLAIM_RESPONSE.to_vec());

        proof.update_state();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateAccepted as u32);
        assert_eq!(proof.get_proof_state(), ProofStateType::ProofInvalid as u32);
        assert_eq!(proof.prover_did, "GxtnGN6ypZYgEqcftSQFnC");
        /* converting proof to a string produces non-deterministic results */
    }

    #[test]
    fn test_build_claim_defs_json_with_multiple_claims() {
        let claim_result = r#"{"auditPath":["7hRA1eWgHDmqFfXQHmHLzCE1ZeXvvkq5VaJEpb6NWz74","4QvchQ6JGxvU57kyzHzKJvUV7rb12jpFX7FBP9LrN9qA","G14qswNCM1mxhRHPMLx4h5qmbLEDQkczjJUVUEedUGxQ","4B6hCrJc2TubiFE1rgxjM1Hj7zvTTjxkzo9Gikhy4MVZ"],"data":{"attr_names":["name","male"],"name":"name","version":"1.0"},"identifier":"VsKV7grR1BUE29mG2Fm2kX","reqId":1515795761424583710,"rootHash":"C98M4qjp4zzHw6APDWwGxTBHkEdAhjUQepi3Bxz2auna","seqNo":299,"signature":"4iFhpLknpRiCU6Axrj8HcFxMaxGaMmnzwJ1WMKndK653k4B7LYGZD2PNHEEGZQEBVXwhgDxPFe1t9bSzdVcEQ3eL","txnTime":1515795761,"type":"101"}"#;
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let data = r#"{"ref":1,"origin":"NcYxiDXkpYi6ov5FcYDi1e","signature_type":"CL","data":{"primary":{"n":"9","s":"8","rms":"7","r":{"height":"6","sex":"5","age":"4","name":"3"},"rctxt":"2","z":"1"},"revocation":null}}"#;
        let mut proof = Proof {
            handle: 1,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: String::from("[]"),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            proof_state: ProofStateType::ProofUndefined,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: None,
            proof_request: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        let claim1 = ClaimData {
            schema_seq_no: Some(1),
            issuer_did: Some("11".to_string()),
            claim_uuid: Some("claim1".to_string()),
            attr_info: Some(Attr {
                name: "claim1Name".to_string(),
                value: serde_json::to_value("val1").unwrap(),
                attr_type: "attr1".to_string(),
                predicate_type: None,
            })
        };
        let claim2 = ClaimData {
            schema_seq_no: Some(2),
            issuer_did: Some("22".to_string()),
            claim_uuid: Some("claim2".to_string()),
            attr_info: Some(Attr {
                name: "claim2Name".to_string(),
                value: serde_json::to_value("val2").unwrap(),
                attr_type: "attr2".to_string(),
                predicate_type: None,
            })
        };
        let claim3 = ClaimData {
            schema_seq_no: Some(3),
            issuer_did: Some("33".to_string()),
            claim_uuid: Some("claim3".to_string()),
            attr_info: Some(Attr {
                name: "claim3Name".to_string(),
                value: serde_json::to_value("val3").unwrap(),
                attr_type: "attr3".to_string(),
                predicate_type: None,
            })
        };
        let claims = vec![claim1.clone(), claim2.clone(), claim3.clone()];
        let claim_json = proof.build_claim_defs_json(claims.as_ref()).unwrap();
        let test_claim_json = format!(r#"{{"{}":{},"{}":{},"{}":{}}}"#,
                                      claim1.claim_uuid.unwrap(), data,
                                      claim2.claim_uuid.unwrap(), data,
                                      claim3.claim_uuid.unwrap(), data);
        assert!(claim_json.contains("\"claim1\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
        assert!(claim_json.contains("\"claim2\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
        assert!(claim_json.contains("\"claim3\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
    }

    #[test]
    fn test_build_schemas_json_with_multiple_schemas() {
        let claim_result = r#"{"auditPath":["7hRA1eWgHDmqFfXQHmHLzCE1ZeXvvkq5VaJEpb6NWz74","4QvchQ6JGxvU57kyzHzKJvUV7rb12jpFX7FBP9LrN9qA","G14qswNCM1mxhRHPMLx4h5qmbLEDQkczjJUVUEedUGxQ","4B6hCrJc2TubiFE1rgxjM1Hj7zvTTjxkzo9Gikhy4MVZ"],"data":{"attr_names":["name","male"],"name":"name","version":"1.0"},"identifier":"VsKV7grR1BUE29mG2Fm2kX","reqId":1515795761424583710,"rootHash":"C98M4qjp4zzHw6APDWwGxTBHkEdAhjUQepi3Bxz2auna","seqNo":299,"signature":"4iFhpLknpRiCU6Axrj8HcFxMaxGaMmnzwJ1WMKndK653k4B7LYGZD2PNHEEGZQEBVXwhgDxPFe1t9bSzdVcEQ3eL","txnTime":1515795761,"type":"101"}"#;
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let data = r#"{"ref":1,"origin":"NcYxiDXkpYi6ov5FcYDi1e","signature_type":"CL","data":{"primary":{"n":"9","s":"8","rms":"7","r":{"height":"6","sex":"5","age":"4","name":"3"},"rctxt":"2","z":"1"},"revocation":null}}"#;
        let mut proof = Proof {
            handle: 1,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: String::from("[]"),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            proof_state: ProofStateType::ProofUndefined,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: None,
            proof_request: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        let claim1 = ClaimData {
            schema_seq_no: Some(1),
            issuer_did: Some("11".to_string()),
            claim_uuid: Some("claim1".to_string()),
            attr_info: Some(Attr {
                name: "claim1Name".to_string(),
                value: serde_json::to_value("val1").unwrap(),
                attr_type: "attr1".to_string(),
                predicate_type: None,
            })
        };
        let claim2 = ClaimData {
            schema_seq_no: Some(2),
            issuer_did: Some("22".to_string()),
            claim_uuid: Some("claim2".to_string()),
            attr_info: Some(Attr {
                name: "claim2Name".to_string(),
                value: serde_json::to_value("val2").unwrap(),
                attr_type: "attr2".to_string(),
                predicate_type: None,
            })
        };
        let claim3 = ClaimData {
            schema_seq_no: Some(3),
            issuer_did: Some("33".to_string()),
            claim_uuid: Some("claim3".to_string()),
            attr_info: Some(Attr {
                name: "claim3Name".to_string(),
                value: serde_json::to_value("val3").unwrap(),
                attr_type: "attr3".to_string(),
                predicate_type: None,
            })
        };
        let claims = vec![claim1.clone(), claim2.clone(), claim3.clone()];
        let schemas_json = proof.build_schemas_json(claims.as_ref()).unwrap();
        assert!(schemas_json.contains("\"claim1\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
        assert!(schemas_json.contains("\"claim2\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
        assert!(schemas_json.contains("\"claim3\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
    }

    #[test]
    fn test_get_proof() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let proof_msg = r#"{"proofs":{"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"sex":"5944657099558967239210949258394887428692050081607692519917050011144233115103","name":"1139481716457488690172217916278103335"},"a_prime":"55115757663642844902979276276581544287881791112969892277372135316353511833640150801244335663890109536491278379177551666081054765286807563008348637104046950934828407012194403360724040287698135607556244297972578864339500981366412262454282194811242239615009347165118318516694216754501345324782597475927199400880006212632553233049354866295429520527445980181939247828351677971991914388778860092824318440481574181300185829423762990910739241691289976584754979812272223819007422499654272590946235912914032826994670588466080422906806402660885408376207875827950805200378568062518210110828954480363081643567615791016011737856977","e":"34976147138641338975844073241645969211530343885520088294714132974884138611036204288689212378023649179372520412699253155486970203797562324","v":"961473607552945346906354315658276499450491951690969023699851664262072769313929148332129868528140265952852653009499943891795293148107502144091334703992581737220352761140064276811372868396353572957613845323343723271098601244774874235526135299483412285009916812621185291842845156342501611029106982811773616231232684804116984093651972537804480090649736612551759833591251845595059217608938213987633789344584340351801507541774726753840600143685051258161251666953243698589585559347435011414292427590918153421953579895479604685390401357681887618798200391305919594609949167659780330698000168295871428737686822637913218269005987492318466661186509308179489615192663542904993253626728197630057096161118638090776180812895097232529119979970798938360220605280817954648588493778338816318524451785027916181454650102696493927306340658666852294316562458212054696739343800993703515542777264448535624584845146378512183572107830260813929222999","m":{},"m1":"75548120024969192086664289521241751069844239013520403238642886571169851979005373784309432586593371476370934469326730539754613694936161784687213609047455188306625204249706249661640538349287762196100659095340756990269587317065862046598569445591945049204366911309949910119711238973099702616527117177036784698661","m2":"287944186286321709724396773443214682376883853676549188669693055373059354657799325692443906346632814001611911026063358134413175852024773765930829079850890920811398176944587192618"},"ge_proofs":[{"u":{"2":"2","0":"2","3":"2","1":"2"},"r":{"1":"2","3":"2","DELTA":"2","2":"2","0":"2"},"mj":"3","alpha":"2","t":{"0":"2","2":"2","DELTA":"4","1":"5","3":"3"},"predicate":{"attr_name":"predicate2","p_type":"LE","value":99,"schema_seq_no":778,"issuer_did":"12345"}}]},"non_revoc_proof":null},"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f"}},"aggregated_proof":{"c_hash":"63330487197040957750863022608534150304998351350639315143102570772502292901825","c_list":[[1,180,153,212,162,132,5,189,14,181,140,112,236,109,182,76,91,6,161,215,62,207,205,135,86,211,49,197,215,198,104,201,14,22,48,6,112,170,31,191,110,118,121,15,62,114,126,249,221,107,114,161,163,234,19,233,150,236,182,217,195,6,218,217,193,6,94,160,33,23,103,147,109,221,81,38,138,20,225,141,68,37,142,10,225,79,164,119,168,250,188,186,47,229,165,8,237,230,14,35,53,176,97,28,82,105,87,210,117,16,154,222,66,11,96,172,90,13,239,190,29,71,11,88,53,36,219,139,67,21,136,58,161,164,97,106,56,230,55,157,59,35,187,235,154,194,111,93,168,135,67,15,97,136,38,169,87,142,32,255,50,247,111,83,44,88,251,99,6,226,182,170,146,229,118,164,118,228,235,51,137,168,135,50,1,14,1,201,72,175,102,241,149,117,88,83,84,37,205,130,26,155,124,158,211,89,112,33,46,24,94,93,202,8,127,172,214,178,6,156,79,188,132,223,239,127,200,158,95,247,139,101,51,162,168,175,74,1,67,201,94,108,192,14,130,109,217,248,193,10,142,37,95,231,227,251,209]]},"requested_proof":{"revealed_attrs":{"attr2_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],"attr1_uuid":["claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{"self_attested_attr":"self_value"},"predicates":{"pred1":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4"}},"remoteDid":"KP8AaEBc368CMK1PqZaEzX","userPairwiseDid":"PofTCeegEXT7S2aAePhM6a"}"#;
        let new_handle = 1121;
        let proof = Proof {
            handle: new_handle,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: String::from("[]"),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            proof_state: ProofStateType::ProofUndefined,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: Some(ProofMessage::from_str(proof_msg).unwrap()),
            proof_request: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        let proof_str = proof.get_proof().unwrap();
        assert!(proof_str.contains(r#"{"schema_seq_no":103,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f","claim_uuid":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","attr_info":{"name":"name","value":"Alex","type":"revealed"}}"#));
        assert!(proof_str.contains(r#"{"name":"self_attested_attr","value":"self_value","type":"self_attested"}"#));
        assert!(proof_str.contains(r#"{"schema_seq_no":778,"issuer_did":"12345","claim_uuid":"claim::71b6070f-14ba-45fa-876d-1fe8491fe5d4","attr_info":{"name":"predicate2","value":99,"type":"predicate","predicate_type":"LE"}}"#));
    }

    #[test]
    fn test_release_all() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let h1 = create_proof(None, REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h2 = create_proof(None, REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h3 = create_proof(None, REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h4 = create_proof(None, REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h5 = create_proof(None, REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        release_all();
        assert_eq!(release(h1), error::INVALID_PROOF_HANDLE.code_num);
        assert_eq!(release(h2), error::INVALID_PROOF_HANDLE.code_num);
        assert_eq!(release(h3), error::INVALID_PROOF_HANDLE.code_num);
        assert_eq!(release(h4), error::INVALID_PROOF_HANDLE.code_num);
        assert_eq!(release(h5), error::INVALID_PROOF_HANDLE.code_num);
    }

    #[ignore]
    #[test]
    fn test_proof_validation_with_predicate() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        pool::open_sandbox_pool();
        //Generated proof from a script using libindy's python wrapper
        let proof_libindy = r#"{"proofs":{"claim::1f927d68-8905-4188-afd6-374b93202802":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"height":"101","weight":"200"},"a_prime":"96825357504213820414920712887062035178734220080803428497609883672984582907009099983101574047222923191172901087572029807234151946901292592958722247994272968564810499919300106044605207990057917809541832623852323791793311699413524993596221669223340445229137623978342462134206199825853206867063996113501801994264194743641731037981074653346341669505587949133838740152370376461215475461071948757081816711059135548641059460671478570669934528058630540187953747821539169198114058851772920394076402690746410309424689457804467340432629509418396341637312297063848355683710144090865231980347430473575136584749971336607872735221481","e":"173797707086412224638317654877859078924059373314980330653273352384790685281330188710472447391993098340873509685594459760045118633810933189","v":"817471077330446042801182583278289771289939899862029945076522952536065298759596646234441125348749218649517963039993752411760911373696622306417612712541094410891004790314292818814366685563995233608999796109598000161662493062853421565696169350546027944863673676086478166460362895885803634323690475920074417094037040935347402139651274019219402676999841249239965723785316578360113393116610025042363124284525391995329580061841736426328774210022128121295069448914290656014180519168890133233038958063218641899739335907276444414243209025143049431600130541188759223138211506107377349139195417531858438343513980308300691115889309183927808937128403031600964187596017158389035128670751910466244582788345871859741880784391935682362926324291609666439201495908248254759163419830166895575411946245430341531037523316546692349875449961544668569143489718563622163239197265014753082903704824839586079949261854071576781373207187423207921842830","m":{"age":"15822941617894338766233233658363264215333584225107865867255097101337240720962242368328289996604483085044274203376532496405725498102090855037798745873303669426664372253887825720219","name":"13964975736408658520087388652512629489372660995256458553254103444734218937382021087114414237718252264962805304656549104857929447944575961598713806963229381922407043533207465727255"},"m1":"96892808406380415027889979610758682722844380024509794965656695309603144945553873163394888111015064896074982096261748169391443152799015882346651661021879622831679779409251822010294932053550075486870760993659254772491544354732571948365621031241138016035965757649630576402013034966385574374940295922113808748379","m2":"11661093607806638073781188747084292103515742861070700215778059783362149490064162350612272534962281735520949733852364820875657324177187266468853072206402518876096159768752059101257"},"ge_proofs":[{"u":{"2":"13578349425016411853296070984615297374744740853356759535005637919110260401076671653667151106586899689163439312338350934128141247305476662351514147622217966638022724978926219721004","1":"9826801536654035279437804411721354692428821623443482842159853372073568018439342854077273656337145568231528331617294053198372493410882511177276033294945945111573497303544457501728","3":"1037042123153514204573119204948220605299154049076633583354494606925165405121997811285708025281091619836054847337623028945671223408927294913105554993423464283121766578018864123196","0":"12718528291019054839844788454092267738226962302199765676405440941777392322367575064447939193728922414125816500300704366932567255319945007387631305169707929310781367799553235731195"},"r":{"2":"676003009211374935252164028385587930900618111572610399426532998074314176520930974705233431019951593825021806950341323910450419034970606973321513599566350797174538980828130307785786557887300817047726185221834713357185388049771205972344981062426988232787725638212286251011377158986400244422481866675017475958221617542616886523578162467958820668241643670589961050978074077535490218790750363852878560521919793191258571933037651104202430172137316611448318949425860592212666636165975739856428347510362144740067880890894360528173831335810168020561465966403337553765679504833616090358440891118488405211760098270732931764838035175408241309661677257083620331075539644189461637790052172121872006766032436123633898664409978638073","1":"349920139330498666322081874316604276639135453395613764189224988132844853265045879120276342391960567534902476163362661427585506904593292240512436239768343344020840139731013755726350197160348526788473254697708006671949590195528004855851813998600251595425408516191961975148403040078689445518994216611674310364645986083380328389623233149622754428146641794304281871878839456540045792674039433595107726043753004551269064154794115110023958392809750311907103056910478193724397710793243930374968440072989638161331833988373816982213266342530892063408779433568830855447112125083761023018526471994625982947051477089824649064442071004215728834491161129012753324627972041484890033442644393421099298426040030376350168665512471459479","DELTA":"2526861470651903540720320018766930888167900681682572985661381884467392618261641856254430321050624591171634372539030989936575279755283501847530618484185680702377431490667509592453287600128690017511250680484481133961861751059007941559620907769444959808378674078261135597016002423629779764678309550341449689628473308469826002228492041549462958786314803913047193762779808717108120438193021264832085926863710931620643041459463102773527316790458618123142903026612355945473887976952917085200867522116229217560970070697531438042172285209097472270369463486770759423978942812405424200436136331327024144131075040754281213553630388015531456435099854225963455009795976950482619918888522438664319539530239692821118623498424705652688","0":"227303741690308584797477735295198715784450946011850008177335476924050116857528863066627143458345318419409022081275373403345149661090008119849070683784280250685201823571318077869452613406858352540571555672813191272680806756442006396004567813444680548660322777493560315010733507370370161664280035580095533420578643487584761342906033815327063485750670841378883836444706052123766243319539590055884098533366810292531802775218401102860128587806228913886205861632717335037889596809104614549387408469316136746435173165124283115934244156928615362558648611222495646316614605352983318421663202797530744683820376707735248535628241924036161159932614335675040166337188851722591939429381437201923348800017362165521600184137378045300","3":"3782452160435244994305640620230284161266259587201416530630470391027710708201060896177599902317657509626066419632586010723213684427144396790304233645629004486904918218889579976490822963130314560352901741006252157192573268665291014408296857349571599513829377373687890199830766410332168601378519296407733260867717996031978610376955157034921668979895646241395908494759353090391980276896853697572039562508540269169332498254418040610566201842821643971775644086961593615913466915734572607003688651137260706788106040417008493617376835432217197185193923550093974933964025367861102086898081282048543090725604786529286396408208426923343342954201647541710771833352212650429563441418100463235725465955936565304905889131254959216319"},"mj":"15822941617894338766233233658363264215333584225107865867255097101337240720962242368328289996604483085044274203376532496405725498102090855037798745873303669426664372253887825720219","alpha":"34872707441943668555962227729623400706242062517309368451671475803555808473187122063738927425040029300261020414303710039209650320639097613388752346534836205478373404197421118163014327190520168843262674444521462882803672700118424453553908472888130344475236438054360549283772776268640869922694861668712566466484887949507361083049658486343750189393246195656647488079143964161212845625008980862764518649842627453020680924712648193263208528312853679683228786584439975576863302447553405100844483837440953623822117190363576643413897846168067157699019206898112653103033000653475724212201686935687689024186799069317581503304557753696132227445419998169511623645183170393763321474021474958075467971929328636007432251087414982085846113658316577011438437385644862462177079041099232014812219880868911453340614507218114391184041012451427647238636954423819","t":{"DELTA":"41307165120648610210323255658467084452588302969846130774080005528201577710926430139509593597163806485727726321757620805725587238993012402861710253413968045137568893879142473824911402781538120363984548568090293196159021817426447880420077899157810592309532889430989748749212042322667074051035312388704149017546618871610810735892041428534437648254019061553962258578634820738954140745472608119502104487739701641542862157803049040862713586918668540093258564294174369528845162482050479041840521474021843342836399729827428484368223534373219690727249480848630020293513261955840306892859816114010488426893769291815215205630742","2":"62841161458855690996661651614952070328705117022122283350316212662191494402593888806663350453883794826175484795499980021938644983903204992828166594403033230659509880531580187666263705641237995616859688468467613386802320437019449932939897064616566553110257459390946159059302421500358482472933517952960045455017229612538539208134069848925837034454447340658980181834790694456465850853974096220940799699361291520251683551669461390294007400713815067265039052964834756176299612320355263269984123484010851258116715025722655624854990040291762792880651376692064821746681638346056600286274510147910417870117450958869861465104899","1":"24703978606461118787105274354111842894893620834428715915938731240274908551111997585221881006416640821568172482461362275778386250296067233873665795472590400167255066699547396117589752898735251402483444776797066115291089304822000811043791445531767379190577594453868005586497081678504906992895932614822283189035748567383414492415611639449221381930702212102776768096535362890717753305938703607625761918979935187659029267871521564781565601905811572608291644252029147522049737646727602051717123718947045720827732051530300459585591890913538025411047507139522732042367301130637372288499204115179013530369697698354218594721475","0":"65806469094079822737051837922865696200577864864107883279450881789586610718279496879851772473067628842827615462460682005595227318016434368804753625199234729545808425915177309271666399564866366736529856443214583663269313307496418865238919492752930091988186505318922307627810577876406015223604199609319912951876126340656935317188225430904803331926648078715341421077348335918957748469977656446644923975237875910547351861966775766500545717552745804431969361710725386833003295067518704359823179893207876740778265886237227131457770819915147795395535727448356111980197115804613063948383652897499711327385710045228782003512936","3":"31322646464903454693494910691778082184612167667104866741028094713666840341508471387868538560123257236281474016330876597578553272613264509719738069121206685832934379843974913138572244384840932089473855198176658010692215649659329309967499853745027730849426692596913387514954466271796502348182196650439320529805947269694409354754272076934002666758951198589640361711184102691319537066346399151607042152550989492756769996181803310957811031813265690379435194143469156984862424644808054287393681339622619828302442397475709013742545822239900931405276555943268011671508253894537619267643377370399976878254795845211576746634008"},"predicate":{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":694,"issuer_did":"DunkM3x1y7S4ECgSL4Wkru"}}]},"non_revoc_proof":null},"schema_seq_no":694,"issuer_did":"DunkM3x1y7S4ECgSL4Wkru"}},"aggregated_proof":{"c_hash":"103325140275918938867265803420842782489379566578616035509312119027147548936535","c_list":[[2,255,1,43,219,18,174,224,110,47,48,204,200,108,166,218,236,150,253,40,245,107,249,151,169,23,213,77,132,219,111,189,93,49,128,111,97,179,145,71,54,31,66,185,179,77,166,145,209,119,48,224,231,103,209,111,164,165,115,40,210,186,42,229,211,169,127,72,200,124,224,109,227,124,216,135,243,38,133,132,44,79,215,125,254,12,38,125,140,237,26,97,188,57,16,189,13,124,67,199,101,221,67,14,225,160,229,169,38,143,183,36,63,47,23,237,246,4,69,164,95,175,237,148,16,26,97,174,155,19,244,135,242,237,152,61,100,254,4,252,80,169,46,57,131,97,71,106,126,60,218,2,70,211,188,55,90,37,182,127,64,69,142,173,228,180,216,84,219,211,184,151,43,113,18,207,133,30,198,241,161,27,160,158,27,201,56,23,131,91,221,241,251,136,194,157,243,199,185,44,79,63,64,53,0,64,169,35,153,98,68,249,54,85,76,203,244,70,140,79,3,164,69,38,230,198,241,165,68,10,87,156,39,236,192,215,171,48,148,86,156,3,101,126,229,50,190,186,193,155,107,23,222,53,103,150,233],[2,9,73,163,78,16,195,84,123,149,109,254,233,117,196,207,204,142,208,251,249,144,186,168,40,217,175,177,107,23,54,175,221,146,5,41,110,169,241,186,251,82,215,12,72,225,54,108,152,40,208,184,206,146,180,235,204,239,65,193,120,217,102,238,132,206,216,182,238,131,77,189,92,190,100,126,110,216,81,109,107,40,129,14,125,147,128,99,167,87,144,21,124,35,1,50,77,80,121,150,208,131,87,41,4,142,237,137,61,53,170,7,32,113,157,183,186,223,5,57,106,141,128,198,104,188,103,62,190,184,173,10,78,159,187,186,53,9,21,9,91,179,198,149,28,90,126,254,30,52,255,103,36,85,255,206,188,255,224,159,161,85,226,91,247,55,105,174,66,48,160,239,113,27,143,15,47,238,250,119,65,108,86,164,164,83,160,120,55,102,107,147,187,101,33,10,36,155,20,246,63,98,38,12,183,78,168,28,134,244,159,154,152,117,211,196,104,203,173,92,171,94,129,82,113,88,178,148,68,110,255,161,124,181,38,80,78,173,59,16,61,17,243,165,109,40,151,245,70,188,113,156,194,57,233,102,104],[195,177,117,241,39,31,43,103,4,123,155,155,185,3,157,169,135,13,153,177,142,64,120,109,184,15,93,245,23,215,19,182,204,189,106,207,67,242,60,194,53,77,5,224,96,26,118,224,26,220,52,186,158,165,224,247,61,146,248,144,86,160,129,234,209,178,210,130,127,60,153,174,86,129,84,148,181,203,162,29,194,119,223,5,108,238,168,4,82,111,163,91,112,138,96,137,26,204,136,233,73,52,41,164,7,33,99,249,171,223,149,6,192,64,190,176,167,244,56,186,71,50,230,27,130,76,137,128,59,228,169,183,39,182,251,106,102,116,112,237,29,75,104,38,196,238,38,23,47,196,181,19,146,215,179,136,173,224,35,152,248,166,130,73,116,172,120,25,83,106,81,52,89,201,160,127,145,213,204,70,173,143,162,125,98,104,240,83,134,8,118,201,187,62,165,53,60,181,231,134,246,209,240,6,21,44,51,150,69,230,127,10,178,185,174,58,89,188,136,68,182,54,126,84,131,20,122,190,123,17,243,72,211,34,75,131,189,184,97,31,37,211,171,168,158,114,5,199,146,151,90,102,180,159,222,195],[1,241,204,66,80,252,128,83,100,117,100,132,79,145,93,20,101,39,163,107,252,237,3,107,107,230,109,62,6,167,61,94,12,242,106,117,88,89,153,187,143,81,192,215,24,69,245,141,23,165,126,204,149,10,67,170,209,53,64,7,220,225,23,156,120,250,25,76,77,175,80,113,141,103,109,174,77,151,4,62,53,186,145,104,111,7,187,45,158,210,108,157,175,167,74,35,104,180,92,122,217,97,148,247,197,236,22,202,31,22,145,156,183,59,146,121,58,171,248,150,167,180,208,115,138,206,22,38,216,201,211,218,160,228,116,84,59,209,66,19,234,94,121,236,12,113,144,61,39,7,221,58,167,147,40,238,121,127,249,183,99,169,148,1,177,177,188,178,24,56,231,52,51,107,93,6,107,131,237,67,25,50,120,49,178,142,172,222,20,2,53,207,102,84,88,181,15,114,208,213,199,29,23,110,199,57,165,218,190,236,107,255,190,124,44,24,17,57,27,196,66,240,0,240,59,215,68,94,205,18,152,206,166,29,136,65,24,70,110,231,44,120,104,65,91,162,25,145,191,62,143,169,94,145,92,162,3],[248,31,135,16,9,253,156,61,189,71,113,131,172,238,66,26,77,193,63,68,248,114,19,177,8,54,49,185,168,212,253,22,13,233,162,77,65,26,78,132,204,253,141,123,6,184,36,50,89,139,87,111,127,131,93,193,8,194,45,76,115,115,234,44,235,18,123,228,2,164,247,7,66,97,238,121,171,60,109,190,75,171,114,147,81,88,222,179,119,18,52,76,172,38,253,92,249,13,180,129,231,218,166,96,208,154,186,25,120,63,59,56,88,75,245,246,202,56,130,78,136,64,241,221,246,198,247,198,228,117,60,223,64,104,161,23,63,206,186,192,252,4,53,64,13,167,101,248,23,105,55,24,158,91,121,186,2,111,141,50,153,117,79,231,184,184,128,110,6,230,191,245,4,112,15,206,159,71,77,184,109,61,1,125,35,152,134,28,219,110,90,135,75,18,96,142,14,247,177,169,37,172,131,236,124,176,131,44,128,35,85,11,88,246,196,167,208,105,164,38,234,103,67,137,5,215,123,21,86,70,252,110,167,59,224,98,30,60,82,56,116,231,41,247,124,110,55,170,188,52,231,127,148,83,235,24],[1,71,55,61,51,44,122,177,208,25,203,86,30,135,194,207,116,97,170,229,137,112,155,230,212,255,196,247,61,83,15,69,139,144,157,86,190,140,90,27,227,10,129,121,229,41,240,201,2,149,186,22,136,100,79,71,204,113,85,58,21,170,239,97,25,133,87,172,138,25,154,209,109,116,84,45,30,112,204,76,30,187,4,186,41,63,60,167,129,154,239,75,68,223,15,39,181,75,180,35,144,107,93,91,31,74,249,84,245,171,241,244,48,71,0,204,18,95,230,5,7,124,95,218,229,92,60,119,67,78,185,255,59,181,133,254,210,230,225,189,51,164,163,219,171,190,204,138,36,200,205,7,15,101,61,118,252,9,152,43,84,33,109,178,53,80,198,229,247,183,22,207,29,92,173,206,84,162,243,107,185,8,18,72,21,151,28,33,91,26,185,25,183,109,88,207,39,207,175,129,34,95,211,87,211,5,155,175,244,248,122,187,92,130,172,11,106,52,201,35,232,147,116,112,11,199,124,44,163,219,143,67,9,22,158,72,129,209,165,185,205,32,54,111,33,139,136,137,189,22,131,248,234,115,230,195,22]]},"requested_proof":{"revealed_attrs":{"height_0":["claim::1f927d68-8905-4188-afd6-374b93202802","101","101"],"weight_1":["claim::1f927d68-8905-4188-afd6-374b93202802","200","200"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{"age_2":"claim::1f927d68-8905-4188-afd6-374b93202802"}}}"#;
        let proof_req = r#"{"name":"proof name","nonce":"2771519439","requested_attrs":{"height_0":{"issuer_did":"DunkM3x1y7S4ECgSL4Wkru","name":"height","schema_seq_no":694},"weight_1":{"issuer_did":"DunkM3x1y7S4ECgSL4Wkru","name":"weight","schema_seq_no":694}},"requested_predicates":{"age_2":{"attr_name":"age","p_type":"GE","issuer_did":"DunkM3x1y7S4ECgSL4Wkru","schema_seq_no":694,"value":18}},"version":"0.1"}"#;
        let proof_msg: ProofMessage = serde_json::from_str(proof_libindy).unwrap();
        let mut proof_req_msg = ProofRequestMessage::create();
        proof_req_msg.proof_request_data = serde_json::from_str(proof_req).unwrap();
        let mut proof = Proof {
            handle: 1237852,
            source_id: "12".to_string(),
            msg_uid: String::from("1234"),
            ref_msg_id: String::new(),
            requested_attrs: String::from("[]"),
            requested_predicates: REQUESTED_PREDICATES.to_string(),
            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
            prover_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateRequestReceived,
            proof_state: ProofStateType::ProofUndefined,
            name: String::new(),
            version: String::from("1.0"),
            nonce: generate_nonce().unwrap(),
            proof: Some(proof_msg),
            proof_request: Some(proof_req_msg),
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
        };
        let rc = proof.proof_validation();
        assert!(rc.is_ok());
        assert_eq!(proof.proof_state, ProofStateType::ProofValidated);

        let proof_data = proof.get_proof().unwrap();
        assert!(proof_data.contains(r#""schema_seq_no":694,"issuer_did":"DunkM3x1y7S4ECgSL4Wkru","claim_uuid":"claim::1f927d68-8905-4188-afd6-374b93202802","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"}}"#));
    }
}
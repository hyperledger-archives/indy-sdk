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
        info!("successfully validated proof request {}", self.handle);
        Ok(error::SUCCESS.code_num)
    }

    fn validate_proof_indy(&mut self,
                           proof_req_json: &str,
                           proof_json: &str,
                           schemas_json: &str,
                           claim_defs_json: &str,
                           revoc_regs_json: &str) -> Result<u32, u32> {
        if settings::test_indy_mode_enabled() {return Ok(error::SUCCESS.code_num);}

        info!("starting libindy proof verification");
        let valid = match libindy_verifier_verify_proof(proof_req_json,
                                                         proof_json,
                                                         schemas_json,
                                                         claim_defs_json,
                                                         revoc_regs_json) {
            Ok(x) => x,
            Err(x) => {
                error!("Error: {}, Proof wasn't valid {}", x, self.handle);
                self.proof_state = ProofStateType::ProofInvalid;
                return Err(error::INVALID_PROOF.code_num)
            }
        };

        if !valid {
            warn!("indy returned false when validating proof");
            self.proof_state = ProofStateType::ProofInvalid;
            return Ok(error::SUCCESS.code_num)
        }
        info!("Indy validated Proof: {:?}", self.handle);
        self.proof_state = ProofStateType::ProofValidated;
        Ok(error::SUCCESS.code_num)
    }

    fn build_claim_defs_json(&mut self, claim_data:&Vec<ClaimData>) -> Result<String, u32> {
        info!("building claimdef json for proof validation");
        let mut claim_json: HashMap<String, ClaimDefinition> = HashMap::new();
        for claim in claim_data.iter() {
            let claim_def = RetrieveClaimDef::new()
                .retrieve_claim_def("GGBDg1j8bsKmr4h5T9XqYf", claim.schema_seq_no, Some(SigTypes::CL), &claim.issuer_did)?;
            let claim_obj: ClaimDefinition = match serde_json::from_str(&claim_def) {
                Ok(x) => x,
                Err(_) => return Err(error::INVALID_JSON.code_num),
            };
            claim_json.insert(claim.claim_uuid.clone(), claim_obj);
        }

        match serde_json::to_string(&claim_json) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_CLAIM_DEF_JSON.code_num)
        }
    }

    fn build_proof_json(&mut self) -> Result<String, u32> {
        info!("building proof json for proof validation");
        match self.proof {
            Some(ref x) => x.to_string(),
            None => Err(error::INVALID_PROOF.code_num),
        }
    }

    fn build_schemas_json(&mut self, claim_data:&Vec<ClaimData>) -> Result<String, u32> {
        info!("building schemas json for proof validation");

        let mut schema_json: HashMap<String, SchemaTransaction> = HashMap::new();
        for schema in claim_data.iter() {
            let schema_obj = LedgerSchema::new_from_ledger(schema.schema_seq_no as i32)?;
            let data = match schema_obj.data {
                Some(x) => x,
                None => return Err(error::INVALID_PROOF.code_num)
            };
            schema_json.insert(schema.claim_uuid.clone(), data);
        }

        match serde_json::to_string(&schema_json) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_SCHEMA.code_num)
        }
    }

    fn build_proof_req_json(&mut self) -> Result<String, u32> {
        info!("building proof request json for proof validation");
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
        let claim_data = proof_msg.get_claim_schema_info()?;

        if claim_data.len() == 0 {
            return Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
        }
        let claim_def_msg = self.build_claim_defs_json(&claim_data)?;
        let schemas_json = self.build_schemas_json(&claim_data)?;
        let proof_json = self.build_proof_json()?;
        let proof_req_json = self.build_proof_req_json()?;
        info!("*******\n{}\n********", claim_def_msg);
        info!("*******\n{}\n********", schemas_json);
        info!("*******\n{}\n********", proof_json);
        info!("*******\n{}\n********", proof_req_json);
        proof_compliance(&proof_req_msg.proof_request_data, &proof_msg)?;
        self.validate_proof_indy(&proof_req_json, &proof_json, &schemas_json, &claim_def_msg, INDY_REVOC_REGS_JSON)
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("proof {} has invalid state {} for sending proofRequest", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }
        info!("sending proof request with proof: {}, and connection {}", self.handle, connection_handle);
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
//            .requested_predicates(&self.requested_predicates)
            .serialize_message()?;

        self.proof_request = Some(proof_obj);
        let data = connection::generate_encrypted_payload(&self.prover_vk, &self.remote_vk, &proof_request, "PROOF_REQUEST")?;
        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_CLAIM_OFFER_RESPONSE.to_vec()); }

        match messages::send_message().to(&self.prover_did)
            .to_vk(&self.prover_vk)
            .msg_type("proofReq")
            .agent_did(&self.agent_did)
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
        info!("updating state for proof {}", self.handle);
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
                    info!("Proof format was validated for proof {}", self.handle);
                    self.proof_state = ProofStateType::ProofValidated;
                }
            }
            Err(x) => {
                if x == error::TIMEOUT_LIBINDY_ERROR.code_num {
                    info!("Proof {} unable to be validated", self.handle);
                    self.proof_state = ProofStateType::ProofUndefined;
                } else {
                    info!("Proof {} had invalid format with err {}", self.handle, x);
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
    info!("creating proof with name: {}, requested_attrs: {}, requested_predicates: {}", name, requested_attrs, requested_predicates);

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
        info!("inserting handle {} into proof table", new_handle);
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

pub fn to_string(handle: u32) -> Result<String, u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num)
    }
}

pub fn from_string(proof_data: &str) -> Result<u32, u32> {
    let derived_proof: Proof = match serde_json::from_str(proof_data) {
        Ok(x) => x,
        Err(y) => return Err(error::INVALID_JSON.code_num),
    };
    let new_handle = derived_proof.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let proof = Box::from(derived_proof);

    {
        let mut m = PROOF_MAP.lock().unwrap();
        info!("inserting handle {} into proof table", new_handle);
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
                    info!("response had no uid");
                    return Err(error::INVALID_JSON.code_num)
                },
            };
            Ok(String::from(detail))
        },
        Err(_) => {
            info!("Proof called without a valid response from server");
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

    let my_claim_req = match ProofMessage::from_str(&data) {
        Ok(x) => x,
        Err(x) => {
            warn!("invalid json {}", x);
            return Err(error::INVALID_JSON.code_num);
        },
    };
    Ok(my_claim_req)
}

pub fn get_proof(handle: u32) -> Result<String,u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(proof) => Ok(proof.get_proof()?),
        None => Err(error::INVALID_PROOF.code_num),
    }
}

pub fn generate_nonce() -> Result<String, u32> {
    let mut bn = match BigNum::new() {
        Ok(x) => x,
        Err(_) => return Err(error::BIG_NUMBER_ERROR.code_num)
    };

    match BigNumRef::rand(&mut bn, LARGE_NONCE as i32, openssl::bn::MSB_MAYBE_ZERO, false){
        Ok(x) => x,
        Err(_) => return Err(error::BIG_NUMBER_ERROR.code_num)

    };
    match bn.to_dec_str() {
        Ok(x) => Ok(x.to_string()),
        Err(_) => return Err(error::BIG_NUMBER_ERROR.code_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use connection::build_connection;

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";

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

        match create_proof(None,
                           REQUESTED_ATTRS.to_owned(),
                           REQUESTED_PREDICATES.to_owned(),
                           "Optional".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1),
        }
    }

    #[test]
    fn test_nonce() {
        let nonce = generate_nonce().unwrap();
        assert!(BigNum::from_dec_str(&nonce).unwrap().num_bits() < 81)
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let proof_string = to_string(handle).unwrap();
        assert!(!proof_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
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
        let handle = match create_proof(Some("1".to_string()),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(release(handle), 0);
        assert!(!is_valid_handle(handle));
    }

    #[test]
    fn test_send_proof_request() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_handle = build_connection("test_send_proof_request".to_owned()).unwrap();
        connection::set_agent_verkey(connection_handle, VERKEY);
        connection::set_agent_did(connection_handle, DID);
        connection::set_their_pw_verkey(connection_handle, VERKEY);

        let handle = match create_proof(Some("1".to_string()),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
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

        let handle = match create_proof(Some("1".to_string()),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        match send_proof_request(handle, connection_handle) {
            Ok(x) => panic!("Should have failed in send_proof_request"),
            Err(y) => assert_eq!(y, error::INVALID_DID.code_num)
        }
    }

    #[test]
    fn test_get_proof_fails_with_no_proof() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(Some("1".to_string()),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Optional".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
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
        ::utils::logger::LoggerUtils::init();
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
        ::utils::logger::LoggerUtils::init();
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
        ::utils::logger::LoggerUtils::init();
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
            schema_seq_no: 1,
            issuer_did: "11".to_string(),
            claim_uuid: "claim1".to_string(),
            name: "claim1Name".to_string(),
            value: serde_json::to_value("val1").unwrap(),
            attr_type: "attr1".to_string(),
        };
        let claim2 = ClaimData {
            schema_seq_no: 2,
            issuer_did: "22".to_string(),
            claim_uuid: "claim2".to_string(),
            name: "claim2Name".to_string(),
            value: serde_json::to_value("val2").unwrap(),
            attr_type: "attr2".to_string(),
        };
        let claim3 = ClaimData {
            schema_seq_no: 3,
            issuer_did: "33".to_string(),
            claim_uuid: "claim3".to_string(),
            name: "claim3Name".to_string(),
            value: serde_json::to_value("val3").unwrap(),
            attr_type: "attr3".to_string(),
        };
        let claims = vec![claim1.clone(), claim2.clone(), claim3.clone()];
        let claim_json = proof.build_claim_defs_json(claims.as_ref()).unwrap();
        let test_claim_json = format!(r#"{{"{}":{},"{}":{},"{}":{}}}"#,
                                      claim1.claim_uuid, data,
                                      claim2.claim_uuid, data,
                                      claim3.claim_uuid, data);
        assert!(claim_json.contains("\"claim1\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
        assert!(claim_json.contains("\"claim2\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
        assert!(claim_json.contains("\"claim3\":{\"ref\":1,\"origin\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"signature_type\":\"CL\""));
    }

    #[test]
    fn test_build_schemas_json_with_multiple_schemas() {
        let claim_result = r#"{"auditPath":["7hRA1eWgHDmqFfXQHmHLzCE1ZeXvvkq5VaJEpb6NWz74","4QvchQ6JGxvU57kyzHzKJvUV7rb12jpFX7FBP9LrN9qA","G14qswNCM1mxhRHPMLx4h5qmbLEDQkczjJUVUEedUGxQ","4B6hCrJc2TubiFE1rgxjM1Hj7zvTTjxkzo9Gikhy4MVZ"],"data":{"attr_names":["name","male"],"name":"name","version":"1.0"},"identifier":"VsKV7grR1BUE29mG2Fm2kX","reqId":1515795761424583710,"rootHash":"C98M4qjp4zzHw6APDWwGxTBHkEdAhjUQepi3Bxz2auna","seqNo":299,"signature":"4iFhpLknpRiCU6Axrj8HcFxMaxGaMmnzwJ1WMKndK653k4B7LYGZD2PNHEEGZQEBVXwhgDxPFe1t9bSzdVcEQ3eL","txnTime":1515795761,"type":"101"}"#;
        ::utils::logger::LoggerUtils::init();
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
            schema_seq_no: 1,
            issuer_did: "11".to_string(),
            claim_uuid: "claim1".to_string(),
            name: "claim1Name".to_string(),
            value: serde_json::to_value("val1").unwrap(),
            attr_type: "attr1".to_string(),
        };
        let claim2 = ClaimData {
            schema_seq_no: 2,
            issuer_did: "22".to_string(),
            claim_uuid: "claim2".to_string(),
            name: "claim2Name".to_string(),
            value: serde_json::to_value("val2").unwrap(),
            attr_type: "attr2".to_string(),
        };
        let claim3 = ClaimData {
            schema_seq_no: 3,
            issuer_did: "33".to_string(),
            claim_uuid: "claim3".to_string(),
            name: "claim3Name".to_string(),
            value: serde_json::to_value("val3").unwrap(),
            attr_type: "attr3".to_string(),
        };
        let claims = vec![claim1.clone(), claim2.clone(), claim3.clone()];
        let schemas_json = proof.build_schemas_json(claims.as_ref()).unwrap();
        assert!(schemas_json.contains("\"claim1\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
        assert!(schemas_json.contains("\"claim2\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
        assert!(schemas_json.contains("\"claim3\":{\"seqNo\":344,\"identifier\":\"VsKV7grR1BUE29mG2Fm2kX\",\"txnTime\":1516284381,\"type\":\"101\",\"data\":{\"name\":\"get schema attrs\",\"version\":\"1.0\",\"attr_names\":[\"test\",\"get\",\"schema\",\"attrs\"]}}"));
    }

//    #[test]
//    fn test_validate_proof() {
//        settings::set_defaults();
//        ::claim_def::tests::open_sandbox_pool();
//
//        let proof_msg = r#"{"msg_type":"proof","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::e5fec91f-d03d-4513-813c-ab6db5715d55":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"22605045280481376895214546474258256134055560453004805058368015338423404000586901936329279496160366852115900235316791489357953785379851822281248296428005020302405076144264617943389810572564188437603815231794326272302243703078443007359698858400857606408856314183672828086906560155576666631125808137726233827430076624897399072853872527464581329767287002222137559918765406079546649258389065217669558333867707240780369514832185660287640444094973804045885379406641474693993903268791773620198293469768106363470543892730424494655747935463337367735239405840517696064464669905860189004121807576749786474060694597244797343224031","e":"70192089123105616042684481760592174224585053817450673797400202710878562748001698340846985261463026529360990669802293480312441048965520897","v":"1148619141217957986496757711054111791862691178309410923416837802801708689012670430650138736456223586898110113348220116209094530854607083005898964558239710027534227973983322542548800291320747321452329327824406430787211689678096549398458892087551551587767498991043777397791000822007896620414888602588897806008609113730393639807814070738699614969916095861363383223421727858670289337712185089527052065958362840287749622133424503902085247641830693297082507827948006947829401008622239294382186995101394791468192083810475776455445579931271665980788474331866572497866962452476638881287668931141052552771328556458489781734943404258692308937784221642452132005267809852656378394530342203469943982066011466088478895643800295937901139711103301249691253510784029114718919483272055970725860849610885050165709968510696738864528287788491998027072378656038991754015693216663830793243584350961586874315757599094357535856429087122365865868729","m":{"address2":"11774234640096848605908744857306447015748098256395922562149769943967941106193320512788344020652220849708117081570187385467979956319507248530701654682748372348387275979419669108338","city":"4853213962270369118453000522408430296589146124488849630769837449684434138367659379663124155088827069418193027370932024893343033367076071757003149452226758383807126385017161888440","address1":"12970590675851114145396120869959510754345567924518524026685086869487243290925032320159287997675756075512889990901552679591155319959039145119122576164798225386578339739435869622811","zip":"8333721522340131864419931745588776943042067606218561135102011966361165456174036379901390244538991611895455576519950813910672825465382312504250936740379785802177629077591444977329"},"m1":"92853615502250003546205004470333326341901175168428906399291824325990659330595200000112546157141090642053863739870044907457400076448073272490169488870502566172795456430489790324815765612798273406119873266684053517977802902202155082987833343670942161987285661291655743810590661447300059024966135828466539810035","m2":"14442362430453309930284822850357071315613831915865367971974791350454381198894252834180803515368579729220423713315556807632571621646127926114010380486713602821529657583905131582938"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"68430476900085482958838239880418115228681348197588159723604944078288347793331","c_list":[[179,17,2,242,194,227,92,203,28,32,255,113,112,20,5,243,9,111,220,111,21,210,116,12,167,119,253,181,37,40,143,215,140,42,179,97,75,229,96,94,54,248,206,3,48,14,61,219,160,122,139,227,166,183,37,43,197,200,28,220,217,10,65,42,6,195,124,44,164,65,114,206,51,231,254,156,170,141,21,153,50,251,237,65,147,97,243,17,157,116,213,201,80,119,106,70,88,60,55,36,33,160,135,106,60,212,191,235,116,57,78,177,61,86,44,226,205,100,134,118,93,6,26,58,220,66,232,166,202,62,90,174,231,207,19,239,233,223,70,191,199,100,157,62,139,176,28,184,9,70,116,199,142,237,198,183,12,32,53,84,207,202,77,56,97,177,154,169,223,201,212,163,212,101,184,255,215,167,16,163,136,44,25,123,49,15,229,41,149,133,159,86,106,208,234,73,207,154,194,162,141,63,159,145,94,47,174,51,225,91,243,2,221,202,59,11,212,243,197,208,116,42,242,131,221,137,16,169,203,215,239,78,254,150,42,169,202,132,172,106,179,130,178,130,147,24,173,213,151,251,242,44,54,47,208,223]]},"requested_proof":{"revealed_attrs":{"sdf":["claim::e5fec91f-d03d-4513-813c-ab6db5715d55","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}"#;
//        let new_handle = 1;
//        let proof_req = messages::proof_request()
//            .type_version("0.1")
//            .prover_did("BnRXf8yDMUwGyZVDkSENeq")
//            .requester_did("GxtnGN6ypZYgEqcftSQFnC")
//            .tid(1)
//            .mid(9)
//            .nonce("123")
//            .proof_name("proof_req_1")
//            .proof_data_version(".01")
//            .requested_attrs(REQUESTED_ATTRS).clone();
//
//        let mut proof = Proof {
//            handle: 1,
//            source_id: "12".to_string(),
//            msg_uid: String::from("1234"),
//            ref_msg_id: String::new(),
//            requested_attrs: String::from("[]"),
//            requested_predicates: String::from("[]"),
//            prover_did: String::from("GxtnGN6ypZYgEqcftSQFnC"),
//            prover_vk: VERKEY.to_string(),
//            state: VcxStateType::VcxStateOfferSent,
//            proof_state: ProofStateType::ProofUndefined,
//            name: String::new(),
//            version: String::from("1.0"),
//            nonce: generate_nonce().unwrap(),
//            proof: Some(ProofMessage::from_str(&proof_msg).unwrap()),
//            proof_request: Some(proof_req.clone()),
//            remote_did: DID.to_string(),
//            remote_vk: VERKEY.to_string(),
//            agent_did: DID.to_string(),
//            agent_vk: VERKEY.to_string(),
//        };
//
//        {
//            let mut m = PROOF_MAP.lock().unwrap();
//            info!("inserting handle {} into proof table", new_handle);
//            m.insert(new_handle, proof.clone());
//        }
//        proof.proof_validation().unwrap();
//    }

}

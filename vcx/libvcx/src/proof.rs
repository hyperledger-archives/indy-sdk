extern crate rand;
extern crate serde_json;
extern crate libc;
extern crate openssl;

use self::openssl::bn::{ BigNum, BigNumRef };
use settings;
use connection;
use api::{ VcxStateType, ProofStateType };
use std::collections::HashMap;
use messages::proofs::proof_message::{ProofMessage};
use messages;
use messages::proofs::proof_request::{ ProofRequestMessage };
use messages::GeneralMessage;
use utils::error;
use utils::constants::*;
use utils::libindy::anoncreds::libindy_verifier_verify_proof;
use credential_def::{ retrieve_credential_def };
use schema::{ LedgerSchema };
use error::proof::ProofError;
use error::ToErrorCode;
use serde_json::Value;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use object_cache::ObjectCache;

lazy_static! {
    static ref PROOF_MAP: ObjectCache<Proof> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Proof {
    source_id: String,
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
    proof: Option<ProofMessage>, // Refactoring this name to 'proof_message' causes some tests to fail.
    proof_request: Option<ProofRequestMessage>,
    remote_did: String,
    remote_vk: String,
    agent_did: String,
    agent_vk: String,
}

impl Proof {
    // leave this returning a u32 until we actually implement this method to do something
    // other than return success.
    fn validate_proof_request(&self) -> Result<u32, u32> {
        //TODO: validate proof request
        Ok(error::SUCCESS.code_num)
    }

    fn validate_proof_indy(&mut self,
                           proof_req_json: &str,
                           proof_json: &str,
                           schemas_json: &str,
                           credential_defs_json: &str,
                           rev_reg_defs_json: &str,
                           rev_regs_json: &str) -> Result<u32, ProofError> {
        if settings::test_indy_mode_enabled() {return Ok(error::SUCCESS.code_num);}

        debug!("starting libindy proof verification for {}", self.source_id);
        let valid = libindy_verifier_verify_proof(proof_req_json,
                                                  proof_json,
                                                  schemas_json,
                                                  credential_defs_json,
                                                  rev_reg_defs_json,
                                                  rev_regs_json).map_err(|err| {
                error!("Error: {}, Proof {} wasn't valid", err, self.source_id);
                self.proof_state = ProofStateType::ProofInvalid;
                ProofError::InvalidProof()
        })?;

        if !valid {
            warn!("indy returned false when validating proof {}", self.source_id);
            self.proof_state = ProofStateType::ProofInvalid;
            return Ok(error::SUCCESS.code_num)
        }
        debug!("Indy validated proof: {}", self.source_id);
        self.proof_state = ProofStateType::ProofValidated;
        Ok(error::SUCCESS.code_num)
    }

    fn build_credential_defs_json(&self, credential_data: &Vec<(String, String, String)>) -> Result<String, ProofError> {
        debug!("{} building credentialdef json for proof validation", self.source_id);
        let mut credential_json: HashMap<String, serde_json::Value> = HashMap::new();

        for &(_, ref cred_def_id, _) in credential_data.iter() {
            let (_, credential_def) = retrieve_credential_def(cred_def_id)
                .map_err(|ec| ProofError::CommonError(ec.to_error_code()))?;

            let credential_def = serde_json::from_str(&credential_def)
                .or(Err(ProofError::InvalidCredData()))?;

            credential_json.insert(cred_def_id.to_string(), credential_def);
        }

        serde_json::to_string(&credential_json).map_err(|err| {
            ProofError::CommonError(error::INVALID_CREDENTIAL_DEF_JSON.code_num)
        })
    }

    fn build_proof_json(&self) -> Result<String, ProofError> {
        debug!("{} building proof json for proof validation", self.source_id);
        match self.proof {
            Some(ref x) => Ok(x.libindy_proof.clone()),
            None => Err(ProofError::InvalidProof()),
        }
    }

    fn build_schemas_json(&self, credential_data: &Vec<(String, String, String)>) -> Result<String, ProofError> {
        debug!("{} building schemas json for proof validation", self.source_id);

        let mut schema_json: HashMap<String, serde_json::Value> = HashMap::new();

        for &(ref schema_id, _, _) in credential_data.iter() {
            let schema = LedgerSchema::new_from_ledger(schema_id)
                .or(Err(ProofError::InvalidSchema()))?;

            let schema_val = serde_json::from_str(&schema.schema_json)
                .or(Err(ProofError::InvalidSchema()))?;

            schema_json.insert(schema_id.to_string(), schema_val);
        }

        serde_json::to_string(&schema_json).or(Err(ProofError::InvalidSchema()))
    }

    fn build_proof_req_json(&self) -> Result<String, ProofError> {
        debug!("{} building proof request json for proof validation", self.source_id);
        match self.proof_request {
            Some(ref x) => {
                Ok(x.get_proof_request_data())
            },
            None => Err(ProofError::InvalidProof()),
        }
    }

    fn proof_validation(&mut self) -> Result<u32, ProofError> {
        let proof_req_msg = match self.proof_request.clone() {
            Some(x) => x,
            None => return Err(ProofError::InvalidProof()),
        };

        let proof_msg = match self.proof.clone() {
            Some(x) => x,
            None => return Err(ProofError::InvalidProof()),
        };

        let credential_data = proof_msg.get_credential_info()?;

        //if credential_data.len() == 0 {
        //    return Err(ProofError::InvalidCredData())
        //}

        let credential_def_msg = match self.build_credential_defs_json(&credential_data) {
            Ok(x) => x,
            Err(_) => format!("{{}}"),
        };

        let schemas_json = match self.build_schemas_json(&credential_data) {
            Ok(x) => x,
            Err(_) => format!("{{}}"),
        };
        let proof_json = self.build_proof_json()?;
        let proof_req_json = self.build_proof_req_json()?;
        debug!("*******\n{}\n********", credential_def_msg);
        debug!("*******\n{}\n********", schemas_json);
        debug!("*******\n{}\n********", proof_json);
        debug!("*******\n{}\n********", proof_req_json);
//        proof_compliance(&proof_req_msg.proof_request_data, &proof_msg)?;
        self.validate_proof_indy(&proof_req_json, &proof_json, &schemas_json, &credential_def_msg, "{}", "{}")
    }

    fn send_proof_request(&mut self, connection_handle: u32) -> Result<u32, ProofError> {
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("proof {} has invalid state {} for sending proofRequest", self.source_id, self.state as u32);
            return Err(ProofError::ProofNotReadyError())
        }
        debug!("sending proof request with proof: {}, and connection {}", self.source_id, connection_handle);
        self.prover_did = connection::get_pw_did(connection_handle).map_err(|ec| ProofError::InvalidConnection())?;
        self.agent_did = connection::get_agent_did(connection_handle).map_err(|ec| ProofError::InvalidConnection())?;
        self.agent_vk = connection::get_agent_verkey(connection_handle).map_err(|ec| ProofError::InvalidConnection())?;
        self.remote_vk = connection::get_their_pw_verkey(connection_handle).map_err(|ec| ProofError::InvalidConnection())?;
        self.prover_vk = connection::get_pw_verkey(connection_handle).map_err(|ec| ProofError::InvalidConnection())?;

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
            .serialize_message()
            .map_err(|ec| ProofError::ProofMessageError(ec))?;

        self.proof_request = Some(proof_obj);
        let data = connection::generate_encrypted_payload(&self.prover_vk, &self.remote_vk, &proof_request, "PROOF_REQUEST").map_err(|_| ProofError::ProofConnectionError())?;
        let title = format!("{} wants you to share {}", settings::get_config_value(settings::CONFIG_INSTITUTION_NAME).map_err(|e| ProofError::CommonError(e))?, self.name);

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
                warn!("{} could not send proofReq: {}", self.source_id, x);
                return Err(ProofError::ProofMessageError(x));
            }
        }
    }

    fn get_proof(&self) -> Result<String, ProofError> {
        Ok(self.proof.as_ref().ok_or(ProofError::InvalidHandle())?.libindy_proof.clone())
    }

    fn get_proof_request_status(&mut self) -> Result<u32, ProofError> {
        debug!("updating state for proof {} with msg_id {:?}", self.source_id, self.msg_uid);
        if self.state == VcxStateType::VcxStateAccepted {
            return Ok(self.get_state());
        }
        else if self.state != VcxStateType::VcxStateOfferSent || self.msg_uid.is_empty() || self.prover_did.is_empty() {
            return Ok(self.get_state());
        }

        let (_, payload) = messages::get_message::get_ref_msg(&self.msg_uid, &self.prover_did,
                                                         &self.prover_vk, &self.agent_did,
                                                         &self.agent_vk)
            .map_err(|ec| ProofError::ProofMessageError(ec))?;

        self.proof = match parse_proof_payload(&payload) {
            Err(err) => return Ok(self.get_state()),
            Ok(x) => Some(x),
        };

        self.state = VcxStateType::VcxStateAccepted;

        match self.proof_validation() {
            Ok(x) => {
                if self.proof_state != ProofStateType::ProofInvalid {
                    debug!("Proof format was validated for proof {}", self.source_id);
                    self.proof_state = ProofStateType::ProofValidated;
                }
            }
            Err(x) => {
                self.state = VcxStateType::VcxStateRequestReceived;
                if x == ProofError::CommonError(error::TIMEOUT_LIBINDY_ERROR.code_num) {
                    warn!("Proof {} unable to be validated", self.source_id);
                    self.proof_state = ProofStateType::ProofUndefined;
                } else {
                    warn!("Proof {} had invalid format with err {}", self.source_id, x);
                    self.proof_state = ProofStateType::ProofInvalid;
                }
            }
        };

        Ok(self.get_state())
    }

    fn update_state(&mut self) -> Result<u32, ProofError> {
        self.get_proof_request_status()
    }

    fn get_state(&self) -> u32 {let state = self.state as u32; state}

    fn get_proof_state(&self) -> u32 {let state = self.proof_state as u32; state}

    fn get_proof_uuid(&self) -> &String { &self.msg_uid }

    fn get_source_id(&self) -> &String { &self.source_id }

    fn to_string(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }

    fn from_str(s: &str) -> Result<Proof, ProofError> {
        let s:Value = serde_json::from_str(&s)
            .or(Err(ProofError::InvalidJson()))?;
        let proof: Proof = serde_json::from_value(s["data"].clone())
            .or(Err(ProofError::InvalidJson()))?;
        Ok(proof)
    }

}

pub fn create_proof(source_id: String,
                    requested_attrs: String,
                    requested_predicates: String,
                    name: String) -> Result<u32, ProofError> {

    // TODO: Get this to actually validate as json, not just check length.
    let length = requested_attrs.len();
    if length <= 0 {
        return Err(ProofError::CommonError(error::INVALID_JSON.code_num))
    }

    debug!("creating proof with source_id: {}, name: {}, requested_attrs: {}, requested_predicates: {}", source_id, name, requested_attrs, requested_predicates);

    let mut new_proof = Proof {
        source_id,
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
        nonce: generate_nonce().map_err(|ec| ProofError::CommonError(ec))?,
        proof: None,
        proof_request: None,
        remote_did: String::new(),
        remote_vk: String::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
    };

    new_proof.validate_proof_request().map_err(|ec| ProofError::CommonError(ec))?;

    new_proof.state = VcxStateType::VcxStateInitialized;

    let new_handle = PROOF_MAP.add(new_proof).map_err(|ec|ProofError::CreateProofError())?;

    Ok(new_handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    PROOF_MAP.has_handle(handle)
}

pub fn update_state(handle: u32) -> Result<u32, ProofError> {
    PROOF_MAP.get_mut(handle,|p|{
        match p.update_state() {
            Ok(x) => Ok(x),
            Err(x) => {
                warn!("could not update state for proof {}: {}", p.get_source_id(), x);
                Ok(p.get_state())
            },
        }
    }).map_err(|ec|ProofError::CommonError(ec))
}

pub fn get_state(handle: u32) -> Result<u32, ProofError> {
    PROOF_MAP.get(handle,|p|{
        Ok(p.get_state())
    }).map_err(|ec|ProofError::CommonError(ec))
}

pub fn get_proof_state(handle: u32) -> Result<u32, ProofError> {
    PROOF_MAP.get(handle,|p|{
        Ok(p.get_proof_state())
    }).map_err(|ec|ProofError::CommonError(ec))
}

pub fn release(handle: u32) -> Result<(), ProofError> {
    match PROOF_MAP.release(handle) {
        Ok(_) => Ok(()),
        Err(_) => Err(ProofError::InvalidHandle()),
    }
}

pub fn release_all() {
    match PROOF_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}
pub fn to_string(handle: u32) -> Result<String, ProofError> {
    PROOF_MAP.get(handle,|p|{
        Ok(Proof::to_string(&p))
    }).map_err(|ec|ProofError::CommonError(ec))
}

pub fn get_source_id(handle: u32) -> Result<String, ProofError> {
    PROOF_MAP.get(handle,|p|{
        Ok(p.get_source_id().clone())
    }).map_err(|ec|ProofError::CommonError(ec))
}

pub fn from_string(proof_data: &str) -> Result<u32, ProofError> {
    let derived_proof: Proof = Proof::from_str(proof_data).map_err(|err| {
        warn!("{} with serde error: {}",error::INVALID_JSON.message, err);
        ProofError::CommonError(error::INVALID_JSON.code_num)
    })?;

    let source_id = derived_proof.source_id.clone();
    let new_handle = PROOF_MAP.add(derived_proof).map_err(|ec|ProofError::CommonError(ec))?;

    Ok(new_handle)
}

pub fn send_proof_request(handle: u32, connection_handle: u32) -> Result<u32, ProofError> {
    PROOF_MAP.get_mut(handle,|p|{
        p.send_proof_request(connection_handle).map_err(|ec|ec.to_error_code())
    }).map_err(|ec|ProofError::CommonError(ec))
}

fn get_proof_details(response: &str) -> Result<String, ProofError> {
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = match json["uids"].as_array() {
                Some(x) => x[0].as_str().ok_or(ProofError::CommonError(error::INVALID_JSON.code_num))?,
                None => {
                    warn!("response had no uid");
                    return Err(ProofError::CommonError(error::INVALID_JSON.code_num))
                },
            };
            Ok(String::from(detail))
        },
        Err(_) => {
            warn!("Proof called without a valid response from server");
            return Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        },
    }
}

pub fn get_proof_uuid(handle: u32) -> Result<String,u32> {
    PROOF_MAP.get(handle,|p|{
        Ok(p.get_proof_uuid().clone())
    })
}

fn parse_proof_payload(payload: &Vec<u8>) -> Result<ProofMessage, u32> {
    debug!("parsing proof payload: {:?}", payload);
    let data = messages::extract_json_payload(payload)?;

    let my_credential_req = ProofMessage::from_str(&data).map_err(|err| {
        warn!("invalid json {}", err);
        error::INVALID_JSON.code_num
    })?;
    Ok(my_credential_req)
}

pub fn get_proof(handle: u32) -> Result<String, ProofError> {
    PROOF_MAP.get(handle,|p|{
        p.get_proof().map_err(|ec|ec.to_error_code())
    }).map_err(|ec|ProofError::CommonError(ec))
}

// TODO: This doesnt feel like it should be here (maybe utils?)
pub fn generate_nonce() -> Result<String, u32> {
    let mut bn = BigNum::new().map_err(|err| error::BIG_NUMBER_ERROR.code_num)?;

    BigNumRef::rand(&mut bn, LARGE_NONCE as i32, openssl::bn::MsbOption::MAYBE_ZERO, false)
        .map_err(|_| error::BIG_NUMBER_ERROR.code_num)?;
    Ok(bn.to_dec_str().map_err(|err| error::BIG_NUMBER_ERROR.code_num)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::httpclient;
    use connection::build_connection;
    use utils::libindy::{pool, set_libindy_rc};
    static PROOF_MSG: &str = r#"{"msg_type":"proof","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::e5fec91f-d03d-4513-813c-ab6db5715d55":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"22605045280481376895214546474258256134055560453004805058368015338423404000586901936329279496160366852115900235316791489357953785379851822281248296428005020302405076144264617943389810572564188437603815231794326272302243703078443007359698858400857606408856314183672828086906560155576666631125808137726233827430076624897399072853872527464581329767287002222137559918765406079546649258389065217669558333867707240780369514832185660287640444094973804045885379406641474693993903268791773620198293469768106363470543892730424494655747935463337367735239405840517696064464669905860189004121807576749786474060694597244797343224031","e":"70192089123105616042684481760592174224585053817450673797400202710878562748001698340846985261463026529360990669802293480312441048965520897","v":"1148619141217957986496757711054111791862691178309410923416837802801708689012670430650138736456223586898110113348220116209094530854607083005898964558239710027534227973983322542548800291320747321452329327824406430787211689678096549398458892087551551587767498991043777397791000822007896620414888602588897806008609113730393639807814070738699614969916095861363383223421727858670289337712185089527052065958362840287749622133424503902085247641830693297082507827948006947829401008622239294382186995101394791468192083810475776455445579931271665980788474331866572497866962452476638881287668931141052552771328556458489781734943404258692308937784221642452132005267809852656378394530342203469943982066011466088478895643800295937901139711103301249691253510784029114718919483272055970725860849610885050165709968510696738864528287788491998027072378656038991754015693216663830793243584350961586874315757599094357535856429087122365865868729","m":{"address2":"11774234640096848605908744857306447015748098256395922562149769943967941106193320512788344020652220849708117081570187385467979956319507248530701654682748372348387275979419669108338","city":"4853213962270369118453000522408430296589146124488849630769837449684434138367659379663124155088827069418193027370932024893343033367076071757003149452226758383807126385017161888440","address1":"12970590675851114145396120869959510754345567924518524026685086869487243290925032320159287997675756075512889990901552679591155319959039145119122576164798225386578339739435869622811","zip":"8333721522340131864419931745588776943042067606218561135102011966361165456174036379901390244538991611895455576519950813910672825465382312504250936740379785802177629077591444977329"},"m1":"92853615502250003546205004470333326341901175168428906399291824325990659330595200000112546157141090642053863739870044907457400076448073272490169488870502566172795456430489790324815765612798273406119873266684053517977802902202155082987833343670942161987285661291655743810590661447300059024966135828466539810035","m2":"14442362430453309930284822850357071315613831915865367971974791350454381198894252834180803515368579729220423713315556807632571621646127926114010380486713602821529657583905131582938"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"68430476900085482958838239880418115228681348197588159723604944078288347793331","c_list":[[179,17,2,242,194,227,92,203,28,32,255,113,112,20,5,243,9,111,220,111,21,210,116,12,167,119,253,181,37,40,143,215,140,42,179,97,75,229,96,94,54,248,206,3,48,14,61,219,160,122,139,227,166,183,37,43,197,200,28,220,217,10,65,42,6,195,124,44,164,65,114,206,51,231,254,156,170,141,21,153,50,251,237,65,147,97,243,17,157,116,213,201,80,119,106,70,88,60,55,36,33,160,135,106,60,212,191,235,116,57,78,177,61,86,44,226,205,100,134,118,93,6,26,58,220,66,232,166,202,62,90,174,231,207,19,239,233,223,70,191,199,100,157,62,139,176,28,184,9,70,116,199,142,237,198,183,12,32,53,84,207,202,77,56,97,177,154,169,223,201,212,163,212,101,184,255,215,167,16,163,136,44,25,123,49,15,229,41,149,133,159,86,106,208,234,73,207,154,194,162,141,63,159,145,94,47,174,51,225,91,243,2,221,202,59,11,212,243,197,208,116,42,242,131,221,137,16,169,203,215,239,78,254,150,42,169,202,132,172,106,179,130,178,130,147,24,173,213,151,251,242,44,54,47,208,223]]},"requested_proof":{"revealed_attrs":{"sdf":["claim::e5fec91f-d03d-4513-813c-ab6db5715d55","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}"#;
    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }

    fn create_boxed_proof() -> Box<Proof> {
        Box::new(Proof {
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
        })
    }

    #[test]
    fn test_create_proof_succeeds() {
        init!("true");

        create_proof("1".to_string(),
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
        init!("true");
        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        let proof_string = to_string(handle).unwrap();
        let s:Value = serde_json::from_str(&proof_string).unwrap();
        assert_eq!(s["version"], DEFAULT_SERIALIZE_VERSION);
        assert!(!proof_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        init!("true");
        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        let proof_data = to_string(handle).unwrap();
        let proof1: Proof = Proof::from_str(&proof_data).unwrap();
        assert!(release(handle).is_ok());

        let new_handle = from_string(&proof_data).unwrap();
        let proof2 : Proof = Proof::from_str(&to_string(new_handle).unwrap()).unwrap();
        assert_eq!(proof1, proof2);
    }

    #[test]
    fn test_release_proof() {
        init!("true");
        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert!(release(handle).is_ok());
        assert!(!is_valid_handle(handle));
    }

    #[test]
    fn test_send_proof_request() {
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();
        connection::set_agent_verkey(connection_handle, VERKEY).unwrap();
        connection::set_agent_did(connection_handle, DID).unwrap();
        connection::set_their_pw_verkey(connection_handle, VERKEY).unwrap();

        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert_eq!(send_proof_request(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_proof_uuid(handle).unwrap(), "ntc2ytb");
    }


    #[test]
    fn test_send_proof_request_fails_with_no_pw() {
        //This test has 2 purposes:
        //1. when send_proof_request fails, Ok(c.send_proof_request(connection_handle)?) returns error instead of Ok(_)
        //2. Test that when no PW connection exists, send message fails on invalid did
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "").unwrap();

        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();

        assert!(send_proof_request(handle, connection_handle).is_err());
    }

    #[test]
    fn test_get_proof_fails_with_no_proof() {
        init!("true");
        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        assert!(is_valid_handle(handle));
        assert!(get_proof(handle).is_err())
    }

    #[test]
    fn test_update_state_with_pending_proof() {
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();

        let mut proof = Box::new(Proof {
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

        proof.update_state().unwrap();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateRequestReceived as u32);
    }

    #[test]
    fn test_get_proof_returns_proof_when_proof_state_invalid() {
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();

        let mut proof = Box::new(Proof {
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
        //httpclient::set_next_u8_response(GET_PROOF_OR_CREDENTIAL_RESPONSE.to_vec());

        proof.update_state().unwrap();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateRequestReceived as u32);
        assert_eq!(proof.get_proof_state(), ProofStateType::ProofInvalid as u32);
        assert_eq!(proof.prover_did, "GxtnGN6ypZYgEqcftSQFnC");
        let proof_data = proof.get_proof().unwrap();
        assert!(proof_data.contains(r#""cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0""#));
        assert!(proof_data.contains(r#""schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0""#));
        /* converting proof to a string produces non-deterministic results */
    }

    #[test]
    fn test_build_credential_defs_json_with_multiple_credentials() {
        init!("true");
        let proof = create_boxed_proof();

        let cred1 = ("schema_key1".to_string(), "cred_def_key1".to_string(), "".to_string());
        let cred2 = ("schema_key2".to_string(), "cred_def_key2".to_string(), "".to_string());
        let cred3 = ("schema_key3".to_string(), "cred_def_key3".to_string(), "".to_string());
        let credentials = vec![cred1.clone(), cred2.clone(), cred3.clone()];
        let credential_json = proof.build_credential_defs_json(&credentials).unwrap();

        assert!(credential_json.contains(r#""cred_def_key1":{"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471""#));
        assert!(credential_json.contains(r#""cred_def_key2":{"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471""#));
        assert!(credential_json.contains(r#""cred_def_key3":{"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471""#));
    }

    #[test]
    fn test_build_schemas_json_with_multiple_schemas() {
        init!("true");
        let proof = create_boxed_proof();
        let cred1 = ("schema_key1".to_string(), "cred_def_key1".to_string(), "".to_string());
        let cred2 = ("schema_key2".to_string(), "cred_def_key2".to_string(), "".to_string());
        let cred3 = ("schema_key3".to_string(), "cred_def_key3".to_string(), "".to_string());
        let credentials = vec![cred1.clone(), cred2.clone(), cred3.clone()];
        let credential_json = proof.build_schemas_json(&credentials).unwrap();

        assert!(credential_json.contains(r#""schema_key1":{"attrNames":["height","name","sex","age"],"id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4""#));
        assert!(credential_json.contains(r#""schema_key2":{"attrNames":["height","name","sex","age"],"id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4""#));
        assert!(credential_json.contains(r#""schema_key3":{"attrNames":["height","name","sex","age"],"id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4""#));
    }

    #[test]
    fn test_get_proof() {
        init!("true");

        let mut proof_msg_obj = ProofMessage::new();
        proof_msg_obj.libindy_proof = PROOF_JSON.to_string();

        let mut proof = create_boxed_proof();
        proof.proof = Some(proof_msg_obj);

        let proof_str = proof.get_proof().unwrap();
        assert_eq!(&proof_str, PROOF_JSON);
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let h1 = create_proof("1".to_string(), REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h2 = create_proof("1".to_string(), REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h3 = create_proof("1".to_string(), REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h4 = create_proof("1".to_string(), REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        let h5 = create_proof("1".to_string(), REQUESTED_ATTRS.to_owned(), REQUESTED_PREDICATES.to_owned(), "Optional".to_owned()).unwrap();
        release_all();
        assert_eq!(release(h1).err(), Some(ProofError::InvalidHandle()));
        assert_eq!(release(h2).err(), Some(ProofError::InvalidHandle()));
        assert_eq!(release(h3).err(), Some(ProofError::InvalidHandle()));
        assert_eq!(release(h4).err(), Some(ProofError::InvalidHandle()));
        assert_eq!(release(h5).err(), Some(ProofError::InvalidHandle()));
    }

    #[ignore]
    #[test]
    fn test_proof_validation_with_predicate() {
        use utils::constants::{PROOF_LIBINDY, PROOF_REQUEST};
        init!("false");
        pool::tests::open_sandbox_pool();
        //Generated proof from a script using libindy's python wrapper

        let proof_msg: ProofMessage = serde_json::from_str(PROOF_LIBINDY).unwrap();
        let mut proof_req_msg = ProofRequestMessage::create();
        proof_req_msg.proof_request_data = serde_json::from_str(PROOF_REQUEST).unwrap();
        let mut proof = Proof {
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
        assert!(proof_data.contains(r#""schema_seq_no":694,"issuer_did":"DunkM3x1y7S4ECgSL4Wkru","credential_uuid":"claim::1f927d68-8905-4188-afd6-374b93202802","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"}}"#));
    }

    #[ignore]
    #[test]
    fn test_send_proof_request_can_be_retried() {
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();
        connection::set_agent_verkey(connection_handle, VERKEY).unwrap();
        connection::set_agent_did(connection_handle, DID).unwrap();
        connection::set_their_pw_verkey(connection_handle, VERKEY).unwrap();

        let handle = create_proof("1".to_string(),
                                  REQUESTED_ATTRS.to_owned(),
                                  REQUESTED_PREDICATES.to_owned(),
                                  "Optional".to_owned()).unwrap();
        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        assert_eq!(send_proof_request(handle, connection_handle).err(), Some(ProofError::CommonError(error::TIMEOUT_LIBINDY_ERROR.code_num)));
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateInitialized as u32);
        assert_eq!(get_proof_uuid(handle).unwrap(), "");

        // Retry sending proof request
        assert_eq!(send_proof_request(handle, connection_handle).unwrap(), 0);
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_proof_uuid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_get_proof_request_status_can_be_retried() {
        init!("true");

        let connection_handle = build_connection("test_send_proof_request").unwrap();

        let new_handle = 1;

        let mut proof = create_boxed_proof();

        httpclient::set_next_u8_response(PROOF_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_PROOF_RESPONSE.to_vec());
        //httpclient::set_next_u8_response(GET_PROOF_OR_CREDENTIAL_RESPONSE.to_vec());

        proof.get_proof_request_status().unwrap();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateRequestReceived as u32);
        assert_eq!(proof.get_proof_state(), ProofStateType::ProofInvalid as u32);

        // Changing the state and proof state to show that validation happens again
        // and resets the values to received and Invalid
        httpclient::set_next_u8_response(PROOF_RESPONSE.to_vec());
        httpclient::set_next_u8_response(UPDATE_PROOF_RESPONSE.to_vec());
        proof.state = VcxStateType::VcxStateOfferSent;
        proof.proof_state = ProofStateType::ProofUndefined;
        proof.get_proof_request_status().unwrap();
        proof.update_state().unwrap();
        assert_eq!(proof.get_state(), VcxStateType::VcxStateRequestReceived as u32);
        assert_eq!(proof.get_proof_state(), ProofStateType::ProofInvalid as u32);
    }

    #[test]
    fn test_proof_errors() {
        use utils::error::{ INVALID_JSON, POST_MSG_FAILURE };
        init!("false");

        let mut proof = create_boxed_proof();

        assert_eq!(proof.validate_proof_indy("{}", "{}", "{}", "{}","", "").err(),
                   Some(ProofError::InvalidProof()));

        let bad_handle = 100000;
        // TODO: Do something to guarantee that this handle is bad
        assert_eq!(proof.send_proof_request(bad_handle).err(),
                   Some(ProofError::ProofNotReadyError()));
        // TODO: Add test that returns a INVALID_PROOF_CREDENTIAL_DATA
        assert_eq!(proof.get_proof_request_status().err(),
                   Some(ProofError::ProofMessageError(POST_MSG_FAILURE.code_num)));

        let empty = r#""#;

        assert_eq!(create_proof("my source id".to_string(),
                                empty.to_string(),
                                "{}".to_string(),
                                "my name".to_string()).err(),
            Some(ProofError::CommonError(INVALID_JSON.code_num)));

        assert_eq!(to_string(bad_handle).err(), Some(ProofError::CommonError(error::INVALID_OBJ_HANDLE.code_num)));
        assert_eq!(get_source_id(bad_handle).err(), Some(ProofError::CommonError(error::INVALID_OBJ_HANDLE.code_num)));
        assert_eq!(from_string(empty).err(), Some(ProofError::CommonError(INVALID_JSON.code_num)));
        let mut proof_good = create_boxed_proof();
        assert_eq!(proof_good.get_proof_request_status().err(), Some(ProofError::ProofMessageError(POST_MSG_FAILURE.code_num)));
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_proof_verification() {
        init!("ledger");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schemas, cred_defs, proof_req, proof) = ::utils::libindy::anoncreds::tests::create_proof();

        let mut proof_req_obj = ProofRequestMessage::create();
        proof_req_obj.proof_request_data = serde_json::from_str(&proof_req).unwrap();

        let mut proof_msg = ProofMessage::new();
        proof_msg.libindy_proof = proof;

        let mut proof = create_boxed_proof();
        proof.proof = Some(proof_msg);
        proof.proof_request = Some(proof_req_obj);

        let rc = proof.proof_validation();

        println!("{}", serde_json::to_string(&proof).unwrap());
        assert!(rc.is_ok());
        assert_eq!(proof.proof_state,ProofStateType::ProofValidated);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_self_attested_proof_verification() {
        init!("ledger");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (proof_req, proof) = ::utils::libindy::anoncreds::tests::create_self_attested_proof();

        let mut proof_req_obj = ProofRequestMessage::create();
        proof_req_obj.proof_request_data = serde_json::from_str(&proof_req).unwrap();

        let mut proof_msg = ProofMessage::new();
        proof_msg.libindy_proof = proof;

        let mut proof = create_boxed_proof();
        proof.proof = Some(proof_msg);
        proof.proof_request = Some(proof_req_obj);

        let rc = proof.proof_validation();

        println!("{}", serde_json::to_string(&proof).unwrap());
        assert!(rc.is_ok());
        assert_eq!(proof.proof_state,ProofStateType::ProofValidated);
    }
}


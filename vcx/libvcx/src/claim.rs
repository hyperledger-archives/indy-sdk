extern crate rand;
extern crate serde_json;
extern crate libc;
extern crate serde;
extern crate rmp_serde;

use object_cache::ObjectCache;
use api::VcxStateType;
use utils::error;
use issuer_claim::ClaimOffer;

use claim_request::ClaimRequest;

use messages;
use messages::to_u8;
use messages::GeneralMessage;
use messages::send_message::parse_msg_uid;
use messages::extract_json_payload;

use utils::libindy::anoncreds::{libindy_prover_create_and_store_claim_req, libindy_prover_store_claim_offer, libindy_prover_store_claim};
use utils::libindy::SigTypes;
use utils::libindy::wallet;
use utils::libindy::crypto;

use claim_def::{ RetrieveClaimDef, ClaimDefCommon };
use connection;

use settings;
use utils::httpclient;
use utils::constants::SEND_MESSAGE_RESPONSE;

use serde_json::Value;

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<Claim>  = Default::default();
}

const LINK_SECRET_ALIAS: &str = "main";

impl Default for Claim {
    fn default() -> Claim
    {
        Claim {
            source_id: String::new(),
            state: VcxStateType::VcxStateNone,
            claim_name: None,
            claim_request: None,
            agent_did: None,
            agent_vk: None,
            my_did: None,
            my_vk: None,
            their_did: None,
            their_vk: None,
            claim_offer: None,
            link_secret_alias: Some(String::from("main")), //TODO this should not be hardcoded
            msg_uid: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    source_id: String,
    state: VcxStateType,
    claim_name: Option<String>,
    claim_request: Option<ClaimRequest>,
    claim_offer: Option<ClaimOffer>,
    link_secret_alias: Option<String>,
    msg_uid: Option<String>,
    // the following 6 are pulled from the connection object
    agent_did: Option<String>,
    agent_vk: Option<String>,
    my_did: Option<String>,
    my_vk: Option<String>,
    their_did: Option<String>,
    their_vk: Option<String>,
}

impl Claim {

    fn _find_claim_def(&self, issuer_did: &str, schema_seq_num: u32) -> Result<String, u32> {
        RetrieveClaimDef::new()
            .retrieve_claim_def("GGBDg1j8bsKmr4h5T9XqYf",
                                schema_seq_num,
                                Some(SigTypes::CL),
                                issuer_did)
    }

    fn _build_request(&self, my_did: &str, their_did: &str) -> Result<ClaimRequest, u32> {

        if self.state != VcxStateType::VcxStateRequestReceived { return Err(error::NOT_READY.code_num); }

        let wallet_h = wallet::get_wallet_handle();

        let prover_did = self.my_did.as_ref().ok_or(error::INVALID_DID.code_num)?;
        let claim_offer = self.claim_offer.as_ref().ok_or(error::INVALID_CLAIM_JSON.code_num)?;

        let claim_def = self._find_claim_def(&claim_offer.issuer_did,
                                             claim_offer.schema_seq_no)?;

        let claim_offer = serde_json::to_string(claim_offer).or(Err(error::INVALID_CLAIM_JSON.code_num))?;

        libindy_prover_store_claim_offer(wallet_h, &claim_offer)?;

        let req = libindy_prover_create_and_store_claim_req(wallet_h,
                                                            &prover_did,
                                                            &claim_offer,
                                                            &claim_def,
                                                            LINK_SECRET_ALIAS)?;

        let mut  req : Value = serde_json::from_str(&req)
            .or_else(|e|{
                error!("Unable to create claim request - libindy error: {}", e);
                Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
            })?;

        if let Value::Object(ref mut map) = req {
            map.insert(String::from("version"), Value::from("0.1"));
            map.insert(String::from("tid"), Value::from(""));
            map.insert(String::from("to_did"), Value::from(their_did));
            map.insert(String::from("from_did"), Value::from(my_did));
            map.insert(String::from("mid"), Value::from(""));
        }
        else {
            warn!("Unable to create claim request -- invalid json from libindy");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }
        Ok(serde_json::from_value(req).or(Err(error::INVALID_CLAIM_JSON.code_num))?)
    }

    fn send_request(&mut self, connection_handle: u32) -> Result<u32, u32> {
        debug!("sending claim offer via connection connection: {}", connection_handle);
        self.my_did = Some(connection::get_pw_did(connection_handle)?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle)?);
        self.agent_did = Some(connection::get_agent_did(connection_handle)?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle)?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle)?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle)?);

        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let local_their_did = self.their_did.as_ref().ok_or(e_code)?;
        let local_their_vk = self.their_vk.as_ref().ok_or(e_code)?;
        let local_agent_did = self.agent_did.as_ref().ok_or(e_code)?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(e_code)?;
        let local_my_did = self.my_did.as_ref().ok_or(e_code)?;
        let local_my_vk = self.my_vk.as_ref().ok_or(e_code)?;

        let req: ClaimRequest = self._build_request(local_my_did, local_their_did)?;
        let req = serde_json::to_string(&req).or(Err(error::INVALID_CLAIM_JSON.code_num))?;
        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &req, "CLAIM_REQ")?;
        let offer_msg_id = self.claim_offer.as_ref().unwrap().msg_ref_id.as_ref().ok_or(error::CREATE_CLAIM_REQUEST_ERROR.code_num)?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("claimReq")
            .agent_did(local_agent_did)
            .agent_vk(local_agent_vk)
            .edge_agent_payload(&data)
            .ref_msg_id(offer_msg_id)
            .send_secure() {
            Ok(response) => {
                self.msg_uid = Some(parse_msg_uid(&response[0])?);
                self.state = VcxStateType::VcxStateOfferSent;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proof: {}", x);
                return Err(x);
            }
        }
    }

    fn _check_msg(&mut self) -> Result<(), u32> {
        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let agent_did = self.agent_did.as_ref().ok_or(e_code)?;
        let agent_vk = self.agent_vk.as_ref().ok_or(e_code)?;
        let my_did = self.my_did.as_ref().ok_or(e_code)?;
        let my_vk = self.my_vk.as_ref().ok_or(e_code)?;
        let msg_uid = self.msg_uid.as_ref().ok_or(e_code)?;

        let payload = messages::get_message::get_all_message(my_did,
                                                         my_vk,
                                                         agent_did,
                                                         agent_vk)?;


        for msg in payload {
            if msg.msg_type.eq("claim") {
                match msg.payload {
                    Some(ref data) => {
                        let data = to_u8(data);
                        let data = crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice())?;

                        let claim = extract_json_payload(&data)?;
                        let claim: Value = serde_json::from_str(&claim).or(Err(error::INVALID_CLAIM_JSON.code_num)).unwrap();

                        let wallet_h = wallet::get_wallet_handle();

                        let claim = serde_json::to_string_pretty(&claim).unwrap();
                        libindy_prover_store_claim(wallet_h, &claim)?;
                        debug!("received claim");
                        self.state = VcxStateType::VcxStateAccepted;
                    },
                    None => return Err(error::INVALID_HTTP_RESPONSE.code_num)
                };
            }
        }
        Ok(())
    }

    fn update_state(&mut self) {
        match self.state {
            VcxStateType::VcxStateOfferSent => {
                //Check for messages
                let _ = self._check_msg();
            },
            VcxStateType::VcxStateAccepted => {
                //Check for revocation
            }
            _ => {
                // NOOP there is nothing the check for a changed state
            }
        }
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }

    fn get_source_id(&self) -> String {self.source_id.clone()}

    fn set_claim_offer(&mut self, offer: ClaimOffer){
        self.claim_offer = Some(offer);
    }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(code_num: u32) -> u32 {
    if code_num == error::INVALID_OBJ_HANDLE.code_num {
        error::INVALID_CLAIM_HANDLE.code_num
    }
    else {
        code_num
    }
}

pub fn claim_create_with_offer(source_id: &str, offer: &str) -> Result<u32, u32> {
    let mut new_claim = _claim_create(source_id);

    let offer: ClaimOffer = serde_json::from_str(offer).map_err(|_|error::INVALID_JSON.code_num)?;
    new_claim.set_claim_offer(offer);

    new_claim.state = VcxStateType::VcxStateRequestReceived;

    debug!("inserting claim into handle map");
    Ok(HANDLE_MAP.add(new_claim)?)
}

fn _claim_create(source_id: &str) -> Claim {

    let mut new_claim: Claim = Default::default();

    new_claim.state = VcxStateType::VcxStateInitialized;
    new_claim.set_source_id(source_id);

    new_claim
}

pub fn update_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.update_state();
        Ok(error::SUCCESS.code_num)
    })

}

pub fn get_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

pub fn send_claim_request(handle: u32, connection_handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get_mut(handle, |obj| {
        obj.send_request(connection_handle)
    }).map_err(handle_err)
}

pub fn get_claim_offer_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, u32> {
    let my_did = connection::get_pw_did(connection_handle)?;
    let my_vk = connection::get_pw_verkey(connection_handle)?;
    let agent_did = connection::get_agent_did(connection_handle)?;
    let agent_vk = connection::get_agent_verkey(connection_handle)?;

    let payload = messages::get_message::get_all_message(&my_did,
                                                     &my_vk,
                                                     &agent_did,
                                                     &agent_vk)?;

    let mut messages: Vec<ClaimOffer> = Default::default();

    for msg in payload {
        if msg.msg_type.eq("claimOffer") {
            let msg_data = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice())?
                },
                None => return Err(error::INVALID_MESSAGES.code_num)
            };

            let offer = extract_json_payload(&msg_data)?;

            let mut offer: ClaimOffer = serde_json::from_str(&offer)
                .or(Err(error::INVALID_JSON.code_num))?;

            offer.msg_ref_id = Some(msg.uid.to_owned());
            messages.push(offer);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).unwrap())
}

pub fn release(handle: u32) -> Result<(), u32> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj|{
        serde_json::to_string(&obj).map_err(|e|{
            warn!("Unable to serialize: {:?}", e);
            error::SERIALIZATION_ERROR.code_num
        })
    })
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id())
    }).map_err(handle_err)
}

pub fn from_string(claim_data: &str) -> Result<u32, u32> {
    let claim: Claim = match serde_json::from_str(claim_data) {
        Ok(x) => x,
        Err(y) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = HANDLE_MAP.add(claim)?;

    debug!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use utils::httpclient;
    use issuer_claim;
    use std::thread;
    use std::time::Duration;
    use api::VcxStateType;

    pub const BAD_CLAIM_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    #[test]
    fn test_claim_defaults() {
        let claim = Claim::default();
        match claim._build_request("test1","test2") {
            Ok(_) => panic!("test should fail!"),
            Err(x) => assert_eq!(x, error::NOT_READY.code_num),
        };
    }

    #[test]
    fn test_claim_create_with_offer() {
        let handle = claim_create_with_offer("test_claim_create_with_offer", ::utils::constants::CLAIM_OFFER_JSON).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_claim_create_with_bad_offer() {
        match claim_create_with_offer("test_claim_create_with_bad_offer",BAD_CLAIM_OFFER) {
            Ok(_) => panic!("should have failed with bad claim offer"),
            Err(x) => assert_eq!(x,error::INVALID_JSON.code_num),
        };
    }

    #[test]
    fn test_claim_serialize_deserialize() {
        let handle = claim_create_with_offer("test_claim_serialize_deserialize", ::utils::constants::CLAIM_OFFER_JSON).unwrap();
        let claim_string = to_string(handle).unwrap();
        release(handle).unwrap();
        match release(handle) {
            Ok(_) => panic!("should have failed"),
            Err(x) => assert_eq!(x,error::INVALID_CLAIM_HANDLE.code_num),
        };
        let handle = from_string(&claim_string).unwrap();
        assert_eq!(claim_string,to_string(handle).unwrap());
    }

    #[test]
    fn full_claim_test(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        wallet::init_wallet("full_claim_test").unwrap();

        let connection_h = connection::build_connection("test_send_claim_offer").unwrap();

        httpclient::set_next_u8_response(::utils::constants::NEW_CLAIM_OFFER_RESPONSE.to_vec());

        let offers = get_claim_offer_messages(connection_h, None).unwrap();
        println!("{}", offers);
        let offers:Value = serde_json::from_str(&offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();

        let c_h = claim_create_with_offer("TEST_CLAIM", &offers).unwrap();
        assert_eq!(3, get_state(c_h).unwrap());

        send_claim_request(c_h, connection_h).unwrap();

        assert_eq!(2, get_state(c_h).unwrap());

        httpclient::set_next_u8_response(::utils::constants::CLAIM_RESPONSE.to_vec());

        update_state(c_h).unwrap();
        assert_eq!(4, get_state(c_h).unwrap());

        wallet::delete_wallet("full_claim_test").unwrap();
    }

    #[test]
    fn test_real_claim() {
        settings::set_to_defaults();
        //BE INSTITUTION AND GENERATE INVITE FOR CONSUMER
        ::utils::devsetup::setup_dev_env("test_real_claim");
        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(wallet::get_wallet_handle(), LINK_SECRET_ALIAS).unwrap();
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
        let claim_offer = issuer_claim::issuer_claim_create(22,
                                                            None,
                                                            settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                            "claim_name".to_string(),
                                                            claim_data.to_owned()).unwrap();
        issuer_claim::send_claim_offer(claim_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER SEND CLAIM REQUEST
        ::utils::devsetup::be_consumer();
        let claim_offers = get_claim_offer_messages(faber, None).unwrap();
        let offers:Value = serde_json::from_str(&claim_offers).unwrap();
        let offers = serde_json::to_string(&offers[0]).unwrap();
        println!("claim_offer: {}", offers);
        let claim = claim_create_with_offer("TEST_CLAIM", &offers).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(claim).unwrap());
        send_claim_request(claim, faber).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS INSTITUTION SEND CLAIM
        ::utils::devsetup::be_institution();
        issuer_claim::update_state(claim_offer);
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, issuer_claim::get_state(claim_offer));
        issuer_claim::send_claim(claim_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER STORE CLAIM
        ::utils::devsetup::be_consumer();
        update_state(claim).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(claim).unwrap());
        ::utils::devsetup::cleanup_dev_env("test_real_claim");
    }
}

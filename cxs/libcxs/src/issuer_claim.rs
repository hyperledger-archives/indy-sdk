extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use messages;
use settings;
use messages::GeneralMessage;
use connection;
use claim_request::ClaimRequest;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuerClaim {
    source_id: String,
    handle: u32,
    claim_attributes: String,
    msg_uid: String,
    schema_seq_no: u32,
    issuer_did: String,
    issued_did: String,
    state: CxsStateType,
    claim_request: Option<ClaimRequest>,
}

impl IssuerClaim {
    fn validate_claim_offer(&self) -> Result<u32, String> {
        //TODO: validate claim_attributes against claim_def
        Ok(error::SUCCESS.code_num)
    }

    fn send_claim_offer(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateInitialized {
            warn!("claim {} has invalid state {} for sending claimOffer", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_claim_offer", connection_handle);
            return Err(error::INVALID_CONNECTION_HANDLE.code_num);
        }

        //TODO: call to libindy to encrypt payload
        let to_did = connection::get_pw_did(connection_handle).unwrap();
        let from_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT).unwrap();
        let payload = format!("{{\"msg_type\":\"CLAIM_OFFER\",\"version\":\"0.1\",\"to_did\":\"{}\",\"from_did\":\"{}\",\"claim\":{},\"schema_seq_no\":{},\"issuer_did\":\"{}\"}}",to_did,from_did,self.claim_attributes,self.schema_seq_no,self.issuer_did);

        match messages::send_message().to(&to_did).msg_type("claimOffer").edge_agent_payload(&payload).send() {
            Err(x) => {
                warn!("could not send claimOffer: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = match get_offer_details(&response) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };
                self.issued_did = to_did;
                self.state = CxsStateType::CxsStateOfferSent;
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn send_claim(&mut self, connection_handle: u32) -> Result<u32, u32> {
        if self.state != CxsStateType::CxsStateRequestReceived {
            warn!("claim {} has invalid state {} for sending claim", self.handle, self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_claim_offer", connection_handle);
            return Err(error::INVALID_CONNECTION_HANDLE.code_num);
        }

        //TODO: call to libindy to encrypt payload
        let data = format!("{{ \"claimDefNo\":\"{}\",\"claimAttributes\":\"{}\"}}",self.schema_seq_no,self.claim_attributes);
        let to = connection::get_pw_did(connection_handle).unwrap();

        match messages::send_message().to(&to).msg_type("claim").edge_agent_payload(&data).send() {
            Err(x) => {
                warn!("could not send claim: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = match get_offer_details(&response) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };
                self.issued_did = to;
                self.state = CxsStateType::CxsStateAccepted;
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn get_claim_req(&mut self, msg_uid: &str) {
        info!("Checking for outstanding claimReq for {} with uid: {}", self.handle, msg_uid);
         let response = match messages::get_messages().to(&self.issued_did).uid(msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for claim {}", self.handle);
                return
            },
        };

        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for claim {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for claim {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["typ"] == String::from("claimReq") {
                //get the followup-claim-req using refMsgId
                self.state = CxsStateType::CxsStateRequestReceived;
                let payload: serde_json::Value = match serde_json::from_str(&msg["edgeAgentPayload"].as_str().unwrap()){
                    Ok(x) => x,
                    Err(_) => {
                        warn!("invalid json for claimReq's edgeAgentPayload");
                        return
                    },
                };
                self.claim_request = Some(ClaimRequest::create_from_api_msg(&payload));
                return
            }
        }

        info!("no claimReqs found for claim {}", self.handle);
    }

    fn get_claim_offer_status(&mut self) {
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        else if self.state != CxsStateType::CxsStateOfferSent || self.msg_uid.is_empty() || self.issued_did.is_empty() {
            return;
        }

        // state is "OfferSent" so check to see if there is a new claimReq
        let response = match messages::get_messages().to(&self.issued_did).uid(&self.msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for claim {}", self.handle);
                return
            },
        };
        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for claim {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for claim {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["statusCode"].to_string() == "\"MS-104\"" {
                //get the followup-claim-req using refMsgId
                self.get_claim_req(&msg["refMsgId"].as_str().unwrap());
            }
        }
    }

    fn update_state(&mut self) {
        self.get_claim_offer_status();
        //There will probably be more things here once we do other things with the claim
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn get_offer_uid(&self) -> String { self.msg_uid.clone() }
    fn set_offer_uid(&mut self, uid: &str) {self.msg_uid = uid.to_owned();}
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(claim) => Ok(claim.get_offer_uid()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn issuer_claim_create(schema_seq_no: u32,
                           source_id: Option<String>,
                           issuer_did: String,
                           claim_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        claim_attributes: claim_data,
        issued_did: String::new(),
        issuer_did,
        state: CxsStateType::CxsStateNone,
        schema_seq_no,
        claim_request: None,
    });

    match new_issuer_claim.validate_claim_offer() {
        Ok(_) => info!("successfully validated issuer_claim {}", new_handle),
        Err(x) => return Err(x),
    };

    new_issuer_claim.state = CxsStateType::CxsStateInitialized;

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, new_issuer_claim);
    }

    Ok(new_handle)
}

pub fn update_state(handle: u32) {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match ISSUER_CLAIM_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_ISSUER_CLAIM_HANDLE.code_num,
    }
}

pub fn is_valid_handle(handle: u32) -> bool {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(serde_json::to_string(&c).unwrap().to_owned()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn from_string(claim_data: &str) -> Result<u32,u32> {
    let derived_claim: IssuerClaim = match serde_json::from_str(claim_data) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = derived_claim.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let claim = Box::from(derived_claim);

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, claim);
    }

    Ok(new_handle)
}

pub fn send_claim_offer(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => match c.send_claim_offer(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(x) => Err(x),
        },
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn send_claim(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
        Some(c) => match c.send_claim(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(x) => Err(x),
        },
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

fn get_offer_details(response: &str) -> Result<String,u32> {
    if settings::test_mode_enabled() {return Ok("test_mode_response".to_owned());}
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["uid"];
            Ok(String::from(detail.as_str().unwrap()))
        },
        Err(_) => {
            info!("Connect called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;
    use settings;
    use connection::create_connection;
    use std::thread;
    use std::time::Duration;
    use super::*;

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
    }

    #[test]
    fn test_issuer_claim_create_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        match issuer_claim_create(0,
                                  None,
                                  "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                  "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0,1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_claim_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle,"8XFh8yBzrpJQmNyZzgoTqB");

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claimOffer\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(send_claim_offer(handle,connection_handle).unwrap(),error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(500));
        assert_eq!(get_state(handle),CxsStateType::CxsStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(),"6a9u7Jt");
        _m.assert();
    }

    #[test]
    fn test_send_claim() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claim\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let mut claim = IssuerClaim {
            handle: 123,
            source_id: "test_has_pending_claim_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issuer_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateRequestReceived,
            claim_request: None,
            };

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle,"8XFh8yBzrpJQmNyZzgoTqB");

        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0,0),
            Err(x) => assert_eq!(x,0),
        };
        _m.assert();
        assert_eq!(claim.state,CxsStateType::CxsStateAccepted);
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
        release(handle);
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_handle,handle);
        assert_eq!(new_string,string);
    }

    #[test]
    fn test_update_state_with_pending_claim_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let response = "{\"msgs\":[{\"uid\":\"6gmsuWZ\",\"typ\":\"conReq\",\"statusCode\":\"MS-102\",\"statusMsg\":\"message sent\"},\
        {\"statusCode\":\"MS-104\",\"edgeAgentPayload\":\"{\\\"attr\\\":\\\"value\\\"}\",\"sendStatusCode\":\"MSS-101\",\"typ\":\"claimOffer\",\"statusMsg\":\"message accepted\",\"uid\":\"6a9u7Jt\",\"refMsgId\":\"CKrG14Z\"},\
        {\"msg_type\":\"CLAIM_REQUEST\",\"typ\":\"claimReq\",\"edgeAgentPayload\":\"{\\\"blinded_ms\\\":{\\\"prover_did\\\":\\\"FQ7wPBUgSPnDGJnS1EYjTK\\\",\\\"u\\\":\\\"923...607\\\",\\\"ur\\\":\\\"null\\\"},\\\"version\\\":\\\"0.1\\\",\\\"mid\\\":\\\"\\\",\\\"to_did\\\":\\\"BnRXf8yDMUwGyZVDkSENeq\\\",\\\"from_did\\\":\\\"GxtnGN6ypZYgEqcftSQFnC\\\",\\\"iid\\\":\\\"cCanHnpFAD\\\",\\\"issuer_did\\\":\\\"QTrbV4raAcND4DWWzBmdsh\\\",\\\"schema_seq_no\\\":48,\\\"optional_data\\\":{\\\"terms_of_service\\\":\\\"<Large block of text>\\\",\\\"price\\\":6}}\"}]}";
        let _m = mockito::mock("POST", "/agency/route")
        .with_status(200)
        .with_body(response)
        .expect(2)
        .create();

        let mut claim = IssuerClaim {
        handle: 123,
        source_id: "test_has_pending_claim_request".to_owned(),
        schema_seq_no: 32,
        msg_uid: "1234".to_owned(),
        claim_attributes: "nothing".to_owned(),
        issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
        issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
        state: CxsStateType::CxsStateOfferSent,
        claim_request: None,
        };

        claim.update_state();
        _m.assert();
        assert_eq !(claim.get_state(), CxsStateType::CxsStateRequestReceived as u32);
        let claim_request = claim.claim_request.unwrap();
        assert_eq!(claim_request.issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(claim_request.schema_seq_no, "48");
    }

    #[test]
    fn test_issuer_claim_changes_state_after_being_validated(){
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0,
                                         None,
                                         "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                         "{\"att\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        fn get_state_from_string(s:String)-> u32 {
            let json: serde_json::Value = serde_json::from_str(&s).unwrap();
            if json["state"].is_number() {
                return json["state"].as_u64().unwrap() as u32
            }
            0
        }
        assert_eq!(get_state_from_string(string), 1);
    }

}


extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use messages;
use messages::GeneralMessage;
use connection;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuerClaim {
    source_id: String,
    handle: u32,
    claim_def: u32,
    claim_attributes: String,
    claim_offer_uid: String,
    issued_did: String,
    state: CxsStateType,
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
        let data = format!("{{ \"claimDefNo\":\"{}\",\"claimAttributes\":\"{}\"}}",self.claim_def,self.claim_attributes);
        let to = connection::get_pw_did(connection_handle).unwrap();

        match messages::send_message().to(&to).msg_type("claimOffer").edge_agent_payload(&data).send() {
            Err(x) => {
                warn!("could not send claimOffer: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.claim_offer_uid = match get_offer_details(&response) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };
                self.issued_did = to;
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
        let data = format!("{{ \"claimDefNo\":\"{}\",\"claimAttributes\":\"{}\"}}",self.claim_def,self.claim_attributes);
        let to = connection::get_pw_did(connection_handle).unwrap();

        match messages::send_message().to(&to).msg_type("claim").edge_agent_payload(&data).send() {
            Err(x) => {
                warn!("could not send claim: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.claim_offer_uid = match get_offer_details(&response) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };
                self.issued_did = to;
                self.state = CxsStateType::CxsStateAccepted;
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn get_pending_claim_req(&mut self) {
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        else if self.state != CxsStateType::CxsStateOfferSent || self.claim_offer_uid.is_empty() || self.issued_did.is_empty() {
            return;
        }

        // state is "OfferSent" so check to see if there is a new claimReq
        let response = match messages::get_messages().to(&self.issued_did).uid(&self.claim_offer_uid).send() {
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
            if msg["typ"].to_string() == "\"claimReq\"" {
                self.state = CxsStateType::CxsStateRequestReceived;
                //TODO: store the claim request, blinded-master-secret, etc
                return
            }
        }

        info!("no claimReqs found for claim {}", self.handle);
    }

    fn update_state(&mut self) {
        self.get_pending_claim_req();
        //There will probably be more things here once we do other things with the claim
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn get_offer_uid(&self) -> String { self.claim_offer_uid.clone() }
    fn set_offer_uid(&mut self, uid: &str) {self.claim_offer_uid = uid.to_owned();}
}

pub fn get_offer_uid(handle: u32) -> Result<String,u32> {
    match ISSUER_CLAIM_MAP.lock().unwrap().get(&handle) {
        Some(claim) => Ok(claim.get_offer_uid()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn issuer_claim_create(claim_def_handle: u32,
                           source_id: Option<String>,
                           claim_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        source_id: source_id_unwrap,
        claim_def: claim_def_handle,
        claim_offer_uid: String::new(),
        claim_attributes: claim_data,
        issued_did: String::new(),
        state: CxsStateType::CxsStateNone,
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
        None => error::INVALID_CONNECTION_HANDLE.code_num,
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
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
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
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["uid"];
            Ok(detail.to_string())
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
        match issuer_claim_create(0, None, "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0,1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
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

        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(send_claim_offer(handle,connection_handle).unwrap(),error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(500));
        assert_eq!(get_state(handle),CxsStateType::CxsStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(),"\"6a9u7Jt\"");
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
            claim_def: 32,
            claim_offer_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateRequestReceived,
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
        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
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
            {\"uid\":\"6a8S8EE\",\"typ\":\"conReq\",\"statusCode\":\"MS-104\",\"statusMsg\":\"message accepted\"},\
            {\"statusCode\":\"MS-104\",\"edgeAgentPayload\":\"{\\\"attr\\\":\\\"value\\\"}\",\"sendStatusCode\":\"MSS-101\",\"typ\":\"claimOffer\",\"statusMsg\":\"message accepted\",\"uid\":\"6a9u7Jt\"},\
            {\"statusCode\":\"MS-103\",\"edgeAgentPayload\":\"{\\\"attr\\\":\\\"value\\\"}\",\"typ\":\"claimReq\",\"statusMsg\":\"message pending\",\"uid\":\"CCBXoDR\"}]}";

        let _m = mockito::mock("POST", "/agency/route")
        .with_status(200)
        .with_body(response)
        .expect(1)
        .create();

        let mut claim = IssuerClaim {
        handle: 123,
        source_id: "test_has_pending_claim_request".to_owned(),
        claim_def: 32,
        claim_offer_uid: "1234".to_owned(),
        claim_attributes: "nothing".to_owned(),
        issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
        state: CxsStateType::CxsStateOfferSent,
        };

        claim.update_state();
        _m.assert();
        assert_eq ! (claim.get_state(), CxsStateType::CxsStateRequestReceived as u32);
    }

    #[test]
    fn test_issuer_claim_changes_state_after_being_validated(){
        ::utils::logger::LoggerUtils::init();
        set_default_and_enable_test_mode();
        let handle = issuer_claim_create(0, None, "{\"att\":\"value\"}".to_owned()).unwrap();
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


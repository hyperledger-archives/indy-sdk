extern crate rand;
extern crate serde_json;
extern crate libc;

use utils::wallet;
use utils::error;
use utils::httpclient;
use api::CxsStateType;
use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;
use settings;
use messages::GeneralMessage;
use messages;

lazy_static! {
    static ref CONNECTION_MAP: Mutex<HashMap<u32, Box<Connection>>> = Default::default();
}

#[derive(Serialize, Deserialize)]
struct ConnectionOptions {
    #[serde(default)]
    connection_type: String,
    #[serde(default)]
    phone: String,
}

#[derive(Serialize, Deserialize)]
struct Connection {
    source_id: String,
    handle: u32,
    pw_did: String,
    pw_verkey: String,
    did_endpoint: String,
    state: CxsStateType,
    uuid: String,
    endpoint: String,
    // For QR code invitation
    invite_detail: String,
}

impl Connection {
    fn connect(&mut self, options: String) -> u32 {
        info!("handle {} called connect", self.handle);
        if self.state != CxsStateType::CxsStateInitialized {
            info!("connection {} in state {} not ready to connect",self.handle,self.state as u32);
            return error::NOT_READY.code_num;
        }

        let options_obj: ConnectionOptions = match serde_json::from_str(options.trim()) {
            Ok(val) => val,
            Err(_) => return error::INVALID_OPTION.code_num
        };

        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = match messages::send_invite()
            .to(&self.pw_did)
            .key_delegate("key")
            .phone_number(&options_obj.phone)
            .serialize_message(){
            Ok(x) => x,
            Err(x) => return x
        };

        match httpclient::post(&json_msg,&url) {
            Err(_) => {
                println!("better message");
                return error::POST_MSG_FAILURE.code_num
            },
            Ok(response) => {
                self.state = CxsStateType::CxsStateOfferSent;
                self.invite_detail = get_invite_detail(&response);
                return error::SUCCESS.code_num;
            }
        }
    }

    fn get_state(&self) -> u32 { self.state as u32 }
    fn set_pw_did(&mut self, did: &str) { self.pw_did = did.to_string(); }
    fn set_state(&mut self, state: CxsStateType) { self.state = state; }
    fn get_pw_did(&self) -> String { self.pw_did.clone() }
    fn get_pw_verkey(&self) -> String { self.pw_verkey.clone() }
    fn set_pw_verkey(&mut self, verkey: &str) { self.pw_verkey = verkey.to_string(); }
    fn get_uuid(&self) -> String { self.uuid.clone() }
    fn get_endpoint(&self) -> String { self.endpoint.clone() }
    fn set_uuid(&mut self, uuid: &str) { self.uuid = uuid.to_string(); }
    fn set_endpoint(&mut self, endpoint: &str) { self.endpoint = endpoint.to_string(); }
}

fn find_connection(source_id: &str) -> Result<u32,u32> {
    for (handle, connection) in CONNECTION_MAP.lock().unwrap().iter() { //TODO this could be very slow with lots of objects
        if connection.source_id == source_id {
            return Ok(*handle);
        }
    };

    Err(0)
}

pub fn is_valid_handle(handle: u32) -> bool {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn set_pw_did(handle: u32, did: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_pw_did(did),
        None => {}
    };
}

pub fn set_state(handle: u32, state: CxsStateType) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_state(state),
        None => {}
    };
}

pub fn get_pw_did(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_did()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn get_uuid(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_uuid()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn set_uuid(handle: u32, uuid: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_uuid(uuid),
        None => {}
    };
}

pub fn set_endpoint(handle: u32, endpoint: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_endpoint(endpoint),
        None => {}
    };
}

pub fn set_pw_verkey(handle: u32, verkey: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_pw_verkey(verkey),
        None => {}
    };
}

pub fn get_endpoint(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_endpoint()),
        None => Err(error::NO_ENDPOINT.code_num),
    }
}

pub fn get_pw_verkey(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_verkey()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn get_state(handle: u32) -> u32 {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None=> CxsStateType::CxsStateNone as u32,
    }
}

pub fn create_agent_pairwise(handle: u32) -> Result<u32, u32> {
    let enterprise_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENCY).unwrap();
    let pw_did = match get_pw_did(handle) {
        Ok(x) => x,
        Err(x) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let pw_verkey = get_pw_verkey(handle).unwrap();
    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let json_msg = match messages::create_keys()
        .for_did(&pw_did)
        .to(&enterprise_did)
        .for_verkey(&pw_verkey)
        .nonce("anything")
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    match httpclient::post(&json_msg, &url) {
        Ok(_) => return Ok(error::SUCCESS.code_num),
        Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
    }
}

pub fn update_agent_profile(handle: u32) -> Result<u32, u32> {
    let enterprise_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT).unwrap();
    let pw_did = match get_pw_did(handle) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let json_msg = match messages::update_data()
        .to(&pw_did)
        .name(&settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME).unwrap())
        .logo_url(&settings::get_config_value(settings::CONFIG_LOGO_URL).unwrap())
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return Err(x)
    };

    match httpclient::post(&json_msg, &url) {
        Ok(_) => return Ok(error::SUCCESS.code_num),
        Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
    }
}

//TODO may want to split between the code path where did is pass and is not passed
pub fn build_connection(source_id: String) -> u32 {
    // Check to make sure source_id is unique

    let new_handle = match find_connection(&source_id) {
        Ok(x) => return x,
        Err(_) => rand::thread_rng().gen::<u32>(),
    };

    info!("creating connection with handle {} and id {}", new_handle, source_id);
    // This is a new connection

    let c = Box::new(Connection {
        source_id,
        handle: new_handle,
        pw_did: String::new(),
        pw_verkey: String::new(),
        did_endpoint: String::new(),
        state: CxsStateType::CxsStateNone,
        uuid: String::new(),
        endpoint: String::new(),
        invite_detail: String::new(),
    });

    {
        let mut m = CONNECTION_MAP.lock().unwrap();
        m.insert(new_handle, c);
    }

    match wallet::create_and_store_my_did(new_handle, "{}") {
        Ok(_) => info!("successfully created new did"),
        Err(x) => error!("could not create DID: {}", x),
    };

    new_handle
}

pub fn update_state(handle: u32) -> u32{
    let pw_did = match get_pw_did(handle) {
        Ok(did) => did,
        Err(x) => return x,
    };

    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let json_msg = match messages::get_messages()
        .to(&pw_did)
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return x,
    };

    match httpclient::post(&json_msg, &url) {
        Err(_) => {error::POST_MSG_FAILURE.code_num}
        Ok(response) => {
            if response.contains("message accepted") { set_state(handle, CxsStateType::CxsStateAccepted); }
            error::SUCCESS.code_num
            //TODO: add expiration handling
        }
    }
}

pub fn connect(handle: u32, options: String) -> u32 {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.connect(options),
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    }
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(t) => Ok(serde_json::to_string(&t).unwrap()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn from_string(connection_data: &str) -> Result<u32,u32> {
    let derived_connection: Connection = match serde_json::from_str(connection_data) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };

    let new_handle = derived_connection.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}

    let connection = Box::from(derived_connection);

    {
        let mut m = CONNECTION_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, connection);
    }

    Ok(new_handle)
}

pub fn release(handle: u32) -> u32 {
    match CONNECTION_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    }
}

fn get_invite_detail(response: &str) -> String {
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["inviteDetail"];
            detail.to_string()
        }
        Err(_) => {
            info!("Connect called without a valid response from server");
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;
    use super::*;
    use utils::wallet;
    use std::thread;
    use std::time::Duration;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }

    #[test]
    fn test_create_connection() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(3)
            .create();

        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        wallet::tests::make_wallet("test_create_connection");
        let handle = build_connection("test_create_connection".to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), CxsStateType::CxsStateInitialized as u32);
        connect(handle, "{}".to_string());
        _m.assert();

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("message accepted")
            .expect(1)
            .create();

        assert_eq!(update_state(handle),error::SUCCESS.code_num);
        assert_eq!(get_state(handle), CxsStateType::CxsStateAccepted as u32);
        wallet::tests::delete_wallet("test_create_connection");
        _m.assert();
        release(handle);
    }

    #[test]
    fn test_create_idempotency() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_create_idempotency".to_owned());
        let handle2 = build_connection("test_create_idempotency".to_owned());
        assert_eq!(handle,handle2);
        release(handle);
        release(handle2);
    }

    #[test]
    fn test_create_drop_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_create_drop_create".to_owned());
        let did1 = get_pw_did(handle).unwrap();
        release(handle);
        let handle2 = build_connection("test_create_drop_create".to_owned());
        assert_ne!(handle,handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        release(handle2);
    }

    #[test]
    fn test_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxn_release".to_owned());
        assert!(handle > 0);
        let rc = release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_state_not_connected() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_state_not_connected".to_owned());
        thread::sleep(Duration::from_secs(1));
        let state = get_state(handle);
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
        release(handle);
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);
        assert_eq!(state, CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        match to_string(0) {
            Ok(_) => assert_eq!(1,0), //fail if we get here
            Err(_) => assert_eq!(0,0),
        };
    }

    #[test]
    fn test_set_get_pw_verkey() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_set_get_pw_verkey".to_owned());
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        set_pw_verkey(handle, &"HELLODOLLY");
        assert!(!get_pw_did(handle).unwrap().is_empty());
        release(handle);
    }

    #[test]
    fn test_create_agent_pairwise() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = rand::thread_rng().gen::<u32>();

        let c = Box::new(Connection {
            source_id: "1".to_string(),
            handle,
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            did_endpoint: String::new(),
            state: CxsStateType::CxsStateNone,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: String::new(),
        });

        {
            let mut m = CONNECTION_MAP.lock().unwrap();
            m.insert(handle, c);
        }

        match create_agent_pairwise(handle) {
            Ok(x) => assert_eq!(x, error::SUCCESS.code_num),
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
        };
    }

    #[test]
    fn test_create_agent_profile() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = rand::thread_rng().gen::<u32>();

        let c = Box::new(Connection {
            source_id: "1".to_string(),
            handle: handle,
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            did_endpoint: String::new(),
            state: CxsStateType::CxsStateNone,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: String::new(),
        });

        {
            let mut m = CONNECTION_MAP.lock().unwrap();
            m.insert(handle, c);
        }

        match update_agent_profile(handle) {
            Ok(x) => assert_eq!(x, error::SUCCESS.code_num),
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
        };
        release(handle);
    }

    #[test]
    fn test_get_set_uuid_and_endpoint() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let uuid = "THISISA!UUID";
        let endpoint = "hello";
        let test_name = "test_get_set_uuid_and_endpoint";
        let wallet_name = test_name;
        let handle = build_connection(test_name.to_owned());
        assert_eq!(get_endpoint(handle).unwrap(), "");
        set_uuid(handle, uuid);
        set_endpoint(handle, endpoint);
        assert_eq!(get_uuid(handle).unwrap(), uuid);
        assert_eq!(get_endpoint(handle).unwrap(), endpoint);
        release(handle);
    }

    #[test]
    fn test_get_qr_code_data() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let test_name = "test_get_qr_code_data";
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(2)
            .create();

        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        wallet::tests::make_wallet(test_name);
        let handle = build_connection(test_name.to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), CxsStateType::CxsStateInitialized as u32);
        _m.assert();

        let response = "{ \"inviteDetail\": {
                \"senderEndpoint\": \"34.210.228.152:80\",
                \"connReqId\": \"CXqcDCE\",
                \"senderAgentKeyDlgProof\": \"sdfsdf\",
                \"senderName\": \"Evernym\",
                \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
                \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
                \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
                \"targetName\": \"there\"
            }}";

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body(response)
            .expect(1)
            .create();

        connect(handle, "{}".to_string());
        let data = to_string(handle).unwrap();
        info!("Data from to_string(i.e. 'get_data()'{}", data);
        assert!(data.contains("there"));

        _m.assert();

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("message accepted")
            .expect(1)
            .create();

        assert_eq!(update_state(handle),error::SUCCESS.code_num);
        assert_eq!(get_state(handle), CxsStateType::CxsStateAccepted as u32);
        wallet::tests::delete_wallet(test_name);
        _m.assert();
        release(handle);
    }

    #[test]
    fn test_jsonfying_invite_details() {
        let response = "{ \"inviteDetail\": {
                \"senderEndpoint\": \"34.210.228.152:80\",
                \"connReqId\": \"CXqcDCE\",
                \"senderAgentKeyDlgProof\": \"sdfsdf\",
                \"senderName\": \"Evernym\",
                \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
                \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
                \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
                \"targetName\": \"there\"
            }}";

        let invite_detail = get_invite_detail(response);
        info!("Invite Detail Test: {}", invite_detail);
        assert!(invite_detail.contains("sdfsdf"));
    }

    #[test]
    fn test_serialize_deserialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_serialize_deserialize".to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let first_string = to_string(handle).unwrap();
        release(handle);
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        release(handle);
        println!("{}",first_string);
        println!("{}",second_string);
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_deserialize_existing() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_serialize_deserialize".to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let first_string = to_string(handle).unwrap();
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        println!("{}",first_string);
        println!("{}",second_string);
        assert_eq!(first_string,second_string);
    }
}

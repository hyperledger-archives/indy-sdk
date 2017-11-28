extern crate rand;
extern crate serde_json;
extern crate libc;

use utils::wallet;
use utils::error;
use utils::signus::SignusUtils;
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
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InviteDetail {
    e: String,
    rid: String,
    sakdp: String,
    sn: String,
    sD: String,
    lu: String,
    sVk: String,
    tn: String,
}
impl InviteDetail {
    fn new() -> InviteDetail {
        InviteDetail {
            e: String::new(),
            rid: String::new(),
            sakdp: String::new(),
            sn: String::new(),
            sD: String::new(),
            lu: String::new(),
            sVk: String::new(),
            tn: String::new(),
        }
    }
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
    invite_detail: InviteDetail,
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

        match messages::send_invite()
            .to(&self.pw_did)
            .key_delegate("key")
            .phone_number(&options_obj.phone)
            .send() {
            Err(_) => {
                error::POST_MSG_FAILURE.code_num
            },
            Ok(response) => {
                self.state = CxsStateType::CxsStateOfferSent;
                self.invite_detail = get_invite_detail(&response);
                error::SUCCESS.code_num
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
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
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
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
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
    let pw_did = get_pw_did(handle)?;
    let pw_verkey = get_pw_verkey(handle)?;

    match messages::create_keys()
        .for_did(&pw_did)
        .to(&enterprise_did)
        .for_verkey(&pw_verkey)
        .nonce("anything")
        .send() {
        Ok(_) => Ok(error::SUCCESS.code_num),
        Err(x) => Err(x),
    }
}

pub fn update_agent_profile(handle: u32) -> Result<u32, u32> {
    let pw_did = get_pw_did(handle)?;

    match messages::update_data()
        .to(&pw_did)
        .name(&settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME).unwrap())
        .logo_url(&settings::get_config_value(settings::CONFIG_LOGO_URL).unwrap())
        .send() {
        Ok(_) => Ok(error::SUCCESS.code_num),
        Err(x) => Err(x),
    }
}

//
// NOTE: build_connection and create_connection are broken up to make it easier to create connections in tests
//       you can call create_connection without test_mode and you don't have to build a wallet or
//       mock the agency during the connection phase
//
// TODO: This should take a ref?
pub fn create_connection(source_id: String) -> u32 {
    let new_handle = rand::thread_rng().gen::<u32>();

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
        invite_detail: InviteDetail::new()
    });

    CONNECTION_MAP.lock().unwrap().insert(new_handle, c);;

    new_handle
}

pub fn build_connection(source_id: String) -> Result<u32,u32> {
    // Check to make sure source_id is unique

    let new_handle = create_connection(source_id);

    let (my_did, my_verkey) = match SignusUtils::create_and_store_my_did(wallet::get_wallet_handle(),None) {
        Ok(y) => y,
        Err(x) => {
            error!("could not create DID/VK: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    info!("handle: {} did: {} verkey: {}", new_handle, my_did, my_verkey);
    set_pw_did(new_handle, &my_did);
    set_pw_verkey(new_handle, &my_verkey);

    match create_agent_pairwise(new_handle) {
        Err(x) => {
            error!("could not create pairwise key on agent: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
        Ok(_) => info!("created pairwise key on agent"),
    };

    match update_agent_profile(new_handle) {
        Err(x) => {
            error!("could not update profile on agent: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
        Ok(_) => info!("updated profile on agent"),
    };

    set_state(new_handle, CxsStateType::CxsStateInitialized);

    Ok(new_handle)
}

pub fn update_state(handle: u32) -> u32{
    let pw_did = match get_pw_did(handle) {
        Ok(did) => did,
        Err(x) => return x,
    };

    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    match messages::get_messages()
        .to(&pw_did)
        .send() {
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
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = derived_connection.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}

    let connection = Box::from(derived_connection);

    info!("inserting handle {} into connection table", new_handle);

    CONNECTION_MAP.lock().unwrap().insert(new_handle, connection);

    Ok(new_handle)
}

pub fn release(handle: u32) -> u32 {
    match CONNECTION_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    }
}

fn get_invite_detail(response: &str) -> InviteDetail {

    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            convert_invite_details(&json["inviteDetail"])
        }
        Err(_) => {
            info!("Connect called without a valid response from server");
            InviteDetail::new()
        }
    }
}
/* mappings
`senderEndpoint` -> `e`
`connReqId` -> `rid`
`senderAgentKeyDlgProof` -> `sakdp`
`senderName` -> `sn`
`senderDID` -> `sD`
`senderLogoUrl` -> `lu`
`senderDIDVerKey` -> `sVk`
`targetName` -> `tn`
*/
pub fn convert_invite_details(json: &serde_json::Value) -> InviteDetail {
    println!("{}",json["senderEndpoint"]);
    let json_converted = InviteDetail {
        e: String::from(json["senderEndpoint"].as_str().unwrap()),
        rid: String::from(json["connReqId"].as_str().unwrap()),
        sakdp: String::from(json["senderAgentKeyDlgProof"].as_str().unwrap()),
        sn: String::from(json["senderName"].as_str().unwrap()),
        sD: String::from(json["senderDID"].as_str().unwrap()),
        lu: String::from(json["senderLogoUrl"].as_str().unwrap()),
        sVk: String::from(json["senderDIDVerKey"].as_str().unwrap()),
        tn: String::from(json["targetName"].as_str().unwrap()),
    };
    println!("{}", serde_json::to_string(&json_converted).unwrap());
    json_converted
}

#[cfg(test)]
mod tests {
    extern crate mockito;
    use super::*;
    use utils::wallet;

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
        let handle = build_connection("test_create_connection".to_owned()).unwrap();
        assert!(handle > 0);
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
    fn test_create_drop_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_create_drop_create".to_owned()).unwrap();
        let did1 = get_pw_did(handle).unwrap();
        release(handle);
        let handle2 = build_connection("test_create_drop_create".to_owned()).unwrap();
        assert_ne!(handle,handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        release(handle2);
    }

    #[test]
    fn test_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxn_release".to_owned()).unwrap();
        assert!(handle > 0);
        let rc = release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_state_not_connected() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_state_not_connected".to_owned()).unwrap();
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
        let handle = build_connection("test_set_get_pw_verkey".to_owned()).unwrap();
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
            invite_detail: InviteDetail::new()
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
            invite_detail: InviteDetail::new()
        });

        CONNECTION_MAP.lock().unwrap().insert(handle, c);

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
        let handle = build_connection(test_name.to_owned()).unwrap();
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
        let handle = build_connection(test_name.to_owned()).unwrap();
        assert!(handle > 0);
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
        /*
        `senderEndpoint` -> `e`
        `connReqId` -> `rid`
        `senderAgentKeyDlgProof` -> `sakdp`
        `senderName` -> `sn`
        `senderDID` -> `sD`
        `senderLogoUrl` -> `lu`
        `senderDIDVerKey` -> `sVk`
        `targetName` -> `tn`
        */
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
        let stringifyied = serde_json::to_string(&invite_detail).unwrap();
        info!("Invite Detail Test: {}", stringifyied);
        assert!(stringifyied.contains("sdfsdf"));
        assert_eq!(invite_detail.sakdp, "sdfsdf");
    }

    #[test]
    fn test_convert_invite_details(){
        let response = r#"{ "inviteDetail": {"senderEndpoint": "34.210.228.152:80",
                "connReqId": "CXqcDCE",
                "senderAgentKeyDlgProof": "sdfsdf",
                "senderName": "Evernym",
                "senderDID": "JiLBHundRhwYaMbPWno8Vg",
                "senderLogoUrl": "https://postimg.org/image/do2r09ain/",
                "senderDIDVerKey": "AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1",
                "targetName": "there"
            }}"#;
        use serde_json::Value;
        let original_json:Value = serde_json::from_str(&response).unwrap();
        let invite_detail:&Value = &original_json["inviteDetail"];
        let converted_json = convert_invite_details(invite_detail);
        assert_eq!(converted_json.e, original_json["inviteDetail"]["senderEndpoint"])
    }

    #[test]
    fn test_serialize_deserialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_serialize_deserialize".to_owned()).unwrap();
        assert!(handle > 0);
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
        let handle = build_connection("test_serialize_deserialize".to_owned()).unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        println!("{}",first_string);
        println!("{}",second_string);
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_bad_wallet_connection_fails() {
        wallet::tests::delete_wallet("dummy");
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        assert_eq!(build_connection("test_bad_wallet_connection_fails".to_owned()).unwrap_err(),error::UNKNOWN_ERROR.code_num);
    }
}

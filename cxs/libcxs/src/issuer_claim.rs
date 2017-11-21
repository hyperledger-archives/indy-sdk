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
use utils::issuer_claim::CLAIM_REQ_STRING;
use self::libc::c_char;
use utils::callback::CallbackUtils;
use std::sync::mpsc::channel;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use utils::wallet;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}




extern {
    fn indy_issuer_create_and_store_claim_def(command_handle: i32,
                                              wallet_handle: i32,
                                              issuer_did: *const c_char,
                                              schema_json: *const c_char,
                                              signature_type: *const c_char,
                                              create_non_revoc: bool,
                                              cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                   claim_def_json: *const c_char)>) -> i32;
    fn indy_issuer_create_claim(command_handle: i32,
                                wallet_handle: i32,
                                claim_req_json: *const c_char,
                                claim_json: *const c_char,
                                user_revoc_index: i32,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                     revoc_reg_update_json: *const c_char, //TODO must be OPTIONAL
                                                     xclaim_json: *const c_char
                                )>)-> i32;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IssuerClaim {
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
        //Todo: call to message class to build payload
        let added_data = r#""claim_name":"Profile detail","issuer_name":"Test Enterprise","optional_data":{"terms_of_service":"<Large block of text>","price":6}"#;
        let payload = format!("{{\"msg_type\":\"CLAIM_OFFER\",\"version\":\"0.1\",\"to_did\":\"{}\",\"from_did\":\"{}\",\"claim\":{},\"schema_seq_no\":{},\"issuer_did\":\"{}\",{}}}",to_did,from_did,self.claim_attributes,self.schema_seq_no,self.issuer_did,added_data);
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

        let attrs_with_encodings = self.create_attributes_encodings()?;

        //TODO: call to libindy to encrypt payload
        let data = match self.claim_request.clone() {
            Some(d) => match create_claim_payload_using_wallet(&d, &attrs_with_encodings, wallet::get_wallet_handle()){
                Ok(p) => p,
                Err(e) => return Err(error::UNKNOWN_ERROR.code_num),
            },
            // TODO: change this to error and handle the error.
            None => panic!("Cant create a claim without a claim request"),
        };

        let to = connection::get_pw_did(connection_handle).unwrap();

        match messages::send_message().to(&to).msg_type("claim").edge_agent_payload(&data).send() {
            Err(x) => {
                warn!("could not send claim: {}", x);
                return Err(x);
            },
            Ok(response) => {
                self.msg_uid = match get_offer_details(&response) {
                    Ok(x) => x,
                    Err(x) => {
                        info!("Error in response: {}", x);
                        return Err(x);
                    },
                };
                self.issued_did = to;
                self.state = CxsStateType::CxsStateAccepted;
                return Ok(error::SUCCESS.code_num);
            }
        }
    }

    fn create_attributes_encodings(&self) -> Result<String, u32> {
        let mut attributes: serde_json::Value = match serde_json::from_str(&self.claim_attributes) {
            Ok(x) => x,
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        let mut map = match attributes.as_object_mut() {
            Some(x) => x,
            None => {
                warn!("Invalid Json for Attribute data");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        for (attr, mut vec) in map.iter_mut(){
            let mut list = match vec.as_array_mut() {
                Some(x) => x,
                None => {
                    warn!("Invalid Json for Attribute data");
                    return Err(error::INVALID_JSON.code_num)
                }
            };
//          FIXME This is hardcode but should have logic for finding strings and integers and
//          doing a real encoding (sha256)
            let encoded = serde_json::Value::from("1139481716457488690172217916278103335");
            list.push(encoded)
        }

        match serde_json::to_string_pretty(&map) {
            Ok(x) => Ok(x),
            Err(x) => {
                warn!("Invalid Json for Attribute data");
                Err(error::INVALID_JSON.code_num)
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

                let string_payload = match msg["edgeAgentPayload"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("claim request has no edge agent payload");
                        return
                    }
                };

                let payload: serde_json::Value = match serde_json::from_str(string_payload) {
                    Ok(x) => x,
                    Err(x) => {
                        warn!("invalid json for claim requests edgeAgentPayload");
                        return
                    },
                };

                self.claim_request = match ClaimRequest::create_from_api_msg_json(&payload) {
                    Ok(x) => Some(x),
                    Err(_) => {
                        warn!("invalid claim request for claim {}", self.handle);
                        return
                    }
                };
                return
            }
        }
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
                let ref_msg_id = match msg["refMsgId"].as_str() {
                    Some(x) => x,
                    None => {
                        warn!("invalid message reference id for claim {}", self.handle);
                        return
                    }
                };
                self.get_claim_req(ref_msg_id);
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
    fn set_claim_request(&mut self, claim_request:&ClaimRequest){
        self.claim_request = Some(claim_request.clone());
    }
    pub fn create_standard_issuer_claim() -> Result<IssuerClaim, u32> {
        let claim_req_value = &serde_json::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_claim = IssuerClaim {
            handle: 123,
            source_id: "standard_claim".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
//            claim_request: Some(ClaimRequest::create_from_api_msg_json(claim_req_value).clone()),
            claim_request: match ClaimRequest::create_from_api_msg_json(claim_req_value) {
                Ok(x) => Some(x.clone()),
                Err(_) => {
                    warn!("invalid claim request for claim {}", 123);
                    return Err(error::INVALID_CLAIM_REQUEST.code_num)
                }
            },
        };
        Ok(issuer_claim)
    }
}

pub fn create_claim_payload_using_wallet<'a>(claim_req: &ClaimRequest, claim_data: &str, wallet_handle: i32) -> Result< String, u32> {
    println!("claim data: {}", claim_data);
    println!("claim request: {:?}", serde_json::to_string(&claim_req));
    let (sender, receiver) = channel();

    let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
        sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
    });
    info!("wallet_handle: {}", wallet_handle);
    let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

    let claim_req_master_secret = match claim_req.blinded_ms.clone() {
        Some(ms) => ms,
        // TODO: need new error
        None => {
            error!("No Master Secret in the Claim Request!");
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    let claim_req_str = match serde_json::to_string(&claim_req) {
        Ok(s) => s,
        // TODO: need new error
        Err(x) => {
            error!("Claim Request is not properly formatted/formed: {}", x);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    unsafe {
        let err = indy_issuer_create_claim(command_handle,
                                           wallet_handle,
                                           CString::new(claim_req_str).unwrap().as_ptr(),
                                           CString::new(claim_data).unwrap().as_ptr(),
                                           -1,
                                           cb);
        if err != 0 {
            error!("could not create claim: {}", err);
            return Err(error::UNKNOWN_ERROR.code_num);
        }
    }

    let (err, revoc_reg_update_json, xclaim_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

    if err != 0 {
        error!("could not create claim: {}", err);
        return Err(error::UNKNOWN_ERROR.code_num);
    };

    info!("xclaim_json: {}", xclaim_json);
    Ok(xclaim_json)
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

    info!("inserting handle {} into claim_issuer table", new_handle);
    ISSUER_CLAIM_MAP.lock().unwrap().insert(new_handle, new_issuer_claim);;

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
            info!("Connect called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

pub fn set_claim_request(handle: u32, claim_request: &ClaimRequest) -> Result<u32,u32>{
   match ISSUER_CLAIM_MAP.lock().unwrap().get_mut(&handle) {
       Some(c) => {c.set_claim_request(claim_request);
                    Ok(error::SUCCESS.code_num)},
       None => Err(error::UNKNOWN_ERROR.code_num),
   }
}



#[cfg(test)]
mod tests {
    extern crate mockito;
    use settings;
    use connection::create_connection;
    use std::thread;
    use std::time::Duration;
    use utils::signus::SignusUtils;
    use utils::wallet::init_wallet;
    use utils::issuer_claim::tests::{put_claim_def_in_issuer_wallet, create_default_schema};
    use super::*;
    static SCHEMA: &str = r#"{{
                            "seqNo":32,
                            "data":{{
                                "name":"gvt",
                                "version":"1.0",
                                "keys":["age","sex","height","name"]
                            }}
                         }}"#;

    static CLAIM_REQ_STRING: &str =
        r#"{
           "msg_type":"CLAIM_REQUEST",
           "version":"0.1",
           "to_did":"BnRXf8yDMUwGyZVDkSENeq",
           "from_did":"GxtnGN6ypZYgEqcftSQFnC",
           "iid":"cCanHnpFAD",
           "mid":"",
           "blinded_ms":{
              "prover_did":"FQ7wPBUgSPnDGJnS1EYjTK",
              "u":"923...607",
              "ur":null
           },
           "issuer_did":"QTrbV4raAcND4DWWzBmdsh",
           "schema_seq_no":48,
           "optional_data":{
              "terms_of_service":"<Large block of text>",
              "price":6
           }
        }"#;

    static CLAIM_DATA: &str =
        r#"{"address2":["101 Wilson Lane"],
        "zip":["87121"],
        "state":["UT"],
        "city":["SLC"],
        "address1":["101 Tela Lane"]
        }"#;

    static X_CLAIM_JSON: &str =
        r#"{"claim":{"sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],
            "name":["Alex","1139481716457488690172217916278103335"],
            "height":["175","175"],
            "age":["28","28"]},
            "schema_seq_no":48,"signature":{"primary_claim":{"m2":"20422830146126298072435154364609688311215455372812191522510963615911197566669",
            "a":"63278417442659036669400207009188145697780040051013688149129256743084966944528018225851811786642635489831571302866283859548986662378660197412546425265143614707831895279255687895675751698590585098567712192115877143987215992997043294541884675031280360751560521858749232517644822329119678418891734891999969994336787838346708066475554811401305388198469874303955982449914596797006164947169494007654191130837373504283790479819949019734180572560323746301426795874966758705582341228577546833918138882259158566308938859465340183493113227787793173036569687904567822911316789916700474950018489718630556431551954131019312798482435",
            "e":"259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742929880197442747734002082458742544271217",
            "v":"5448939297853492897399717699539987539533578749867908562762944680268135685568567694260903659584370412643098340744441419451405915373661562565021706607145415122811375641444019538229177574690774531481658120571799333405654388187880203110551180255667817449809910102311767393528025399975601051786676433371392016118214791671204242832547887535483159158088937798628236468649812562884826102823687360319110876054605559703891759045504465130542443075386046668867837639362548961441542181537142758771598043861916300605771981322856145348134135739082583728247027138545297124443408399679058667885317337958616542267235439777554294124026816217852292197406039776334339390803515774060293652261032039824844850903758098551881051874925659952378263321151966685500514992357258732078333637170928029095753968644343981540303973079686119201809045542624"},
            "non_revocation_claim":null},"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e"}"#;

    fn util_put_claim_def_in_issuer_wallet(schema_seq_num: u32, wallet_handle: i32){

        let schema = &create_default_schema(schema_seq_num);

        let stored_xclaim = String::from("");

        info!("wallet_handle: {}", wallet_handle);
        let issuer_did = &settings::get_config_value(settings::CONFIG_ENTERPRISE_DID).unwrap();

        put_claim_def_in_issuer_wallet(issuer_did, schema, wallet_handle);

    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
    }

    fn stand_up_a_wallet()-> (String, i32, String){

        let pool_name = String::from("pool1");
        let wallet_name = String::from("wallet1");
        let wallet_type = String::from("default");
        let wallet_handle = init_wallet(&wallet_name, &pool_name, &wallet_type).unwrap();
        info!("Wallet Handle: {}", wallet_handle);
        let (did, _) = SignusUtils::create_and_store_my_did(wallet_handle, None).unwrap();
        info!("Successfully used wallet handle {} to create_and_store_my_did", wallet_handle);
        (wallet_name, wallet_handle, did)
    }

    fn create_standard_issuer_claim() -> IssuerClaim {
        let claim_req_value = &serde_json::from_str(CLAIM_REQ_STRING).unwrap();
        let issuer_claim = IssuerClaim {
            handle: 123,
            source_id: "standard_claim".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: CLAIM_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
            claim_request: match ClaimRequest::create_from_api_msg_json(claim_req_value) {
                Ok(x) => Some(x.clone()),
                Err(_) => {
                    panic!("invalid claim request for claim {}", 123);
                }
            },
        };
        issuer_claim
    }

    fn normalize_claims(c1: &str, c2: &str) -> (serde_json::Value, serde_json::Value) {
        let mut v1:serde_json::Value = serde_json::from_str(c1.clone()).unwrap();
        let mut v2:serde_json::Value = serde_json::from_str(c2.clone()).unwrap();
        v1["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        (v1, v2)
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
    fn test_send_a_claim() {
        let test_name = "test_send_a_claim";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,"QTrbV4raAcND4DWWzBmdsh");
        wallet::tests::make_wallet(test_name);

        let claim_req_value = &serde_json::from_str(CLAIM_REQ_STRING).unwrap();
        let claim_req:ClaimRequest = match ClaimRequest::create_from_api_msg_json(&claim_req_value) {
            Ok(x) => x,
            Err(_) => panic!("error with claim request"),
        };
        let issuer_did = claim_req.issuer_did;
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claim\",\"statusCode\":\"MS-101\"}")
            .expect(1)
            .create();

        let mut claim = create_standard_issuer_claim();
        claim.state = CxsStateType::CxsStateRequestReceived;
        util_put_claim_def_in_issuer_wallet(48, wallet::get_wallet_handle());

        let connection_handle = create_connection("test_send_claim_offer".to_owned());
        connection::set_pw_did(connection_handle,"8XFh8yBzrpJQmNyZzgoTqB");

        match claim.send_claim(connection_handle) {
            Ok(_) => assert_eq!(0,0),
            Err(x) => {
                info!("error message: {}", error::error_message(&x));
                assert_eq!(x, 0)
            },
         };
        _m.assert();
        assert_eq!(claim.state,CxsStateType::CxsStateAccepted);
        wallet::close_wallet(wallet::get_wallet_handle()).unwrap();
        wallet::delete_wallet(test_name).unwrap();
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

        let claim_req_value = &serde_json::from_str(CLAIM_REQ_STRING).unwrap();
        let mut claim = IssuerClaim {
            handle: 123,
            source_id: "test_has_pending_claim_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            claim_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
            claim_request: match ClaimRequest::create_from_api_msg_json(claim_req_value) {
                Ok(x) => Some(x.clone()),
                Err(_) => {
                    panic!("invalid claim request for claim {}", 123);
                }
            },
        };

        claim.update_state();
        _m.assert();
        assert_eq !(claim.get_state(), CxsStateType::CxsStateRequestReceived as u32);
        let claim_request = claim.claim_request.unwrap();
        assert_eq!(claim_request.issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(claim_request.schema_seq_no, 48);
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
// FIXME Mark get these test working again
//    #[test]
//    fn test_issuer_claim_can_build_claim_from_correct_parts(){
//        let test_name = "test_issuer_claim_can_build_from_correct_parts";
//        let schema_str = SCHEMA;
//        let mut issuer_claim = create_standard_issuer_claim();
//        let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e".to_owned();
//        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
//        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
//        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID, &issuer_did);
//        wallet::tests::make_wallet(test_name);
//        let wallet_handle = wallet::get_wallet_handle();
//        SignusUtils::create_and_store_my_did(wallet_handle,None).unwrap();
//        util_put_claim_def_in_issuer_wallet(48, wallet_handle);
//
//        // set the claim request issuer did to the correct (enterprise) did.
//        let mut claim_req = issuer_claim.claim_request.clone().unwrap();
//        claim_req.issuer_did = issuer_did.to_owned();
//        issuer_claim.claim_request = Some(claim_req);
//
//        let claim_payload = match create_claim_payload_using_wallet(&issuer_claim.claim_request.clone().unwrap(), &CLAIM_DATA, wallet::get_wallet_handle()) {
//            Ok(c) => c,
//            Err(_) => panic!("Error creating claim payload"),
//        };
//        let claim_payload_json:serde_json::Value = serde_json::from_str(&claim_payload).unwrap();
//        let x_claim_json:serde_json::Value = serde_json::from_str(X_CLAIM_JSON).unwrap();
//
//        // remove primary claims signatures
//        // as they will never match
//        let (n1, n2) = normalize_claims(&claim_payload, &X_CLAIM_JSON);
//
//        assert_eq!(serde_json::to_string(&n1).unwrap(),serde_json::to_string(&n2).unwrap());
//        wallet::close_wallet(wallet_handle).unwrap();
//        wallet::delete_wallet(test_name).unwrap();
//
//    }
//
//    #[test]
//    fn test_issuer_claim_request_changes_reflect_in_claim_payload(){
//        // TODO: Is this duplicate of the above test?
//        settings::set_defaults();
//        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
//        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,"NcYxiDXkpYi6ov5FcYDi1e");
//        wallet::tests::make_wallet("test_issuer_claim_request_changes_reflect_in_claim");
//        let wallet_handle = wallet::get_wallet_handle();
//
//        util_put_claim_def_in_issuer_wallet(48, wallet_handle);
//        let issuer_claim = create_standard_issuer_claim();
//        let mut claim_request = issuer_claim.claim_request.clone().unwrap();
//        claim_request.issuer_did = String::from("NcYxiDXkpYi6ov5FcYDi1e");
//        assert_eq!(claim_request.schema_seq_no, 48);
//        info!("claim request: {:?}" , serde_json::to_string(&claim_request));
//        info!("claim data: {:?}", &CLAIM_DATA);
//        let claim_payload = match create_claim_payload_using_wallet(&claim_request, &CLAIM_DATA, wallet_handle) {
//            Ok(c) => c,
//            Err(_) => panic!("Error creating claim payload"),
//        };
//
//        let (n1, n2) = normalize_claims(&claim_payload, &X_CLAIM_JSON);
//        info!("claim_payload: {}", claim_payload);
//        assert_eq!(n1, n2);
//
//        wallet::close_wallet(wallet_handle).unwrap();
//        wallet::delete_wallet("test_issuer_claim_request_changes_reflect_in_claim").unwrap();
//    }

    #[test]
    fn basic_add_attribute_encoding() {
        // FIXME Make this a real test and add additional test for create_attributes_encodings
        let issuer_claim = create_standard_issuer_claim();
        issuer_claim.create_attributes_encodings();
        info!("{}", issuer_claim.create_attributes_encodings().unwrap())
    }

}


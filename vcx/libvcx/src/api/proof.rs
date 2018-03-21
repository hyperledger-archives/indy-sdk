extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use proof;
use connection;
use std::thread;
use std::ptr;

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: attributes in json format prover is expected to include in proof.
///
/// # Example requested_attrs -> "[{"name":"attrName","issuer_did":"did","schema_seq_no":1}]"
///
/// requested_predicates: specific requirements regarding the prover's attributes.
///
/// # Example requested_predicates -> "[{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":1,"issuer_did":"DID"}]"
/// /// name: Name of the proof request - ex. Drivers Licence.
///
/// cb: Callback that provides proof handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_attrs, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_predicates, error::INVALID_OPTION.code_num);
    check_useful_c_str!(name, error::INVALID_OPTION.code_num);

    let source_id_opt = if !source_id.is_null() {
        check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
        let val = source_id.to_owned();
        Some(val)
    } else { None };

    info!("vcx_proof_create(command_handle: {}, source_id: {:?}, requested_attrs: {}, requested_predicates: {}, name: {})",
          command_handle, source_id_opt, requested_attrs, requested_predicates, name);

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(
            source_id_opt, requested_attrs, requested_predicates, name) {
            Ok(x) => {
                info!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, proof::get_source_id(x).unwrap_or_default());
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change and updates the proof state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_update_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        proof::update_state(proof_handle);

        info!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), proof_handle, proof::get_state(proof_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_proof_get_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), proof_handle, proof::get_state(proof_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
    });

    error::SUCCESS.code_num
}

/// Takes the proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides json string of the proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_serialize(command_handle: {}, proof_handle: {}), source_id: {:?}", command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match proof::to_string(proof_handle) {
            Ok(x) => {
                info!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_data: json string representing a proof object
///
/// # Examples proof_data -> {"source_id":"id","handle":1,"requested_attrs":"[{\"issuerDid\":\"did\",\"schemaSeqNo\":1,\"name\":\"\"}]","requested_predicates":"[]","msg_uid":"","prover_did":"","state":1,"name":"Proof Name"}
///
/// cb: Callback that provides proof handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_data, error::INVALID_OPTION.code_num);

    info!("vcx_proof_deserialize(command_handle: {}, proof_data: {})",
          command_handle, proof_data);

    thread::spawn( move|| {
        let (rc, handle) = match proof::from_string(&proof_data) {
            Ok(x) => {
                info!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, "");
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the proof object by de-allocating memory
///
/// #Params
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_release(proof_handle: u32) -> u32 {
    info!("vcx_proof_release(proof_handle: {}), source_id: {:?}",
          proof_handle, proof::get_source_id(proof_handle).unwrap_or_default());
    proof::release(proof_handle)
}

/// Sends a proof request to pairwise connection
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: provides any error status of the proof_request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_proof_send_request(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, 0, proof_handle);
                x
            },
            Err(x) => {
                warn!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, x, proof_handle);
                x
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Get Proof
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to identify proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides Proof attributes and error status of sending the claim
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_get_proof(command_handle: u32,
                                        proof_handle: u32,
                                        connection_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state:u32, response_data: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_get_proof(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    //update the state to see if proof has come
    proof::update_state(proof_handle);

    thread::spawn(move|| {
        match proof::get_proof(proof_handle) {
            Ok(x) => {
                info!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, 0, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, proof::get_proof_state(proof_handle), msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, x, "null");
                cb(command_handle, x, proof::get_proof_state(proof_handle), ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}


#[allow(unused_variables)]
pub extern fn vcx_proof_accepted(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use proof::{ create_proof };
    use proof;
    use api::VcxStateType;
    use connection;
    use api::{ ProofStateType };

    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";
    static PROOF_WITH_INVALID_STATE: &'static str = r#"{"source_id":"12","handle":765432,"requested_attrs":"[]","requested_predicates":"[]","msg_uid":"1234","ref_msg_id":"","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"GxtnGN6ypZYgEqcftSQFnC","state":4,"proof_state":2,"name":"","version":"1.0","nonce":"1581479668","proof":{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::e5fec91f-d03d-4513-813c-ab6db5715d55":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"22605045280481376895214546474258256134055560453004805058368015338423404000586901936329279496160366852115900235316791489357953785379851822281248296428005020302405076144264617943389810572564188437603815231794326272302243703078443007359698858400857606408856314183672828086906560155576666631125808137726233827430076624897399072853872527464581329767287002222137559918765406079546649258389065217669558333867707240780369514832185660287640444094973804045885379406641474693993903268791773620198293469768106363470543892730424494655747935463337367735239405840517696064464669905860189004121807576749786474060694597244797343224031","e":"70192089123105616042684481760592174224585053817450673797400202710878562748001698340846985261463026529360990669802293480312441048965520897","v":"1148619141217957986496757711054111791862691178309410923416837802801708689012670430650138736456223586898110113348220116209094530854607083005898964558239710027534227973983322542548800291320747321452329327824406430787211689678096549398458892087551551587767498991043777397791000822007896620414888602588897806008609113730393639807814070738699614969916095861363383223421727858670289337712185089527052065958362840287749622133424503902085247641830693297082507827948006947829401008622239294382186995101394791468192083810475776455445579931271665980788474331866572497866962452476638881287668931141052552771328556458489781734943404258692308937784221642452132005267809852656378394530342203469943982066011466088478895643800295937901139711103301249691253510784029114718919483272055970725860849610885050165709968510696738864528287788491998027072378656038991754015693216663830793243584350961586874315757599094357535856429087122365865868729","m":{"city":"4853213962270369118453000522408430296589146124488849630769837449684434138367659379663124155088827069418193027370932024893343033367076071757003149452226758383807126385017161888440","address1":"12970590675851114145396120869959510754345567924518524026685086869487243290925032320159287997675756075512889990901552679591155319959039145119122576164798225386578339739435869622811","zip":"8333721522340131864419931745588776943042067606218561135102011966361165456174036379901390244538991611895455576519950813910672825465382312504250936740379785802177629077591444977329","address2":"11774234640096848605908744857306447015748098256395922562149769943967941106193320512788344020652220849708117081570187385467979956319507248530701654682748372348387275979419669108338"},"m1":"92853615502250003546205004470333326341901175168428906399291824325990659330595200000112546157141090642053863739870044907457400076448073272490169488870502566172795456430489790324815765612798273406119873266684053517977802902202155082987833343670942161987285661291655743810590661447300059024966135828466539810035","m2":"14442362430453309930284822850357071315613831915865367971974791350454381198894252834180803515368579729220423713315556807632571621646127926114010380486713602821529657583905131582938"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"68430476900085482958838239880418115228681348197588159723604944078288347793331","c_list":[[179,17,2,242,194,227,92,203,28,32,255,113,112,20,5,243,9,111,220,111,21,210,116,12,167,119,253,181,37,40,143,215,140,42,179,97,75,229,96,94,54,248,206,3,48,14,61,219,160,122,139,227,166,183,37,43,197,200,28,220,217,10,65,42,6,195,124,44,164,65,114,206,51,231,254,156,170,141,21,153,50,251,237,65,147,97,243,17,157,116,213,201,80,119,106,70,88,60,55,36,33,160,135,106,60,212,191,235,116,57,78,177,61,86,44,226,205,100,134,118,93,6,26,58,220,66,232,166,202,62,90,174,231,207,19,239,233,223,70,191,199,100,157,62,139,176,28,184,9,70,116,199,142,237,198,183,12,32,53,84,207,202,77,56,97,177,154,169,223,201,212,163,212,101,184,255,215,167,16,163,136,44,25,123,49,15,229,41,149,133,159,86,106,208,234,73,207,154,194,162,141,63,159,145,94,47,174,51,225,91,243,2,221,202,59,11,212,243,197,208,116,42,242,131,221,137,16,169,203,215,239,78,254,150,42,169,202,132,172,106,179,130,178,130,147,24,173,213,151,251,242,44,54,47,208,223]]},"requested_proof":{"revealed_attrs":{"sdf":["claim::e5fec91f-d03d-4513-813c-ab6db5715d55","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}},"proof_request":null}"#;
    extern "C" fn create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        println!("successfully called serialize_cb: {}", proof_string);
    }

    extern "C" fn get_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        assert!(proof_state > 1);
        println!("successfully called get_proof_cb: {}", proof_string);
    }

    extern "C" fn no_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, error::INVALID_PROOF.code_num);
        assert!(proof_string.is_null());
        assert_eq!(proof_state, ProofStateType::ProofUndefined as u32);
        println!("successfully called no_proof_cb: null");
    }

    extern "C" fn verify_invalid_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        assert_eq!(proof_string, r#"[{"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::e5fec91f-d03d-4513-813c-ab6db5715d55","attr_info":{"name":"state","value":"UT","type":"revealed"}}]"#);
        assert_eq!(proof_state, ProofStateType::ProofInvalid as u32);
        println!("successfully called verify_invalid_proof_cb");
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_proof_serialize(0, proof_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"source_id":"source id","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","msg_uid":"","ref_msg_id":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","prover_vk":"","state":2,"proof_state":0,"name":"Name Data","version":"1.0","nonce":"123456","proof":null,"proof_request":null,"remote_did":"","remote_vk":"","agent_did":"","agent_vk":""}"#;
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(expected,new);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state, VcxStateType::VcxStateInitialized as u32);
    }


    extern "C" fn send_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof) {}",err)}
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_proof_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    ptr::null(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
                                    Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_create_proof_fails() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(
            0,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    ptr::null(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional data").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null}"#;
        vcx_proof_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_proof_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = vcx_proof_update_state(0, handle, Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_proof_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = match create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(proof::get_state(handle),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        assert_eq!(vcx_proof_send_request(0,handle,connection_handle,Some(send_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(proof::get_state(handle),VcxStateType::VcxStateOfferSent as u32);
    }

    #[test]
    fn test_get_proof_fails_when_not_ready_with_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = create_proof(None,
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()).unwrap();
        assert!(handle > 0);
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");

        thread::sleep(Duration::from_millis(300));
        let rc = vcx_get_proof(0, handle, connection_handle, Some(no_proof_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_get_proof_returns_proof_with_proof_state_invalid() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");
        thread::sleep(Duration::from_millis(300));

        let proof_handle = proof::from_string(PROOF_WITH_INVALID_STATE).unwrap();
        let rc = vcx_get_proof(0, proof_handle, connection_handle, Some(verify_invalid_proof_cb));
        thread::sleep(Duration::from_millis(900));
        assert_eq!(rc, 0);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = proof::from_string(r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null}"#).unwrap();
        assert!(handle > 0);
        let rc = vcx_proof_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }
}

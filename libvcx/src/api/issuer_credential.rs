extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use connection;
use settings;
use issuer_credential;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/**
 * credential object
 */

/// Create a Issuer Credential object that provides a credential for an enterprise's user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// cred_def_id: id of credential definition given during creation of the credential definition
///
/// issuer_did: did corresponding to entity issuing a credential. Needs to have Trust Anchor permissions on ledger
///
/// credential_data: data attributes offered to person in the credential
///
/// # Example credential_data -> "{"state":["UT"]}"
///
/// credential_name: Name of the credential - ex. Drivers Licence
///
/// price: price of credential
///
/// cb: Callback that provides credential handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_create_credential(command_handle: u32,
                                      source_id: *const c_char,
                                      cred_def_id: *const c_char,
                                      issuer_did: *const c_char,
                                      credential_data: *const c_char,
                                      credential_name: *const c_char,
                                      price: u64,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_data, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(cred_def_id, error::INVALID_OPTION.code_num);

    let issuer_did: String = if !issuer_did.is_null() {
        check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);
        issuer_did.to_owned()
    } else {
        match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
            Ok(x) => x,
            Err(x) => return x
        }
    };

    info!("vcx_issuer_create_credential(command_handle: {}, source_id: {}, cred_def_id: {}, issuer_did: {}, credential_data: {}, credential_name: {})",
          command_handle,
          source_id,
          cred_def_id,
          issuer_did,
          credential_data,
          credential_name);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_credential::issuer_credential_create(cred_def_id, source_id, issuer_did, credential_name, credential_data, price) {
            Ok(x) => {
                info!("vcx_issuer_create_credential_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, issuer_credential::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_create_credential_cb(command_handle: {}, rc: {}, handle: {}, source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Send a credential offer to user showing what will be included in the actual credential
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of credential offer
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_credential_offer(command_handle: u32,
                                          credential_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_send_credential(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, credential_handle, connection_handle, source_id);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match issuer_credential::send_credential_offer(credential_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {}), source_id: {:?}",
                      command_handle, credential_handle, error_string(x), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {}, source_id: {:?})",
                      command_handle, credential_handle, error_string(x.to_error_code()), source_id);
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the credential and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_update_state(command_handle: u32,
                                            credential_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_update_state(command_handle: {}, credential_handle: {})",
          command_handle, credential_handle);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    thread::spawn(move|| {
        issuer_credential::update_state(credential_handle);

        info!("vcx_issuer_credential_update_state_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
              command_handle, credential_handle, error_string(0), issuer_credential::get_state(credential_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, issuer_credential::get_state(credential_handle));
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_issuer_credential_get_state(command_handle: u32,
                                         credential_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_get_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, credential_handle, source_id);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_issuer_credential_get_state_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
              command_handle, credential_handle, error_string(0), issuer_credential::get_state(credential_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, issuer_credential::get_state(credential_handle));
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_get_credential_request(credential_handle: u32, credential_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_issuer_accept_credential(credential_handle: u32) -> u32 { error::SUCCESS.code_num }

/// Send Credential that was requested by user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of sending the credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_send_credential(command_handle: u32,
                                    credential_handle: u32,
                                    connection_handle: u32,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_send_credential(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, credential_handle, connection_handle, source_id);
    thread::spawn(move|| {
        let err = match issuer_credential::send_credential(credential_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {})",
                      command_handle, credential_handle, error_string(x));
                x
            },
            Err(x) => {
                warn!("vcx_issuer_send_credential_cb(command_handle: {}, credential_handle: {}, rc: {})",
                      command_handle, credential_handle, error_string(x.to_error_code()));
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

#[allow(unused_variables)]
pub extern fn vcx_issuer_terminate_credential(credential_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }

/// Takes the credential object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides json string of the credential's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_serialize(command_handle: u32,
                                         credential_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !issuer_credential::is_valid_handle(credential_handle) {
        return error::INVALID_ISSUER_CREDENTIAL_HANDLE.code_num;
    }

    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    info!("vcx_issuer_credential_serialize(credential_serialize(command_handle: {}, credential_handle: {}), source_id: {:?}",
          command_handle, credential_handle, source_id);
    thread::spawn(move|| {
        match issuer_credential::to_string(credential_handle) {
            Ok(x) => {
                info!("vcx_issuer_credential_serialize_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, credential_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num,msg.as_ptr());
            },
            Err(x) => {
                info!("vcx_issuer_credential_serialize_cb(command_handle: {}, credential_handle: {}, rc: {}, state: {}, source_id: {:?})",
                      command_handle, credential_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle,x.to_error_code(),ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an issuer credential object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_data: json string representing a credential object
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_issuer_credential_deserialize(command_handle: u32,
                                      credential_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credential_data, error::INVALID_OPTION.code_num);

    info!("vcx_issuer_credential_deserialize(command_handle: {}, credential_data: {})", command_handle, credential_data);

    thread::spawn(move|| {
        let (rc, handle) = match issuer_credential::from_string(&credential_data) {
            Ok(x) => {
                info!("vcx_issuer_credential_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, issuer_credential::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_issuer_credential_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, issuer_credential::get_source_id(x));
                (x, 0)
            },
        };

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the issuer credential object by deallocating memory
///
/// #Params
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_issuer_credential_release(credential_handle: u32) -> u32 {
    let source_id = issuer_credential::get_source_id(credential_handle).unwrap_or_default();
    match issuer_credential::release(credential_handle) {
        Ok(_) => info!("(vcx_issuer_credential_release credential_handle: {}, rc: {}), source_id: {:?}",
                       credential_handle, error_string(0), source_id),
        Err(e) => warn!("(vcx_issuer_credential_release credential_handle: {}, rc: {}), source_id: {:?}",
                       credential_handle, error_string(e.to_error_code()), source_id),
    };
    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;
    use settings;
    use connection;
    use api::VcxStateType;
    use utils::constants::{CRED_DEF_ID, DEFAULT_SERIALIZED_ISSUER_CREDENTIAL, CREDENTIAL_REQ_STRING};
    use credential_request::CredentialRequest;
    use error::issuer_cred::IssuerCredError;

    static DEFAULT_CREDENTIAL_NAME: &str = "Credential Name Default";
    static DEFAULT_DID: &str = "8XFh8yBzrpJQmNyZzgoTqB";
    static DEFAULT_ATTR: &str = "{\"attr\":\"value\"}";
    static DEFAULT_SCHEMA_SEQ_NO: u32 = 32;
    static ISSUER_CREDENTIAL_STATE_ACCEPTED: &str = r#"{"source_id":"standard_credential","credential_attributes":"{\"address2\":[\"101 Wilson Lane\"],\n        \"zip\":[\"87121\"],\n        \"state\":[\"UT\"],\n        \"city\":[\"SLC\"],\n        \"address1\":[\"101 Tela Lane\"]\n        }","msg_uid":"1234","schema_seq_no":32,"issuer_did":"QTrbV4raAcND4DWWzBmdsh","state":3,"credential_request":{"libindy_cred_req":"{\"prover_did\":\"2hoqvcwupRTUNkXn6ArYzs\",\"cred_def_id\":\"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766\",\"blinded_ms\":{\"u\":\"8732071602357015307810566138808197234658312581785137109788113302982640059349967050965447489217593298616209988826723701562661343443517589847218013366407845073616266391756009264980040238952349445643778936575656535779015458023493903785780518101975701982901383514030208868847307622362696880263163343848494510595690307613204277848599695882210459126941797459019913953592724097855109613611647709745072773427626720401442235193011557232562555622244156336806151662441234847773393387649719209243455960347563274791229126202016215550120934775060992031280966045894859557271641817491943416048075445449722000591059568013176905304195\",\"ur\":null},\"blinded_ms_correctness_proof\":{\"c\":\"26530740026507431379491385424781000855170637402280225419270466226736067904512\",\"v_dash_cap\":\"143142764256221649591394190756594263575252787336888260277569702754606119430149731374696604981582865909586330696038557351486556018124278706293019764236792379930773289730781387402321307275066512629558473696520197393762713894449968058415758200647216768004242460019909604733610794104180629190082978779757591726666340720737832809779281945323437475154340615798778337960748836468199407007775031657682302038533398039806427675709453395148841959462470861915712789403465722659960342165041260269463103782446132475688821810775202828210979373826636650138063942962121467854349698464501455098258293105554402435773328031261630390919907379686173528652481917022556931483089035786146580024468924714494948737711000361399753716101561779590\",\"ms_cap\":\"6713785684292289748157544902063599004332363811033155861083956757033688921010462943169460951559595511857618896433311745591610892377735569122165958960965808330552472093346163460366\"},\"nonce\":\"1154549882365416803296713\"}","libindy_cred_req_meta":"{\"master_secret_blinding_data\":{\"v_prime\":\"5395355128172250143169068089431956784792642542761864362402228480600989694874966075941384260155648520933482583695015613159862636260075389615716222159662546164168786411292929058350829109114076583253317335067228793239648602609298582418017531463540043998240957993320093249294158252626231822371040785324638542033761124918129739329505169470758613520824786030494489920230941474441127178440612550463476183902911947132651422614577934309909240587823495239211344374406789215531181787691051240041033304085509402896936138071991158258582839272399829973882057207073602788766808713962858580770439194397272070900372124998541828707590819468056588985228490934\",\"vr_prime\":null},\"nonce\":\"1154549882365416803296713\",\"master_secret_name\":\"main\"}","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766","tid":"cCanHnpFAD","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","version":"0.1","mid":""},"credential_offer":{"msg_type":"CLAIM_OFFER","version":"0.1","to_did":"8XFh8yBzrpJQmNyZzgoTqB","from_did":"8XFh8yBzrpJQmNyZzgoTqB","libindy_offer":"{\"schema_id\":\"2hoqvcwupRTUNkXn6ArYzs:2:schema_name:0.0.11\",\"cred_def_id\":\"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766\",\"key_correctness_proof\":{\"c\":\"81455034389059130581506970475392033040313255495112570189348030990050944959723\",\"xz_cap\":\"313645697267968767252234073635675430449902008059550004460259716107399731378591839990019486954341409015811398444145390509019258403747288031702507727573872041899321045924287139508392740014051146807378366748171039375722083582850094590251566094137198468729226768809401256609008814847622114541957109991869490323195581928533376835343922482073783968747913611549869005687592623346914265913612170394649557294382253996246104002213172081216651539025706643350612557508228429410997102814965307308636524874409734625285377555470610010065029649043789306111101285927931757335536116856245613021564584847709796772325323716389295248332887528840195072737364278387101996545501723112970168561425282691953586374723401\",\"xr_cap\":{\"age\":\"882754630824080045376337358848444600715931719237593270810742883245639461185815851876695993155364347227577960272007297643455666310248109151421699898719086697252758726897984721300131927517824869533193272729923436764134176057310403382007926964744387461941410106739551156849252510593074993038770740497381973934250838808938096281745915721201706218145129356389886319652075267352853728443472451999347485331725183791798330085570375973775830893185375873153450320600510970851511952771344003741169784422212142610068911032856394030732377780807267819554991221318614567131747542069695452212861957610989952712388162117309870024706736915145245688230386906705817571265829695877232812698581971245658766976413035\",\"height\":\"987637616420540109240639213457114631238834322455397854134075974962516028070241761486895351636137675737583463907200584608953198912009428606796987435233170230262246507002244616435810064614719873830573727071246389627645604379157359983051337498205555868770767724876429776832782322071025598605854225056296405802351270140259313942108556513054492873024197036931111152136704979025907027537437514085689067466225661223523070057146052814725207863140129032189711026590245299845102901392525049014890473357388530510591717159458757929233202259332009161834669583439224425159885860519286698297401104830776447810193871233628235105641793685350321428066559473844839135685992587694149460959649026855973744322255314\",\"name\":\"1546639434545851623074023662485597065284112939224695559955181790271051962463722945049040324831863838273446566781589598791986646525127962031342679728936610678403807319789934638790962870799709103831307094501191346766422178361730723105585107221227683700136793784629414737866344469139276697568820727798174438114746109084012381033673759358527018948810066386903378176283974585934466197449653414224049202874335628877153172622300824161652402616917051692229112366954543190460604470158025596786552965425465904108943932508335616457348969058666355825158659883154681844070175331759147881082936624886840666700175491257446990494466033687900546604556189308597860524376648979247121908124398665458633017197827236\",\"sex\":\"716474787042335984121980741678479956610893721743783933016481046646620232719875607171626872246169633453851120125820240948330986140162546620706675695953306343625792456607323180362022779776451183315417053730047607706403536921566872327898942782065882640264019040337889347226013768331343768976174940163847488834059250858062959921604207705933170308295671034308248661208253191415678118624962846251281290296191433330052514696549137940098226268222146864337521249047457556625050919427268119508782974114298993324181252788789806496387982332099887944556949042187369539832351477275159404450154234059063271817130338030393531532967222197942953924825232879558249711884940237537025210406407183892784259089230597\"}},\"nonce\":\"161126724054910446992163\"}","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766","credential_attrs":{"address1":["101 Tela Lane"],"address2":["101 Wilson Lane"],"city":["SLC"],"state":["UT"],"zip":["87121"]},"schema_seq_no":1487,"claim_name":"Credential","claim_id":"defaultCredentialId","msg_ref_id":"abcd"},"credential_name":"Credential","credential_id":"defaultCredentialId","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766","price":0,"ref_msg_id":null,"agent_did":"FhrSrYtQcw3p9xwf7NYemf","agent_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE","issued_did":"8XFh8yBzrpJQmNyZzgoTqB","issued_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE","remote_did":"FhrSrYtQcw3p9xwf7NYemf","remote_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE"}"#;
    extern "C" fn create_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, credential_string: *const c_char) {
        assert_eq!(err, 0);
        if credential_string.is_null() {
            panic!("credential_string is null");
        }
        check_useful_c_str!(credential_string, ());
        println!("successfully called serialize_cb: {}", credential_string);
    }

    #[test]
    fn test_vcx_issuer_create_credential_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           CString::new(CRED_DEF_ID).unwrap().into_raw(),
                                           ptr::null(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           1,
                                           Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_create_credential_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(
            0,
            CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
            CString::new(CRED_DEF_ID).unwrap().into_raw(),
            ptr::null(),
            ptr::null(),
            CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
            1,
            Some(create_cb)),error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, credential_handle: u32) {
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_issuer_credential_serialize(0,credential_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_issuer_credential_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                                CString::new(CRED_DEF_ID).unwrap().into_raw(),
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           1,
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send credential(offer) {}",err)}
    }

    #[test]
    fn test_vcx_issuer_send_credential_offer() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = issuer_credential::from_string(DEFAULT_SERIALIZED_ISSUER_CREDENTIAL).unwrap();
        assert_eq!(issuer_credential::get_state(handle),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_credential_offer").unwrap();

        assert_eq!(vcx_issuer_send_credential_offer(0,handle,connection_handle,Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }

    #[test]
    fn test_vcx_issuer_send_a_credential() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);

        let test_name = "test_vcx_issuer_send_a_credential";

        let handle = issuer_credential::from_string(ISSUER_CREDENTIAL_STATE_ACCEPTED).unwrap();

        // create connection
        let connection_handle = connection::build_connection("test_send_credential").unwrap();

        // send the credential
        assert_eq!(vcx_issuer_send_credential(0, handle, connection_handle, Some(send_offer_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
    }
    extern "C" fn deserialize_cb(command_handle: u32, err: u32, credential_handle: u32) {
        fn formatter(original: &str) -> String {
            let original_json: serde_json::Value = serde_json::from_str(&original).unwrap();
            serde_json::to_string(&original_json).unwrap()
        }
        assert_eq!(err, 0);
        assert!(credential_handle > 0);
        println!("successfully called deserialize_cb");
        let serialized_issuer_credential = r#"{"source_id":"1","credential_attributes":"{\"attr\":\"value\"}","msg_uid":"","schema_seq_no":0,"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","state":1,"credential_request":null,"credential_offer":null,"credential_name":"credential_name","credential_id":"2936720225","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766","price":0,"payment_address":null,"ref_msg_id":null,"agent_did":"","agent_vk":"","issued_did":"","issued_vk":"","remote_did":"","remote_vk":""}"#;
        let original = formatter(&serialized_issuer_credential);
        let new = formatter(&issuer_credential::to_string(credential_handle).unwrap());
        assert_eq!(original, new);
    }

    #[test]
    fn test_vcx_issuer_credential_deserialize_succeeds() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let string = DEFAULT_SERIALIZED_ISSUER_CREDENTIAL;
        vcx_issuer_credential_deserialize(0,CString::new(string).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_create_credential_arguments_correct(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, DEFAULT_DID);
        assert_eq!(vcx_issuer_create_credential(0,
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                                CString::new(CRED_DEF_ID).unwrap().into_raw(),
                                           CString::new(DEFAULT_DID).unwrap().into_raw(),
                                           CString::new(DEFAULT_ATTR).unwrap().into_raw(),
                                           CString::new(DEFAULT_CREDENTIAL_NAME).unwrap().into_raw(),
                                           1,
                                           Some(create_and_serialize_cb)), error::SUCCESS.code_num);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_issuer_credential_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = issuer_credential::from_string(DEFAULT_SERIALIZED_ISSUER_CREDENTIAL).unwrap();
        assert!(handle > 0);
        let rc = vcx_issuer_credential_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_errors(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let credential_request = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let invalid_handle = 1234388;
        assert_eq!(issuer_credential::set_credential_request(invalid_handle, credential_request), Err(IssuerCredError::InvalidHandle()));
    }

}

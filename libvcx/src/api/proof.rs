extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use proof;
use connection;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: attributes/claims prover must provide in proof
///
/// # Example requested_attrs -> "[{"name":"attrName","restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// requested_predicates: predicate specifications prover must provide claim for
///
/// # Example requested_predicates -> "[{"name":"attrName","p_type":"GE","p_value":9,"restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
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
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    info!("vcx_proof_create(command_handle: {}, source_id: {}, requested_attrs: {}, requested_predicates: {}, name: {})",
          command_handle, source_id, requested_attrs, requested_predicates, name);

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(source_id, requested_attrs, requested_predicates, name) {
            Ok(x) => {
                info!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, proof::get_source_id(x.to_error_code()).unwrap_or_default());
                (x.to_error_code(), 0)
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
                      command_handle, proof_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
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
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
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
/// Success
#[no_mangle]
pub extern fn vcx_proof_release(proof_handle: u32) -> u32 {
    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    match proof::release(proof_handle) {
        Ok(x) => info!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {:?}",
                       proof_handle, error_string(0), source_id),
        Err(e) => warn!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {:?}",
                       proof_handle, error_string(e.to_error_code()), source_id),
    };
    error::SUCCESS.code_num
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
                warn!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, x.to_error_code(), proof_handle);
                x.to_error_code()
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
/// cb: Callback that provides Proof attributes and error status of sending the credential
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
                warn!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, x.to_error_code(), "null");
                cb(command_handle, x.to_error_code(), proof::get_proof_state(proof_handle), ptr::null_mut());
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
    use utils::constants::*;

    static DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";
    static PROOF_WITH_INVALID_STATE: &'static str = r#"{"source_id":"12","requested_attrs":"[]","requested_predicates":"[]","msg_uid":"1234","ref_msg_id":"","prover_did":"GxtnGN6ypZYgEqcftSQFnC","prover_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE","state":2,"proof_state":2,"name":"","version":"1.0","nonce":"961863172979029799927308","proof":{"version":null,"to_did":null,"from_did":"2hoqvcwupRTUNkXn6ArYzs","proof_request_id":null,"libindy_proof":"{\"proof\":{\"proofs\":[{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"height\":\"25730877424947290072821310314181366395232879096832067784637233452620527354832\"},\"a_prime\":\"38468418127731713846178166829900143659393677229712995051558764704141198507982648296401969636617588869861413307083753301762347483561957199290251336021254439417128789026797145955984477221928182716204666352890028385023937842540450361420097431103250392570474089279140901696959752391635073270330468558954154194880702790759376380828440599338338035102331855027981011522557698834567753067206379930537016802075588341699458415972853098876747173416240143668330291197735072667092484763377784337539369362424727574865068876688736052128096968676805203613672531258006236254876884539045951639996619312445298338421294090709530014047972\",\"e\":\"154078967579028599771552225886166146397929690979656551248794150917378672515744224327631618260919015840285570941655123195202127789024078135\",\"v\":\"206650133702888797106822089576260511315673996444862232713919667631626480886927207636881658994691173459660978470403788734073760986620439580327132172963903472672583889688091094464120932708544221005169282402276486398643728400258904453278589049261526042465920275552343146712126256206336127806445434742377573251943058722259168851068891655316911187921990994131523755157817493625393978273746841320890182523140193423913928559137162375489508181000985374480875452950168078202002947841784773449595843073868172932201241350786838279665427992380489948833966021265302305724133918107232723382414241129228666424999973733399390438351369295116174512952324655959161674143294515337045202195459469964668483620985630335061682143952781081831116452202093796615884290232292781138886212342340265668009181731599782785279647873100827946769634016500720049437476277189417112183735003803866677542572168327201194005588295394938009893292601407316947642057\",\"m\":{\"name\":\"14317716492247065262512085019242237626449349441784394518514556593022994618325172736939996637063811425293094487652328383060228760217994402651170775306896722104418394329136611665343\",\"sex\":\"8273745146209996536038161334350089227113461716010701639169426048225428692159803922820821608832094611045622626669925382646938907151072414621800860596480606194039134705353081803165\",\"age\":\"4231054238529313674658593809238740704947764327405783946044127139188368334326617067522789723682113480091638434843954405217871598117884495148611034594431191670951925727063609682142\"},\"m1\":\"16077226648328465055653412522911855274309958773843607345305448528622673753821631144391688614271906713516184930726137071718220068885208665576942267877314141485128646733383927169667\",\"m2\":\"307531169989829093446786072508374599980728424269742153927811014293368496751029791103279482568696302054852869211813229594821072258566732852546275657912143510656140808146570824720\"},\"ge_proofs\":[]},\"non_revoc_proof\":null},{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"zip\":\"87121\"},\"a_prime\":\"22980705139335906693053137711136685933424776233380522453236439180929171299011892824616949981164043529861955976054341945627864882207456050318677121965935325809960446974596053408189551134515661563862881524571666187501122727801571323962966939474493588390360235667182579376145777468800550303664425178193631930564561723049183538282927599646527763284632796539606623768493060689100637116557667392220137410257431636511734992150836591814150936385237171600961922867616324375992788124580682105182900641444243976480856417385889949104486232380947645106164632331171061660796550392089851795426395070995493017794224520143054337746226\",\"e\":\"76282690374532772355879731548661978686850655961351295143503770941243495894338646017819430303027631933773624894143890625703962889768334045\",\"v\":\"163871248639398190945096440069662043127433951676852686546662953990786336996280725854469331726507733342023726300968336464180819246217513732235319949041996672970221003956107560478434062889302155642951932803028117639377419264243738845986591937119464296481011393554214365156921584039126625544193907434264722552626541996733765842362796948507495611245373172036135730178153875129763269752283501582374835061149405781305302348777575898058826139676172730891145943354354439852572793023260967354663689451699372085619446239471849305899996510995214404287577854904161278837483206040916543645383485994543595801626414351505157344120135188489094726450444310061126540914936137480569107724783635454491729295985282727332506496566440072535547642673629304980130245867106221256256513324156573289395988175082198939634551744626563343338338645642842406849216774021364088580494818675743281114570002722646356579330947996683078403354861328714045713581\",\"m\":{\"address1\":\"2165248344508520353766357402265701632395315972098389246389194385640539802337506425769768511519194552740720360820738302295920953335678232689667564165385439841496989825062838947259\",\"city\":\"14062441033587386074492953540858319607901106300870471376856462969255832720301575817298818494188653003203361668892633055754558023030908562519641589325943130928722511367589407296271\",\"state\":\"11954929001972442840069739994692033321109974653475671754863492167150247470566249523034076167519681518942499808749455624562697764021989162427160744700038173075463513230932710770940\",\"address2\":\"2198595304337627823960842218004760109114526286118976571409953402213833883269372274404781681446225341390202046798836378163125679905151333013089084872570388631925688246279177041270\"},\"m1\":\"16077226648328465055653412522911855274309958773843607345305448528622673753821631144391688614271906713516184930726137071718220068885208665576942267877314141485128646733383927169667\",\"m2\":\"7254196164614359653064600331737708502722271621243294598936016708686019460661844338759513800739179938931900307018966658914402446151530199048125880029627236169040751994424664466186\"},\"ge_proofs\":[]},\"non_revoc_proof\":null}],\"aggregated_proof\":{\"c_hash\":\"53950352519396118727110126559791411098081490918154654750464505328600997082590\",\"c_list\":[[1,48,186,131,141,25,34,1,69,221,204,87,153,98,112,57,80,249,204,167,228,160,102,249,42,73,87,42,83,247,68,76,140,62,243,157,1,67,210,213,245,34,93,156,193,240,22,134,110,195,171,11,248,100,90,14,108,42,146,160,37,91,237,36,30,57,0,172,118,172,207,33,92,200,45,238,106,252,153,58,215,235,157,121,77,41,193,216,116,13,160,165,177,98,69,99,130,143,105,84,194,117,25,167,232,41,190,252,91,231,65,99,131,73,216,51,114,109,233,68,87,174,112,159,188,34,13,169,83,136,51,228,244,232,219,169,221,151,163,120,224,13,215,143,179,213,168,229,220,73,152,33,102,196,44,26,182,32,80,1,91,20,161,55,14,46,224,70,38,169,6,219,209,25,58,152,14,112,111,19,65,99,232,176,88,251,145,33,229,114,86,11,54,17,217,142,139,159,55,64,246,217,192,1,85,223,140,35,11,151,176,210,178,65,134,197,123,0,179,72,89,144,101,15,119,34,174,255,43,215,253,212,234,79,246,91,189,0,230,32,200,19,93,232,133,78,144,215,203,110,42,219,69,126,50,222,228],[182,10,208,238,76,37,81,249,58,137,231,34,175,186,222,26,202,1,232,233,178,215,26,96,239,149,81,239,221,218,232,30,207,112,70,179,59,218,119,6,136,201,242,60,160,129,213,64,160,136,197,82,4,219,218,190,77,123,58,226,181,7,53,200,121,117,148,189,65,226,196,60,168,22,173,0,42,112,149,63,153,25,58,94,132,124,99,198,201,220,40,245,155,16,209,167,91,144,27,203,65,167,119,193,231,194,190,224,118,210,97,94,175,53,166,214,219,243,91,11,114,181,169,167,32,21,188,175,110,61,208,240,118,27,223,230,28,3,68,48,57,214,247,228,205,40,142,118,30,58,81,103,103,252,161,173,205,123,124,65,143,27,2,13,55,174,45,95,146,46,214,64,128,239,151,161,51,118,0,208,244,100,255,245,195,225,91,251,116,147,238,130,243,151,236,88,49,153,186,103,87,229,169,54,26,183,176,164,68,243,206,233,8,245,104,255,198,51,252,90,168,223,23,81,48,216,217,153,203,119,127,249,211,53,17,66,144,113,201,77,199,224,53,226,175,209,181,55,14,97,54,54,78,240,37,50]]}},\"requested_proof\":{\"revealed_attrs\":{\"height_1\":{\"sub_proof_index\":0,\"raw\":\"4'11\",\"encoded\":\"25730877424947290072821310314181366395232879096832067784637233452620527354832\"},\"zip_2\":{\"sub_proof_index\":1,\"raw\":\"87121\",\"encoded\":\"87121\"}},\"self_attested_attrs\":{},\"unrevealed_attrs\":{},\"predicates\":{}},\"identifiers\":[{\"schema_id\":\"2hoqvcwupRTUNkXn6ArYzs:2:schema_name:0.0.11\",\"cred_def_id\":\"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766\",\"rev_reg_id\":null,\"timestamp\":null},{\"schema_id\":\"2hoqvcwupRTUNkXn6ArYzs:2:Home Address - Test:0.0.1\",\"cred_def_id\":\"2hoqvcwupRTUNkXn6ArYzs:3:CL:2200\",\"rev_reg_id\":null,\"timestamp\":null}]}"},"proof_request":{"@type":{"name":"PROOF_REQUEST","version":""},"@topic":{"mid":0,"tid":0},"proof_request_data":{"nonce":"123432421212","name":"proof_req_1","version":"0.1","requested_attributes":{"height_1":{"name":"height","restrictions":[{"schema_id":null,"schema_issuer_did":null,"schema_name":null,"schema_version":null,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs","cred_def_id":null}]},"zip_2":{"name":"zip","restrictions":[{"schema_id":null,"schema_issuer_did":null,"schema_name":null,"schema_version":null,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs","cred_def_id":null}]}},"requested_predicates":{}},"msg_ref_id":null},"remote_did":"FhrSrYtQcw3p9xwf7NYemf","remote_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE","agent_did":"FhrSrYtQcw3p9xwf7NYemf","agent_vk":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE"}"#;

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
        assert_eq!(err, error::INVALID_PROOF_HANDLE.code_num);
        assert!(proof_string.is_null());
        assert_eq!(proof_state, ProofStateType::ProofUndefined as u32);
        println!("successfully called no_proof_cb: null");
    }

    extern "C" fn verify_invalid_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
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
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
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
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
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
        let handle = match create_proof("1".to_string(),
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

        let handle = match create_proof("1".to_string(),
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
        let handle = create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()).unwrap();
        assert!(handle > 0);
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();

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
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();
        thread::sleep(Duration::from_millis(300));

        let proof_handle = proof::from_string(PROOF_WITH_INVALID_STATE).unwrap();
        let rc = vcx_get_proof(0, proof_handle, connection_handle, Some(verify_invalid_proof_cb));
        thread::sleep(Duration::from_millis(900));
        assert_eq!(rc, 0);
        vcx_proof_release(proof_handle);
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

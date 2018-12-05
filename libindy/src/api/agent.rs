//extern crate libc;
//
//use api::ErrorCode;
//use commands::agent::AgentCommand;
//use commands::{Command, CommandExecutor};
//use errors::ToErrorCode;
//use utils::ctypes;
//
//use self::libc::c_char;
//
///// Verify a signature with a verkey.
/////
///// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
///// for specific DID.
/////
///// #Params
///// command_handle: command handle to map callback to user context.
///// wallet_handle: wallet identifier containing the key of the sender's private key
///// message: the message which is going to be packed up
///// receiver_keys: a string in the format of a json list which will contain the list of receiver's keys
/////                the message is being encrypted for. For example:
/////                "[<receiver edge_device_1 verkey>, <receiver edge_device_2 verkey>]"
///// cb: Callback that takes command result as parameter.
/////
///// #Returns
///// a JWE in the format defined below:
///// {
/////    "protected": "b64URLencoded({
/////        "enc": "xsalsa20poly1305",
/////        "typ": "JWM/1.0",
/////        "aad_hash_alg": "BLAKE2b",
/////        "cek_enc": "authcrypt"
/////    })"
/////    "recipients": [
/////        {
/////            "encrypted_key": <b64URLencode(encrypt(cek))>,
/////            "header": {
/////                "sender": <b64URLencode(anoncrypt(r_key))>,
/////                "kid": "did:sov:1234512345#key-id",
/////                "key": "b64URLencode(ver_key)"
/////            }
/////        },
/////    ],
/////    "aad": <b64URLencode(aad_hash_alg(b64URLencode(recipients)))>,
/////    "iv": <b64URLencode()>,
/////    "ciphertext": <b64URLencode(encrypt({'@type'...}, cek)>,
/////    "tag": <b64URLencode()>
///// }
/////
///// #Errors
///// Common*
///// Wallet*
///// Ledger*
///// Crypto*
//
//
//#[no_mangle]
//pub fn indy_auth_pack_message(
//    command_handle: i32,
//    wallet_handle: i32,
//    message: *const c_char,
//    receiver_keys: *const c_char,
//    sender: *const c_char,
//    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
//) -> ErrorCode {
//    trace!("indy_auth_pack_message: >>> wallet_handle: {:?}, message: {:?}, receiver_keys: {:?}, sender: {:?}",
//           wallet_handle, message, receiver_keys, sender);
//
//    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
//    check_useful_c_str!(receiver_keys, ErrorCode::CommonInvalidParam4);
//    check_useful_c_str!(sender, ErrorCode::CommonInvalidParam5);
//    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);
//
//    trace!("indy_auth_pack_message: entities >>> wallet_handle: {:?}, message: {:?}, receiver_keys: {:?}, sender: {:?}",
//           wallet_handle, message, receiver_keys, sender);
//
//    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::AuthPackMessage(
//        message,
//        receiver_keys,
//        sender,
//        wallet_handle,
//        Box::new(move |result| {
//            let (err, jwe) = result_to_err_code_1!(result, String::new());
//            trace!(
//                "indy_auth_pack_message: cb command_handle: {:?}, err: {:?}, jwe: {:?}",
//                command_handle,
//                err,
//                jwe
//            );
//            let jwe = ctypes::string_to_cstring(jwe);
//            cb(command_handle, err, jwe.as_ptr())
//        }),
//    )));
//
//    let res = result_to_err_code!(result);
//
//    trace!("indy_auth_pack_message: <<< res: {:?}", res);
//
//    res
//}
//
//#[no_mangle]
//pub fn indy_anon_pack_message(
//    command_handle: i32,
//    message: *const c_char,
//    receiver_keys: *const c_char,
//    cb: Option<extern "C" fn(xcommand_handle: i32, err: ErrorCode, jwe: *const c_char)>,
//) -> ErrorCode {
//    trace!(
//        "indy_anon_pack_message: >>> message: {:?}, receiver_keys: {:?}",
//        message,
//        receiver_keys
//    );
//
//    check_useful_c_str!(message, ErrorCode::CommonInvalidParam3);
//    check_useful_c_str!(receiver_keys, ErrorCode::CommonInvalidParam4);
//    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);
//
//    trace!(
//        "indy_anon_pack_message: entities >>> message: {:?}, receiver_keys: {:?}",
//        message,
//        receiver_keys
//    );
//
//    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::AnonPackMessage(
//        message,
//        receiver_keys,
//        Box::new(move |result| {
//            let (err, jwe) = result_to_err_code_1!(result, String::new());
//            trace!(
//                "indy_anon_pack_message: cb command_handle: {:?}, err: {:?}, jwe: {:?}",
//                command_handle,
//                err,
//                jwe
//            );
//            let verkey = ctypes::string_to_cstring(jwe);
//            cb(command_handle, err, verkey.as_ptr())
//        }),
//    )));
//
//    let res = result_to_err_code!(result);
//
//    trace!("indy_anon_pack_message: <<< res: {:?}", res);
//
//    res
//}
//
////update function to return key used
//#[no_mangle]
//pub fn indy_unpack_message(
//    command_handle: i32,
//    wallet_handle: i32,
//    jwe: *const c_char,
//    sender: *const c_char,
//    cb: Option<
//        extern "C" fn(
//            xcommand_handle: i32,
//            err: ErrorCode,
//            plaintext: *const c_char,
//            sender_vk: *const c_char,
//        ),
//    >,
//) -> ErrorCode {
//    trace!(
//        "indy_unpack_message: >>> wallet_handle: {:?}, jwe: {:?}, sender: {:?}",
//        wallet_handle,
//        jwe,
//        sender
//    );
//
//    check_useful_c_str!(jwe, ErrorCode::CommonInvalidParam3);
//    check_useful_c_str!(sender, ErrorCode::CommonInvalidParam4);
//    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);
//
//    trace!(
//        "indy_unpack_message: entities >>> wallet_handle: {:?}, jwe: {:?}, sender: {:?}",
//        wallet_handle,
//        jwe,
//        sender
//    );
//
//    let result = CommandExecutor::instance().send(Command::Agent(AgentCommand::UnpackMessage(
//        jwe,
//        sender,
//        wallet_handle,
//        Box::new(move |result| {
//            let (err, plaintext, sender_vk) =
//                result_to_err_code_2!(result, String::new(), String::new());
//            trace!(
//                "indy_unpack_message: cb command_handle: {:?}, err: {:?}, plaintext: {:?}",
//                command_handle,
//                err,
//                plaintext
//            );
//            let plaintext = ctypes::string_to_cstring(plaintext);
//            let sender_vk = ctypes::string_to_cstring(sender_vk);
//            cb(command_handle, err, plaintext.as_ptr(), sender_vk.as_ptr())
//        }),
//    )));
//
//    let res = result_to_err_code!(result);
//
//    trace!("indy_unpack_message: <<< res: {:?}", res);
//
//    res
//}

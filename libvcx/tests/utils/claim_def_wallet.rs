extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::ptr::null;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use utils::cstring::CStringUtils;
//use utils::demo::{build_credential_def_txn, sign_and_send_request};

#[allow(dead_code)]
extern {
    fn indy_issuer_create_and_store_claim_def(command_handle: i32,
                                              wallet_handle: i32,
                                              issuer_did: *const c_char,
                                              schema_json: *const c_char,
                                              signature_type: *const c_char,
                                              create_non_revoc: bool,
                                              cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                   credential_def_json: *const c_char)>) -> i32;
}
lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

#[allow(dead_code)]
pub fn create_default_schema(schema_seq_no: u32) -> String {
    let schema = format!(r#"{{
                            "seqNo":{},
                            "data":{{
                                "name":"gvt",
                                "version":"1.0",
                                "attr_names":["age","sex","height","name"]
                            }}
                         }}"#, schema_seq_no);
    String::from(schema)
}

#[allow(dead_code)]
pub fn put_credential_def_in_wallet(wallet_handle: i32, issuer_did: &str, schema_json_str: &str) -> i32{
    fn closure_to_store_credential_def( closure: Box<FnMut(i32) + Send>) ->
        (i32, Option<extern fn(command_handle: i32, err: i32, credential_def:*const c_char)>){
        lazy_static! {
            static ref CALLBACK_CREDENTIAL_DEF_TO_WALLET: Mutex<HashMap<i32, Box<FnMut(i32) + Send>>> = Default::default();
            }

        extern "C" fn callback(command_handle: i32, err: i32, credential_def_string: *const c_char) {
            let mut callback = CALLBACK_CREDENTIAL_DEF_TO_WALLET.lock().unwrap();
            let mut cb = callback.remove(&command_handle).unwrap();
            assert_eq!(err,0);
            if credential_def_string.is_null() {
                panic!("credential def is empty");
            }
            check_useful_c_str!(credential_def_string, ());
//            let credential_def: serde_json::Value = serde_json::from_str(credential_def_string).unwrap();
//            let credential_def_data = credential_def.get("data").unwrap().to_string();
//            let credential_def_txn = build_credential_def_txn(issuer_did, 103, "CL", &credential_def_data).unwrap();
//            let credential_def = sign_and_send_request(get_pool_handle().unwrap() as u32,
//                                                  wallet_handle,
//                                                  issuer_did,
//                                                  &credential_def_txn).unwrap();
            println!("successfully called store credential def: {}", credential_def_string);
            cb(err)
        }

        let mut callbacks = CALLBACK_CREDENTIAL_DEF_TO_WALLET.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);
        (command_handle, Some(callback))
    }
    let (sender, receiver) = channel();
    let cb = Box::new(move |err|{sender.send(err).unwrap();});
    let (command_handle, cb) = closure_to_store_credential_def(cb);
    unsafe {
        let did = CString::new(issuer_did.clone()).unwrap().as_ptr();
        let schema = CString::new(schema_json_str.clone()).unwrap().as_ptr();
        println!("command_handle: {:?}",  command_handle);
        println!("wallet_handle: {:?}", wallet_handle);
        println!("issuer_did_str: {:?}", issuer_did);
        println!("did: {:?}", did);
        println!("schema_str: {:?}", schema_json_str);
        println!("schema: {:?}", schema);

        let rc = indy_issuer_create_and_store_claim_def(command_handle,
                                                        wallet_handle,
                                                        CString::new(issuer_did.clone()).unwrap().as_ptr(),
                                                        CString::new(schema_json_str.clone()).unwrap().as_ptr(),
                                                        null(),
                                                        false,
                                                        cb);
        assert_eq!(rc, 0);
    }
    receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap()

}

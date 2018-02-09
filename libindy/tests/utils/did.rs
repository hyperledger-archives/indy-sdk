extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;

use indy::api::did::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::ledger::LedgerUtils;

pub struct DidUtils {}

impl DidUtils {
    pub fn create_store_and_publish_my_did_from_trustee(wallet_handle: i32, pool_handle: i32) -> Result<(String, String), ErrorCode> {
        let (trustee_did, _) = DidUtils::create_and_store_my_did(wallet_handle, Some(::utils::constants::TRUSTEE_SEED))?;
        let (my_did, my_vk) = DidUtils::create_and_store_my_did(wallet_handle, None)?;
        let nym = LedgerUtils::build_nym_request(&trustee_did, &my_did, Some(&my_vk), None, None)?;
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &nym)?; //TODO check response type
        Ok((my_did, my_vk))
    }

    pub fn create_and_store_my_did(wallet_handle: i32, seed: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
        let create_and_store_my_did_cb = Box::new(move |err, did, verkey| {
            create_and_store_my_did_sender.send((err, did, verkey)).unwrap();
        });
        let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);

        let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));

        let my_did_json = CString::new(my_did_json).unwrap();

        let err =
            indy_create_and_store_my_did(create_and_store_my_did_command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         create_and_store_my_did_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, my_did, my_verkey) = create_and_store_my_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok((my_did, my_verkey))
    }

    pub fn create_my_did(wallet_handle: i32, my_did_json: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, did, verkey| {
            sender.send((err, did, verkey)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_and_store_my_did_cb(cb);

        let my_did_json = CString::new(my_did_json).unwrap();

        let err =
            indy_create_and_store_my_did(command_handle,
                                         wallet_handle,
                                         my_did_json.as_ptr(),
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_did, my_verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((my_did, my_verkey))
    }

    pub fn store_their_did(wallet_handle: i32, identity_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_store_their_did_cb(cb);

        let identity_json = CString::new(identity_json).unwrap();


        let err =
            indy_store_their_did(command_handle,
                                 wallet_handle,
                                 identity_json.as_ptr(),
                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn store_their_did_from_parts(wallet_handle: i32, their_did: &str, their_verkey: &str) -> Result<(), ErrorCode> {
        let (store_their_did_sender, store_their_did_receiver) = channel();
        let store_their_did_cb = Box::new(move |err| { store_their_did_sender.send((err)).unwrap(); });
        let (store_their_did_command_handle, store_their_did_callback) = CallbackUtils::closure_to_store_their_did_cb(store_their_did_cb);

        let their_identity_json = format!("{{\"did\":\"{}\",\
                                            \"verkey\":\"{}\"\
                                           }}",
                                          their_did, their_verkey);

        let their_identity_json = CString::new(their_identity_json).unwrap();

        let err =
            indy_store_their_did(store_their_did_command_handle,
                                 wallet_handle,
                                 their_identity_json.as_ptr(),
                                 store_their_did_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = store_their_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn replace_keys_start(wallet_handle: i32, did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, verkey| {
            sender.send((err, verkey)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_replace_keys_start_cb(cb);

        let did = CString::new(did).unwrap();
        let identity_json = CString::new(identity_json).unwrap();

        let err =
            indy_replace_keys_start(command_handle,
                                    wallet_handle,
                                    did.as_ptr(),
                                    identity_json.as_ptr(),
                                    cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, my_verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(my_verkey)
    }

    pub fn replace_keys_apply(wallet_handle: i32, did: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_replace_keys_apply_cb(cb);

        let did = CString::new(did).unwrap();

        let err =
            indy_replace_keys_apply(command_handle,
                                    wallet_handle,
                                    did.as_ptr(),
                                    cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn replace_keys(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let verkey = DidUtils::replace_keys_start(wallet_handle, did, "{}").unwrap();

        let nym_request = LedgerUtils::build_nym_request(did, did, Some(&verkey), None, None).unwrap();
        LedgerUtils::sign_and_submit_request(pool_handle, wallet_handle, did, &nym_request).unwrap();

        DidUtils::replace_keys_apply(wallet_handle, did).unwrap();

        Ok(verkey)
    }

    pub fn key_for_did(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, verkey| {
            sender.send((err, verkey)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_key_for_did_cb(cb);

        let did = CString::new(did).unwrap();

        let err = indy_key_for_did(command_handle,
                                   pool_handle,
                                   wallet_handle,
                                   did.as_ptr(),
                                   callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(verkey)
    }

    pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, verkey| {
            sender.send((err, verkey)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_key_for_local_did_cb(cb);

        let did = CString::new(did).unwrap();

        let err = indy_key_for_local_did(command_handle,
                                         wallet_handle,
                                         did.as_ptr(),
                                         callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(verkey)
    }

    pub fn set_endpoint_for_did(wallet_handle: i32, did: &str, address: &str, transport_key: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_set_endpoint_for_did_cb(cb);

        let did = CString::new(did).unwrap();
        let address = CString::new(address).unwrap();
        let transport_key = CString::new(transport_key).unwrap();

        let err = indy_set_endpoint_for_did(command_handle,
                                            wallet_handle,
                                            did.as_ptr(),
                                            address.as_ptr(),
                                            transport_key.as_ptr(),
                                            callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn get_endpoint_for_did(wallet_handle: i32, pool_handle: i32, did: &str) -> Result<(String, Option<String>), ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, endpoint, transport_vk| {
            sender.send((err, endpoint, transport_vk)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_get_endpoint_for_did_cb(cb);

        let did = CString::new(did).unwrap();

        let err = indy_get_endpoint_for_did(command_handle,
                                            wallet_handle,
                                            pool_handle,
                                            did.as_ptr(),
                                            callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, endpoint, transport_vk) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok((endpoint, transport_vk))
    }

    pub fn set_did_metadata(wallet_handle: i32, did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_store_did_metadata_cb(cb);

        let did = CString::new(did).unwrap();
        let metadata = CString::new(metadata).unwrap();

        let err = indy_set_did_metadata(command_handle,
                                        wallet_handle,
                                        did.as_ptr(),
                                        metadata.as_ptr(),
                                        callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(())
    }

    pub fn get_did_metadata(wallet_handle: i32, did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, metadata| {
            sender.send((err, metadata)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_get_did_metadata_cb(cb);

        let did = CString::new(did).unwrap();

        let err = indy_get_did_metadata(command_handle,
                                        wallet_handle,
                                        did.as_ptr(),
                                        callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, metadata) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(metadata)
    }

    pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, metadata| {
            sender.send((err, metadata)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_get_abbr_verkey_cb(cb);

        let did = CString::new(did).unwrap();
        let verkey = CString::new(verkey).unwrap();

        let err = indy_abbreviate_verkey(command_handle,
                                         did.as_ptr(),
                                         verkey.as_ptr(),
                                         callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(verkey)
    }
}
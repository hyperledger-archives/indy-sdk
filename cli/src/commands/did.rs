use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata, DynamicCompletionType};
use crate::commands::*;
use crate::utils::table::print_list_table;

use indy::{WalletHandle, PoolHandle, ErrorCode};

use crate::libindy::did::Did;
use crate::libindy::ledger::Ledger;

use std::fs::File;
use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use crate::commands::ledger::{
    handle_transaction_response,
    set_request_fees,
    parse_response_with_fees,
    print_response_receipts,
    Response
};

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("did", "Identity management commands"));
}

pub mod new_command {
    use super::*;

    command!(CommandMetadata::build("new", "Create new DID")
                .add_optional_param("did", "Known DID for new wallet instance")
                .add_optional_deferred_param("seed", "Seed for creating DID key-pair (UTF-8, base64 or hex)")
                .add_optional_param("method", "Method name to create fully qualified DID")
                .add_optional_param("metadata", "DID metadata")
                .add_example("did new")
                .add_example("did new did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("did new did=VsKV7grR1BUE29mG2Fm2kX method=indy")
                .add_example("did new did=VsKV7grR1BUE29mG2Fm2kX seed=00000000000000000000000000000My1")
                .add_example("did new seed=00000000000000000000000000000My1 metadata=did_metadata")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let did = get_opt_str_param("did", params).map_err(error_err!())?;
        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;
        let method = get_opt_str_param("method", params).map_err(error_err!())?;
        let metadata = get_opt_empty_str_param("metadata", params).map_err(error_err!())?;

        let config = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "did", did);
            update_json_map_opt_key!(json, "seed", seed);
            update_json_map_opt_key!(json, "method_name", method);
            JSONValue::from(json).to_string()
        };

        trace!(r#"Did::new try: config {:?}"#, secret!(&config));

        let res =
            Did::new(wallet_handle, config.as_str())
                .and_then(|(did, vk)|
                    match Did::abbreviate_verkey(&did, &vk) {
                        Ok(vk) => Ok((did, vk)),
                        Err(_) => Ok((did, vk))
                    });

        trace!(r#"Did::new return: {:?}"#, res);

        let res = match res {
            Ok((did, vk)) => {
                println_succ!("Did \"{}\" has been created with \"{}\" verkey", did, vk);
                Ok(did)
            }
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            }
        };

        let res = if let Some(metadata) = metadata {
            res.and_then(|did| {
                let res = Did::set_metadata(wallet_handle, &did, metadata);
                match res {
                    Ok(()) => {
                        println_succ!("Metadata has been saved for DID \"{}\"", did);
                        Ok(())
                    },
                    Err(err) => {
                        handle_indy_error(err, None, None, None);
                        Err(())
                    }
                }
            })
        } else {
            res.map(|_| ())
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod import_command {
    use super::*;
    use std::io::Read;

    command!(CommandMetadata::build("import", "Import DIDs entities from file to the current wallet.
        File format:
        {
            \"version\": 1,
            \"dids\": [{
                \"did\": \"did\",
                \"seed\": \"UTF-8, base64 or hex string\"
            }]
        }")
                .add_main_param("file", "Path to file with DIDs")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let path = get_str_param("file", params).map_err(error_err!())?;

        let mut buf = String::new();
        let res = File::open(path)
            .and_then(|mut file| {
                file.read_to_string(&mut buf)
            })
            .map_err(|err| format!("Error during reading file {}", err))
            .and_then(|_| {
                serde_json::from_str::<JSONValue>(&buf)
                    .map_err(|err| format!("Can't parse JSON {:?}", err))
                    .and_then(|json: JSONValue| -> Result<JSONValue, String> {
                        let is_correct_version = json["version"].as_i64().map(|ver| (ver == 1)).unwrap_or(false);
                        if is_correct_version { Ok(json) } else { Err("Invalid or missed version".to_owned()) }
                    })
                    .and_then(|json| {
                        json["dids"].as_array().map(Clone::clone).ok_or("missed DIDs".to_owned())
                    })
                    .and_then(|dids| {
                        for did in dids {
                            match Did::new(wallet_handle, &did.to_string())
                                .and_then(|(did, vk)|
                                    match Did::abbreviate_verkey(&did, &vk) {
                                        Ok(vk) => Ok((did, vk)),
                                        Err(err) => Err(err)
                                    }) {
                                Ok((did, vk)) =>
                                    println_succ!("Did \"{}\" has been created with \"{}\" verkey", did, vk),
                                Err(err) =>
                                    println_warn!("Indy SDK error occured {} while importing DID {}", err.message, did)
                            }
                        }
                        Ok(())
                    })
            });

        match res {
            Err(err) =>
                println_err!("{}", err),
            Ok(_) =>
                println_succ!("DIDs import finished")
        };

        trace!("execute << ");
        Ok(())
    }
}

pub mod use_command {
    use super::*;

    command!(CommandMetadata::build("use", "Use DID")
                .add_main_param_with_dynamic_completion("did", "Did stored in wallet", DynamicCompletionType::Did)
                .add_example("did use VsKV7grR1BUE29mG2Fm2kX")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?}, params {:?}", ctx, params);

        let did = get_str_param("did", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;

        let res = match Did::get_did_with_meta(wallet_handle, did) {
            Ok(_) => {
                set_active_did(ctx, Some(did.to_owned()));
                println_succ!("Did \"{}\" has been set as active", did);
                Ok(())
            }
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletItemNotFound => {
                        println_err!("Requested DID not found");
                        Err(())
                    },
                    _ => {
                        handle_indy_error(err, Some(&did), None, None);
                        Err(())
                    },
                }
            }
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod rotate_key_command {
    use super::*;

    command!(CommandMetadata::build("rotate-key", "Rotate keys for active did")
                .add_optional_deferred_param("seed", "If not provide then a random one will be created (UTF-8, base64 or hex)")
                .add_optional_param("source_payment_address","Payment address of sender.")
                .add_optional_param("fee","Transaction fee set on the ledger.")
                .add_optional_param("fees_inputs","The list of source inputs")
                .add_optional_param("fees_outputs","The list of outputs in the following format: (recipient, amount)")
                .add_optional_param("extra","Optional information for fees payment operation")
                .add_optional_param("resume", "Resume interrupted operation")
                .add_example("did rotate-key")
                .add_example("did rotate-key seed=00000000000000000000000000000My2")
                .add_example("did rotate-key fees_inputs=pay:null:111_rBuQo2A1sc9jrJg fees_outputs=(pay:null:FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4,100)")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;

        let resume = get_opt_bool_param("resume", params).map_err(error_err!())?.unwrap_or(false);

        let did = ensure_active_did(&ctx)?;
        let (pool_handle, pool_name) = ensure_connected_pool(&ctx)?;
        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let (new_verkey, update_ledger) = if resume {
            // get temp and current verkey from wallet.
            let (temp_verkey, curr_verkey) = Did::get_did_with_meta(wallet_handle, &did)
                .map_err(|e| println_err!("Unable to get did: {}", e.message))
                .and_then(|did_info| {
                    serde_json::from_str::<JSONValue>(&did_info)
                        .map_err(|e| println_err!("{}", e))
                        .and_then(|did_info| {
                            let temp_verkey = match did_info["tempVerkey"].as_str() {
                                Some(temp_verkey) => Ok(temp_verkey.to_owned()),
                                None => {
                                    println_err!("Unable to resume, have you already run rotate-key?");
                                    Err(())
                                }
                            }?;
                            let verkey = match did_info["verkey"].as_str() {
                                Some(verkey) => Ok(verkey.to_owned()),
                                None => {
                                    println_err!("Fatal error, no verkey in wallet");
                                    Err(())
                                }
                            }?;
                            Ok((temp_verkey, verkey))
                        })
                })?;

            // get verkey from ledger
            let ledger_verkey = _get_current_verkey(pool_handle, &pool_name, wallet_handle, &wallet_name, &did)?;

            match ledger_verkey {
                Some(ledger_verkey) => {
                    // if ledger verkey is abbreviated, abbreviate other also.
                    let (temp_verkey, curr_verkey) = if ledger_verkey.starts_with('~') {
                        let temp_verkey = Did::abbreviate_verkey(&did, &temp_verkey)
                            .map_err(|_e| println_err!("Invalid temp verkey: {}", temp_verkey))?;
                        let curr_verkey = Did::abbreviate_verkey(&did, &curr_verkey)
                            .map_err(|_e| println_err!("Invalid current verkey: {}", curr_verkey))?;
                        Ok((temp_verkey, curr_verkey))
                    } else {
                        Ok((temp_verkey, curr_verkey))
                    }?;

                    println_succ!("Verkey on ledger: {}", ledger_verkey);
                    println_succ!("Current verkey in wallet: {}", curr_verkey);
                    println_succ!("Temp verkey in wallet: {}", temp_verkey);

                    if ledger_verkey == temp_verkey {
                        // ledger is updated, need to apply change to wallet.
                        Ok((temp_verkey, false))
                    } else if ledger_verkey == curr_verkey {
                        // ledger have old state, send nym request and apply change to wallet.
                        Ok((temp_verkey, true))
                    } else {
                        // some invalid state
                        println_err!("Unable to resume, verkey on ledger is completely different from verkey in wallet");
                        Err(())
                    }
                }
                None => {
                    println_err!("No verkey on ledger for did: {}", did);
                    Err(())
                }
            }?
        } else {
            let identity_json = {
                let mut json = JSONMap::new();
                update_json_map_opt_key!(json, "seed", seed);
                JSONValue::from(json).to_string()
            };

            let new_verkey = match Did::replace_keys_start(wallet_handle, &did, &identity_json) {
                Ok(request) => Ok(request),
                Err(err) => {
                    match err.error_code {
                        ErrorCode::WalletItemNotFound => {
                            println_err!("Active DID: \"{}\" not found", did);
                            Err(())
                        },
                        _ => {
                            handle_indy_error(err, Some(&did), Some(&pool_name), Some(&wallet_name));
                            Err(())
                        },
                    }
                }
            }?;

            (new_verkey, true)
        };

        let receipts = if update_ledger {
            let mut request = Ledger::build_nym_request(&did, &did, Some(&new_verkey), None, None)
                .map_err(|err| handle_indy_error(err, Some(&did), Some(&pool_name), Some(&wallet_name)))?;

            ledger::set_author_agreement(ctx, &mut request)?;

            let payment_method = set_request_fees(ctx, params, &mut request, wallet_handle, Some(&did))?;

            let response_json = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &request)
                .map_err(|err| {
                    match err.error_code {
                        ErrorCode::PoolLedgerTimeout => {
                            println_err!("Transaction response has not beed received");
                            println_err!("Use command `did rotate-key resume=true` to complete");
                        }
                        _ => handle_indy_error(err, Some(&did), Some(&pool_name), Some(&wallet_name))
                    }
                })?;

            let response: Response<serde_json::Value> = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
                .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;

            handle_transaction_response(response)?;

            parse_response_with_fees(&response_json, payment_method)?
        } else { None };

        match Did::replace_keys_apply(wallet_handle, &did)
            .and_then(|_| Did::abbreviate_verkey(&did, &new_verkey)) {
            Ok(vk) => {
                println_succ!("Verkey for did \"{}\" has been updated", did);
                println_succ!("New verkey is \"{}\"", vk);
                Ok(())
            },
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletItemNotFound => {
                        println_err!("Active DID: \"{}\" not found", did);
                        Err(())
                    },
                    _ => {
                        handle_indy_error(err, Some(&did), Some(&pool_name), Some(&wallet_name));
                        Err(())
                    },
                }
            }
        }?;

        let res = print_response_receipts(receipts);

        trace!("execute << {:?}", res);
        res
    }
}

fn _get_current_verkey(pool_handle: PoolHandle, pool_name: &str, wallet_handle: WalletHandle, wallet_name: &str, did: &str) -> Result<Option<String>, ()> {
    //TODO: There nym is requested. Due to freshness issues response might be stale or outdated. Something should be done with it
    let response_json = Ledger::build_get_nym_request(Some(did), did)
        .and_then(|request| Ledger::sign_and_submit_request(pool_handle, wallet_handle, did, &request))
        .map_err(|err| handle_indy_error(err, Some(did), Some(pool_name), Some(wallet_name)))?;
    let response: Response<serde_json::Value> = serde_json::from_str::<Response<serde_json::Value>>(&response_json)
        .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;
    let result = handle_transaction_response(response)?;
    let data = serde_json::from_str::<serde_json::Value>(&result["data"].as_str().unwrap_or(""))
        .map_err(|_| println_err!("Wrong data has been received"))?;
    let verkey = data["verkey"].as_str().map(String::from);
    Ok(verkey)
}

pub mod list_command {
    use super::*;

    command!(CommandMetadata::build("list", "List my DIDs stored in the opened wallet.")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let res = match Did::list_dids_with_meta(wallet_handle) {
            Ok(dids) => {
                let mut dids: Vec<serde_json::Value> = serde_json::from_str(&dids)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                for did_info in dids.iter_mut() {
                    match Did::abbreviate_verkey(did_info["did"].as_str().unwrap_or(""),
                                                 did_info["verkey"].as_str().unwrap_or("")) {
                        Ok(vk) => did_info["verkey"] = serde_json::Value::String(vk),
                        Err(err) => {
                            handle_indy_error(err, None, None, None);
                            return Err(())
                        }
                    }
                }

                print_list_table(&dids,
                                 &[("did", "Did"),
                                     ("verkey", "Verkey"),
                                     ("metadata", "Metadata")],
                                 "There are no dids");
                if let Some(cur_did) = get_active_did(ctx) {
                    println_succ!("Current did \"{}\"", cur_did);
                }
                Ok(())
            }
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod qualify_command {
    use super::*;

    command!(CommandMetadata::build("qualify", "Update DID stored in the wallet to make fully qualified, or to do other DID maintenance.")
                .add_main_param_with_dynamic_completion("did", "Did stored in wallet", DynamicCompletionType::Did)
                .add_required_param("method", "Method to apply to the DID.")
                .add_example("did qualify VsKV7grR1BUE29mG2Fm2kX method=did:peer")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?}, params {:?}", ctx, params);

        let did = get_str_param("did", params).map_err(error_err!())?;
        let method = get_str_param("method", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;

        let res = match Did::qualify_did(wallet_handle, &did, &method) {
            Ok(full_qualified_did) => {
                println_succ!("Fully qualified DID \"{}\"", full_qualified_did);

                if let Some(active_did) = get_active_did(&ctx) {
                    if active_did == did {
                        set_active_did(ctx, Some(full_qualified_did.to_owned()));
                        println_succ!("Target DID is the same as CLI active. Active DID has been updated");
                    }
                }

                Ok(())
            }
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletItemNotFound => {
                        println_err!("Requested DID not found");
                        Err(())
                    },
                    _ => {
                        handle_indy_error(err, Some(&did), None, None);
                        Err(())
                    },
                }
            }
        };

        trace!("execute << {:?}", res);
        res
    }
}

fn _list_dids(ctx: &CommandContext) -> Vec<serde_json::Value> {
    get_opened_wallet(ctx)
        .and_then(|(wallet_handle, _)|
            Did::list_dids_with_meta(wallet_handle).ok()
        )
        .and_then(|dids|
            serde_json::from_str::<Vec<serde_json::Value>>(&dids).ok()
        )
        .unwrap_or(vec![])
}

pub fn dids(ctx: &CommandContext) -> Vec<(String, String)> {
    _list_dids(ctx)
        .into_iter()
        .map(|did|
            (did["did"].as_str().map(String::from).unwrap_or(String::new()), did["verkey"].as_str().map(String::from).unwrap_or(String::new()))
        )
        .map(|(did, verkey)| {
            let verkey_ = Did::abbreviate_verkey(&did, &verkey).unwrap_or(verkey);
            (did, verkey_)
        })
        .collect()
}

pub fn list_dids(ctx: &CommandContext) -> Vec<String> {
    _list_dids(ctx)
        .into_iter()
        .map(|did|
            did["did"].as_str().map(String::from).unwrap_or(String::new())
        )
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::libindy::did::Did;
    use crate::commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet};
    use crate::commands::pool::tests::{create_and_connect_pool, disconnect_and_delete_pool};
    use crate::commands::ledger::tests::send_nym;

    pub const SEED_TRUSTEE: &'static str = "000000000000000000000000Trustee1";
    pub const DID_TRUSTEE: &'static str = "V4SGRU86Z58d6TV7PBUe6f";
    pub const VERKEY_TRUSTEE: &'static str = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";

    pub const SEED_MY1: &'static str = "00000000000000000000000000000My1";
    pub const DID_MY1: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
    pub const VERKEY_MY1: &'static str = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

    pub const SEED_MY3: &'static str = "00000000000000000000000000000My3";
    pub const DID_MY3: &'static str = "5Uu7YveFSGcT3dSzjpvPab";
    pub const VERKEY_MY3: &'static str = "3SeuRm3uYuQDYmHeuMLu1xNHozNTtzS3kbZRFMMCWrX4";

    mod did_new {
        use super::*;

        #[test]
        pub fn new_works() {
            let ctx = setup_with_wallet();
            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(&ctx);
            assert_eq!(1, dids.len());

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_did() {
            let ctx = setup_with_wallet();
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let did = get_did_info(&ctx, DID_TRUSTEE);
            assert_eq!(did["did"].as_str().unwrap(), DID_TRUSTEE);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_seed() {
            let ctx = setup_with_wallet();
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", SEED_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let did = get_did_info(&ctx, DID_TRUSTEE);
            assert_eq!(did["did"].as_str().unwrap(), DID_TRUSTEE);
            assert_eq!(did["verkey"].as_str().unwrap(), VERKEY_TRUSTEE);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_hex_seed() {
            let ctx = setup_with_wallet();
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", "94a823a6387cdd30d8f7687d95710ebab84c6e277b724790a5b221440beb7df6".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            get_did_info(&ctx, "HWvjYf77k1dqQAk6sE4gaS");

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_meta() {
            let ctx = setup_with_wallet();
            let metadata = "metadata";
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("metadata", metadata.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(&ctx);
            assert_eq!(1, dids.len());
            assert_eq!(dids[0]["metadata"].as_str().unwrap(), metadata);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_no_opened_wallet() {
            let ctx = setup();
            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }

        #[test]
        pub fn new_works_for_wrong_seed() {
            let ctx = setup_with_wallet();
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", "invalid_base58_string".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_method_name() {
            let ctx = setup_with_wallet();
            let method = "sov";
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", SEED_TRUSTEE.to_string());
                params.insert("method", method.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let expected_did = format!("did:{}:{}", method, DID_TRUSTEE);
            let did = get_did_info(&ctx, &expected_did);
            assert_eq!(did["did"].as_str().unwrap(), &expected_did);
            assert_eq!(did["verkey"].as_str().unwrap(), VERKEY_TRUSTEE);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_not_abbreviatable() {
            let ctx = setup_with_wallet();
            let method = "indy";
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", SEED_TRUSTEE.to_string());
                params.insert("method", method.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let expected_did = format!("did:{}:{}", method, DID_TRUSTEE);
            let did = get_did_info(&ctx, &expected_did);
            assert_eq!(did["did"].as_str().unwrap(), &expected_did);
            assert_eq!(did["verkey"].as_str().unwrap(), VERKEY_TRUSTEE);

            tear_down_with_wallet(&ctx);
        }
    }

    mod did_use {
        use super::*;

        #[test]
        pub fn use_works() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_TRUSTEE);
            {
                let cmd = use_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert_eq!(ensure_active_did(&ctx).unwrap(), DID_TRUSTEE);
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn use_works_for_unknown_did() {
            let ctx = setup_with_wallet();
            {
                let cmd = use_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn use_works_for_closed_wallet() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_TRUSTEE);
            close_and_delete_wallet(&ctx);
            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    mod did_list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_TRUSTEE);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_empty_result() {
            let ctx = setup_with_wallet();
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_closed_wallet() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_TRUSTEE);
            close_and_delete_wallet(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    mod did_rotate_key {
        use super::*;
        #[cfg(feature = "nullpay_plugin")]
        use crate::commands::ledger::tests::{set_fees, create_address_and_mint_sources, get_source_input, FEES, OUTPUT};

        fn ensure_nym_written(ctx: &CommandContext, did: &str, verkey: &str) {
            let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
            let request = Ledger::build_get_nym_request(None, did).unwrap();
            let request = Ledger::sign_request(wallet_handle, did, &request).unwrap();
            submit_retry(ctx, &request, |response| {
                let res = req_for_nym(response);
                match res {
                    Some(ref verkey_received) if verkey_received == verkey => Ok(()),
                    _ => Err(())
                }
            }).unwrap()
        }

        fn req_for_nym(response: &str) -> Option<String> {
            let parsed = serde_json::from_str::<serde_json::Value>(&response).ok()?;
            let data = parsed["result"]["data"].as_str()?;
            let data = serde_json::from_str::<serde_json::Value>(&data).ok()?;
            let verkey = data["verkey"].as_str()?;
            Some(verkey.to_string())
        }

        #[test]
        pub fn rotate_works() {
            let ctx = setup_with_wallet_and_pool();

            new_did(&ctx, SEED_TRUSTEE);

            let wallet_handle = ensure_opened_wallet_handle(&ctx).unwrap();
            let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, &did, &verkey, None);
            ensure_nym_written(&ctx, &did, &verkey);
            use_did(&ctx, &did);

            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), verkey);
            {
                let cmd = rotate_key_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let did_info = get_did_info(&ctx, &did);
            assert_ne!(did_info["verkey"].as_str().unwrap(), verkey);

            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn rotate_resume_works_when_ledger_updated() {
            let ctx = setup();

            let wallet_handle = create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            let pool_handle = ensure_connected_pool_handle(&ctx).unwrap();

            new_did(&ctx, SEED_TRUSTEE);

            let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, &did, &verkey, None);
            use_did(&ctx, &did);

            let new_verkey = Did::replace_keys_start(wallet_handle, &did, "{}").unwrap();
            let request = Ledger::build_nym_request(&did, &did, Some(&new_verkey), None, None).unwrap();
            Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &request).unwrap();
            ensure_nym_written(&ctx, &did, &new_verkey);

            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), verkey);
            assert_eq!(did_info["tempVerkey"].as_str().unwrap(), new_verkey);
            {
                let cmd = rotate_key_command::new();
                let mut params = CommandParams::new();
                params.insert("resume", "true".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), new_verkey);
            assert_eq!(did_info["tempVerkey"].as_str(), None);

            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn rotate_resume_works_when_ledger_not_updated() {
            let ctx = setup();

            let wallet_handle = create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);

            let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, &did, &verkey, None);
            use_did(&ctx, &did);
            ensure_nym_written(&ctx, &did, &verkey);

            let new_verkey = Did::replace_keys_start(wallet_handle, &did, "{}").unwrap();

            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), verkey);
            assert_eq!(did_info["tempVerkey"].as_str().unwrap(), new_verkey);
            {
                let cmd = rotate_key_command::new();
                let mut params = CommandParams::new();
                params.insert("resume", "true".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), new_verkey);
            assert_eq!(did_info["tempVerkey"].as_str(), None);

            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        pub fn rotate_resume_without_started_rotation_rejected() {
            let ctx = setup();

            let wallet_handle = create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);

            let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, &did, &verkey, None);
            use_did(&ctx, &did);

            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), verkey);
            assert_eq!(did_info["tempVerkey"].as_str(), None);
            {
                let cmd = rotate_key_command::new();
                let mut params = CommandParams::new();
                params.insert("resume", "true".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), verkey); // it is not changed.
            assert_eq!(did_info["tempVerkey"].as_str(), None);

            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
            tear_down();
        }

        #[test]
        #[cfg(feature = "nullpay_plugin")]
        pub fn rotate_works_for_set_fees() {
            let ctx = setup();

            let wallet_handle = create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            use_did(&ctx, DID_TRUSTEE);

            let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();

            send_nym(&ctx, &did, &verkey, None);
            set_fees(&ctx, FEES);

            use_did(&ctx, &did);

            let payment_address_from = create_address_and_mint_sources(&ctx);
            let input = get_source_input(&ctx, &payment_address_from);

            let did_info = get_did_info(&ctx, &did);
            assert_eq!(did_info["verkey"].as_str().unwrap(), &verkey);
            {
                let cmd = rotate_key_command::new();
                let mut params = CommandParams::new();
                params.insert("fees_inputs", input);
                params.insert("fees_outputs", OUTPUT.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let did_info = get_did_info(&ctx, &did);
            assert_ne!(did_info["verkey"].as_str().unwrap(), &verkey);

            tear_down_with_wallet_and_pool(&ctx);
        }

        #[test]
        pub fn rotate_works_for_no_active_did() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = rotate_key_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet_and_pool(&ctx);
        }
    }

    mod qualify_did {
        use super::*;

        const METHOD: &str = "peer";

        #[test]
        pub fn qualify_did_works() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_MY1);
            {
                let cmd = qualify_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("method", METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn qualify_did_works_for_active() {
            let ctx = setup_with_wallet();
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_MY1);
            {
                let cmd = qualify_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("method", METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn qualify_did_works_for_unknown_did() {
            let ctx = setup_with_wallet();
            {
                let cmd = qualify_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_MY1.to_string());
                params.insert("method", METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }
    }

    fn get_did_info(ctx: &CommandContext, did: &str) -> serde_json::Value {
        let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
        let did_info = Did::get_did_with_meta(wallet_handle, did).unwrap();
        serde_json::from_str(&did_info).unwrap()
    }

    fn get_dids(ctx: &CommandContext) -> Vec<serde_json::Value> {
        let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
        let dids = Did::list_dids_with_meta(wallet_handle).unwrap();
        serde_json::from_str(&dids).unwrap()
    }

    pub fn new_did(ctx: &CommandContext, seed: &str) {
        {
            let cmd = new_command::new();
            let mut params = CommandParams::new();
            params.insert("seed", seed.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn use_did(ctx: &CommandContext, did: &str) {
        {
            let cmd = use_command::new();
            let mut params = CommandParams::new();
            params.insert("did", did.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }
}

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use utils::table::print_table;

use libindy::ErrorCode;

use libindy::did::Did;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("did", "Identity management commands"));
}

pub mod new_command {
    use super::*;

    command!(CommandMetadata::build("new", "Create new DID")
                .add_param("did", true, "Known DID for new wallet instance")
                .add_param("seed", true, "Seed for creating DID key-pair")
                .add_param("cid", true, "Create DID as CID (default false)")
                .add_param("metadata", true, "DID metadata")
                .add_example("did new")
                .add_example("did new did=VsKV7grR1BUE29mG2Fm2kX")
                .add_example("did new did=VsKV7grR1BUE29mG2Fm2kX seed=00000000000000000000000000000My1")
                .add_example("did new seed=00000000000000000000000000000My1 cid=true")
                .add_example("did new seed=00000000000000000000000000000My1 cid=true metadata=did_metadata")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let did = get_opt_str_param("did", params).map_err(error_err!())?;
        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;
        let cid = get_opt_bool_param("cid", params).map_err(error_err!())?;
        let metadata = get_opt_str_param("metadata", params).map_err(error_err!())?;

        let config = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "did", did);
            update_json_map_opt_key!(json, "seed", seed);
            update_json_map_opt_key!(json, "cid", cid);
            JSONValue::from(json).to_string()
        };

        trace!(r#"Did::new try: config {:?}"#, config);

        let res = Did::new(wallet_handle, config.as_str());

        trace!(r#"Did::new return: {:?}"#, res);

        let res = match res {
            Ok((did, vk)) => {
                println_succ!("Did \"{}\" has been created with \"{}\" verkey", did, vk);
                Ok(did)
            }
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        let res = if let Some(metadata) = metadata {
            res.and_then(|did| {
                let res = Did::set_metadata(wallet_handle, &did, metadata);
                match res {
                    Ok(()) => Ok(println_succ!("Metadata has been saved for DID \"{}\"", did)),
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            })
        } else {
            res.map(|_| ())
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod use_command {
    use super::*;

    command!(CommandMetadata::build("use", "Use DID")
                .add_main_param("did", "Did stored in wallet")
                .add_example("did use VsKV7grR1BUE29mG2Fm2kX")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?}, params {:?}", ctx, params);

        let did = get_str_param("did", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(ctx)?;

        let res = match Did::get_did_with_meta(wallet_handle, did) {
            Ok(_) => {
                set_active_did(ctx, Some(did.to_owned()));
                Ok(println_succ!("Did \"{}\" has been set as active", did))
            }
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Invalid DID format")),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Requested DID not found")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod rotate_key_command {
    use super::*;

    command!(CommandMetadata::build("rotate-key", "Rotate keys for active did")
                .add_param("seed", true, "If not provide then a random one will be created")
                .add_example("did rotate-key")
                .add_example("did rotate-key seed=00000000000000000000000000000My2")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;

        let did = ensure_active_did(&ctx)?;
        let pool_handle = ensure_connected_pool_handle(&ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let identity_json = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "seed", seed);
            JSONValue::from(json).to_string()
        };

        let new_verkey = match Did::replace_keys_start(wallet_handle, &did, &identity_json) {
            Ok(request) => Ok(request),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Active DID: \"{}\" not found", did)),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        let request = match Ledger::build_nym_request(&did, &did, Some(&new_verkey), None, None) {
            Ok(request) => Ok(request),
            Err(_) => return Err(println_err!("Wrong command params")),
        }?;

        match Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did, &request) {
            Ok(response) => Ok(response),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Active DID: \"{}\" not found", did)),
            Err(ErrorCode::WalletIncompatiblePoolError) => Err(println_err!("Pool handle \"{}\" invalid for wallet handle \"{}\"", pool_handle, wallet_handle)),
            Err(ErrorCode::LedgerInvalidTransaction) => Err(println_err!("Invalid NYM transaction \"{}\"", request)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        }?;

        let res = match Did::replace_keys_apply(wallet_handle, &did) {
            Ok(_) => Ok(println_succ!("Verkey has been updated. New verkey: \"{}\"", new_verkey)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Active DID: \"{}\" not found", did)),
            Err(_) => return Err(println_err!("Wrong command params")),
        };

        trace!("execute << {:?}", res);
        res
    }
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
                let dids: Vec<serde_json::Value> = serde_json::from_str(&dids)
                    .map_err(|_| println_err!("Wrong data has been received"))?;
                if dids.len() > 0 {
                    print_table(&dids,
                                &vec![("did", "Did"),
                                      ("verkey", "Verkey"),
                                      ("metadata", "Metadata")]);
                } else {
                    println_succ!("There are no dids");
                }
                if let Some(cur_did) = get_active_did(ctx) {
                    println_succ!("Current did \"{}\"", cur_did);
                }
                Ok(())
            }
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use libindy::did::Did;
    use commands::wallet::tests::{create_and_open_wallet, close_and_delete_wallet};
    use commands::pool::tests::{create_and_connect_pool, disconnect_and_delete_pool};
    use commands::ledger::tests::send_nym;

    pub const SEED_TRUSTEE: &'static str = "000000000000000000000000Trustee1";
    pub const DID_TRUSTEE: &'static str = "V4SGRU86Z58d6TV7PBUe6f";
    pub const VERKEY_TRUSTEE: &'static str = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";

    pub const SEED_MY1: &'static str = "00000000000000000000000000000My1";
    pub const DID_MY1: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
    pub const VERKEY_MY1: &'static str = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

    pub const SEED_MY2: &'static str = "00000000000000000000000000000My2";
    pub const DID_MY2: &'static str = "2PRyVHmkXQnQzJQKxHxnXC";
    pub const VERKEY_MY2: &'static str = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";

    pub const SEED_MY3: &'static str = "00000000000000000000000000000My3";
    pub const DID_MY3: &'static str = "5Uu7YveFSGcT3dSzjpvPab";

    mod did_new {
        use super::*;

        #[test]
        pub fn new_works() {
            let ctx = CommandContext::new();

            let wallet_handle = create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_eq!(1, dids.len());

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_did() {
            let ctx = CommandContext::new();

            let wallet_handle = create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_eq!(1, dids.len());
            assert_eq!(dids[0]["did"].as_str().unwrap(), DID_TRUSTEE);

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_seed() {
            let ctx = CommandContext::new();

            let wallet_handle = create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", SEED_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_eq!(1, dids.len());
            assert_eq!(dids[0]["did"].as_str().unwrap(), DID_TRUSTEE);
            assert_eq!(dids[0]["verkey"].as_str().unwrap(), VERKEY_TRUSTEE);

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_meta() {
            let ctx = CommandContext::new();

            let metadata = "metadata";

            let wallet_handle = create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("metadata", metadata.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_eq!(1, dids.len());
            assert_eq!(dids[0]["metadata"].as_str().unwrap(), metadata);

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_cid() {
            let ctx = CommandContext::new();

            let wallet_handle = create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("cid", "true".to_string());
                params.insert("seed", SEED_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_eq!(1, dids.len());
            assert_eq!(dids[0]["did"].as_str().unwrap(), VERKEY_TRUSTEE);

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn new_works_for_no_opened_wallet() {
            let ctx = CommandContext::new();

            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }

        #[test]
        pub fn new_works_for_wrong_seed() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = new_command::new();
                let mut params = CommandParams::new();
                params.insert("seed", "invalid_base58_string".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
        }
    }

    mod did_use {
        use super::*;

        #[test]
        pub fn use_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            {
                let cmd = use_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert_eq!(ensure_active_did(&ctx).unwrap(), DID_TRUSTEE);

            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn use_works_for_unknow_did() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = use_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID_TRUSTEE.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn use_works_for_closed_wallet() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            close_and_delete_wallet(&ctx);
            {
                let cmd = new_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }
    }

    mod did_list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_empty_result() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_closed_wallet() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_TRUSTEE);
            close_and_delete_wallet(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }
    }

    mod did_rotate_key {
        use super::*;

        #[test]
        pub fn rotate_works() {
            let ctx = CommandContext::new();

            let wallet_handle = create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);

            new_did(&ctx, SEED_TRUSTEE);
            new_did(&ctx, SEED_MY2);
            use_did(&ctx, DID_TRUSTEE);
            send_nym(&ctx, DID_MY2, VERKEY_MY2, None);
            use_did(&ctx, DID_MY2);

            let dids = get_dids(wallet_handle);
            assert_eq!(dids[0]["verkey"].as_str().unwrap(), VERKEY_MY2);
            {
                let cmd = rotate_key_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let dids = get_dids(wallet_handle);
            assert_ne!(dids[0]["verkey"].as_str().unwrap(), VERKEY_MY2);

            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn rotate_works_for_no_active_did() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            create_and_connect_pool(&ctx);
            {
                let cmd = rotate_key_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            disconnect_and_delete_pool(&ctx);
        }
    }

    fn get_dids(wallet_handle: i32) -> Vec<serde_json::Value> {
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
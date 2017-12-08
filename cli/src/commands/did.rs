use application_context::ApplicationContext;
use indy_context::IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::*;
use utils::table::print_table;

use libindy::ErrorCode;

use libindy::did::Did;
use libindy::ledger::Ledger;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Group {
    metadata: GroupMetadata
}

impl Group {
    pub fn new() -> Group {
        Group {
            metadata: GroupMetadata::new("did", "Identity management commands")
        }
    }
}

impl GroupTrait for Group {
    fn metadata(&self) -> &GroupMetadata {
        &self.metadata
    }
}

pub mod NewCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("new", "Create new DID")
                .add_param("did", true, "Known DID for new wallet instance")
                .add_param("seed", true, "Seed for creating DID key-pair")
                .add_param("cid", true, "Create DID as CID (default false)")
                .add_param("metadata", true, "DID metadata")
                .finalize()
    );

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("NewCommand::execute >> self {:?} params {:?}", ctx, params);

        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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

        trace!("NewCommand::execute << {:?}", res);
        res
    }
}

pub mod UseCommand {
    use super::*;

    command_with_indy_and_indy_ctx!(CommandMetadata::build("use", "Use DID")
                .add_main_param("did", "Did stored in wallet")
                .finalize());

    fn execute(app_ctx: Rc<ApplicationContext>, indy_ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("UseCommand::execute >> app_ctx {:?}, indy_ctx {:?}, params {:?}", app_ctx, indy_ctx, params);

        let did = get_str_param("did", params).map_err(error_err!())?;

        let wallet_handle = get_opened_wallet_handle(&indy_ctx)?;

        let res = match Did::get_did_with_meta(wallet_handle, did) {
            Ok(_) => {
                app_ctx.set_sub_prompt(3, &format!("did({}...{})", &did[..3], &did[did.len() - 3..]));
                indy_ctx.set_active_did(did);
                Ok(println_succ!("Did \"{}\" has been set as active", did))
            }
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Invalid DID format")),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Requested DID not found")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err))
        };

        trace!("UseCommand::execute << {:?}", res);
        res
    }
}

pub mod RotateKeyCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("rotate-key", "Rotate keys for active did")
                .add_param("seed", true, "If not provide then a random one will be created")
                .finalize());

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("RotateKeyCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;

        let did = get_active_did(&ctx)?;
        let pool_handle = get_connected_pool_handle(&ctx)?;
        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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

        trace!("RotateKeyCommand::execute << {:?}", res);
        res
    }
}

pub mod ListCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("list", "List my DIDs stored in the opened wallet.").finalize());

    fn execute(ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("ListCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = get_opened_wallet_handle(&ctx)?;

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
                if let Some(cur_did) = ctx.get_active_did() {
                    println_succ!("Current did \"{}\"", cur_did);
                }
                Ok(())
            }
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("ListCommand::execute << {:?}", res);
        res
    }
}


use indy_context::IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::{get_opt_bool_param, get_opt_str_param};

use libindy::did::Did;

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

#[derive(Debug)]
pub struct NewCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}


impl NewCommand {
    pub fn new(ctx: Rc<IndyContext>) -> NewCommand {
        NewCommand {
            ctx,
            metadata: CommandMetadata::build("new", "Create new DID")
                .add_param("did", true, "Known DID for new wallet instance")
                .add_param("seed", true, "Seed for creating DID key-pair")
                .add_param("cid", true, "Create DID as CID (default false)")
                .add_param("metadata", true, "DID metadata")
                .add_param("publish_to_ledger", true, "Send DID to ledger from current DID")
                .finalize()
        }
    }
}

impl Command for NewCommand {
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("NewCommand::execute >> self {:?} params {:?}", self, params);

        let wallet_handle = if let Some(wallet_handle) = self.ctx.get_opened_wallet_handle() {
            wallet_handle
        } else {
            return Err(println_err!("There is no opened wallet"));
        };

        let did = get_opt_str_param("did", params).map_err(error_err!())?;
        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;
        let cid = get_opt_bool_param("cid", params).map_err(error_err!())?;
        let metadata = get_opt_str_param("metadata", params).map_err(error_err!())?;
        let publish_to_ledger = get_opt_bool_param("publish_to_ledger", params).map_err(error_err!())?;

        let config = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "did", did);
            update_json_map_opt_key!(json, "seed", seed);
            update_json_map_opt_key!(json, "cid", cid);
            update_json_map_opt_key!(json, "metadata", metadata);
            update_json_map_opt_key!(json, "publish_to_ledger", publish_to_ledger);
            JSONValue::from(json).to_string()
        };

        trace!(r#"Did::new try: config {:?}"#, config);

        let res = Did::new(wallet_handle, config.as_str());

        trace!(r#"Did::new return: {:?}"#, res);

        let res = match res {
            Ok((did, vk)) => Ok(println_succ!("Did \"{}\" has been created with \"{}\" verkey", did, vk)),
            Err(err) => Err(println_err!("Did create failed with unexpected Indy SDK error {:?}", err)),
        };

        //TODO implement sending did

        trace!("NewCommand::execute << {:?}", res);
        res
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}
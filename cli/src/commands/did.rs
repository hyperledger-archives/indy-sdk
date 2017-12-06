use indy_context::IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::{get_opt_i64_param, get_str_param, get_opt_str_param};

use libindy::ErrorCode;
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
                //TODO implement parameters
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

        let config: String = "{ }".to_string(); //FIXME

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
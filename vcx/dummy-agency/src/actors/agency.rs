use actix::prelude::*;
use domain::config::AgencyConfig;
use errors::*;
use futures::*;
use utils::{did, wallet};

#[allow(unused)] // TODO: FIXME: Remove
pub struct Agency {
    // Agency wallet handle
    wallet_handle: i32,
    // Agency did
    did: String,
    // Agency did verkey
    did_verkey: String,
    // Storage type for agency and agents wallets
    storage_type: Option<String>,
    // Storage config for agency and agents wallets
    storage_config: Option<String>,
    // Storage credentials for agency and agents wallets
    storage_credentials: Option<String>,
}

impl Agency {
    pub fn new(config: AgencyConfig) -> BoxedFuture<Self> {
        let res = f_ok(())
            .and_then(move |_| {
                wallet::ensure_created(config.wallet_id
                                           .as_ref(),
                                       config.wallet_passphrase
                                           .as_ref(),
                                       config.storage_type
                                           .as_ref()
                                           .map(String::as_str),
                                       config.storage_config
                                           .as_ref()
                                           .map(String::as_str),
                                       config.storage_credentials
                                           .as_ref()
                                           .map(String::as_str))
                    .map(|_| config)
                    .chain_err(|| "Can't ensure agency wallet created")
            })
            .and_then(move |config| {
                wallet::open(config.wallet_id
                                 .as_ref(),
                             config.wallet_passphrase
                                 .as_ref(),
                             config.storage_type
                                 .as_ref()
                                 .map(String::as_str),
                             config.storage_config
                                 .as_ref()
                                 .map(String::as_str),
                             config.storage_credentials
                                 .as_ref()
                                 .map(String::as_str))
                    .map(|wallet_handle| (config, wallet_handle))
                    .chain_err(|| "Can't open agency wallet ")
            })
            .and_then(move |(config, wallet_handle)| {
                did::ensure_created(wallet_handle,
                                    config.did
                                        .as_ref(),
                                    config.did_seed
                                        .as_ref()
                                        .map(String::as_str))
                    .map(move |_| (config, wallet_handle))
                    .chain_err(|| "Can't open agency wallet")
            })
            .and_then(move |(config, wallet_handle)| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |did_verkey| (config, wallet_handle, did_verkey))
                    .chain_err(|| "Can't get agency did key")
            })
            .map(move |(config, wallet_handle, did_verkey)| {
                Agency {
                    wallet_handle,
                    did: config.did,
                    did_verkey,
                    storage_type: config.storage_type,
                    storage_config: config.storage_config,
                    storage_credentials: config.storage_credentials,
                }
            });

        Box::new(res)
    }
}

impl Actor for Agency {
    type Context = Context<Self>;
}

pub struct Get {}

#[derive(Serialize)]
pub struct GetResponse {
    did: String,
    did_verkey: String,
}

impl Message for Get {
    type Result = Result<GetResponse>;
}

impl Handler<Get> for Agency {
    type Result = Result<GetResponse>;

    fn handle(&mut self, _: Get, _: &mut Self::Context) -> Self::Result {
        let res = GetResponse {
            did: self.did.clone(),
            did_verkey: self.did_verkey.clone(),
        };

        Ok(res)
    }
}

pub struct Post(pub String); // FIXME: Just to illustrate async handler

#[derive(Serialize)]
pub struct PostResponse(String); // FIXME: Just to illustrate async handler

impl Message for Post {
    type Result = Result<PostResponse>;
}

impl Handler<Post> for Agency {
    type Result = BoxedFuture<PostResponse>;

    fn handle(&mut self, _: Post, _: &mut Self::Context) -> Self::Result {

        // FIXME: Just to illustrate async handler

        let res = did::key_for_local_did(self.wallet_handle, self.did.as_ref())
            .map(|key| PostResponse(key));

        Box::new(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;

    #[test]
    fn agency_new_works() {
        let mut core = Core::new().unwrap();

        let res = core.run(
            Agency::new(AgencyConfig {
                wallet_id: "agency_wallet_id".into(),
                wallet_passphrase: "agency_wallet_passphrase".into(),
                did: "VsKV7grR1BUE29mG2Fm2kX".into(),
                did_seed: Some("00000000000000000000000000000My1".into()),
                storage_type: None,
                storage_config: None,
                storage_credentials: None,
            }));

        res.unwrap();
    }
}
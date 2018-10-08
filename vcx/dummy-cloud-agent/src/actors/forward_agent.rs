use actix::prelude::*;
use domain::config::AgentConfig;
use failure::{Error, Fail};
use futures::*;
use indy::{did, wallet, IndyError};
use utils::futures::*;

pub struct ForwardAgent {
    wallet_handle: i32,
    verkey: String,
    config: AgentConfig,
}

impl ForwardAgent {
    pub fn new(config: AgentConfig) -> ResponseFuture<Self, Error> {

        let wallet_config = json!({
            "id": config.wallet_id,
            "storage_type": config.storage_type,
            "storage_config": config.storage_config,
         })
            .to_string();

        let wallet_credentials = json!({
            "key": config.wallet_passphrase,
            "storage_credentials": config.storage_credentials,
        })
            .to_string();

        let did_info = json!({
            "did": config.did,
            "seed": config.did_seed,
        })
            .to_string();

        future::ok(())
            .and_then(move |_| {
                wallet::create_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .then(|res| {
                        match res {
                            Err(IndyError::WalletAlreadyExistsError) => Ok(()),
                            r => r
                        }
                    })
                    .map(|_| (wallet_config, wallet_credentials))
                    .map_err(|err| err.context("Can't ensure Forward Agent wallet created.").into())
            })
            .and_then(|(wallet_config, wallet_credentials)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Forward Agent wallet.`").into())
            })
            .and_then(move |wallet_handle| {
                did::create_and_store_my_did(wallet_handle, did_info.as_ref())
                    .then(|res| match res {
                        Ok(_) => Ok(()),
                        Err(IndyError::DidAlreadyExistsError) => Ok(()), // Already exists
                        Err(err) => Err(err),
                    })
                    .map(move |_| wallet_handle)
                    .map_err(|err| err.context("Can't create Forward Agent did.").into())
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |verkey| (wallet_handle, verkey, config))
                    .map_err(|err| err.context("Can't get Forward Agent did key").into())
            })
            .map(move |(wallet_handle, verkey, config)| {
                ForwardAgent {
                    wallet_handle,
                    verkey,
                    config,
                }
            })
            .into_box()
    }
}

impl Actor for ForwardAgent {
    type Context = Context<Self>;
}

pub struct GetForwardDetail {}

#[derive(Serialize, Debug)]
pub struct ForwardDetail {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
}

impl Message for GetForwardDetail {
    type Result = Result<ForwardDetail, Error>;
}

impl Handler<GetForwardDetail> for ForwardAgent {
    type Result = Result<ForwardDetail, Error>;

    fn handle(&mut self, _: GetForwardDetail, _: &mut Self::Context) -> Self::Result {

        let res = ForwardDetail {
            did: self.config.did.clone(),
            verkey: self.verkey.clone(),
        };

        Ok(res)
    }
}

pub struct ForwardMessage(pub Vec<u8>);

#[derive(Serialize)]
pub struct ForwardMessageResponse(pub Vec<u8>);

impl Message for ForwardMessage {
    type Result = Result<ForwardMessageResponse, Error>;
}

impl Handler<ForwardMessage> for ForwardAgent {
    type Result = ResponseFuture<ForwardMessageResponse, Error>;

    fn handle(&mut self, _: ForwardMessage, _: &mut Self::Context) -> Self::Result {

        // FIXME: Just to illustrate async handler
        did::key_for_local_did(self.wallet_handle, self.config.did.as_ref())
            .map(|key| ForwardMessageResponse(key.as_bytes().to_owned()))
            .map_err(|err| err.context("Can't get Forward Agent did").into())
            .into_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;

    #[test]
    fn forward_agent_new_works() {
        let mut core = Core::new().unwrap();

        let res = core.run(
            ForwardAgent::new(AgentConfig {
                wallet_id: "Forward Agent_wallet_id".into(),
                wallet_passphrase: "Forward Agent_wallet_passphrase".into(),
                did: "VsKV7grR1BUE29mG2Fm2kX".into(),
                did_seed: Some("00000000000000000000000000000My1".into()),
                storage_type: None,
                storage_config: None,
                storage_credentials: None,
            }));

        res.unwrap();
    }
}
use actix::prelude::*;
use domain::config::AgencyConfig;
use errors::*;
use futures::*;
use indy::{did, wallet};
use indy::errors::{Error as IndyError, ErrorKind as IndyErrorKind};

pub struct Agency {
    // Agency wallet handle
    wallet_handle: i32,
    // Agency verkey
    verkey: String,
    // Agency config
    config: AgencyConfig,
}

impl Agency {
    pub fn new(config: AgencyConfig) -> BoxedFuture<Self> {
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

        let res = f_ok(())
            .and_then(move |_| {
                wallet::create_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .then(|res| {
                        match res {
                            Err(IndyError(IndyErrorKind::WalletAlreadyExistsError, _)) => Ok(()),
                            r => r,
                        }
                    })
                    .map(|_| (wallet_config, wallet_credentials))
                    .chain_err(|| "Can't ensure agency wallet created")
            })
            .and_then(|(wallet_config, wallet_credentials)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .chain_err(|| "Can't open agency wallet ")
            })
            .and_then(move |wallet_handle| {
                did::create_and_store_my_did(wallet_handle, did_info.as_ref())
                    .then(|res| match res {
                        Ok(_) => Ok(()),
                        Err(IndyError(IndyErrorKind::DidAlreadyExistsError, _)) => Ok(()), // Already exists
                        Err(err) => Err(err),
                    })
                    .map(move |_| wallet_handle)
                    .chain_err(|| "Can't create did")
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |verkey| (wallet_handle, verkey, config))
                    .chain_err(|| "Can't get agency did key")
            })
            .map(move |(wallet_handle, verkey, config)| {
                Agency {
                    wallet_handle,
                    verkey,
                    config,
                }
            });

        Box::new(res)
    }
}

impl Actor for Agency {
    type Context = Context<Self>;
}

pub struct GetAgencyDetail {}

#[derive(Serialize, Debug)]
pub struct AgencyDetail {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
}

impl Message for GetAgencyDetail {
    type Result = Result<AgencyDetail>;
}

impl Handler<GetAgencyDetail> for Agency {
    type Result = Result<AgencyDetail>;

    fn handle(&mut self, _: GetAgencyDetail, _: &mut Self::Context) -> Self::Result {
        let res = AgencyDetail {
            did: self.config.did.clone(),
            verkey: self.verkey.clone(),
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

        let res = did::key_for_local_did(self.wallet_handle, self.config.did.as_ref())
            .map(|key| PostResponse(key))
            .chain_err(|| "Can't get agency did");

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
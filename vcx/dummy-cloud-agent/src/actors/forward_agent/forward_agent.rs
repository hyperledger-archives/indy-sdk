use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::{Either, ok};
use serde_json;

use crate::actors::{Endpoint, ForwardA2AMsg, GetEndpoint, HandleA2AMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::forward_agent_connection::forward_agent_connection::ForwardAgentConnection;
use crate::actors::router::Router;
use crate::domain::a2a::*;
use crate::domain::admin_message::{ResAdminQuery, ResQueryForwardAgent};
use crate::domain::config::{ForwardAgentConfig, WalletStorageConfig};
use crate::domain::invite::ForwardAgentDetail;
use crate::indy::{did, ErrorCode, IndyError, pairwise, pairwise::Pairwise, wallet, WalletHandle};
use crate::utils::config_env::get_app_env_config;
use crate::utils::futures::*;

/// When the agency is initially started, single instance of forward agent is created. Forward agent
/// is somewhat like agency representative. It has its own DID and Verkey based on configuration
/// provided via configuration file. Any incoming messages must be on it outer most encryption
/// layer be addressed for the Forward Agent.
/// Forward agent is entity capable bootstrap personal agents within the agency.
pub struct ForwardAgent {
    /// handle to Forward Agent's wallet
    pub(super) wallet_handle: WalletHandle,
    /// Agency DID, addressable via router
    pub(super) did: String,
    /// Agency Verkey, addressable via router
    pub(super) verkey: String,
    /// Reference to router actor
    pub(super) router: Rc<RwLock<Router>>,
    /// Agency DID, Agency Verkey and Agency endpoint
    pub(super) forward_agent_detail: ForwardAgentDetail,
    /// Configuration data to access wallet storage used across Agency
    pub(super) wallet_storage_config: WalletStorageConfig,
    pub(super) admin: Option<Arc<RwLock<Admin>>>,
}

impl ForwardAgent {
    /// Called at start of agency. Because forward agent keeps track of connections which has been
    /// established between a vcx clients and the agency, if any connections has been previously
    /// created, they will be restored.
    pub fn create_or_restore(config: ForwardAgentConfig,
                             wallet_storage_config: WalletStorageConfig,
                             admin: Option<Arc<RwLock<Admin>>>) -> ResponseFuture<Addr<ForwardAgent>, Error> {
        debug!("ForwardAgent::create_or_restore >> {:?} {:?}", config, wallet_storage_config);
        future::ok(())
            .and_then(move |_| {
                // Ensure Forward Agent wallet created
                let wallet_config = json!({
                    "id": config.wallet_id,
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

                let wallet_credentials = json!({
                    "key": config.wallet_passphrase,
                    "storage_credentials": wallet_storage_config.credentials,
                }).to_string();

                wallet::create_wallet(&wallet_config, &wallet_credentials)
                    .then(|res| match res {
                        Err(IndyError { error_code: ErrorCode::WalletAlreadyExistsError, .. }) => Ok(()),
                        r => r
                    })
                    .map(|_| (config, wallet_storage_config, wallet_config, wallet_credentials))
                    .map_err(|err| err.context("Can't ensure Forward Agent wallet created.").into())
            })
            .and_then(|(config, wallet_storage_config, wallet_config, wallet_credentials)| {
                // Open Forward Agent wallet

                wallet::open_wallet(&wallet_config, &wallet_credentials)
                    .map(|wallet_handle| (wallet_handle, config, wallet_storage_config))
                    .map_err(|err| err.context("Can't open Forward Agent wallet.`").into())
            })
            .and_then(move |(wallet_handle, config, wallet_storage_config)| {
                #[cfg(test)]
                    unsafe {
                    crate::utils::tests::FORWARD_AGENT_WALLET_HANDLE = wallet_handle;
                }

                // Ensure Forward Agent DID created
                let did_info = json!({
                    "did": config.did,
                    "seed": config.did_seed,
                }).to_string();

                did::create_and_store_my_did(wallet_handle, &did_info)
                    .then(|res| match res {
                        Ok(_) => Ok(()),
                        Err(IndyError { error_code: ErrorCode::DidAlreadyExistsError, .. }) => Ok(()), // Already exists
                        Err(err) => Err(err),
                    })
                    .map(move |_| (wallet_handle, config, wallet_storage_config))
                    .map_err(|err| err.context("Can't create Forward Agent did.").into())
            })
            .and_then(move |(wallet_handle, config, wallet_storage_config)| {
                // Resolve verkey for Forward Agent DID

                did::key_for_local_did(wallet_handle,
                                       &config.did)
                    .map(move |verkey| (wallet_handle, config.did, verkey, config.endpoint, wallet_storage_config))
                    .map_err(|err| err.context("Can't get Forward Agent did key").into())
            })
            .and_then(move |(wallet_handle, did, verkey, endpoint, wallet_storage_config)| {
                Router::new()
                    .map(move |router| (wallet_handle, did, verkey, endpoint, wallet_storage_config, router))
                    .map_err(|err| err.context("Can't create Router.").into())
            })
            .and_then(move |(wallet_handle, did, verkey, endpoint, wallet_storage_config, router)| {
                // Resolve information about existing connections from the wallet
                // and start Forward Agent Connection actor for each exists connection

                let forward_agent_detail = ForwardAgentDetail {
                    did: did.clone(),
                    verkey: verkey.clone(),
                    endpoint: endpoint.clone(),
                };

                if get_app_env_config().restore_on_demand == false {
                    info!("Forward agent begins restoration of entities.");
                    Either::A(
                        Self::_restore_connections(wallet_handle,
                                                   forward_agent_detail.clone(),
                                                   wallet_storage_config.clone(),
                                                   router.clone(),
                                                   admin.clone())
                            .map(move |_| (wallet_handle, did, verkey,
                                           router, wallet_storage_config, forward_agent_detail, admin)))
                } else {
                    info!(" Forward agent will be restoring individual agency entities on demand.");
                    Either::B(Box::new(future::ok((wallet_handle, did, verkey, router, wallet_storage_config, forward_agent_detail, admin))))
                }
            })
            .and_then(|(wallet_handle, did, verkey, router,
                           wallet_storage_config, forward_agent_detail, admin)| {
                let forward_agent = ForwardAgent {
                    wallet_handle,
                    did: did.clone(),
                    verkey: verkey.clone(),
                    router: router.clone(),
                    wallet_storage_config,
                    forward_agent_detail,
                    admin: admin.clone(),
                };

                let forward_agent = forward_agent.start();

                router.write().unwrap()
                    .add_a2a_route(did.clone(), verkey.clone(), forward_agent.clone().recipient());
                if let Some(admin) = admin {
                    admin.write().unwrap().register_forward_agent(forward_agent.clone())
                };
                future::ok((forward_agent))
            })
            .into_box()
    }

    fn _restore_connections(wallet_handle: WalletHandle,
                            forward_agent_detail: ForwardAgentDetail,
                            wallet_storage_config: WalletStorageConfig,
                            router: Rc<RwLock<Router>>,
                            admin: Option<Arc<RwLock<Admin>>>) -> ResponseFuture<(), Error> {
        debug!("ForwardAgent::_restore_connections >> {:?}", wallet_handle);

        future::ok(())
            .and_then(move |_| {
                pairwise::list_pairwise(wallet_handle)
                    .map(move |pairwise_list| (pairwise_list, wallet_handle))
                    .map_err(|err| err.context("Can't get Forward Agent pairwise list").into())
            })
            .and_then(|(pairwise_list, wallet_handle)| {
                serde_json::from_str::<Vec<String>>(&pairwise_list)
                    .map(move |pairwise_list| (pairwise_list, wallet_handle))
                    .map_err(|err| err.context("Can't deserialize Forward Agent pairwise list").into())
            })
            .and_then(|(pairwise_list, wallet_handle)| {
                pairwise_list
                    .iter()
                    .map(|pairwise| serde_json::from_str::<Pairwise>(&pairwise))
                    .collect::<Result<Vec<_>, _>>()
                    .map(move |pairwise_list| (pairwise_list, wallet_handle))
                    .map_err(|err| err.context("Can't deserialize Forward Agent pairwise").into())
            })
            .and_then(move |(pairwise_list, wallet_handle)| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .map(move |pairwise| {
                        debug!("Restorin forward agent connection {:?}", pairwise);
                        ForwardAgentConnection::restore(wallet_handle,
                                                        pairwise.their_did.clone(),
                                                        forward_agent_detail.clone(),
                                                        wallet_storage_config.clone(),
                                                        router.clone(),
                                                        admin.clone())
                    })
                    .collect();
                future::join_all(futures)
                    .map(|_| ())
                    .map_err(|err| err.context("Can't restore Forward Agent connections").into())
            })
            .into_box()
    }
}

impl Actor for ForwardAgent {
    type Context = Context<Self>;
}

impl Handler<ForwardA2AMsg> for ForwardAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: ForwardA2AMsg, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Handler<ForwardA2AMsg>::handle >> {:?}", msg);
        self._forward_a2a_msg(msg.0)
    }
}

impl Handler<GetEndpoint> for ForwardAgent {
    type Result = Result<Endpoint, Error>;

    fn handle(&mut self, _msg: GetEndpoint, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Handler<GetEndpoint>::handle >> {:?}", _msg);
        let (did, verkey) = self._get_endpoint();
        Ok(Endpoint { did, verkey })
    }
}

impl Handler<HandleAdminMessage> for ForwardAgent {
    type Result = Result<ResAdminQuery, Error>;

    fn handle(&mut self, _msg: HandleAdminMessage, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Forward Agent Handler<HandleAdminMessage>::handle >>", );
        let (endpoint, pairwise_list, wallet_handle) = self._get_forward_agent_details();
        Ok(ResAdminQuery::ForwardAgent(ResQueryForwardAgent { endpoint, pairwise_list, wallet_handle }))
    }
}

impl Handler<HandleA2AMsg> for ForwardAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self._handle_a2a_msg(msg.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::*;

    use super::*;

    #[test]
    fn forward_agent_create_or_restore_works() {
        run_test(|_, _| {
            Ok(())
        });
    }

    #[test]
    fn forward_agent_get_endpoint_works() {
        run_test(|forward_agent, _| {
            forward_agent
                .send(GetEndpoint {})
                .from_err()
                .map(|res| match res {
                    Ok(endpoint) => {
                        assert_eq!(endpoint.did, FORWARD_AGENT_DID);
                        assert_eq!(endpoint.verkey, FORWARD_AGENT_DID_VERKEY);
                    }
                    Err(err) => panic!("Can't get endpoint: {:?}", err),
                })
        });
    }

    #[test]
    fn forward_agent_connect_works() {
        run_test(|forward_agent, _| {
            future::ok(())
                .map(|_| {
                    let e_wallet_handle = edge_wallet_setup().wait().unwrap();
                    let connect_msg = compose_connect(e_wallet_handle).wait().unwrap();
                    (e_wallet_handle, connect_msg)
                })
                .and_then(move |(e_wallet_handle, connect_msg)| {
                    forward_agent
                        .send(ForwardA2AMsg(connect_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |connected_msg| (e_wallet_handle, connected_msg))
                })
                .map(|(e_wallet_handle, connected_msg)| {
                    let (sender_verkey, pairwise_did, pairwise_verkey) = decompose_connected(e_wallet_handle, &connected_msg).wait().unwrap();
                    assert_eq!(sender_verkey, FORWARD_AGENT_DID_VERKEY);
                    assert!(!pairwise_did.is_empty());
                    assert!(!pairwise_verkey.is_empty());
                    e_wallet_handle
                })
                .map(|e_wallet_handle| crate::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }
}


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
use crate::actors::forward_agent_connection::ForwardAgentConnection;
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
    wallet_handle: WalletHandle,
    /// Agency DID, addressable via router
    did: String,
    /// Agency Verkey, addressable via router
    verkey: String,
    /// Reference to router actor
    router: Rc<RwLock<Router>>,
    /// Agency DID, Agency Verkey and Agency endpoint
    forward_agent_detail: ForwardAgentDetail,
    /// Configuration data to access wallet storage used across Agency
    wallet_storage_config: WalletStorageConfig,
    admin: Option<Arc<RwLock<Admin>>>,
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

    fn _get_endpoint(&self) -> (String, String) {
        trace!("ForwardAgent::_get_endpoint >>");
        (self.did.clone(), self.verkey.clone())
    }

    /// Returns list of pairwise DIDs representing connections established with Agency
    fn _get_forward_agent_details(&self) -> (String, Vec<String>, WalletHandle) {
        trace!("ForwardAgent::_get_forward_agent_details >>");
        let endpoint = self.forward_agent_detail.endpoint.clone();
        let wallet_handle = self.wallet_handle.clone();
        let pairwise_list_string = pairwise::list_pairwise(wallet_handle).wait().expect("Couldn't resolve pairwise list");
        let pairwise_list = serde_json::from_str::<Vec<String>>(&pairwise_list_string)
            .expect("Couldn't pair list of pairwises");
        (endpoint, pairwise_list, wallet_handle)
    }

    /// Handles forward messages. The assumption is that the received message is
    /// anoncrypted (using Agency's verkey) forward message.
    /// After decrypting its passed to router which takes care of delivering it to intended recipient.
    ///
    /// * `msg` - Incoming anoncrypted forward message
    fn _forward_a2a_msg(&mut self,
                        msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_forward_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_anoncrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(move |mut msgs, slf, _| {
                let send_to_router = |fwd: String, msg: Vec<u8>| {
                    slf.router.read().unwrap()
                        .route_a2a_msg(fwd, msg)
                        .from_err()
                        .map(|res| res)
                        .into_actor(slf)
                        .into_box()
                };


                match msgs.pop() {
                    Some(A2AMessage::Version1(A2AMessageV1::Forward(msg))) => {
                        send_to_router(msg.fwd, msg.msg)
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::Forward(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        send_to_router(msg.fwd, msg_)
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::ForwardV3(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        send_to_router(msg.to, msg_)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    /// Handles messages other than forward messages. The only other message types the Forward Agent
    /// is capable of handdling is "Connect" message, which translates into request to create
    /// pairwise relationship with a new uknown client. That is represented by creating
    /// a new Forward Agent Connection
    fn _handle_a2a_msg(&mut self,
                       msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Version1(A2AMessageV1::Connect(msg))) => {
                        slf._connect_v1(sender_vk, msg)
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::Connect(msg))) => {
                        slf._connect_v2(sender_vk, msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn _connect_v1(&mut self,
                   sender_vk: String,
                   msg: Connect) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_connect_v1 >> {:?}, {:?}", sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey } = msg;

        self._connect(sender_vk.clone(), their_did.clone(), their_verkey.clone())
            .and_then(move |(my_did, my_verkey), slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                }))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _connect_v2(&mut self,
                   sender_vk: String,
                   msg: Connect) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_connect_v2 >> {:?}, {:?}", sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey, .. } = msg;

        self._connect(sender_vk.clone(), their_did.clone(), their_verkey.clone())
            .and_then(move |(my_did, my_verkey), slf, _| {
                let msg = A2AMessageV2::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.verkey), &their_verkey, &msg)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    /// Creates new pairwise connection between previously unknown client and Agency.
    ///
    /// Returns
    ///
    /// # Arguments
    ///
    /// * `sender_vk` - Verkey of this Connect message sender. Must be same as their_did
    /// * `their_did` - Client DID at ClientToAgency relationship ( Owner.DID@Client:Agency )
    /// * `their_verkey` - Client VKey at ClientToAgency relationship ( Client.Verkey@Client:Agency )
    fn _connect(&mut self,
                sender_vk: String,
                their_did: String,
                their_verkey: String) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("ForwardAgent::_connect >> {:?}, {:?}, {:?}", sender_vk, their_did, their_verkey);

        if their_verkey != sender_vk {
            return err_act!(self, err_msg("Inconsistent sender and connection verkeys"));
        };

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                ForwardAgentConnection::create(slf.wallet_handle,
                                               their_did.clone(),
                                               their_verkey.clone(),
                                               slf.router.clone(),
                                               slf.forward_agent_detail.clone(),
                                               slf.wallet_storage_config.clone(),
                                               slf.admin.clone())
                    .map_err(|err| err.context("Can't create Forward Agent Connection.").into())
                    .into_actor(slf)
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


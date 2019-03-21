use actix::prelude::*;
use actors::{AddA2ARoute, Endpoint, ForwardA2AMsg, GetEndpoint, HandleA2AMsg, RouteA2AMsg};
use actors::forward_agent_connection::ForwardAgentConnection;
use actors::requester::Requester;
use actors::router::Router;
use domain::a2a::*;
use domain::config::{ForwardAgentConfig, WalletStorageConfig};
use domain::invite::ForwardAgentDetail;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, ErrorCode, IndyError, pairwise, pairwise::Pairwise, wallet};
use serde_json;
use std::convert::Into;
use utils::futures::*;

pub struct ForwardAgent {
    wallet_handle: i32,
    did: String,
    verkey: String,
    router: Addr<Router>,
    forward_agent_detail: ForwardAgentDetail,
    wallet_storage_config: WalletStorageConfig,
}

impl ForwardAgent {
    pub fn create_or_restore(config: ForwardAgentConfig,
                             wallet_storage_config: WalletStorageConfig) -> ResponseFuture<Addr<ForwardAgent>, Error> {
        trace!("ForwardAgent::create_or_restore >> {:?} {:?}", config, wallet_storage_config);
        let request = Requester::new().start();
        let router = Router::new(request).start();

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
                    ::utils::tests::FORWARD_AGENT_WALLET_HANDLE = wallet_handle;
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
                // Resolve information about existing connections from the wallet
                // and start Forward Agent Connection actor for each exists connection

                let forward_agent_detail = ForwardAgentDetail {
                    did: did.clone(),
                    verkey: verkey.clone(),
                    endpoint: endpoint.clone(),
                };

                Self::_restore_connections(wallet_handle,
                                           forward_agent_detail.clone(),
                                           wallet_storage_config.clone(),
                                           router.clone())
                    .map(move |_| (wallet_handle, did, verkey,
                                   router, wallet_storage_config, forward_agent_detail))
            })
            .and_then(|(wallet_handle, did, verkey, router,
                           wallet_storage_config, forward_agent_detail)| {
                let forward_agent = ForwardAgent {
                    wallet_handle,
                    did: did.clone(),
                    verkey,
                    router: router.clone(),
                    wallet_storage_config,
                    forward_agent_detail,
                };

                let forward_agent = forward_agent.start();

                router
                    .send(AddA2ARoute(did, forward_agent.clone().recipient()))
                    .from_err()
                    .map(move |_| forward_agent)
                    .map_err(|err: Error| err.context("Can't add route for Forward Agent").into())
            })
            .into_box()
    }

    fn _restore_connections(wallet_handle: i32,
                            forward_agent_detail: ForwardAgentDetail,
                            wallet_storage_config: WalletStorageConfig,
                            router: Addr<Router>) -> ResponseFuture<(), Error> {
        trace!("ForwardAgent::_restore_connections >> {:?}", wallet_handle);

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
                        ForwardAgentConnection::restore(wallet_handle,
                                                        pairwise.their_did.clone(),
                                                        forward_agent_detail.clone(),
                                                        wallet_storage_config.clone(),
                                                        router.clone())
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
                    slf.router
                        .send(RouteA2AMsg(fwd, msg))
                        .from_err()
                        .and_then(|res| res)
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
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

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
                                               slf.wallet_storage_config.clone())
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

impl Handler<HandleA2AMsg> for ForwardAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self._handle_a2a_msg(msg.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::tests::*;

    #[test]
    fn forward_agent_create_or_restore_works() {
        run_test(|_| {
            Ok(())
        });
    }

    #[test]
    fn forward_agent_get_endpoint_works() {
        run_test(|forward_agent| {
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
        run_test(|forward_agent| {
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
                .map(|e_wallet_handle| ::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }
}


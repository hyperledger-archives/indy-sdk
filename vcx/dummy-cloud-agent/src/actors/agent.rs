use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg, RouteA2AMsg};
use actors::agent_connection::{AgentConnection, AgentConnectionConfig};
use actors::router::Router;
use domain::a2a::*;
use domain::config::WalletStorageConfig;
use domain::invite::ForwardAgentDetail;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, pairwise, wallet};
use std::convert::Into;
use utils::futures::*;
use utils::rand;

#[allow(unused)] //FIXME:
pub struct Agent {
    wallet_handle: i32,
    owner_did: String,
    owner_verkey: String,
    did: String,
    verkey: String,
    forward_agent_detail: ForwardAgentDetail,
    router: Addr<Router>,
}

impl Agent {
    pub fn create(owner_did: &str,
                  owner_verkey: &str,
                  router: Addr<Router>,
                  forward_agent_detail: ForwardAgentDetail,
                  wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(String, String, String, String), Error> {
        let wallet_id = rand::rand_string(10);
        let wallet_key = rand::rand_string(10);

        let wallet_config = json!({
                    "id": wallet_id.clone(),
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        let wallet_credentials = json!({
                    "key": wallet_key.clone(),
                    "storage_credentials": wallet_storage_config.credentials,
                }).to_string();

        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();

        future::ok(())
            .and_then(move |_|
                wallet::create_wallet(&wallet_config, &wallet_credentials)
                    .map(|_| (wallet_config, wallet_credentials))
                    .map_err(|err| err.context("Can't create Agent wallet.").into())
            )
            .and_then(move |(wallet_config, wallet_credentials)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Cloud Agent wallet.`").into())
            })
            .and_then(|wallet_handle| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(move |(did, verkey)| (wallet_handle, did, verkey))
                    .map_err(|err| err.context("Can't get Cloud Agent did key").into())
            })
            .and_then(move |(wallet_handle, did, verkey)| {
                let agent = Agent {
                    wallet_handle,
                    verkey: verkey.clone(),
                    did: did.clone(),
                    owner_did,
                    owner_verkey,
                    router: router.clone(),
                    forward_agent_detail,
                };

                let agent = agent.start();

                router
                    .send(AddA2ARoute(did.clone(), agent.clone().recipient()))
                    .from_err()
                    .map(move |_| (wallet_id, wallet_key, did, verkey))
                    .map_err(|err: Error| err.context("Can't add route for Forward Agent").into())
            })
            .into_box()
    }

    pub fn restore(_wallet_id: &str,
                   _wallet_key: &str,
                   _did: &str,
                   _owner_did: &str,
                   _owner_verkey: &str,
                   _router: Addr<Router>,
                   _forward_agent_detail: ForwardAgentDetail,
                   _wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(), Error> {
        // FIXME: Implement me!!!
        ok!(())
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Forward(msg)) => {
                        slf.router
                            .send(RouteA2AMsg(msg.fwd, msg.msg))
                            .from_err()
                            .and_then(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(msg) => slf.handle_agent_msg(sender_vk, msg),
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn handle_agent_msg(&mut self,
                        sender_vk: String,
                        msg: A2AMessage) -> ResponseActFuture<Self, Vec<u8>, Error> {
        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _|
                match msg {
                    A2AMessage::CreateKey(msg) => {
                        slf.handle_create_key(msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            )
            .and_then(move |msgs, slf, _|
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &sender_vk, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
            )
            .into_box()
    }


    fn handle_create_key(&mut self,
                         msg: CreateKey) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::_handle_create_key >> {:?}", msg);

        let CreateKey { for_did, for_did_verkey } = msg;

        let their_did_info = json!({
            "did": for_did,
            "verkey": for_did_verkey,
        }).to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _|
                slf.check_no_pairwise_exists(&for_did)
                    .map(|_| for_did)
                    .into_actor(slf)
            )
            .and_then(move |for_did, slf, _|
                did::store_their_did(slf.wallet_handle, &their_did_info)
                    .map_err(|err| err.context("Can't store their DID for Forward Agent Connection pairwise.").into())
                    .map(|_| for_did)
                    .into_actor(slf)
            )
            .and_then(|for_did, slf, _| {
                did::create_and_store_my_did(slf.wallet_handle, "{}")
                    .map_err(|err| err.context("Can't create DID for agent pairwise connection.").into())
                    .map(|(pairwise_did, pairwise_did_verkey)| (for_did, pairwise_did, pairwise_did_verkey))
                    .into_actor(slf)
            })
            .and_then(|(for_did, pairwise_did, pairwise_did_verkey), slf, _| {
                pairwise::create_pairwise(slf.wallet_handle, &for_did, &pairwise_did, "{}")
                    .map_err(|err| err.context("Can't store agent pairwise connection.").into())
                    .map(|_| (for_did, pairwise_did, pairwise_did_verkey))
                    .into_actor(slf)
            })
            .and_then(move |(for_did, pairwise_did, pairwise_did_verkey), slf, _| {
                let config = AgentConnectionConfig {
                    wallet_handle: slf.wallet_handle,
                    owner_did: slf.owner_did.to_string(),
                    owner_verkey: slf.owner_verkey.to_string(),
                    user_pairwise_did: for_did.to_string(),
                    user_pairwise_verkey: for_did_verkey.to_string(),
                    agent_pairwise_did: pairwise_did.to_string(),
                    agent_pairwise_verkey: pairwise_did_verkey.to_string(),
                    forward_agent_detail: slf.forward_agent_detail.clone(),
                };

                AgentConnection::create(config, slf.router.clone())
                    .map(|_| (pairwise_did, pairwise_did_verkey))
                    .into_actor(slf)
            })
            .map(|(pairwise_did, pairwise_did_verkey), _, _| {
                vec![A2AMessage::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                })]
            })
            .into_box()
    }

    fn check_no_pairwise_exists(&mut self,
                                did: &str) -> ResponseFuture<(), Error> {
        pairwise::is_pairwise_exists(self.wallet_handle, did)
            .map_err(|err| err.context("Can't check if agent pairwise connection exists.").into())
            .and_then(|is_exist|
                if is_exist {
                    err!(err_msg("Agent pairwise connection already exists.")).into()
                } else {
                    future::ok(()).into_box()
                }
            )
            .into_box()
    }
}

impl Actor for Agent {
    type Context = Context<Self>;
}

impl Handler<HandleA2AMsg> for Agent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}

#[cfg(test)]
mod tests {
    use actors::ForwardA2AMsg;
    use super::*;
    use utils::tests::*;

    #[test]
    fn agent_create_key_works() {
        run_test(|forward_agent| {
            future::ok(())
                .and_then(|()| {
                    setup_agent(forward_agent)
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, _, _, forward_agent)| {
                    let (user_pw_did, user_pw_verkey) = did::create_and_store_my_did(e_wallet_handle, "{}").wait().unwrap();

                    let create_key_msg = compose_create_key(e_wallet_handle, &agent_did, &agent_verkey, &user_pw_did, &user_pw_verkey).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(create_key_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |key_created_msg| (e_wallet_handle, key_created_msg, agent_verkey))
                })
                .map(|(e_wallet_handle, key_created_msg, agent_verkey)| {
                    let (sender_vk, key) = decompose_key_created(e_wallet_handle, &key_created_msg).wait().unwrap();
                    assert_eq!(sender_vk, agent_verkey);
                    assert!(!key.with_pairwise_did.is_empty());
                    assert!(!key.with_pairwise_did_verkey.is_empty());

                    wallet::close_wallet(e_wallet_handle).wait().unwrap();
                })
        });
    }
}
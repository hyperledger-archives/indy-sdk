use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg, RouteA2AMsg, RouteA2ConnMsg};
use actors::agent_connection::{AgentConnection, AgentConnectionConfig};
use actors::router::Router;
use domain::a2a::*;
use domain::a2connection::*;
use domain::config::WalletStorageConfig;
use domain::invite::ForwardAgentDetail;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, pairwise, wallet, pairwise::Pairwise, ErrorCode, IndyError};
use std::convert::Into;
use std::collections::HashMap;
use utils::futures::*;
use utils::rand;
use serde_json;

#[allow(unused)] //FIXME:
pub struct Agent {
    wallet_handle: i32,
    owner_did: String,
    owner_verkey: String,
    did: String,
    verkey: String,
    forward_agent_detail: ForwardAgentDetail,
    router: Addr<Router>,
    configs: HashMap<String, String>
}

impl Agent {
    pub fn create(owner_did: &str,
                  owner_verkey: &str,
                  router: Addr<Router>,
                  forward_agent_detail: ForwardAgentDetail,
                  wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(String, String, String, String), Error> {
        trace!("Agent::create >> {:?}, {:?}, {:?}, {:?}",
               owner_did, owner_verkey, forward_agent_detail, wallet_storage_config);

        let wallet_id = format!("dummy_{}_{}", owner_did, rand::rand_string(10));
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
                    .map_err(|err| err.context("Can't open Agent wallet.`").into())
            })
            .and_then(|wallet_handle| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(move |(did, verkey)| (wallet_handle, did, verkey))
                    .map_err(|err| err.context("Can't get Agent did key").into())
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
                    configs: HashMap::new()
                };

                let agent = agent.start();

                router
                    .send(AddA2ARoute(did.clone(), agent.clone().recipient()))
                    .from_err()
                    .map(move |_| (wallet_id, wallet_key, did, verkey))
                    .map_err(|err: Error| err.context("Can't add route for Agent").into())
            })
            .into_box()
    }

    pub fn restore(wallet_id: &str,
                   wallet_key: &str,
                   did: &str,
                   owner_did: &str,
                   owner_verkey: &str,
                   router: Addr<Router>,
                   forward_agent_detail: ForwardAgentDetail,
                   wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(), Error> {
        trace!("Agent::restore >> {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
               wallet_id, did, owner_did, owner_verkey, forward_agent_detail, wallet_storage_config);

        let wallet_config = json!({
                    "id": wallet_id.clone(),
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        let wallet_credentials = json!({
                    "key": wallet_key.clone(),
                    "storage_credentials": wallet_storage_config.credentials,
                }).to_string();

        let did = did.to_string();
        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();

        future::ok(())
            .and_then(move |_| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Agent wallet.").into())
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle, &did)
                    .map(move |verkey| (wallet_handle, did, verkey))
                    .map_err(|err| err.context("Can't get Agent did verkey.").into())
            })
            .and_then(move |(wallet_handle, did, verkey)| {
                did::get_did_metadata(wallet_handle, &did)
                    .then(|res| match res {
                        Err(IndyError { error_code: ErrorCode::WalletItemNotFound, .. }) => Ok("{}".to_string()),
                        r => r
                    })
                    .map(move |metadata| (wallet_handle, did, verkey, metadata))
                    .map_err(|err| err.context("Can't get Agent DID Metadata.").into())
            })
            .and_then(move |(wallet_handle, did, verkey, metadata)| {
                // Resolve information about existing connections from the wallet
                // and start Agent Connection actor for each exists connection

                let configs: HashMap<String, String> = serde_json::from_str(&metadata).expect("Can't restore Agent config.");

                Agent::_restore_connections(wallet_handle,
                                            &owner_did,
                                            &owner_verkey,
                                            &forward_agent_detail,
                                            router.clone(),
                                            configs.clone())
                    .map(move |_| (wallet_handle, did, verkey, owner_did, owner_verkey, forward_agent_detail, router, configs))
            })
            .and_then(move |(wallet_handle, did, verkey, owner_did, owner_verkey, forward_agent_detail, router, configs)| {
                let agent = Agent {
                    wallet_handle,
                    verkey: verkey.clone(),
                    did: did.clone(),
                    owner_did,
                    owner_verkey,
                    router: router.clone(),
                    forward_agent_detail,
                    configs
                };

                let agent = agent.start();

                router
                    .send(AddA2ARoute(did.clone(), agent.clone().recipient()))
                    .from_err()
                    .map_err(|err: Error| err.context("Can't add route for Agent.").into())
            })
            .into_box()
    }

    fn _restore_connections(wallet_handle: i32,
                            owner_did: &str,
                            owner_verkey: &str,
                            forward_agent_detail: &ForwardAgentDetail,
                            router: Addr<Router>,
                            agent_configs: HashMap<String, String>) -> ResponseFuture<(), Error> {
        trace!("Agent::_restore_connections >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, owner_verkey, forward_agent_detail);

        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();
        let forward_agent_detail = forward_agent_detail.clone();

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .map(move |pairwise| {
                        AgentConnection::restore(wallet_handle,
                                                 &owner_did,
                                                 &owner_verkey,
                                                 &pairwise.my_did,
                                                 &pairwise.their_did,
                                                 &pairwise.metadata,
                                                 &forward_agent_detail,
                                                 router.clone(),
                                                 agent_configs.clone())
                    })
                    .collect();

                future::join_all(futures)
                    .map(|_| ())
                    .map_err(|err| err.context("Can't restore Agent connections.").into())
            })
            .into_box()
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Version1(A2AMessageV1::Forward(msg))) => {
                        slf.router
                            .send(RouteA2AMsg(msg.fwd, msg.msg))
                            .from_err()
                            .and_then(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::Forward(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        slf.router
                            .send(RouteA2AMsg(msg.fwd, msg_))
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
        trace!("Agent::handle_agent_msg >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessage::Version1(msg) => self.handle_agent_msg_v1(sender_vk, msg),
            A2AMessage::Version2(msg) => self.handle_agent_msg_v2(sender_vk, msg)
        }
    }

    fn handle_agent_msg_v1(&mut self,
                           sender_vk: String,
                           msg: A2AMessageV1) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_agent_msg_v1 >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessageV1::CreateKey(msg) => self.handle_create_key_v1(msg),
            A2AMessageV1::GetMessagesByConnections(msg) => self.handle_get_messages_by_connections_v1(msg),
            A2AMessageV1::UpdateMessageStatusByConnections(msg) => self.handle_update_messages_by_connections_v1(msg),
            A2AMessageV1::UpdateConfigs(msg) => self.handle_update_configs_v1(msg),
            A2AMessageV1::GetConfigs(msg) => self.handle_get_configs_v1(msg),
            A2AMessageV1::RemoveConfigs(msg) => self.handle_remove_configs_v1(msg),
            A2AMessageV1::UpdateComMethod(msg) => self.handle_update_com_method_v1(msg),
            _ => err_act!(self, err_msg("Unsupported message"))
        }
            .and_then(move |msgs, slf, _|
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &sender_vk, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
                    .into_box()
            )
            .into_box()
    }

    fn handle_agent_msg_v2(&mut self,
                           sender_vk: String,
                           msg: A2AMessageV2) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_agent_msg_v2 >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessageV2::CreateKey(msg) => self.handle_create_key_v2(msg),
            A2AMessageV2::GetMessagesByConnections(msg) => self.handle_get_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateMessageStatusByConnections(msg) => self.handle_update_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateConfigs(msg) => self.handle_update_configs_v2(msg),
            A2AMessageV2::GetConfigs(msg) => self.handle_get_configs_v2(msg),
            A2AMessageV2::RemoveConfigs(msg) => self.handle_remove_configs_v2(msg),
            _ => err_act!(self, err_msg("Unsupported message"))
        }
            .and_then(move |msg,slf,  _|
                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.verkey), &sender_vk, &msg)
                    .map_err(|err| err.context("Can't pack message.").into())
                    .into_actor(slf)
                    .into_box()
            )
            .into_box()
    }

    fn handle_get_messages_by_connections_v1(&mut self,
                                             msg: GetMessagesByConnections) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_get_messages_by_connections_v1 >> {:?}", msg);

        self.handle_get_messages_by_connections(msg)
            .map(|msgs: Vec<A2ConnMessage>| {
                vec![A2AMessage::Version1(A2AMessageV1::MessagesByConnections(
                    MessagesByConnections {
                        msgs: msgs.into_iter().map(|msg| msg.into()).collect()
                    }))]
            })
            .into_actor(self)
            .into_box()
    }


    fn handle_get_messages_by_connections_v2(&mut self,
                                             msg: GetMessagesByConnections) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_get_messages_by_connections_v2 >> {:?}", msg);

        self.handle_get_messages_by_connections(msg)
            .map(|msgs: Vec<A2ConnMessage>| {
                A2AMessageV2::MessagesByConnections(
                    MessagesByConnections {
                        msgs: msgs.into_iter().map(|msg| msg.into()).collect()
                    })
            })
            .into_actor(self)
            .into_box()
    }

    fn handle_get_messages_by_connections(&mut self,
                                          msg: GetMessagesByConnections) -> ResponseFuture<Vec<A2ConnMessage>, Error> {
        trace!("Agent::handle_get_messages_by_connections >> {:?}", msg);

        let GetMessagesByConnections { exclude_payload, uids, status_codes, pairwise_dids } = msg;

        let router = self.router.clone();
        let wallet_handle = self.wallet_handle.clone();

        let msg = GetMessages { exclude_payload, uids, status_codes };

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let pairwises: Vec<_> = pairwise_list
                    .into_iter()
                    .filter(|pairwise| pairwise_dids.is_empty() || pairwise_dids.contains(&pairwise.their_did))
                    .collect();

                if !pairwise_dids.is_empty() && pairwises.is_empty() {
                    return err!(err_msg("Pairwise DID not found.")).into_box();
                }

                let futures: Vec<_> = pairwises
                    .iter()
                    .map(move |pairwise| {
                        router
                            .send(RouteA2ConnMsg(pairwise.my_did.clone(), A2ConnMessage::GetMessages(msg.clone())))
                            .from_err()
                            .and_then(|res| res)
                            .into_box()
                    })
                    .collect();

                future::join_all(futures)
                    .map_err(|err| err.context("Can't get Agent Connection messages").into())
                    .into_box()
            })
            .into_box()
    }

    fn handle_update_messages_by_connections_v1(&mut self,
                                                msg: UpdateMessageStatusByConnections) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_update_messages_by_connections_v1 >> {:?}", msg);

        self.handle_update_messages_by_connections(msg)
            .map(|uids_by_conn: Vec<A2ConnMessage>| {
                vec![A2AMessage::Version1(A2AMessageV1::MessageStatusUpdatedByConnections(
                    MessageStatusUpdatedByConnections {
                        updated_uids_by_conn: uids_by_conn.into_iter().map(|msg| msg.into()).collect(),
                        failed: Vec::new(),
                    }))]
            })
            .into_actor(self)
            .into_box()
    }

    fn handle_update_messages_by_connections_v2(&mut self,
                                                msg: UpdateMessageStatusByConnections) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_update_messages_by_connections_v2 >> {:?}", msg);

        self.handle_update_messages_by_connections(msg)
            .map(|uids_by_conn: Vec<A2ConnMessage>| {
                A2AMessageV2::MessageStatusUpdatedByConnections(
                    MessageStatusUpdatedByConnections {
                        updated_uids_by_conn: uids_by_conn.into_iter().map(|msg| msg.into()).collect(),
                        failed: Vec::new(),
                    })
            })
            .into_actor(self)
            .into_box()
    }

    fn handle_update_messages_by_connections(&mut self,
                                             msg: UpdateMessageStatusByConnections) -> ResponseFuture<Vec<A2ConnMessage>, Error> {
        trace!("Agent::handle_update_messages_by_connections >> {:?}", msg);

        let UpdateMessageStatusByConnections { uids_by_conn, status_code } = msg;

        let router = self.router.clone();
        let wallet_handle = self.wallet_handle.clone();

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .filter_map(|pairwise| uids_by_conn
                        .iter()
                        .find(|uid_by_conn| uid_by_conn.pairwise_did == pairwise.their_did)
                        .map(|uid_by_conn| (uid_by_conn, pairwise))
                    )
                    .map(move |(uid_by_conn, pairwise)|
                        router
                            .send(RouteA2ConnMsg(pairwise.my_did.clone(), A2ConnMessage::UpdateMessages(
                                UpdateMessageStatus { uids: uid_by_conn.uids.clone(), status_code: status_code.clone() }
                            )))
                            .from_err()
                            .and_then(|res| res)
                            .into_box()
                    )
                    .collect();

                future::join_all(futures)
                    .map_err(|err| err.context("Can't get Agent Connection messages").into())
            })
            .into_box()
    }

    fn get_pairwise_list(wallet_handle: i32) -> ResponseFuture<Vec<Pairwise>, Error> {
        future::ok(())
            .and_then(move |_| {
                pairwise::list_pairwise(wallet_handle)
                    .map_err(|err| err.context("Can't get Agent pairwise list").into())
                    .into_box()
            })
            .and_then(move |pairwise_list| {
                let pairwise_list = ftry!(serde_json::from_str::<Vec<String>>(&pairwise_list));

                pairwise_list
                    .iter()
                    .map(|pairwise| serde_json::from_str::<Pairwise>(&pairwise))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| err.context("Can't deserialize Agent pairwise").into())
                    .into_future()
                    .into_box()
            })
            .into_box()
    }

    fn handle_create_key_v1(&mut self,
                            msg: CreateKey) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_create_key_v1 >> {:?}", msg);

        let CreateKey { for_did, for_did_verkey } = msg;

        self.handle_create_key(&for_did, &for_did_verkey)
            .map(|(pairwise_did, pairwise_did_verkey), _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                }))]
            })
            .into_box()
    }

    fn handle_create_key_v2(&mut self,
                            msg: CreateKey) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_create_key_v2 >> {:?}", msg);

        let CreateKey { for_did, for_did_verkey, .. } = msg;

        self.handle_create_key(&for_did, &for_did_verkey)
            .map(|(pairwise_did, pairwise_did_verkey), _, _| {
                A2AMessageV2::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                })
            })
            .into_box()
    }

    fn handle_create_key(&mut self,
                         for_did: &str,
                         for_did_verkey: &str) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("Agent::handle_create_key >> {:?}, {:?}", for_did, for_did_verkey);

        let for_did = for_did.to_string();
        let for_did_verkey = for_did_verkey.to_string();

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
                pairwise::create_pairwise(slf.wallet_handle, &for_did, &pairwise_did, None)
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
                    agent_configs: slf.configs.clone(),
                    forward_agent_detail: slf.forward_agent_detail.clone(),
                };

                AgentConnection::create(config, slf.router.clone())
                    .map(|_| (pairwise_did, pairwise_did_verkey))
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_update_com_method_v1(&mut self, _msg: UpdateComMethod) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("UpdateComMethod: {:?}", _msg);
        let messages = vec![A2AMessage::Version1(A2AMessageV1::ComMethodUpdated(ComMethodUpdated {id: "123".to_string()}))];
        ok_act!(self,  messages)
    }

    fn handle_update_configs_v1(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsUpdated(ConfigsUpdated {}))]
            })
            .into_box()
    }

    fn handle_update_configs_v2(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsUpdated(ConfigsUpdated {})
            })
            .into_box()
    }

    fn handle_update_configs(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, (), Error> {
        for config_option in msg.configs {
            match config_option.name.as_str() {
                "name" | "logo_url" => self.configs.insert(config_option.name, config_option.value),
                _ => continue
            };
        }

        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_get_configs_v1(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = vec![A2AMessage::Version1(A2AMessageV1::Configs(Configs { configs }))];
        ok_act!(self,  messages)
    }

    fn handle_get_configs_v2(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = A2AMessageV2::Configs(Configs { configs });
        ok_act!(self,  messages)
    }

    fn handle_get_configs(&mut self, msg: GetConfigs) -> Vec<ConfigOption> {
        self.configs.iter()
            .filter(|(k, _)| msg.configs.contains(k))
            .map(|(k, v)| ConfigOption { name: k.clone(), value: v.clone() })
            .collect()
    }

    fn handle_remove_configs_v1(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsRemoved(ConfigsRemoved {}))]
            })
            .into_box()
    }

    fn handle_remove_configs_v2(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsRemoved(ConfigsRemoved {})
            })
            .into_box()
    }

    fn handle_remove_configs(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, (), Error> {
        self.configs.retain(|k, _| !msg.configs.contains(k));
        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
                    .into_actor(slf)
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
    use utils::to_i8;
    use utils::tests::*;
    use domain::status::MessageStatusCode;

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

    #[test]
    fn agent_get_messages_by_connection_works() {
        run_agent_test(|(e_wallet_handle, agent_did, agent_verkey, agent_pw_did, agent_pw_vk, forward_agent)| {
            future::ok(())
                .and_then(move |_| {
                    let msg = compose_create_general_message(e_wallet_handle,
                                                             &agent_did,
                                                             &agent_verkey,
                                                             &agent_pw_did,
                                                             &agent_pw_vk,
                                                             RemoteMessageType::CredOffer).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk))
                })
                .and_then(move |(e_wallet_handle, resp, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk)| {
                    let (_, msg_uid) = decompose_general_message_created(e_wallet_handle, &resp).wait().unwrap();

                    let msg = compose_get_messages_by_connection(e_wallet_handle,
                                                                 &agent_did,
                                                                 &agent_verkey,
                                                                 &agent_pw_did,
                                                                 &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_verkey, msg_uid))
                })
                .map(|(e_wallet_handle, resp, agent_verkey, msg_uid)| {
                    let (sender_vk, messages) = decompose_get_messages_by_connection(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_verkey);
                    assert_eq!(1, messages.len());

                    let expected_message = MessagesByConnection {
                        did: EDGE_PAIRWISE_DID.to_string(),
                        msgs: vec![GetMessagesDetailResponse {
                            uid: msg_uid,
                            status_code: MessageStatusCode::Created,
                            sender_did: EDGE_PAIRWISE_DID.to_string(),
                            type_: RemoteMessageType::CredOffer,
                            payload: Some(MessageDetailPayload::V1(to_i8(&PAYLOAD.to_vec()))),
                            ref_msg_id: None,
                        }]
                    };
                    assert_eq!(expected_message, messages[0]);
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_configs_happy_path() {
        run_test(|forward_agent| {
            future::ok(())
                .and_then(|()| {
                    setup_agent(forward_agent)
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, _, _, forward_agent)| {
                    let msg = compose_update_configs(e_wallet_handle,
                                                     &agent_did,
                                                     &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |_| (e_wallet_handle, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, forward_agent)| {
                    let msg = compose_get_configs(e_wallet_handle,
                                                  &agent_did,
                                                  &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, resp, agent_did, agent_verkey, forward_agent)| {
                    let configs = decompose_configs(e_wallet_handle, &resp).wait().unwrap();

                    assert_eq!(configs.len(), 2);
                    assert!(configs.contains(&ConfigOption { name: "name".to_string(), value: "super agent".to_string() }));
                    assert!(configs.contains(&ConfigOption { name: "logo_url".to_string(), value: "http://logo.url".to_string() }));

                    let msg = compose_remove_configs(e_wallet_handle,
                                                     &agent_did,
                                                     &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |_| (e_wallet_handle, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, forward_agent)| {
                    let msg = compose_get_configs(e_wallet_handle,
                                                  &agent_did,
                                                  &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp))
                })
                .map(|(e_wallet_handle, resp)| {
                    let configs = decompose_configs(e_wallet_handle, &resp).wait().unwrap();

                    assert_eq!(configs.len(), 1);
                    assert!(!configs.contains(&ConfigOption { name: "name".to_string(), value: "super agent".to_string() }));
                    assert!(configs.contains(&ConfigOption { name: "logo_url".to_string(), value: "http://logo.url".to_string() }));

                    wallet::close_wallet(e_wallet_handle).wait().unwrap();
                })
        });
    }
}
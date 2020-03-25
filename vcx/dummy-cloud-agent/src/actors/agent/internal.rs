use std::convert::Into;

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use serde_json;

use crate::actors::agent::agent::Agent;
use crate::actors::agent_connection::agent_connection::AgentConnection;
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::indy::{did, pairwise, pairwise::Pairwise, WalletHandle};
use crate::utils::futures::*;

impl Agent {
    /// Handles message which has been addressed for this agent. There's generally 2 classes of
    /// messages this agent can handle:
    /// 1. Forward messages - is data structure with encrypted data and recipient address of some sort.
    /// When agent is receives such message, all it can do is pass it to Router actor to take care
    /// delivery of such message.
    /// See details about forwarding in Aries at
    /// https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0094-cross-domain-messaging/README.md
    ///
    /// 2. VCX Agent messages - messages which agent's owner sends to the agent to make it do something.
    /// For example message types such us GetMessagesByConnections, UpdateMessageStatusByConnections
    /// can download messages which the agent has received on its owner behalf.
    /// Another example are message types such as  UpdateConfigs, GetConfigs, RemoveConfigs
    /// for CRUD operations for agent's configuration.
    pub(super) fn handle_a2a_msg(&mut self,
                                 msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.agent_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Version1(A2AMessageV1::Forward(msg))) => {
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.fwd, msg.msg)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::Forward(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.fwd, msg_)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::ForwardV3(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.to, msg_)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(msg) => slf.handle_agent_msg(sender_vk, msg),
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    /// Handles VCX agent messages the agent's owner might send to control this agent.
    fn handle_agent_msg(&mut self,
                        sender_vk: String,
                        msg: A2AMessage) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_agent_msg >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessage::Version1(msg) => self.handle_agent_msg_v1(sender_vk, msg),
            A2AMessage::Version2(msg) => self.handle_agent_msg_v2(sender_vk, msg)
        }
    }

    /// Retrieves messages according to requested filters. Messages are however not managed by
    /// the agent actor directly - agent actor manages "agent connection" actors. Each
    /// "agent connection" actor represents one pairwise relationship of this agent's owner.
    /// Hence the agent requests every "agent connection" actor to return messages according to
    /// provided filters in "msg" argument.
    ///
    /// # Arguments
    /// * `msg` - represents filters to determine which messages shall be retrieved.
    ///
    pub(super) fn handle_get_messages_by_connections(&mut self,
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
                        router.read().unwrap()
                            .route_a2conn_msg(pairwise.my_did.clone(), A2ConnMessage::GetMessages(msg.clone()))
                            .from_err()
                            .into_box()
                    })
                    .collect();

                future::join_all(futures)
                    // .map_err(|err| err.context("Can't get Agent Connection messages").into())
                    .into_box()
            })
            .into_box()
    }

    /// Updates state of messages. The set of messages which shall have update their status to
    /// a new value are determined by filter provided in "msg" argument.
    ///
    /// # Arguments
    /// * `msg` - represents filters to determine statuses of which messages shall be updated
    ///
    pub(super) fn handle_update_messages_by_connections(&mut self,
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

                        router.read().unwrap()
                            .route_a2conn_msg(pairwise.my_did.clone(),
                                              A2ConnMessage::UpdateMessages(
                                                  UpdateMessageStatus { uids: uid_by_conn.uids.clone(), status_code: status_code.clone() }
                                              ))
                            .from_err()
                            .into_box()
                    )
                    .collect();

                future::join_all(futures)
                // .map_err(|err| err.context("Can't get Agent Connection messages").into())
            })
            .into_box()
    }

    /// Returns list of all pairwise relationships managed by this agent
    pub(super) fn get_pairwise_list(wallet_handle: WalletHandle) -> ResponseFuture<Vec<Pairwise>, Error> {
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


    /// Creates new "Agent Connection" actor managed by this agent. The "Agent Connection"
    /// represents a pairwise relationship between this agent's owner and another party.
    ///
    /// Returns information about created "Agent Connection" as
    /// tuple `(PairwiseAgent.DID@PairwiseAgent:Client, PairwiseAgent.VKey@PairwiseAgent:Client)`
    /// The DID represents an address where the counterparty of client's pairwise relationship
    /// is expected forward messages to. It's also expected that messages arriving to this
    /// Pairwise Agent will be authcrypted using its verkey.

    /// # Arguments
    /// * `for_did` - The DID which Agent's owner generated to identify pairwise relationship with the 3rd party
    /// * `for_did_verkey` - The verkey which Agent's owner generated for E2E encryption within pairwise relationship with the 3rd party
    pub(super) fn handle_create_key(&mut self,
                                    for_did: &str,
                                    for_did_verkey: &str) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("Agent::handle_create_key >> {:?}, {:?}", for_did, for_did_verkey);

        let user_pairwise_did = for_did.to_string();
        let user_pairwise_verkey = for_did_verkey.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                AgentConnection::create_record_load_actor(
                    slf.wallet_handle,
                    slf.owner_did.to_string(),
                    slf.owner_verkey.to_string(),
                    user_pairwise_did,
                    user_pairwise_verkey,
                    slf.configs.clone(),
                    slf.forward_agent_detail.clone(),
                    slf.router.clone(),
                    slf.admin.clone(),
                )
                    .map(|(agent_connection_did, agent_connection_verkey)| (agent_connection_did, agent_connection_verkey))
                    .into_actor(slf)
            })
            .into_box()
    }

    pub(super) fn handle_update_configs(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, (), Error> {
        for config_option in msg.configs {
            match config_option.name.as_str() {
                "name" | "logoUrl" | "notificationWebhookUrl" => self.configs.insert(config_option.name, config_option.value),
                _ => {
                    warn!("Agent was trying to set up unsupported agent configuration option {}", config_option.name.as_str());
                    continue;
                }
            };
        }

        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.agent_did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    pub(super) fn handle_get_configs(&mut self, msg: GetConfigs) -> Vec<ConfigOption> {
        self.configs.iter()
            .filter(|(k, _)| msg.configs.contains(k))
            .map(|(k, v)| ConfigOption { name: k.clone(), value: v.clone() })
            .collect()
    }

    pub(super) fn handle_remove_configs(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, (), Error> {
        self.configs.retain(|k, _| !msg.configs.contains(k));
        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.agent_did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}
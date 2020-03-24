use std::convert::Into;

use actix::prelude::*;
use failure::{err_msg, Error, Fail};

use crate::actors::agent::agent::Agent;
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::utils::futures::*;

impl Agent {
    pub(super) fn handle_agent_msg_v2(&mut self,
                                      sender_vk: String,
                                      msg: A2AMessageV2) -> ResponseActFuture<Self, Vec<u8>, Error> {
        debug!("Agent::handle_agent_msg_v2 >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessageV2::CreateKey(msg) => self.handle_create_key_v2(msg),
            A2AMessageV2::GetMessagesByConnections(msg) => self.handle_get_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateMessageStatusByConnections(msg) => self.handle_update_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateConfigs(msg) => self.handle_update_configs_v2(msg),
            A2AMessageV2::GetConfigs(msg) => self.handle_get_configs_v2(msg),
            A2AMessageV2::RemoveConfigs(msg) => self.handle_remove_configs_v2(msg),
            _ => err_act!(self, err_msg("Unsupported message"))
        }
            .and_then(move |msg, slf, _|
                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.agent_verkey), &sender_vk, &msg)
                    .map_err(|err| err.context("Can't pack message.").into())
                    .into_actor(slf)
                    .into_box()
            )
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

    fn handle_update_configs_v2(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsUpdated(ConfigsUpdated {})
            })
            .into_box()
    }

    fn handle_get_configs_v2(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = A2AMessageV2::Configs(Configs { configs });
        ok_act!(self,  messages)
    }

    fn handle_remove_configs_v2(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsRemoved(ConfigsRemoved {})
            })
            .into_box()
    }
}
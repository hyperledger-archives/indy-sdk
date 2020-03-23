use std::collections::HashMap;
use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::Either;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage, RouteA2ConnMsg};
use crate::actors::admin::Admin;
use crate::actors::agent::agent::Agent;
use crate::actors::router::Router;
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::domain::admin_message::{ResAdminQuery, ResQueryAgent};
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::KeyDerivationDirective;
use crate::indy::{did, ErrorCode, IndyError, pairwise, pairwise::Pairwise, wallet, WalletHandle};
use crate::utils::config_env::*;
use crate::utils::futures::*;
use crate::utils::rand;
use crate::utils::wallet::build_wallet_credentials;

impl Agent {
    pub(super) fn handle_agent_msg_v1(&mut self,
                                      sender_vk: String,
                                      msg: A2AMessageV1) -> ResponseActFuture<Self, Vec<u8>, Error> {
        debug!("Agent::handle_agent_msg_v1 >> {:?}, {:?}", sender_vk, msg);

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
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.agent_verkey, &sender_vk, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
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

    fn handle_update_com_method_v1(&mut self, _msg: UpdateComMethod) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("UpdateComMethod: {:?}", _msg);
        let messages = vec![A2AMessage::Version1(A2AMessageV1::ComMethodUpdated(ComMethodUpdated { id: "123".to_string() }))];
        ok_act!(self,  messages)
    }

    fn handle_update_configs_v1(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsUpdated(ConfigsUpdated {}))]
            })
            .into_box()
    }

    fn handle_get_configs_v1(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = vec![A2AMessage::Version1(A2AMessageV1::Configs(Configs { configs }))];
        ok_act!(self,  messages)
    }

    fn handle_remove_configs_v1(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsRemoved(ConfigsRemoved {}))]
            })
            .into_box()
    }
}
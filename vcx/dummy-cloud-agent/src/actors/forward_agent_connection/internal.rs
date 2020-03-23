use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::Either;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::agent::agent::Agent;
use crate::actors::forward_agent_connection::forward_agent_connection::{AgentWalletInfo, ForwardAgentConnection, ForwardAgentConnectionState};
use crate::actors::router::Router;
use crate::domain::a2a::*;
use crate::domain::admin_message::ResAdminQuery;
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::{KeyDerivationDirective, KeyDerivationFunction};
use crate::indy::{did, pairwise, pairwise::PairwiseInfo, WalletHandle};
use crate::utils::futures::*;

/// Converts the legacy agent state tuple (wallet_id, wallet_key, agent_did) into new data structure
/// AgentWalletInfo for keeping record about a previously created agent
fn convert_from_legacy_agent_to_agent_wallet(agent: (String, String, String)) -> AgentWalletInfo {
    AgentWalletInfo {
        wallet_id: agent.0,
        agent_did: agent.2,
        kdf_directive: KeyDerivationDirective {
            kdf: KeyDerivationFunction::Argon2iMod, // old agents were using Argon2iMod by default
            key: agent.1,
        },
    }
}

impl ForwardAgentConnection {
    /// Handles encrypted message which has been addressed for this actor, presumably coming from
    /// the peer of this pairwise relationship represented by this forward agency connection.
    ///
    /// # Arguments
    ///
    /// * `msg` - Authccrypted data addressed for this actor, forwarded by Router actor
    ///
    pub(super) fn _handle_a2a_msg(&mut self,
                                  msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle a2a message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                if slf.owner_verkey != sender_vk {
                    return err_act!(slf, err_msg("Inconsistent sender and connection pairwise verkeys"));
                };

                match msgs.pop() {
                    Some(A2AMessage::Version1(msg)) => slf._handle_a2a_msg_v1(msg),
                    Some(A2AMessage::Version2(msg)) => slf._handle_a2a_msg_v2(msg),
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    /// Agency client needs send "SignUp" to sing himself up before he can create his Agent
    pub(super) fn _sign_up(&mut self) -> ResponseActFuture<Self, (), Error> {
        trace!("ForwardAgentConnection::_sign_up >>");

        if self.state.is_signed_up {
            return err_act!(self, err_msg("Already signed up"));
        };

        self.state.is_signed_up = true;

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.owner_did, &metadata)
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_actor(slf)
                    .into_box()
            })
            .into_box()
    }

    /// Creates agent for peer of pairwise relationship represented by this Forward Agent Connection
    pub(super) fn _create_agent(&mut self) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("ForwardAgentConnection::_create_agent >> ");

        if !self.state.is_signed_up {
            return err_act!(self, err_msg("Sign up is required."));
        };

        if self.state.agent.is_some() || self.state.agent_v2.is_some() {
            return err_act!(self, err_msg("Agent already created."));
        };

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                Agent::create(&slf.owner_did,
                              &slf.owner_verkey,
                              slf.router.clone(),
                              slf.forward_agent_detail.clone(),
                              slf.wallet_storage_config.clone(),
                              slf.admin.clone(),
                )
                    .into_actor(slf)
                    .into_box()
            })
            .and_then(|(wallet_id, agent_did, agent_verkey, kdf_directive), slf, _| {
                slf.state.agent_v2 = Some(AgentWalletInfo {
                    wallet_id,
                    agent_did: agent_did.clone(),
                    kdf_directive,
                });

                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize agent reference."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.owner_did, &metadata)
                    .map(move |_| (agent_did, agent_verkey))
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_actor(slf)
                    .into_box()
            })
            .into_box()
    }
}

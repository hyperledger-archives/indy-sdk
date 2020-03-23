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

impl ForwardAgentConnection {
    /// Handles messages types used for creating agent in Agency.
    /// See method onboarding_v1 in VCX library.
    pub(super) fn _handle_a2a_msg_v1(&mut self,
                                     msg: A2AMessageV1) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg_v1 >> {:?}", msg);

        match msg {
            A2AMessageV1::SignUp(msg) => {
                self._sign_up_v1(msg)
            }
            A2AMessageV1::CreateAgent(msg) => {
                self._create_agent_v1(msg)
            }
            _ => err_act!(self, err_msg("Unsupported message"))
        }
    }

    fn _sign_up_v1(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_sign_up_v1 >> {:?}", msg);

        self._sign_up()
            .and_then(|_, slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::SignedUp(SignedUp {}))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _create_agent_v1(&mut self, msg: CreateAgent) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_create_agent_v1 >> {:?}", msg);

        self._create_agent()
            .and_then(|(did, verkey), slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::AgentCreated(AgentCreated {
                    with_pairwise_did: did,
                    with_pairwise_did_verkey: verkey,
                }))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

use std::convert::Into;

use actix::prelude::*;
use failure::{err_msg, Error, Fail};

use crate::actors::forward_agent_connection::forward_agent_connection::{ForwardAgentConnection};
use crate::domain::a2a::*;
use crate::utils::futures::*;

impl ForwardAgentConnection {
    /// Handles messages types used for creating agent in Agency.
    /// See method onboarding_v2 in VCX library.
    pub(super) fn _handle_a2a_msg_v2(&mut self,
                                     msg: A2AMessageV2) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg_v2 >> {:?}", msg);

        match msg {
            A2AMessageV2::SignUp(msg) => {
                self._sign_up_v2(msg)
            }
            A2AMessageV2::CreateAgent(msg) => {
                self._create_agent_v2(msg)
            }
            _ => err_act!(self, err_msg("Unsupported message"))
        }
    }

    fn _sign_up_v2(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_sign_up_v2 >> {:?}", msg);

        self._sign_up()
            .and_then(|_, slf, _| {
                let msg = A2AMessageV2::SignedUp(SignedUp {});

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.fwac_verkey), &slf.owner_verkey, &msg)
                    .map_err(|err| err.context("Can't pack signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _create_agent_v2(&mut self, msg: CreateAgent) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_create_agent_v2 >> {:?}", msg);

        self._create_agent()
            .and_then(|(agent_did, agent_verkey), slf, _| {
                let msg = A2AMessageV2::AgentCreated(AgentCreated {
                    with_pairwise_did: agent_did,
                    with_pairwise_did_verkey: agent_verkey,
                });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.fwac_verkey), &slf.owner_verkey, &msg)
                    .map_err(|err| err.context("Can't pack agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

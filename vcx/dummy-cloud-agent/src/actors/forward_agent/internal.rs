use std::convert::Into;

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use serde_json;

use crate::actors::forward_agent::forward_agent::ForwardAgent;
use crate::actors::forward_agent_connection::forward_agent_connection::ForwardAgentConnection;
use crate::domain::a2a::*;
use crate::indy::{pairwise, WalletHandle};
use crate::utils::futures::*;

impl ForwardAgent {
    pub(super) fn _get_endpoint(&self) -> (String, String) {
        trace!("ForwardAgent::_get_endpoint >>");
        (self.did.clone(), self.verkey.clone())
    }

    /// Returns list of pairwise DIDs representing connections established with Agency
    pub(super) fn _get_forward_agent_details(&self) -> (String, Vec<String>, WalletHandle) {
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
    pub(super) fn _forward_a2a_msg(&mut self,
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
    pub(super) fn _handle_a2a_msg(&mut self,
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

    /// Creates new pairwise connection between previously unknown client and Agency.
    ///
    /// Returns
    ///
    /// # Arguments
    ///
    /// * `sender_vk` - Verkey of this Connect message sender. Must be same as their_did
    /// * `their_did` - Client DID at ClientToAgency relationship ( Owner.DID@Client:Agency )
    /// * `their_verkey` - Client VKey at ClientToAgency relationship ( Client.Verkey@Client:Agency )
    pub(super) fn _connect(&mut self,
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

use std::collections::HashMap;

use actix::prelude::*;
use base64;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::ok;
use rmp_serde;
use serde_json;
use uuid::Uuid;

use crate::actors::{RemoteMsg, requester};
use crate::actors::agent_connection::agent_connection::{AgentConnection, MessageHandlerRole, RemoteConnectionDetail};
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::domain::internal_message::InternalMessage;
use crate::domain::invite::{AgentDetail, InviteDetail, RedirectDetail, SenderDetail};
use crate::domain::key_deligation_proof::KeyDlgProof;
use crate::domain::payload::{PayloadKinds, PayloadTypes, PayloadV1, PayloadV2, Thread};
use crate::domain::protocol_type::{ProtocolType, ProtocolTypes};
use crate::domain::status::{ConnectionStatus, MessageStatusCode};
use crate::indy::{crypto, did, ErrorCode, IndyError, pairwise};
use crate::utils::futures::*;
use crate::utils::to_i8;

impl AgentConnection {
    /// The heart of Aries cross-domain communication. All incoming Aries messages are coming in
    /// through this method.
    pub(super) fn handle_forward_message(&mut self, msg: ForwardV3) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        let msg_ = ftry_act!(self, {serde_json::to_vec(&msg.msg)});

        self.create_and_store_internal_message(None,
                                               RemoteMessageType::Other(String::from("aries")),
                                               MessageStatusCode::Received,
                                               &String::new(),
                                               None,
                                               Some(msg_),
                                               None,
                                               None,
                                               None);

        ok_act!(self, vec![])
    }
}

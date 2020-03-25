use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use serde_json;

use crate::actors::agent_connection::agent_connection::AgentConnection;
use crate::domain::a2a::*;
use crate::domain::status::MessageStatusCode;
use crate::utils::futures::*;

/// Implementation of methods exclusively related to usage of VCX in Aries mode
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

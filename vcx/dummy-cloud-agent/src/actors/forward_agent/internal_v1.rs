use std::convert::Into;

use actix::prelude::*;
use failure::{Error, Fail};

use crate::actors::forward_agent::forward_agent::ForwardAgent;
use crate::domain::a2a::*;
use crate::utils::futures::*;

impl ForwardAgent {
    pub(super) fn _connect_v1(&mut self,
                              sender_vk: String,
                              msg: Connect) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_connect_v1 >> {:?}, {:?}", sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey } = msg;

        self._connect(sender_vk.clone(), their_did.clone(), their_verkey.clone())
            .and_then(move |(my_did, my_verkey), slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                }))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}


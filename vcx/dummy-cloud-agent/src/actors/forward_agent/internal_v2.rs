use std::convert::Into;

use actix::prelude::*;
use failure::{Error, Fail};

use crate::actors::forward_agent::forward_agent::ForwardAgent;
use crate::domain::a2a::*;
use crate::utils::futures::*;

impl ForwardAgent {
    pub(super) fn _connect_v2(&mut self,
                              sender_vk: String,
                              msg: Connect) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::_connect_v2 >> {:?}, {:?}", sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey, .. } = msg;

        self._connect(sender_vk.clone(), their_did.clone(), their_verkey.clone())
            .and_then(move |(my_did, my_verkey), slf, _| {
                let msg = A2AMessageV2::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.verkey), &their_verkey, &msg)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

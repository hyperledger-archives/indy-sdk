use std::collections::HashMap;

use actix::prelude::*;
use failure::{err_msg, Error};
use futures::*;
use futures::future::Either;
use futures::*;
use failure::{Fail};

use crate::actors::{AdminRegisterRouter, HandleA2AMsg, HandleA2ConnMsg, HandleAdminMessage, RemoteMsg, RouteA2ConnMsg};
use crate::actors::admin::Admin;
use crate::actors::requester::Requester;
use crate::domain::a2connection::A2ConnMessage;
use crate::domain::admin_message::ResAdminQuery;
use crate::utils::futures::*;
use crate::indy::{did, ErrorCode, IndyError, pairwise, pairwise::Pairwise, wallet, WalletHandle};
use crate::domain::config::WalletStorageConfig;
use std::rc::Rc;
use std::sync::RwLock;

/// Router stores DID and Verkeys and handle all Forward messages. More info on Aries FWD messages:
/// https://github.com/hyperledger/aries-rfcs/tree/master/concepts/0094-cross-domain-messaging
/// When agency (its Forward Agent) receives Forward message, it's passed to Router instance to
/// take care of it. Router is aware of DIDs and Verkeys for Forward Agent, every
/// Forward Agent Connection, every Agent and every Agent Connection, as each of those actors
/// has its own DID and Verkey which can be used to address them a message.
///
/// So when a Forward message arrives to Router, its destination. If the destination is associated
/// with an existing entity whin the agency (some Actix actor), it's forwarded to him to handle.
/// If the destination is unknown, an error is returned.
pub struct Router {
    routes: HashMap<String, Recipient<HandleA2AMsg>>,
    pairwise_routes: HashMap<String, Recipient<HandleA2ConnMsg>>,
    requester: Addr<Requester>,
    fw_agent_wallet_handle: WalletHandle
}

impl Router {
    pub fn new(fw_agent_wallet_handle: WalletHandle) -> ResponseFuture<Rc<RwLock<Router>>, Error> {
        trace!("Router::new >>");
        future::ok(())
            .and_then(move |_| {
                let requester = Requester::new().start();
                let router = Router {
                    routes: HashMap::new(),
                    pairwise_routes: HashMap::new(),
                    requester,
                    fw_agent_wallet_handle
                };
                future::ok( Rc::new(RwLock::new(router)))
            })
            .into_box()
    }


    // fn register_fwac(&mut self, owner_did: String, entity_did: String, entity_verkey: String) {
    //     trace!("Router::register_fwac >> {}, {}", did, verkey);
    //     future::ok(())
    //         .and_then(move |_| {
    //             pairwise::create_pairwise(self.wallet_handle, &user_pairwise_did, &agent_connection_did, Some("{}"))
    //                 .map_err(|err| err.context("Can't store agent pairwise connection.").into())
    //                 .map(|_| (user_pairwise_did, agent_connection_did, agent_connection_verkey))
    //                 .into_actor(slf)
    //         })
    //         .into_box()
    // }

    pub fn add_a2a_route(&mut self, did: String, verkey: String, handler: Recipient<HandleA2AMsg>) {
        trace!("Router::handle_add_route >> {}, {}", did, verkey);
        self.routes.insert(did, handler.clone());
        self.routes.insert(verkey, handler);
    }

    pub fn add_a2conn_route(&mut self, did: String, verkey: String, handler: Recipient<HandleA2ConnMsg>) {
        trace!("Router::add_a2conn_route >> {}, {}", did, verkey);
        self.pairwise_routes.insert(did, handler.clone());
        self.pairwise_routes.insert(verkey, handler);
    }

    // fn _try_restore_route(&mut self, did: &str) -> Recipient<HandleA2AMsg> {
    //
    // }

    pub fn route_a2a_msg(&self, did: String, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("Router::route_a2a_msg >> {:?}, {:?}", did, msg);

        if let Some(addr) = self.routes.get(&did) {
            addr
                .send(HandleA2AMsg(msg))
                .from_err()
                .and_then(|res| res)
                .into_box()
        } else {
            // _try_restore_route(&did)
            panic!("Cant resolve route") // todo: We should not panic but handle this

        }
    }

    pub fn route_a2conn_msg(&self, did: String, msg: A2ConnMessage) -> ResponseFuture<A2ConnMessage, Error> {
        trace!("Router::route_a2conn_msg >> {:?}, {:?}", did, msg);

        if let Some(addr) = self.pairwise_routes.get(&did) {
            addr
                .send(HandleA2ConnMsg(msg))
                .from_err()
                .and_then(|res| res)
                .into_box()
        } else {
            err!(err_msg("No route found."))
        }
    }

    pub fn route_to_requester(&self, msg: RemoteMsg) -> ResponseFuture<(), Error> {
        trace!("Router::route_to_requester >> {:?}", msg);

        self.requester
            .send(msg)
            .from_err()
            .and_then(|res| res)
            .into_box()
    }
}
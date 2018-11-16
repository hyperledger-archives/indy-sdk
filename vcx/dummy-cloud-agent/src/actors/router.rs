use actix::prelude::*;
use actors::{AddA2ARoute, AddA2ConnRoute, HandleA2AMsg, HandleA2ConnMsg, RouteA2AMsg, RouteA2ConnMsg, RemoteMsg};
use actors::requester::Requester;
use domain::a2connection::A2ConnMessage;
use failure::{Error, err_msg};
use futures::*;
use std::collections::HashMap;
use utils::futures::*;

pub struct Router {
    routes: HashMap<String, Recipient<HandleA2AMsg>>,
    pairwise_routes: HashMap<String, Recipient<HandleA2ConnMsg>>,
    requester: Addr<Requester>
}

impl Router {
    pub fn new(requester: Addr<Requester>) -> Router {
        trace!("Router::new >>");

        Router {
            routes: HashMap::new(),
            pairwise_routes: HashMap::new(),
            requester,
        }
    }

    fn add_a2a_route(&mut self, did: String, handler: Recipient<HandleA2AMsg>) {
        trace!("Router::handle_add_route >> {}", did);
        self.routes.insert(did, handler);
    }

    fn add_a2conn_route(&mut self, did: String, handler: Recipient<HandleA2ConnMsg>) {
        trace!("Router::add_a2conn_route >> {}", did);
        self.pairwise_routes.insert(did, handler);
    }

    pub fn route_a2a_msg(&self, did: String, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("Router::route_a2a_msg >> {:?}, {:?}", did, msg);

        if let Some(addr) = self.routes.get(&did) {
            addr
                .send(HandleA2AMsg(msg))
                .from_err()
                .and_then(|res| res)
                .into_box()
        } else {
            err!(err_msg("No route found."))
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

impl Actor for Router {
    type Context = Context<Self>;
}

impl Handler<AddA2ARoute> for Router {
    type Result = ();

    fn handle(&mut self, msg: AddA2ARoute, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AddA2ARoute>::handle >> {}", msg.0);
        self.add_a2a_route(msg.0, msg.1)
    }
}

impl Handler<AddA2ConnRoute> for Router {
    type Result = ();

    fn handle(&mut self, msg: AddA2ConnRoute, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AddA2ConnRoute>::handle >> {}", msg.0);
        self.add_a2conn_route(msg.0, msg.1)
    }
}

impl Handler<RouteA2AMsg> for Router {
    type Result = ResponseFuture<Vec<u8>, Error>;

    fn handle(&mut self, msg: RouteA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<RouteA2AMsg>::handle >> {:?}", msg);
        self.route_a2a_msg(msg.0, msg.1)
    }
}

impl Handler<RouteA2ConnMsg> for Router {
    type Result = ResponseFuture<A2ConnMessage, Error>;

    fn handle(&mut self, msg: RouteA2ConnMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<RouteA2ConnMsg>::handle >> {:?}", msg);
        self.route_a2conn_msg(msg.0, msg.1)
    }
}

impl Handler<RemoteMsg> for Router {
    type Result = ResponseFuture<(), Error>;

    fn handle(&mut self, msg: RemoteMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<RemoteMsg>::handle >> {:?}", msg);
        self.route_to_requester(msg)
    }
}
use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg, RouteA2AMsg};
use failure::{Error, err_msg};
use futures::*;
use std::collections::HashMap;
use utils::futures::*;

pub struct Router {
    routes: HashMap<String, Recipient<HandleA2AMsg>>,
}

impl Router {
    pub fn new() -> Router {
        trace!("Router::new >>");

        Router {
            routes: HashMap::new()
        }
    }

    fn add_a2a_route(&mut self, did: String, handler: Recipient<HandleA2AMsg>) {
        trace!("Router::handle_add_route >> {}", did);
        self.routes.insert(did, handler);
    }

    pub fn route_a2a_msg(&self, did: String, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("Router::handle_route >> {:?}, {:?}", did, msg);

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

impl Handler<RouteA2AMsg> for Router {
    type Result = ResponseFuture<Vec<u8>, Error>;

    fn handle(&mut self, msg: RouteA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<RouteA2AMsg>::handle >> {:?}", msg);
        self.route_a2a_msg(msg.0, msg.1)
    }
}
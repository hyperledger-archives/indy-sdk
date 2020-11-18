use actix::prelude::*;
use failure::{err_msg, Error};
use futures::*;
use hyper::{Body, header, Method, Request};
use hyper::Client;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;

use crate::actors::RemoteMsg;
use crate::utils::futures::*;

lazy_static! {
    pub static ref REQWEST_CLIENT : reqwest::r#async::Client = reqwest::r#async::Client::new();
}

pub struct Requester {
    client: Client<HttpConnector>,
}

impl Requester {
    pub fn new() -> Requester {
        trace!("Requester::new >>");

        Requester {
            client: Client::new()
        }
    }

    pub fn send_message(&self, msg: Vec<u8>, endpoint: String) -> ResponseFuture<(), Error> {
        trace!("Requester::send_message >> {:?}, {:?}", msg, endpoint);

        let request = match self.build_request(msg, &endpoint) {
            Ok(req) => req,
            Err(err) => return err!(err)
        };

        self.client.request(request)
            .map_err(|err| err.into())
            .and_then(|res|
                if res.status().is_success() {
                    future::ok(()).into_box()
                } else {
                    err!(err_msg("Request failed."))
                })
            .into_box()
    }

    fn build_request(&self, message: Vec<u8>, endpoint: &str) -> Result<Request<Body>, Error> {
        let mut req = Request::new(Body::from(message));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = endpoint.parse()?;
        req.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/octet-stream"),
        );
        Ok(req)
    }
}

impl Actor for Requester {
    type Context = Context<Self>;
}

impl Handler<RemoteMsg> for Requester {
    type Result = ResponseFuture<(), Error>;

    fn handle(&mut self, msg: RemoteMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<SendRemoteMessage>::handle >> {:?}", msg);
        self.send_message(msg.body, msg.endpoint)
    }
}
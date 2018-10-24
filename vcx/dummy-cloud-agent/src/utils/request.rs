use failure::Error;
use hyper::Client;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::{Body, Method, Request, header};

use std::sync::Mutex;

lazy_static! {
    pub static ref CLIENT: Mutex<Client<HttpConnector>> = Mutex::new(Client::default());
}

pub fn send_message_to_remote_endpoint(message: Vec<u8>, endpoint: &str) -> Result<(), Error> {
    let mut req = Request::new(Body::from(message));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = endpoint.parse()?;
    req.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream")
    );

    let client = CLIENT.lock().unwrap();

    client.request(req)
        .map(|res| assert!(res.status().is_success())) // TODO: FIXME
        .map_err(|err| err.into())
        .wait()
}
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate nom;

mod mock;

use std::thread;
use std::time::Duration;

use std::str::from_utf8;
use futures::{Future, Stream};
use hyper::{Client,Uri};
use tokio_core::reactor::Core;
use nom::AsBytes;

fn get(uri: Uri) -> Result<hyper::Response, hyper::Error>{
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let work = client.get(uri);

    core.run(work)
}

// Not sure how to do this right
//fn body_string(body: Body) -> &'static str{
//    let body_val = body.concat2().wait().unwrap();
//    let body_bytes = body_val.as_bytes();
//    let str = from_utf8(body_bytes).unwrap();
//    str.clone()
//}


#[test]
fn mock_agent_srv() {
    let server = mock::agent_srv::MockAgentSrv::new(Some(3000), None);
    server.start().unwrap();
    thread::sleep(Duration::from_millis(100));
    let resp = get("http://127.0.0.1:3000".parse().unwrap()).unwrap();
    let body_val = resp.body().concat2().wait().unwrap();
    let body_bytes = body_val.as_bytes();
    assert_eq!(from_utf8(body_bytes).unwrap(), "OK");
    server.stop().unwrap();
}
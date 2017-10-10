extern crate hyper;
extern crate futures;
extern crate rand;

use self::futures::future::Future;

use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response};
use self::hyper::server::Service;
use self::hyper::StatusCode;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use self::rand::StdRng;
use self::rand::Rng;

const PHRASE: &'static str = "OK";

struct GenericServerService {
    handler_fn: fn(hyper::Request) -> Box<Future<Item=hyper::Response, Error=hyper::Error>>
}

impl Service for GenericServerService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let t = self.handler_fn;
        t(req)
    }
}

pub struct MockAgentSrv {
    pub port: u16,
    handler_fn: fn(hyper::Request) -> Box<Future<Item=hyper::Response, Error=hyper::Error>>,
    stay_alive: Arc<Mutex<bool>>,
}

impl MockAgentSrv {
    pub fn new(port_arg: Option<u16>,
               handler: Option<fn(hyper::Request)
                   -> Box<Future<Item=hyper::Response, Error=hyper::Error>>>) -> MockAgentSrv {
        let port = port_arg.unwrap_or_else(|| {
            StdRng::new().unwrap().gen_range(49152,65535)
        });
        let handler_fn = handler.unwrap_or(standard_handler);
        MockAgentSrv {
            port,
            handler_fn,
            stay_alive: Arc::new(Mutex::new(true))
        }
    }

    pub fn start(&self) -> Result<(), &'static str> {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let addr: SocketAddr = SocketAddr::new(localhost, self.port.clone());
        let handler_fn = self.handler_fn.clone();
        let stay_alive = self.stay_alive.clone();
        thread::spawn(move || {
            let mut server = Http::new().bind(
                &addr,
                move || Ok(GenericServerService { handler_fn })
            ).unwrap();
            server.shutdown_timeout(Duration::new(0,1000));
            let r = server.run_until(futures::future::poll_fn( ||{
                if *stay_alive.lock().unwrap() {
                    Ok(futures::Async::NotReady)
                } else {
                    Ok(futures::Async::Ready(()))
                }
            }));
            r.unwrap();
        });
        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    pub fn stop(&self) -> Result<(), &'static str> {
        match self.stay_alive.lock() {
            Ok(mut mutex) => {
                *mutex = false;
                thread::sleep(Duration::from_millis(100));
                Ok(())
            },
            Err(_) => Err("Unable to signal shutdown"),

        }
    }

}

fn standard_handler(req: hyper::Request) -> Box<Future<Item=hyper::Response, Error=hyper::Error>> {
    req.version();
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentLength(PHRASE.len() as u64))
            .with_body(PHRASE)
            .with_status(StatusCode::Ok)
    ))
}
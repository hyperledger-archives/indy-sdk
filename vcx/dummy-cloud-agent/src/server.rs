use actix::prelude::*;
use actix_web::server::{self, IntoHttpHandler, Server};
use domain::config::ServerConfig;

pub fn start<F, U, H>(config: ServerConfig, factory: F) -> Addr<Server>
    where
        F: Fn() -> U + Sync + Send + 'static,
        U: IntoIterator<Item=H> + 'static,
        H: IntoHttpHandler + 'static, {
    let mut server = server::new(factory);

    if let Some(workers) = config.workers {
        server = server.workers(workers);
    }

    for address in config.addresses {
        server = server
            .bind(address)
            .expect("Can't bind to address!");
    }

    server.start()
}
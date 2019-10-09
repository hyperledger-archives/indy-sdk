use actix::prelude::*;
use actix_web::*;
use actors::forward_agent::ForwardAgent;
use domain::config::{AppConfig, ServerConfig};
use actix_web::middleware::Logger;
use api_agent::generate_api_agent_configuration;

pub struct AppData {
    pub forward_agent: Addr<ForwardAgent>
}

pub fn start_app_server(server_config: ServerConfig, app_config: AppConfig, forward_agent: Addr<ForwardAgent>) {
    info!("Forward Agent started");
    info!("Starting Server with config: {:?}", server_config);
    let mut server = HttpServer::new(move || {
        info!("Starting App with config: {:?}", app_config);
        App::new()
            .data(AppData { forward_agent: forward_agent.clone() })
            .wrap(Logger::default())
            .configure(generate_api_agent_configuration(&app_config.prefix))
    });
    if let Some(workers) = server_config.workers {
        server = server.workers(workers);
    }
    for address in server_config.addresses {
        server = server
            .bind(address)
            .expect("Can't bind to address!");
    }
    server.start();
    info!("Server started");
}
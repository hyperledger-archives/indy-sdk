use std::sync::{Arc, RwLock};

use actix::prelude::*;
use actix_web::*;
use actix_web::web::Data;
use bytes::Bytes;

use crate::actors::{ForwardA2AMsg, GetEndpoint};
use crate::actors::admin::Admin;
use crate::actors::forward_agent::ForwardAgent;
use crate::domain::config::{AppConfig, ServerConfig};

pub struct AppData {
    pub forward_agent: Addr<ForwardAgent>,
    pub admin_agent: Option<Arc<RwLock<Admin>>>,
}

pub fn start_app_server(server_config: ServerConfig, app_config: AppConfig, forward_agent: Addr<ForwardAgent>, admin_agent: Option<Arc<RwLock<Admin>>>) {
    info!("Creating HttpServer with config: {:?}", server_config);
    let mut server = HttpServer::new(move || {
        info!("Starting App with config: {:?}", app_config);
        App::new()
            .data(AppData { admin_agent: admin_agent.clone(), forward_agent: forward_agent.clone() })
            .wrap(middleware::Logger::default())
            .service(
                web::resource(&app_config.prefix)
                    .route(web::get().to(_get_endpoint_details))
            )
            .service(
                web::resource(&format!("{}/msg", app_config.prefix))
                    .route(web::post().to(_forward_message))
            )
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


fn _get_endpoint_details(state: Data<AppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.forward_agent
        .send(GetEndpoint {})
        .from_err()
        .map(|res| match res {
            Ok(endpoint) => HttpResponse::Ok().json(&endpoint),
            Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(), // FIXME: Better error
        });
    Box::new(f)
}

fn _forward_message(state: Data<AppData>, stream: web::Payload) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = stream.map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(move |body| {
            state.forward_agent
                .send(ForwardA2AMsg(body.to_vec()))
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(Bytes::from(msg).into()),
                    Err(err) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)).into()), // FIXME: Better error
                })
        });
    Box::new(f)
}

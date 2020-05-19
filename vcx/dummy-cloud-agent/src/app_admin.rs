use actix::prelude::*;
use actix_web::*;
use actix_web::web::Data;

use crate::actors::admin::Admin;
use crate::actors::HandleAdminMessage;
use crate::domain::admin_message::{AdminQuery, GetDetailAgentConnParams, GetDetailAgentParams};
use crate::domain::config::ServerAdminConfig;
use std::rc::Rc;
use std::sync::{RwLock, Arc};

pub struct AdminAppData {
    pub admin_agent: Arc<RwLock<Admin>>,
}

#[derive(Deserialize)]
struct AgentParams {
    did: String,
}

pub fn start_app_admin_server(server_admin_config: &ServerAdminConfig, admin_agent: Arc<RwLock<Admin>>) {
    info!("Creating Admin HttpServer using config {:?}", server_admin_config);
    let mut server = HttpServer::new(move || {
        App::new()
            .data(AdminAppData { admin_agent: admin_agent.clone() })
            .wrap(middleware::Logger::default())
                .service(
                    web::resource("/admin")
                        .route(web::get().to(_get_actor_overview))
                )
                .service(
                    web::resource("/admin/forward-agent")
                        .route(web::get().to(_get_forward_agent_details))
                )
                .service(
                    web::resource("/admin/agent/{did}")
                        .route(web::get().to(_get_agent_details))
                )
                .service(
                    web::resource("/admin/agent-connection/{did}")
                        .route(web::get().to(_get_agent_connection_details))
                )
    });
    for address in &server_admin_config.addresses {
        server = server
            .bind(address)
            .expect(&format!("Can't bind to address {}.", address));
    }
    server.start();
    info!("Admin Server started at addresses: {:?}.", server_admin_config.addresses);
}

fn _get_agent_connection_details(state: Data<AdminAppData>, info: web::Path<AgentParams>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.admin_agent.read().unwrap()
        .get_detail_agent_connection(info.did.clone())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| HttpResponse::InternalServerError().body(format!("{:?}", err)).into());
    Box::new(f)
}

fn _get_agent_details(state: Data<AdminAppData>, info: web::Path<AgentParams>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.admin_agent.read().unwrap()
        .get_detail_agent(info.did.clone())
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| HttpResponse::InternalServerError().body(format!("{:?}", err)).into());
    Box::new(f)
}

fn _get_actor_overview(state: Data<AdminAppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.admin_agent.read().unwrap()
        .get_actor_overview()
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| HttpResponse::InternalServerError().body(format!("{:?}", err)).into());
    Box::new(f)
}

fn _get_forward_agent_details(state: Data<AdminAppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.admin_agent.read().unwrap()
        .get_detail_forward_agents()
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|err| HttpResponse::InternalServerError().body(format!("{:?}", err)).into());
    Box::new(f)
}


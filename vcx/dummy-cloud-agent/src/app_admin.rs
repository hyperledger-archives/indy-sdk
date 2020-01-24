use actix::prelude::*;
use actix_web::*;
use actix_web::web::Data;

use actors::HandleAdminMessage;
use actors::admin::Admin;
use domain::admin_message::{AdminQuery, GetDetailAgentConnParams, GetDetailAgentParams};
use domain::config::ServerAdminConfig;

pub struct AdminAppData {
    pub admin_agent: Addr<Admin>,
}

#[derive(Deserialize)]
struct AgentParams {
    did: String,
}

pub fn start_app_admin_server(server_admin_config: &ServerAdminConfig, admin_agent: Addr<Admin>) {
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

fn _send_admin_message(state: Data<AdminAppData>, admin_msg: HandleAdminMessage) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.admin_agent
                    .send(admin_msg)
                    .from_err()
                    .map(|res| match res {
                        Ok(agent_details) => HttpResponse::Ok().json(&agent_details),
                        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(),
                    });
    Box::new(f)
}

fn _get_agent_connection_details(state: Data<AdminAppData>, info: web::Path<AgentParams>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailAgentConnection(GetDetailAgentConnParams { agent_pairwise_did: info.did.clone() }));
    _send_admin_message(state, msg)
}

fn _get_agent_details(state: Data<AdminAppData>, info: web::Path<AgentParams>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailAgent(GetDetailAgentParams { agent_did: info.did.clone() }));
    _send_admin_message(state, msg)
}

fn _get_actor_overview(state: Data<AdminAppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetActorOverview);
    _send_admin_message(state, msg)
}

fn _get_forward_agent_details(state: Data<AdminAppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailForwardAgents);
    _send_admin_message(state, msg)
}


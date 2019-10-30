use actix::prelude::*;
use actix_web::*;
use actors::{ForwardA2AMsg, GetEndpoint, HandleAdminMessage};
use actors::forward_agent::ForwardAgent;
use actors::admin::Admin;
use bytes::Bytes;
use domain::config::{AppConfig, ServerConfig};
use domain::admin_message::{AdminQuery, GetDetailAgentParams, GetDetailAgentConnParams};
use actix_web::web::Data;
use futures::future;
use futures::future::Either;

pub struct AppData {
    pub forward_agent: Addr<ForwardAgent>,
    pub admin_agent: Option<Addr<Admin>>,
}

#[derive(Deserialize)]
struct AgentParams {
    did: String,
}

pub fn start_app_server(server_config: ServerConfig, app_config: AppConfig, forward_agent: Addr<ForwardAgent>, admin_agent: Option<Addr<Admin>>) {
    info!("Creating HttpServer with config: {:?}", server_config);
    let mut server = HttpServer::new(move || {
        info!("Starting App with config: {:?}", app_config);
        let enable_admin_api = admin_agent.as_ref().map_or( false, |_| true);
        let app = App::new()
            .data(AppData { admin_agent: admin_agent.clone(), forward_agent: forward_agent.clone() })
            .wrap(middleware::Logger::default())
            .service(
                web::resource(&app_config.prefix)
                    .route(web::get().to(_get_endpoint_details))
            )
            .service(
                web::resource(&format!("{}/msg", app_config.prefix))
                    .route(web::post().to(_forward_message))
            );
        if enable_admin_api {
            app
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
        } else {
            app
        }
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


fn _get_endpoint_details(state: Data<AppData>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let f = state.forward_agent
        .send(GetEndpoint {})
        .from_err()
        .map(|res| match res {
            Ok(endpoint) => HttpResponse::Ok().json(&endpoint),
            Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(), // FIXME: Better error
        });
    Box::new(f)
}

fn _send_admin_message(state: Data<AppData>, admin_msg: HandleAdminMessage) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let f = match &state.admin_agent {
        Some(admin) => {
            Either::A(
                admin
                    .send(admin_msg)
                    .from_err()
                    .map(|res| match res {
                        Ok(agent_details) => HttpResponse::Ok().json(&agent_details),
                        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(),
                    }))
        }
        None => {
            Either::B(future::ok(HttpResponse::InternalServerError().body("Admin actor not found.").into()))
        }
    };
    Box::new(f)
}

fn _get_agent_connection_details(state: Data<AppData>, info: web::Path<AgentParams>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailAgentConnection(GetDetailAgentConnParams { agent_pairwise_did: info.did.clone() }));
    _send_admin_message(state, msg)
}

fn _get_agent_details(state: Data<AppData>, info: web::Path<AgentParams>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailAgent(GetDetailAgentParams { agent_did: info.did.clone() }));
    _send_admin_message(state, msg)
}

fn _get_actor_overview(state: Data<AppData>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetActorOverview);
    _send_admin_message(state, msg)
}

fn _get_forward_agent_details(state: Data<AppData>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let msg = HandleAdminMessage(AdminQuery::GetDetailForwardAgents);
    _send_admin_message(state, msg)
}

fn _forward_message(state: Data<AppData>, stream: web::Payload) -> Box<Future<Item=HttpResponse, Error=Error>> {
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

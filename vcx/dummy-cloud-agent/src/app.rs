use actix::prelude::*;
use actix_web::*;
use actors::{ForwardA2AMsg, GetEndpoint};
use actors::forward_agent::ForwardAgent;
use bytes::Bytes;
use domain::config::AppConfig;
use futures::*;

const MAX_PAYLOAD_SIZE: usize = 105_906_176;

pub struct AppState {
    pub forward_agent: Addr<ForwardAgent>,
}

pub fn new(config: AppConfig, forward_agent: Addr<ForwardAgent>) -> App<AppState> {
    App::with_state(AppState { forward_agent })
        .prefix(config.prefix.as_ref())
        .middleware(middleware::Logger::default()) // enable logger
        .resource("", |r| r.method(http::Method::GET).with(_get_endpoint_details))
        .resource("/msg", |r| r.method(http::Method::POST).with(_forward_message))
}

fn _get_endpoint_details(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.forward_agent
        .send(GetEndpoint {})
        .from_err()
        .map(|res| match res {
            Ok(endpoint) => HttpResponse::Ok().json(&endpoint),
            Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(), // FIXME: Better error
        })
        .responder()
}

fn _forward_message((state, req): (State<AppState>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    req
        .body()
        .limit(MAX_PAYLOAD_SIZE)
        .from_err()
        .and_then(move |body| {
            state.forward_agent
                .send(ForwardA2AMsg(body.to_vec()))
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(Bytes::from(msg).into()),
                    Err(err) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)).into()), // FIXME: Better error
                })
        })
        .responder()
}



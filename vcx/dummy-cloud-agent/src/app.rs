use actix::prelude::*;
use actix_web::*;
use actors::forward_agent::{ForwardAgent, ForwardMessage, GetEndpointDetails};
use bytes::Bytes;
use domain::config::AppConfig;
use futures::*;

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
        .send(GetEndpointDetails {})
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(err) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)).into()), // FIXME: Better error
        })
        .responder()
}

fn _forward_message((state, req): (State<AppState>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    req
        .body()
        .from_err()
        .and_then(move |body| {
            state.forward_agent
                .send(ForwardMessage(body.to_vec()))
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(Bytes::from(msg.0).into()),
                    Err(err) => Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)).into()), // FIXME: Better error
                })
        })
        .responder()
}



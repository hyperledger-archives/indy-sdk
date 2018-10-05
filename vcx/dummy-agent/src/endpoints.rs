use actix::prelude::*;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, State};
use actors::forward_agent::{ForwardAgent, GetForwardDetail, Post};
use futures::Future;

pub struct AppState {
    pub forward_agent: Addr<ForwardAgent>,
}

pub fn get(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.forward_agent
        .send(GetForwardDetail {})
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn post_msg(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.forward_agent
        .send(Post("Dummy message".to_owned()))
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
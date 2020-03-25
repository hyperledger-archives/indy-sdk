use actix::prelude::*;
use actix_web::*;
use bytes::Bytes;

use crate::actors::{ForwardA2AMsg, GetEndpoint};
use crate::app::AppData;

const MAX_PAYLOAD_SIZE: usize = 105_906_176;

pub fn generate_api_agent_configuration(api_prefix: &String) -> Box<dyn FnOnce(&mut web::ServiceConfig)>{
    let prefix = api_prefix.clone();
    Box::new( move |cfg: &mut web::ServiceConfig| {
        cfg.service(
        web::scope(&prefix)
            .route("", web::get().to(_get_endpoint_details))
            .route("/msg", web::post().to(_forward_message)));
    })
}

fn _get_endpoint_details(state: web::Data<AppData>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = state.forward_agent
        .send(GetEndpoint {})
        .from_err()
        .map(|res| match res {
            Ok(endpoint) => HttpResponse::Ok().json(&endpoint),
            Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)).into(), // FIXME: Better error
        });
    Box::new(f)
}

/// The entry point of any agent communication. Incoming data are passed to Forward Agent for processing
fn _forward_message(state: web::Data<AppData>, stream: web::Payload) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
    let f = stream.map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
                Err(error::ErrorBadRequest("Payload exceeded maximum size."))
            } else {
                body.extend_from_slice(&chunk);
                Ok::<_, Error>(body)
            }
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


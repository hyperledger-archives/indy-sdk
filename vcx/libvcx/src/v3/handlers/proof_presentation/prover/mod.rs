pub mod states;
pub mod prover;

use self::prover::Prover;

use serde_json;

use object_cache::ObjectCache;
use error::prelude::*;

use v3::handlers::connection;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use v3::messages::A2AMessage;

lazy_static! {
    pub static ref PROVER_MAP: ObjectCache<Prover>  = Default::default();
}

pub fn create_proof(source_id: &str, presentation_request: &str) -> VcxResult<u32> {
    let prover: Prover = Prover::create(source_id, presentation_request)?;
    PROVER_MAP.add(prover)
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get(handle, |obj| {
        Ok(obj.state())
    })
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.update_state(message.as_ref().map(String::as_str))
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        prover.to_string()
    })
}

pub fn from_string(prover_data: &str) -> VcxResult<u32> {
    let prover: Prover = Prover::from_str(prover_data)?;
    PROVER_MAP.add(prover)
}

pub fn release(handle: u32) -> VcxResult<()> {
    PROVER_MAP.release(handle)
}

pub fn release_all() {
    PROVER_MAP.drain().ok();
}

pub fn generate_proof_msg(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.generate_presentation_msg()
    })
}

pub fn send_proof(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.send_proof(connection_handle)
    })
}

pub fn generate_presentation(handle: u32, credentials: String, self_attested_attrs: String) -> VcxResult<u32> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.generate_presentation(credentials.clone(), self_attested_attrs.clone())
    })
}

pub fn retrieve_credentials(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get_mut(handle, |prover| {
        prover.retrieve_credentials()
    })
}

pub fn is_valid_handle(handle: u32) -> bool {
    PROVER_MAP.has_handle(handle)
}

pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
    let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

    match message {
        A2AMessage::PresentationRequest(presentation_request) => {
            serde_json::to_string_pretty(&presentation_request)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize message: {}", err)))
        }
        _ => Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Message has different type"))
    }
}

pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<String> {
    let (messages, _) = connection::get_messages(connection_handle)?;

    let presentation_requests: Vec<PresentationRequest> =
        messages
            .into_iter()
            .filter_map(|(_, message)| {
                match message {
                    A2AMessage::PresentationRequest(presentation_request) => Some(presentation_request),
                    _ => None,
                }
            })
            .collect();

    serde_json::to_string_pretty(&presentation_requests)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize proof request: {}", err)))
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        Ok(prover.get_source_id().clone())
    })
}

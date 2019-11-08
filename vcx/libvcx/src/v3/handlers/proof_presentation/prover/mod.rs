pub mod states;
pub mod prover;

use self::prover::Prover;

use object_cache::ObjectCache;
use utils::error;
use error::prelude::*;

use v3::messages::proof_presentation::presentation_request::PresentationRequest;

lazy_static! {
    pub static ref PROVER_MAP: ObjectCache<Prover>  = Default::default();
}

pub fn create_presentation(source_id: &str, presentation_request: PresentationRequest) -> VcxResult<u32> {
    let prover: Prover = Prover::create(source_id, presentation_request)?;
    PROVER_MAP.add(prover)
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get(handle, |obj| {
        Ok(obj.state())
    })
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    PROVER_MAP.map(handle, |prover| {
        prover.update_state(message.as_ref().map(String::as_str))
    }).map(|_| error::SUCCESS.code_num)
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
    PROVER_MAP.get(handle, |prover| {
        prover.generate_presentation_msg()
    })
}

pub fn send_proof(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    PROVER_MAP.map(handle, |prover| {
        prover.send_presentation(connection_handle)
    }).map(|_| error::SUCCESS.code_num)
}

pub fn generate_presentation(handle: u32, credentials: String, self_attested_attrs: String) -> VcxResult<u32> {
    PROVER_MAP.map(handle, |prover| {
        prover.generate_presentation(credentials.clone(), self_attested_attrs.clone())
    }).map(|_| error::SUCCESS.code_num)
}

pub fn retrieve_credentials(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        prover.retrieve_credentials()
    })
}

pub fn is_valid_handle(handle: u32) -> bool {
    PROVER_MAP.has_handle(handle)
}

pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<PresentationRequest> {
    Prover::get_presentation_request(connection_handle, msg_id)
}

pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<Vec<PresentationRequest>> {
    Prover::get_presentation_request_messages(connection_handle, match_name)
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    PROVER_MAP.get(handle, |prover| {
        Ok(prover.get_source_id().clone())
    })
}

pub fn get_presentation_status(handle: u32) -> VcxResult<u32> {
    PROVER_MAP.get(handle, |prover| {
        Ok(prover.presentation_status())
    })
}

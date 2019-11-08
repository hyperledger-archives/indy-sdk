mod states;
mod verifier;

use self::verifier::Verifier;

use object_cache::ObjectCache;
use utils::error;
use error::prelude::*;

lazy_static! {
    pub static ref VERIFIER_MAP: ObjectCache<Verifier> = Default::default();
}

pub fn create_proof(source_id: String,
                    requested_attrs: String,
                    requested_predicates: String,
                    revocation_details: String,
                    name: String) -> VcxResult<u32> {
    trace!("create_proof >>> source_id: {}, requested_attrs: {}, requested_predicates: {}, name: {}", source_id, requested_attrs, requested_predicates, name);

    let verifier = Verifier::create(source_id,
                                    requested_attrs,
                                    requested_predicates,
                                    revocation_details,
                                    name)?;

    VERIFIER_MAP.add(verifier)
        .or(Err(VcxError::from(VcxErrorKind::CreateProof)))
}

pub fn is_valid_handle(handle: u32) -> bool {
    VERIFIER_MAP.has_handle(handle)
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    VERIFIER_MAP.map(handle, |verifier| {
        verifier.update_state(message.as_ref().map(String::as_str))
    }).map(|_| error::SUCCESS.code_num)
}

pub fn get_state(handle: u32) -> u32 {
    VERIFIER_MAP.get(handle, |verifier| {
        Ok(verifier.state())
    }).unwrap_or(0)
}

pub fn get_presentation_status(handle: u32) -> VcxResult<u32> {
    VERIFIER_MAP.get(handle, |p| {
        Ok(p.presentation_status())
    })
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |verifier| {
        Verifier::to_string(&verifier)
    })
}

pub fn from_string(verifier_data: &str) -> VcxResult<u32> {
    let verifier: Verifier = Verifier::from_str(verifier_data)?;
    VERIFIER_MAP.add(verifier)
}

pub fn release(handle: u32) -> VcxResult<()> {
    VERIFIER_MAP.release(handle).or(Err(VcxError::from(VcxErrorKind::InvalidProofHandle)))
}

pub fn release_all() {
    VERIFIER_MAP.drain().ok();
}

pub fn send_presentation_request(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    VERIFIER_MAP.map(handle, |verifier| {
        verifier.send_presentation_request(connection_handle)
    }).map(|_| error::SUCCESS.code_num)
}

pub fn generate_presentation_request_msg(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |verifier| {
        verifier.generate_presentation_request_msg()
    })
}

pub fn get_presentation(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |verifier| {
        verifier.get_presentation()
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    VERIFIER_MAP.get(handle, |obj| {
        Ok(obj.get_source_id())
    })
}

pub mod issuer;
pub mod states;
pub mod messages;
pub mod holder;

use object_cache::ObjectCache;
use error::prelude::*;
use v3::handlers::issuance::issuer::IssuerSM;
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::holder::HolderSM;
use v3::messages::A2AMessage;
use v3::handlers::connection;
use messages::get_message::Message;

lazy_static! {
    pub static ref ISSUE_CREDENTIAL_MAP: ObjectCache<IssuerSM> = Default::default();
}

lazy_static! {
    pub static ref ISSUE_CREDENTIAL_INCOMING_MAP: ObjectCache<A2AMessage> = Default::default();
}

lazy_static! {
    pub static ref HOLD_CREDENTIAL_MAP: ObjectCache<HolderSM> = Default::default();
}

lazy_static! {
    pub static ref HOLD_CREDENTIAL_INCOMING_MAP: ObjectCache<A2AMessage> = Default::default();
}

// Issuer

pub fn create_issuer_credential(cred_def_handle: u32, credential_data: &str) -> VcxResult<u32> {
    let cred_def_id = ::credential_def::get_cred_def_id(cred_def_handle)?;
    let rev_reg_id = ::credential_def::get_rev_reg_id(cred_def_handle)?;
    let tails_file = ::credential_def::get_tails_file(cred_def_handle)?;
    let credential = IssuerSM::new(&cred_def_id, credential_data, rev_reg_id, tails_file);

    ISSUE_CREDENTIAL_MAP.add(credential)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn send_credential_offer(credential_handle: u32, connection_handle: u32) -> VcxResult<()> {
    ISSUE_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
        issuer_sm.handle_message(CredentialIssuanceMessage::CredentialInit(connection_handle))
    })
}

pub fn update_status(credential_handle: u32, connection_handle: u32, msg: Option<String>) -> VcxResult<u32> {
    match msg {
        Some(msg) => {
            let message: Message = ::serde_json::from_str(&message)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;
            //TODO: get rid of connection message, hide this into SM
            let a2a_message = connection::decode_message(connection_handle, message)?;
            ISSUER_CREDENTIAL_INCOMING_MAP.insert(credential_handle, a2a_message)?;
            Ok(VcxStateType::VcxStateRequestReceived)
        },
        None => {
            ISSUER_CREDENTIAL_MAP.get(credential_handle, |issuer_sm| {

            })?;
            Ok(VcxStateType::VcxStateOfferSent)
        }
    }
}

pub fn send_credential(credential_handle: u32, connection_handle: u32) -> VcxResult<()> {
    ISSUE_CREDENTIAL_INCOMING_MAP.get(credential_handle, |msg| {
        let sm_msg: CredentialIssuanceMessage = (msg, connection_handle).into();
        ISSUE_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
            issuer_sm.handle_message(sm_msg.clone())
        })
    })
}



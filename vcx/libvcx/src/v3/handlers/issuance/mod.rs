pub mod issuer;
pub mod states;
pub mod messages;
pub mod holder;

use std::collections::{HashSet, HashMap};
use api::VcxStateType;
use error::prelude::*;
use messages::get_message::Message;
use messages::update_message::{UIDsByConn, update_messages};
use object_cache::ObjectCache;
use v3::messages::{A2AMessage, MessageId};
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::handlers::connection;
use v3::handlers::issuance::issuer::IssuerSM;
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::holder::HolderSM;
use utils::error;
use messages::MessageStatusCode;

lazy_static! {
    pub static ref ISSUE_CREDENTIAL_MAP: ObjectCache<IssuerSM> = Default::default();
}

lazy_static! {
    pub static ref HOLD_CREDENTIAL_MAP: ObjectCache<HolderSM> = Default::default();
}

// Issuer

pub fn issuer_create_credential(cred_def_handle: u32, credential_data: &str, source_id: &str) -> VcxResult<u32> {
    let cred_def_id = ::credential_def::get_cred_def_id(cred_def_handle)?;
    let rev_reg_id = ::credential_def::get_rev_reg_id(cred_def_handle)?;
    let tails_file = ::credential_def::get_tails_file(cred_def_handle)?;
    let credential = IssuerSM::new(&cred_def_id, credential_data, rev_reg_id, tails_file, source_id.to_string());

    ISSUE_CREDENTIAL_MAP.add(credential)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn send_credential_offer(credential_handle: u32, connection_handle: u32) -> VcxResult<u32> {
    ISSUE_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
        issuer_sm.handle_message(CredentialIssuanceMessage::CredentialInit(connection_handle))
    }).map(|_| error::SUCCESS.code_num)
}

pub fn issuer_update_status(credential_handle: u32, msg: Option<String>) -> VcxResult<u32> {
    let msg = match msg {
        Some(msg) => {
            let message: Message = ::serde_json::from_str(&msg)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;
            Some(ISSUE_CREDENTIAL_MAP.get(credential_handle, |issuer_sm| {
                connection::decode_message(issuer_sm.get_connection_handle(), message.clone())
            })?)
        }
        None => {
            ISSUE_CREDENTIAL_MAP.get(credential_handle, |issuer_sm| {
                issuer_sm.fetch_messages()
            })?
        }
    };

    if let Some(sm_msg) = msg {
        ISSUE_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
            issuer_sm.handle_message((&sm_msg, 0u32).into())
        })?;
        get_state(credential_handle)
    } else {
        get_state(credential_handle)
    }
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    ISSUE_CREDENTIAL_MAP.get(handle, |obj| {
        Ok(obj.get_status() as u32)
    })
}

pub fn send_credential(credential_handle: u32, connection_handle: u32) -> VcxResult<u32> {
    ISSUE_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
        issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend())
    }).map(|_| error::SUCCESS.code_num)
}

pub fn issuer_get_status(credential_handle: u32) -> VcxResult<u32> {
    ISSUE_CREDENTIAL_MAP.get(credential_handle, |issuer_sm| {
        Ok(issuer_sm.get_status() as u32)
    })
}

pub fn get_issuer_source_id(handle: u32) -> VcxResult<String> {
    ISSUE_CREDENTIAL_MAP.get(handle, |issuer_sm| {
        Ok(issuer_sm.get_source_id())
    })
}

// Holder

pub fn holder_create_credential(credential_offer: &str, source_id: &str) -> VcxResult<u32> {
    let cred_offer: CredentialOffer = ::serde_json::from_str(credential_offer)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;
    let holder = HolderSM::new(cred_offer, source_id.to_string());
    HOLD_CREDENTIAL_MAP.add(holder)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn holder_send_request(credential_handle: u32, connection_handle: u32) -> VcxResult<u32> {
    HOLD_CREDENTIAL_MAP.map(credential_handle, |holder_sm| {
        holder_sm.handle_message(CredentialIssuanceMessage::CredentialRequestSend(connection_handle))
    }).map(|_| error::SUCCESS.code_num)
}

pub fn holder_update_status(credential_handle: u32, msg: Option<String>) -> VcxResult<u32> {
    let msg = match msg {
        Some(msg) => {
            let message: Message = ::serde_json::from_str(&msg)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot deserialize Message: {:?}", err)))?;
            Some(HOLD_CREDENTIAL_MAP.get(credential_handle, |holder_sm| {
                connection::decode_message(holder_sm.get_connection_handle(), message.clone())
            })?)
        }
        None => {
            HOLD_CREDENTIAL_MAP.get(credential_handle, |holder_sm| {
                holder_sm.fetch_message()
            })?
        }
    };

    if let Some(sm_msg) = msg {
        HOLD_CREDENTIAL_MAP.map(credential_handle, |issuer_sm| {
            issuer_sm.handle_message((&sm_msg, 0u32).into())
        })?;
        Ok(VcxStateType::VcxStateRequestReceived as u32)
    } else {
        Ok(VcxStateType::VcxStateOfferSent as u32)
    }
}

pub fn holder_get_status(credential_handle: u32) -> VcxResult<u32> {
    HOLD_CREDENTIAL_MAP.get(credential_handle, |holder_sm| {
        Ok(holder_sm.get_status() as u32)
    })
}

pub fn get_credential_offer_messages(conn_handle: u32) -> VcxResult<Vec<CredentialOffer>> {
    let messages = connection::get_messages(conn_handle)?;
    let (uids, msgs): (HashMap<MessageId, String>, Vec<CredentialOffer>) = messages.into_iter().filter_map(|(uid, a2a_message)| {
        match &a2a_message {
            A2AMessage::CredentialOffer(ref credential) => {
                Some((uid, credential.clone()))
            }
            _ => None
        }
    }).fold((HashMap::new(), vec![]), |(mut uids, mut msgs), (uid, msg)| {
        uids.insert(msg.id.clone(), uid);
        msgs.push(msg);
        (uids, msgs)
    });

    connection::add_pending_messages(conn_handle, uids)?;

    Ok(msgs)
}


pub fn get_holder_source_id(handle: u32) -> VcxResult<String> {
    HOLD_CREDENTIAL_MAP.get(handle, |holder_sm| {
        Ok(holder_sm.get_source_id())
    })
}


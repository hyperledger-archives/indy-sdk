use api::VcxStateType;

use v3::handlers::issuance::states::{HolderState, OfferReceivedState};
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::messages::issuance::credential::Credential;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::error::ProblemReport;
use v3::handlers::connection::{send_message, get_messages, get_pw_did, update_message_status};
use v3::messages::A2AMessage;
use v3::messages::ack::Ack;
use v3::handlers::connection;
use v3::messages::status::Status;

use messages::thread::Thread;
use utils::libindy::anoncreds::{self, libindy_prover_store_credential};
use error::prelude::*;

use credential;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HolderSM {
    state: HolderState,
    source_id: String
}

impl HolderSM {
    pub fn new(offer: CredentialOffer, source_id: String) -> Self {
        HolderSM {
            state: HolderState::OfferReceived(OfferReceivedState::new(offer)),
            source_id
        }
    }

    pub fn get_source_id(&self) -> String {
        self.source_id.clone()
    }

    pub fn get_state(&self) -> VcxStateType {
        match self.state {
            HolderState::OfferReceived(_) => VcxStateType::VcxStateRequestReceived,
            HolderState::RequestSent(_) => VcxStateType::VcxStateOfferSent,
            HolderState::Finished(_) => VcxStateType::VcxStateAccepted,
        }
    }

    pub fn fetch_message(&self) -> VcxResult<Option<A2AMessage>> {
        if self.is_terminal_state() { return Ok(None); }

        let conn_handle = self.state.get_connection_handle();
        let thread_id = self.state.get_thread_id();
        let messages = get_messages(conn_handle)?;

        let res: Option<(String, A2AMessage)> = messages.into_iter()
            .filter_map(|(uid, a2a_message)| {
                let thid = match &a2a_message {
                    A2AMessage::CommonProblemReport(ref report) => {
                        report.thread.thid.clone()
                    }
                    A2AMessage::Credential(ref credential) => {
                        credential.thread.thid.clone()
                    }
                    _ => None
                };
                if thid == thread_id {
                    Some((uid, a2a_message))
                } else {
                    None
                }
            })
            .nth(0);

        if let Some((uid, msg)) = res {
            update_message_status(conn_handle, uid)?;
            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }

    pub fn get_connection_handle(&self) -> u32 {
        self.state.get_connection_handle()
    }

    pub fn step(state: HolderState, source_id: String) -> Self {
        HolderSM { state, source_id }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<HolderSM> {
        let HolderSM { state, source_id } = self;
        let state = match state {
            HolderState::OfferReceived(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequestSend(connection_handle) => {
                    let conn_handle = connection_handle;
                    let request = _make_credential_request(conn_handle, &state_data.offer);
                    match request {
                        Ok((cred_request, req_meta, cred_def_json)) => {
                            let cred_request = cred_request
                                .set_thread(Thread::new().set_thid(state_data.offer.id.0.clone()));
                            connection::remove_pending_message(conn_handle, &state_data.offer.id)?;
                            send_message(conn_handle, cred_request.to_a2a_message())?;
                            HolderState::RequestSent((state_data, req_meta, cred_def_json, connection_handle).into())
                        }
                        Err(err) => {
                            let problem_report = ProblemReport::create()
                                .set_comment(err.to_string())
                                .set_thread(Thread::new().set_thid(state_data.offer.id.0.clone()));
                            send_message(conn_handle, problem_report.to_a2a_message())?;
                            HolderState::Finished((state_data, problem_report).into())
                        }
                    }
                }
                _ => {
                    warn!("Credential Issuance can only start on holder side with Credential Offer");
                    HolderState::OfferReceived(state_data)
                }
            },
            HolderState::RequestSent(state_data) => match cim {
                CredentialIssuanceMessage::Credential(credential, connection_handle) => {
                    let result = _store_credential(&credential, &state_data.req_meta, &state_data.cred_def_json);
                    match result {
                        Ok(cred_id) => {
                            let ack = Ack::create()
                                .set_thread(Thread::new().set_thid(state_data.thread_id.clone()));

                            send_message(state_data.connection_handle, ack.to_a2a_message())?;
                            HolderState::Finished((state_data, cred_id, credential).into())
                        }
                        Err(err) => {
                            let problem_report = ProblemReport::create()
                                .set_comment(err.to_string())
                                .set_thread(Thread::new().set_thid(state_data.thread_id.clone()));

                            send_message(state_data.connection_handle, problem_report.to_a2a_message())?;
                            HolderState::Finished((state_data, problem_report).into())
                        }
                    }
                }
                CredentialIssuanceMessage::ProblemReport(problem_report) => {
                    HolderState::Finished((state_data, problem_report).into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Credential and Problem Report");
                    HolderState::RequestSent(state_data)
                }
            },
            HolderState::Finished(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                HolderState::Finished(state_data)
            }
        };
        Ok(HolderSM::step(state, source_id))
    }

    pub fn credential_status(&self) -> u32 {
        match self.state {
            HolderState::Finished(ref state) => state.status.code(),
            _ => Status::Undefined.code()
        }
    }

    pub fn is_terminal_state(&self) -> bool {
        match self.state {
            HolderState::Finished(_) => true,
            _ => false
        }
    }

    pub fn get_credential(&self) -> VcxResult<(String, Credential)> {
        match self.state {
            HolderState::Finished(ref state) => {
                let cred_id = state.cred_id.clone().ok_or(VcxError::from(VcxErrorKind::InvalidState))?;
                let credential = state.credential.clone().ok_or(VcxError::from(VcxErrorKind::InvalidState))?;
                Ok((cred_id, credential))
            },
            _ => Err(VcxError::from(VcxErrorKind::InvalidState))
        }
    }
}

fn _parse_cred_def_from_cred_offer(cred_offer: &str) -> VcxResult<String> {
    let parsed_offer: serde_json::Value = serde_json::from_str(cred_offer)
        .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Offer Json".to_string()))?;
    let cred_def_id = parsed_offer.as_object()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Offer Json".to_string()))?
        .get("cred_def_id")
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Offer Json".to_string()))?
        .as_str()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Offer Json".to_string()))?;
    Ok(cred_def_id.to_string())
}

fn _parse_rev_reg_id_from_credential(credential: &str) -> VcxResult<Option<String>> {
    let parsed_credential: serde_json::Value = serde_json::from_str(credential)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Invalid Credential Json: {}, err: {:?}", credential, err)))?;

    let rev_reg_id = match parsed_credential.as_object()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?
        .get("rev_reg_id") {
        None => None,
        Some(a) => a.as_str().map(|s| s.to_string())
    };

    Ok(rev_reg_id)
}

fn _store_credential(credential: &Credential,
                     req_meta: &str, cred_def_json: &str) -> VcxResult<String> {
    let credential_json = credential.credentials_attach.content()?;
    let rev_reg_id = _parse_rev_reg_id_from_credential(&credential_json)?;
    let rev_reg_def_json = if let Some(rev_reg_id) = rev_reg_id {
        let (_, json) = anoncreds::get_rev_reg_def_json(&rev_reg_id)?;
        Some(json)
    } else {
        None
    };

    libindy_prover_store_credential(None,
                                    req_meta,
                                    &credential_json,
                                    cred_def_json,
                                    rev_reg_def_json.as_ref().map(String::as_str))
}

fn _make_credential_request(conn_handle: u32, offer: &CredentialOffer) -> VcxResult<(CredentialRequest, String, String)> {
    let my_did = get_pw_did(conn_handle)?;
    let cred_offer = offer.offers_attach.content()?;
    let cred_def_id = _parse_cred_def_from_cred_offer(&cred_offer)?;
    let (req, req_meta, cred_def_id, cred_def_json) =
        credential::Credential::create_credential_request(&cred_def_id, &my_did, &cred_offer)?;
    Ok((CredentialRequest::create().set_requests_attach(req)?, req_meta, cred_def_json))
}
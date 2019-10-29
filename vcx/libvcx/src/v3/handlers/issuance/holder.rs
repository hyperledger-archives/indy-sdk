use v3::handlers::issuance::states::{HolderState, InitialHolderState};
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::messages::issuance::{
    self,
    credential_offer::CredentialOffer,
    credential_request::CredentialRequest,
};
use v3::messages::error::ProblemReport;
use v3::messages::attachment::Attachment;
use credential::Credential;
use utils::error::Error;
use error::{VcxError, VcxErrorKind, VcxResult};
use v3::handlers::connection::{send_message, get_pw_did};
use v3::messages::A2AMessage;
use v3::messages::ack::{Ack, AckStatus};
use messages::thread::Thread;
use utils::libindy::anoncreds::{self, libindy_prover_store_credential};

pub struct HolderSM {
    state: HolderState
}

impl HolderSM {
    pub fn new() -> Self {
        HolderSM {
            state: HolderState::Initial(InitialHolderState::new())
        }
    }

    pub fn step(state: HolderState) -> Self {
        HolderSM { state }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<HolderSM> {
        let HolderSM { state } = self;
        let state = match state {
            HolderState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialOffer(offer, connection_handle) => {
                    let conn_handle = connection_handle;
                    let offer_accepted = true;
                    let request = _make_credential_request(conn_handle, offer);
                    let (msg, state) = if let Ok((cred_request, req_meta, cred_def_json)) = request {
                        let msg = A2AMessage::CredentialRequest(cred_request);
                        (msg, HolderState::RequestSent((state_data, req_meta, cred_def_json).into()))
                    } else {
                        let msg = A2AMessage::CommonProblemReport(
                            ProblemReport::create()
                                //TODO define some error codes inside RFC and use them here
                                .set_description(0)
                        );
                        (msg, HolderState::Finished(state_data.into()))
                    };
                    send_message(conn_handle, msg)?;
                    state
                }
                _ => {
                    warn!("Credential Issuance can only start on holder side with Credential Offer");
                    HolderState::Initial(state_data)
                }
            },
            HolderState::RequestSent(state_data) => match cim {
                CredentialIssuanceMessage::Credential(credential, connection_handle) => {
                    let result = _store_credential(&credential, &state_data.req_meta, &state_data.cred_def_json);
                    let (msg, cred_id) = if let Ok(cred_id) = result {
                        (
                            A2AMessage::Ack(
                                Ack::create()
                                    .set_status(AckStatus::Ok)
                                    .set_thread(Thread::new().set_thid(credential.id.0.clone()))
                            ),
                            Some(cred_id)
                        )
                    } else {
                        (
                            A2AMessage::CommonProblemReport(
                                ProblemReport::create()
                                    //TODO define some error codes inside RFC and use them here
                                    .set_description(0)
                            ),
                            None
                        )
                    };
                    send_message(connection_handle, msg)?;
                    HolderState::Finished((state_data, cred_id).into())
                }
                CredentialIssuanceMessage::ProblemReport(_report) => {
                    HolderState::Finished((state_data, None).into())
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
        Ok(HolderSM::step(state))
    }

    pub fn is_terminal_state(&self) -> bool {
        match self.state {
            HolderState::Finished(_) => true,
            _ => false
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

fn _parse_rev_reg_id_from_credential(credential: &str) -> VcxResult<String> {
    let parsed_credential: serde_json::Value = serde_json::from_str(credential)
        .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?;

    let rev_reg_id = parsed_credential.as_object()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?
        .get("rev_reg_id")
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?
        .as_str()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Json".to_string()))?;

    Ok(rev_reg_id.to_string())
}

fn _store_credential(credential: &issuance::credential::Credential,
                     req_meta: &str, cred_def_json: &str) -> VcxResult<String> {
    let credential_json = if let Attachment::JSON(json) = &credential.credentials_attach {
        json.get_data()?
    } else {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Wrong messages"));
    };
    let rev_reg_id = _parse_rev_reg_id_from_credential(&credential_json)?;
    let (_, rev_reg_def_json) = anoncreds::get_rev_reg_def_json(&rev_reg_id)?;

    libindy_prover_store_credential(None,
                                    req_meta,
                                    &credential_json,
                                    cred_def_json,
                                    Some(&rev_reg_def_json))
}

fn _make_credential_request(conn_handle: u32, offer: CredentialOffer) -> VcxResult<(CredentialRequest, String, String)> {
    let my_did = get_pw_did(conn_handle)?;
    let cred_offer = if let Attachment::JSON(json) = offer.offers_attach {
        json.get_data()?
    } else {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Wrong messages"));
    };
    let cred_def_id = _parse_cred_def_from_cred_offer(&cred_offer)?;
    let (req, req_meta, cred_def_id, cred_def_json) =
        Credential::create_credential_request(&my_did, &cred_offer, &cred_def_id)?;
    Ok((CredentialRequest::create().set_requests_attach(req)?, req_meta, cred_def_json))
}
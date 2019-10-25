use v3::handlers::issuance::states::{HolderState, InitialState};
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::error::ProblemReport;
use v3::messages::attachment::Attachment;
use credential::Credential;
use utils::error::Error;
use error::{VcxError, VcxErrorKind, VcxResult};
use v3::handlers::connection::send_message;
use v3::messages::A2AMessage;

pub struct HolderSM {
    state: HolderState
}

impl HolderSM {
    pub fn new(connection_handle: u32) -> Self {
        HolderSM {
            state: HolderState::Initial(InitialState {connection_handle})
        }
    }

    pub fn step(state: HolderState) -> Self {
        HolderSM{state}
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<HolderSM> {
        let HolderSM {state} = self;
        let state = match state {
            HolderState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialOffer(offer) => {
                    let conn_handle = state_data.connection_handle;
                    let offer_accepted = true;
                    let (msg, state) = if offer_accepted {
                        //TODO: get did
                        let my_did = String::new();
                        let cred_offer = if let Attachment::JSON(json) = offer.offers_attach {
                            json.get_data()?
                        } else {
                            panic!("Unexpected attachment type");
                        };
                        let cred_def_id = _parse_cred_def_from_cred_offer(&cred_offer)?;
                        let (req, req_meta, cred_def_id) =
                            Credential::create_credential_request(&my_did, &cred_offer, &cred_def_id)?;
                        let request = CredentialRequest::create()
                            .set_requests_attach(req)?;
                        let msg = A2AMessage::CredentialRequest(request);
                        (msg, HolderState::RequestSent(state_data.into()))
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
                },
                _ => {
                    warn!("Credential Issuance can only start on holder side with Credential Offer");
                    HolderState::Initial(state_data)
                }
            },
            HolderState::RequestSent(state_data) => match cim {
                CredentialIssuanceMessage::Credential(_credential) => {
                    panic!("Accept and send ack or problem report");
                    let ok = true;
                    if ok {
                        panic!("send ack");
                    } else {
                        panic!("send problem report");
                    }
                    HolderState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_report) => {
                    panic!("Finalize state");
                    HolderState::Finished(state_data.into())
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
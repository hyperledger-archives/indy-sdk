use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState};
use v3::handlers::connection::send_message;
use v3::messages::A2AMessage;
use v3::messages::issuance::{
    credential::Credential,
    credential_request::CredentialRequest,
    credential_offer::CredentialOffer
};
use v3::messages::error::ProblemReport;
use v3::messages::attachment::Attachment;
use error::{VcxResult, VcxError, VcxErrorKind};
use utils::libindy::anoncreds::{self, libindy_issuer_create_credential_offer};
use credential_def::{get_rev_reg_id, get_tails_file};

pub struct IssuerSM {
    state: IssuerState,
}

impl IssuerSM {
    pub fn new() -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState {})
        }
    }

    pub fn step(state: IssuerState) -> Self {
        IssuerSM {
            state
        }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<IssuerSM> {
        let IssuerSM { state } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialInit(cred_def_id, credential_json, connection_handle) => {
                    let cred_offer = libindy_issuer_create_credential_offer(&cred_def_id)?;
                    let cred_offer_msg = CredentialOffer::create()
                        .set_offers_attach(&cred_offer)?;
                    let cred_offer_msg = _append_credential_preview(cred_offer_msg, &credential_json)?;
                    let msg = A2AMessage::CredentialOffer(cred_offer_msg);
                    send_message(connection_handle, msg)?;
                    IssuerState::OfferSent((state_data, cred_offer, credential_json).into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            },
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(request, connection_handle, credential_handle) => {
                    let credential_msg = _create_credential(&request, credential_handle, &state_data.offer, &state_data.cred_data);
                    let (msg, state) = match credential_msg {
                        Ok(credential_msg) => {
                            let msg = A2AMessage::Credential(
                                credential_msg
                            );
                            (msg, IssuerState::CredentialSent(state_data.into()))
                        },
                        Err(_err) => {
                            let msg = A2AMessage::CommonProblemReport(
                                ProblemReport::create()
                                    //TODO define some error codes inside RFC and use them here
                                    .set_description(0)
                            );
                            (msg, IssuerState::Finished(state_data.into()))
                        }
                    };
                    send_message(connection_handle, msg)?;
                    state
                }
                CredentialIssuanceMessage::CredentialProposal(_proposal, connection_handle) => {
                    let msg = A2AMessage::CommonProblemReport(
                        ProblemReport::create()
                            //TODO define some error codes inside RFC and use them here
                            .set_description(0)
                    );
                    send_message(connection_handle, msg)?;
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    IssuerState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferSent(state_data)
                }
            }
            IssuerState::CredentialSent(state_data) => match cim {
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    info!("Interaction closed with failure");
                    IssuerState::Finished(state_data.into())
                }
                CredentialIssuanceMessage::Ack(_ack) => {
                    info!("Interaction closed with success");
                    IssuerState::Finished(state_data.into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Ack and Problem Report");
                    IssuerState::CredentialSent(state_data)
                }
            }
            IssuerState::Finished(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                IssuerState::Finished(state_data)
            }
        };
        Ok(IssuerSM::step(state))
    }
}


fn _append_credential_preview(cred_offer_msg: CredentialOffer, credential_json: &str) -> VcxResult<CredentialOffer> {
    let cred_values: serde_json::Value = serde_json::from_str(credential_json)
        .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Preview Json".to_string()))?;

    let values_map = cred_values.as_object()
        .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Preview Json".to_string()))?;

    let mut new_offer = cred_offer_msg;
    for item in values_map.iter() {
        let (key, value) = item;
        new_offer = new_offer.add_credential_preview_data(
            key,
            value.as_str()
                .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Credential Preview Json".to_string()))?,
            "text/plain",
        )?;
    }
    Ok(new_offer)
}

fn _create_credential(request: &CredentialRequest, credential_handle: u32, offer: &str, cred_data: &str) -> VcxResult<Credential> {
    let request = if let Attachment::JSON(json) = &request.requests_attach {
        json.get_data()?
    } else {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Wrong messages"));
    };

    let rev_reg_id = get_rev_reg_id(credential_handle)?;
    let tails_file = get_tails_file(credential_handle)?;
    let (credential, cred_id, revoc_reg_delta) = anoncreds::libindy_issuer_create_credential(offer,
                                                                                             &request,
                                                                                             cred_data,
                                                                                             rev_reg_id,
                                                                                             tails_file)?;
    Credential::create()
        .set_credential(credential)
}
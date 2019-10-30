use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState};
use v3::handlers::connection::{send_message, get_messages, get_pw_did};
use messages::update_message::{UIDsByConn, update_messages};
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
use messages::MessageStatusCode;
use v3::handlers::connection::decode_message;
use messages::thread::Thread;

pub struct IssuerSM {
    state: IssuerState,
}

impl IssuerSM {
    pub fn new(cred_def_id: &str, credential_data: &str, rev_reg_id: Option<String>, tails_file: Option<String>) -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState::new(cred_def_id, credential_data, rev_reg_id, tails_file))
        }
    }

    pub fn step(state: IssuerState) -> Self {
        IssuerSM {
            state
        }
    }

    pub fn fetch_messages(&self) -> VcxResult<Vec<A2AMessage>> {
        let conn_handle = self.state.get_connection_handle();
        let last_id = self.state.get_last_id();
        let (messages, _) = get_messages(conn_handle)?;

        let (uids, msgs): (Vec<String>, Vec<A2AMessage>) = messages.into_iter()
            .filter_map(|message| {
                let a2a_message = decode_message(conn_handle, message).ok()?;
                let thid = match a2a_message {
                    A2AMessage::Ack(ack) => {
                        ack.thread.thid
                    }
                    A2AMessage::CommonProblemReport(report) => {
                        report.thread.thid
                    }
                    A2AMessage::CredentialProposal(proposal) => {
                        match proposal.thread.map(|thread| thread.thid.clone()) {
                            Some(a) => a,
                            None => None
                        }
                    }
                    A2AMessage::CredentialRequest(request) => {
                        request.thread.thid
                    }
                    _ => None
                };
                if thid == last_id {
                    Some((thid, a2a_message))
                } else {
                    None
                }
            })
            .unzip();

        let messages_to_update = vec![UIDsByConn {
            pairwise_did: get_pw_did(conn_handle)?,
            uids
        }];

        update_messages(MessageStatusCode::Reviewed, messages_to_update)?;

        Ok(msgs)
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<IssuerSM> {
        let IssuerSM { state } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialInit(connection_handle) => {
                    let cred_offer = libindy_issuer_create_credential_offer(&state_data.cred_def_id)?;
                    let cred_offer_msg = CredentialOffer::create()
                        .set_offers_attach(&cred_offer)?;
                    let cred_offer_msg = _append_credential_preview(cred_offer_msg, &state_data.credential_json)?;
                    let msg = A2AMessage::CredentialOffer(cred_offer_msg);
                    send_message(connection_handle, msg)?;
                    IssuerState::OfferSent((state_data, cred_offer, cred_offer_msg.id).into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            },
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(request, connection_handle) => {
                    let credential_msg = _create_credential(&request, &state_data.rev_reg_id, &state_data.tails_file, &state_data.offer, &state_data.cred_data);
                    let (msg, state) = match credential_msg {
                        Ok(credential_msg) => {
                            let msg = A2AMessage::Credential(
                                credential_msg
                            );
                            (msg, IssuerState::CredentialSent((state_data, credential_msg.id).into()))
                        },
                        Err(_err) => {
                            let msg = A2AMessage::CommonProblemReport(
                                ProblemReport::create()
                                    //TODO define some error codes inside RFC and use them here
                                    .set_description(0)
                                    .set_thread(Thread::new().set_thid(request.id.0))
                            );
                            (msg, IssuerState::Finished(state_data.into()))
                        }
                    };
                    send_message(connection_handle, msg)?;
                    state
                }
                CredentialIssuanceMessage::CredentialProposal(proposal, connection_handle) => {
                    let msg = A2AMessage::CommonProblemReport(
                        ProblemReport::create()
                            //TODO define some error codes inside RFC and use them here
                            .set_description(0)
                            .set_thread(Thread::new().set_thid(proposal.id.0))
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

fn _create_credential(request: &CredentialRequest, rev_reg_id: &Option<String>, tails_file: &Option<String>, offer: &str, cred_data: &str) -> VcxResult<Credential> {
    let request = if let Attachment::JSON(json) = &request.requests_attach {
        json.get_data()?
    } else {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidMessages, "Wrong messages"));
    };

    let (credential, cred_id, revoc_reg_delta) = anoncreds::libindy_issuer_create_credential(offer,
                                                                                             &request,
                                                                                             cred_data,
                                                                                             rev_reg_id.clone(),
                                                                                             tails_file.clone())?;
    Credential::create()
        .set_credential(credential)
}
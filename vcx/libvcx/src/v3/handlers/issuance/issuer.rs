use api::VcxStateType;
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState};
use v3::handlers::connection::{send_message, get_messages};
use v3::handlers::connection::update_message_status;
use v3::messages::A2AMessage;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential::Credential;
use v3::messages::error::ProblemReport;
use v3::messages::mime_type::MimeType;
use error::{VcxResult, VcxError, VcxErrorKind};
use utils::libindy::anoncreds::{self, libindy_issuer_create_credential_offer};
use messages::thread::Thread;
use issuer_credential::encode_attributes;
use v3::messages::status::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuerSM {
    state: IssuerState,
    source_id: String
}

impl IssuerSM {
    pub fn new(cred_def_id: &str, credential_data: &str, rev_reg_id: Option<String>, tails_file: Option<String>, source_id: String) -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState::new(cred_def_id, credential_data, rev_reg_id, tails_file)),
            source_id
        }
    }

    pub fn get_source_id(&self) -> String {
        self.source_id.clone()
    }

    pub fn step(state: IssuerState, source_id: String) -> Self {
        IssuerSM {
            state,
            source_id
        }
    }

    pub fn get_connection_handle(&self) -> u32 {
        self.state.get_connection_handle()
    }

    pub fn fetch_messages(&self) -> VcxResult<Option<A2AMessage>> {
        if self.is_terminal_state() { return Ok(None); }

        let conn_handle = self.state.get_connection_handle();
        let thread_id = self.state.get_thread_id();
        let messages = get_messages(conn_handle)?;

        let res: Option<(String, A2AMessage)> = messages.into_iter()
            .filter_map(|(uid, a2a_message)| {
                let thid = match &a2a_message {
                    A2AMessage::Ack(ref ack) => {
                        ack.thread.thid.clone()
                    }
                    A2AMessage::CommonProblemReport(ref report) => {
                        report.thread.thid.clone()
                    }
                    A2AMessage::CredentialProposal(ref proposal) => {
                        match proposal.thread.as_ref().map(|thread| thread.thid.clone()) {
                            Some(a) => a,
                            None => None
                        }.clone()
                    }
                    A2AMessage::CredentialRequest(ref request) => {
                        request.thread.thid.clone()
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

    pub fn get_state(&self) -> VcxStateType {
        match self.state {
            IssuerState::Initial(_) => VcxStateType::VcxStateInitialized,
            IssuerState::OfferSent(_) => VcxStateType::VcxStateOfferSent,
            IssuerState::RequestReceived(_) => VcxStateType::VcxStateRequestReceived,
            IssuerState::CredentialSent(_) => VcxStateType::VcxStateAccepted,
            IssuerState::Finished(_) => VcxStateType::VcxStateAccepted,
        }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<IssuerSM> {
        let IssuerSM { state, source_id } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::CredentialInit(connection_handle) => {
                    let cred_offer = libindy_issuer_create_credential_offer(&state_data.cred_def_id)?;
                    let cred_offer_msg = CredentialOffer::create()
                        .set_offers_attach(&cred_offer)?;
                    let cred_offer_msg = _append_credential_preview(cred_offer_msg, &state_data.credential_json)?;
                    send_message(connection_handle, cred_offer_msg.to_a2a_message())?;
                    IssuerState::OfferSent((state_data, cred_offer, connection_handle, cred_offer_msg.id).into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            }
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(request, connection_handle) => {
                    IssuerState::RequestReceived((state_data, request).into())
                }
                CredentialIssuanceMessage::CredentialProposal(proposal, connection_handle) => {
                    let problem_report = ProblemReport::create()
                        .set_comment(String::from("CredentialProposal is not supported"))
                        .set_thread(Thread::new().set_thid(state_data.thread_id.clone()));

                    send_message(connection_handle, problem_report.to_a2a_message())?;
                    IssuerState::Finished((state_data, problem_report).into())
                }
                CredentialIssuanceMessage::ProblemReport(problem_report) => {
                    IssuerState::Finished((state_data, problem_report).into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferSent(state_data)
                }
            },
            IssuerState::RequestReceived(state_data) => match cim {
                CredentialIssuanceMessage::CredentialSend() => {
                    let credential_msg = _create_credential(&state_data.request, &state_data.rev_reg_id, &state_data.tails_file, &state_data.offer, &state_data.cred_data);
                    let conn_handle = state_data.connection_handle;
                    let thread = state_data.request.thread.clone();
                    match credential_msg {
                        Ok(credential_msg) => {
                            let credential_msg = credential_msg.set_thread(thread);
                            send_message(conn_handle, credential_msg.to_a2a_message())?;
                            IssuerState::Finished(state_data.into())
                        }
                        Err(err) => {
                            let problem_report = ProblemReport::create()
                                .set_comment(err.to_string())
                                .set_thread(thread);

                            send_message(conn_handle, problem_report.to_a2a_message())?;
                            IssuerState::Finished((state_data, problem_report).into())
                        }
                    }
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only CredentialSend");
                    IssuerState::RequestReceived(state_data)
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
        Ok(IssuerSM::step(state, source_id))
    }

    pub fn credential_status(&self) -> u32 {
        match self.state {
            IssuerState::Finished(ref state) => state.status.code(),
            _ => Status::Undefined.code()
        }
    }

    pub fn is_terminal_state(&self) -> bool {
        match self.state {
            IssuerState::Finished(_) => true,
            _ => false
        }
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
            MimeType::Plain,
        )?;
    }
    Ok(new_offer)
}

fn _create_credential(request: &CredentialRequest, rev_reg_id: &Option<String>, tails_file: &Option<String>, offer: &str, cred_data: &str) -> VcxResult<Credential> {
    let request = &request.requests_attach.content()?;

    let cred_data = encode_attributes(cred_data)?;

    let (credential, cred_id, revoc_reg_delta) = anoncreds::libindy_issuer_create_credential(offer,
                                                                                             &request,
                                                                                             &cred_data,
                                                                                             rev_reg_id.clone(),
                                                                                             tails_file.clone())?;
    Credential::create()
        .set_credential(credential)
}
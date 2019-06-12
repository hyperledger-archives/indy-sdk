use serde_json;

use std::collections::HashMap;
use api::VcxStateType;
use messages;
use settings;
use messages::{RemoteMessageType, MessageStatusCode, GeneralMessage, ObjectWithVersion};
use messages::payload::{Payloads, PayloadKinds, Thread};
use connection;
use credential_request::CredentialRequest;
use utils::error;
use utils::libindy::{payments, anoncreds};
use utils::constants::{CRED_MSG, DEFAULT_SERIALIZE_VERSION};
use utils::openssl::encode;
use utils::libindy::payments::PaymentTxn;
use object_cache::ObjectCache;
use error::prelude::*;
use messages::get_message::Message;

lazy_static! {
    static ref ISSUER_CREDENTIAL_MAP: ObjectCache < IssuerCredential > = Default::default();
}

static CREDENTIAL_OFFER_ID_KEY: &str = "claim_offer_id";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IssuerCredential {
    source_id: String,
    credential_attributes: String,
    msg_uid: String,
    schema_seq_no: u32,
    issuer_did: String,
    state: VcxStateType,
    pub credential_request: Option<CredentialRequest>,
    pub credential_offer: Option<CredentialOffer>,
    credential_name: String,
    pub credential_id: String,
    pub cred_def_id: String,
    pub cred_def_handle: u32,
    ref_msg_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev_reg_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tails_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev_reg_def_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cred_rev_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev_cred_payment_txn: Option<PaymentTxn>,
    price: u64,
    payment_address: Option<String>,
    // the following 6 are pulled from the connection object
    agent_did: String,
    //agent_did for this relationship
    agent_vk: String,
    issued_did: String,
    //my_pw_did for this relationship
    issued_vk: String,
    remote_did: String,
    //their_pw_did for this relationship
    remote_vk: String,
    thread: Option<Thread>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CredentialOffer {
    pub msg_type: String,
    pub version: String,
    //vcx version of cred_offer
    pub to_did: String,
    //their_pw_did for this relationship
    pub from_did: String,
    //my_pw_did for this relationship
    pub libindy_offer: String,
    pub cred_def_id: String,
    pub credential_attrs: serde_json::Map<String, serde_json::Value>,
    //promised attributes revealed in credential
    pub schema_seq_no: u32,
    pub claim_name: String,
    pub claim_id: String,
    pub msg_ref_id: Option<String>,
    pub thread_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CredentialMessage {
    pub libindy_cred: String,
    pub rev_reg_def_json: String,
    pub cred_def_id: String,
    pub msg_type: String,
    pub claim_offer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_revoc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoc_reg_delta_json: Option<String>,
    pub version: String,
    pub from_did: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct PaymentInfo {
    pub payment_required: String,
    pub payment_addr: String,
    pub price: u64,
}

impl PaymentInfo {
    pub fn get_address(&self) -> String {
        self.payment_addr.to_string()
    }

    pub fn get_price(&self) -> u64 {
        self.price
    }

    pub fn to_string(&self) -> VcxResult<String> {
        serde_json::to_string(&self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize payment info")))
    }
}

impl IssuerCredential {
    fn validate_credential_offer(&self) -> VcxResult<u32> {
        //TODO: validate credential_attributes against credential_def
        debug!("successfully validated issuer_credential {}", self.source_id);
        Ok(error::SUCCESS.code_num)
    }

    fn send_credential_offer(&mut self, connection_handle: u32) -> VcxResult<u32> {
        trace!("IssuerCredential::send_credential_offer >>> connection_handle: {}", connection_handle);

        debug!("sending credential offer for issuer_credential {} to connection {}", self.source_id, connection::get_source_id(connection_handle).unwrap_or_default());
        if self.state != VcxStateType::VcxStateInitialized {
            warn!("credential {} has invalid state {} for sending credentialOffer", self.source_id, self.state as u32);
            return Err(VcxError::from_msg(VcxErrorKind::NotReady, format!("credential {} has invalid state {} for sending credentialOffer", self.source_id, self.state as u32)));
        }

        if !connection::is_valid_handle(connection_handle) {
            warn!("invalid connection handle ({})", connection_handle);
            return Err(VcxError::from_msg(VcxErrorKind::InvalidConnectionHandle, format!("invalid connection handle ({})", connection_handle)));
        }

        self.agent_did = connection::get_agent_did(connection_handle)?;
        self.agent_vk = connection::get_agent_verkey(connection_handle)?;
        self.issued_did = connection::get_pw_did(connection_handle)?;
        self.issued_vk = connection::get_pw_verkey(connection_handle)?;
        self.remote_vk = connection::get_their_pw_verkey(connection_handle)?;

        let payment = self.generate_payment_info()?;
        let credential_offer = self.generate_credential_offer(&self.issued_did)?;
        let cred_json = json!(credential_offer);
        let mut payload = Vec::new();

        let connection_name = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME)?;

        let title = if let Some(x) = payment {
            payload.push(json!(x));
            format!("{} wants you to pay tokens for: {}", connection_name, self.credential_name)
        } else {
            format!("{} is offering you a credential: {}", connection_name, self.credential_name)
        };

        payload.push(cred_json);

        let payload = serde_json::to_string(&payload)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize payload: {}", err)))?;

        debug!("credential offer data: {}", secret!(&payload));

        let response =
            messages::send_message()
                .to(&self.issued_did)?
                .to_vk(&self.issued_vk)?
                .msg_type(&RemoteMessageType::CredOffer)?
                .edge_agent_payload(&self.issued_vk, &self.remote_vk, &payload, PayloadKinds::CredOffer, self.thread.clone())?
                .agent_did(&self.agent_did)?
                .agent_vk(&self.agent_vk)?
                .set_title(&title)?
                .set_detail(&title)?
                .status_code(&MessageStatusCode::Accepted)?
                .send_secure()
                .map_err(|err| err.extend("could not send credential offer"))?;

        self.msg_uid = response.get_msg_uid()?;
        self.state = VcxStateType::VcxStateOfferSent;
        self.credential_offer = Some(credential_offer);

        debug!("sent credential offer for: {}", self.source_id);
        return Ok(error::SUCCESS.code_num);
    }

    fn send_credential(&mut self, connection_handle: u32) -> VcxResult<u32> {
        trace!("IssuerCredential::send_credential >>> connection_handle: {}", connection_handle);

        debug!("sending credential for issuer_credential {} to connection {}", self.source_id, connection::get_source_id(connection_handle).unwrap_or_default());
        if self.state != VcxStateType::VcxStateRequestReceived {
            warn!("credential {} has invalid state {} for sending credential", self.source_id, self.state as u32);
            return Err(VcxError::from_msg(VcxErrorKind::NotReady, format!("credential {} has invalid state {} for sending credential", self.source_id, self.state as u32)));
        }

        if connection::is_valid_handle(connection_handle) == false {
            warn!("invalid connection handle ({}) in send_credential_offer", connection_handle);
            return Err(VcxError::from_msg(VcxErrorKind::InvalidCredentialHandle, format!("invalid connection handle ({}) in send_credential_offer", connection_handle)));
        }

        self.verify_payment()?;

        let to = connection::get_pw_did(connection_handle)?;
        let attrs_with_encodings = self.create_attributes_encodings()?;

        let data = if settings::test_indy_mode_enabled() {
            CRED_MSG.to_string()
        } else {
            let cred = self.generate_credential(&attrs_with_encodings, &to)?;
            serde_json::to_string(&cred)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot serialize credential: {}", err)))?
        };

        debug!("credential data: {}", secret!(&data));

        let cred_req_msg_id = self.credential_request
            .as_ref()
            .and_then(|cred_req| cred_req.msg_ref_id.as_ref())
            .ok_or(VcxError::from(VcxErrorKind::InvalidCredentialRequest))?;

        self.thread.as_mut().map(|thread| thread.sender_order += 1);

        let response = messages::send_message()
            .to(&self.issued_did)?
            .to_vk(&self.issued_vk)?
            .msg_type(&RemoteMessageType::Cred)?
            .status_code(&MessageStatusCode::Accepted)?
            .edge_agent_payload(&self.issued_vk, &self.remote_vk, &data, PayloadKinds::Cred, self.thread.clone())?
            .agent_did(&self.agent_did)?
            .agent_vk(&self.agent_vk)?
            .ref_msg_id(Some(cred_req_msg_id.to_string()))?
            .send_secure()
            .map_err(|err| err.extend("could not send credential offer"))?;

        self.msg_uid = response.get_msg_uid()?;
        self.state = VcxStateType::VcxStateAccepted;

        debug!("issued credential: {}", self.source_id);
        return Ok(error::SUCCESS.code_num);
    }

    pub fn create_attributes_encodings(&self) -> VcxResult<String> {
        encode_attributes(&self.credential_attributes)
    }

    // TODO: The error arm of this Result is never used in any calling functions.
    // So currently there is no way to test the error status.
    fn get_credential_offer_status(&mut self, message: Option<Message>) -> VcxResult<u32> {
        debug!("updating state for credential offer: {} msg_uid: {:?}", self.source_id, self.msg_uid);
        if self.state == VcxStateType::VcxStateRequestReceived {
            return Ok(self.get_state());
        }
        if self.state != VcxStateType::VcxStateOfferSent || self.msg_uid.is_empty() || self.issued_did.is_empty() {
            return Ok(self.get_state());
        }

        let (offer_uid, payload) = match message {
            None => messages::get_message::get_ref_msg(&self.msg_uid,
                                               &self.issued_did,
                                               &self.issued_vk,
                                               &self.agent_did,
                                               &self.agent_vk)?,
            Some(ref message) if (message.payload.is_some()) => (message.uid.clone(), message.payload.clone().unwrap()),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Cannot find referent message")),
        };

        let (payload, thread) = Payloads::decrypt(&self.issued_vk, &payload)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::Common(err.into()), "Cannot decrypt CredentialOffer payload"))?;

        let mut cred_req: CredentialRequest = serde_json::from_str(&payload)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize CredentialRequest: {}", err)))?;

        cred_req.msg_ref_id = Some(offer_uid);

        if let Some(tr) = thread {
            let remote_did = self.remote_did.as_str();
            self.thread.as_mut().map(|thread| thread.increment_receiver(&remote_did));
        }

        self.credential_request = Some(cred_req);
        debug!("received credential request for credential offer: {}", self.source_id);
        self.state = VcxStateType::VcxStateRequestReceived;
        Ok(self.get_state())
    }

    fn update_state(&mut self, message: Option<Message>) -> VcxResult<u32> {
        trace!("IssuerCredential::update_state >>>");
        self.get_credential_offer_status(message)
        //There will probably be more things here once we do other things with the credential
    }

    fn get_state(&self) -> u32 {
        trace!("IssuerCredential::get_state >>>");
        let state = self.state as u32;
        state
    }
    fn get_offer_uid(&self) -> &String { &self.msg_uid }
    fn set_offer_uid(&mut self, uid: &str) { self.msg_uid = uid.to_owned(); }

    fn get_credential_attributes(&self) -> &String { &self.credential_attributes }
    fn get_source_id(&self) -> &String { &self.source_id }

    fn generate_credential(&mut self, credential_data: &str, did: &str) -> VcxResult<CredentialMessage> {
        let indy_cred_offer = self.credential_offer
            .as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredential, "Invalid Credential: `credential_offer` field not found"))?;

        let indy_cred_req = self.credential_request
            .as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredentialRequest, "Invalid Credential: `credential_request` field not found"))?;

        let (cred, cred_revoc_id, revoc_reg_delta_json) =
            anoncreds::libindy_issuer_create_credential(&indy_cred_offer.libindy_offer,
                                                        &indy_cred_req.libindy_cred_req,
                                                        &credential_data,
                                                        self.rev_reg_id.clone(),
                                                        self.tails_file.clone())?;

        self.cred_rev_id = cred_revoc_id.clone();

        Ok(CredentialMessage {
            claim_offer_id: self.msg_uid.clone(),
            from_did: String::from(did),
            version: String::from("0.1"),
            msg_type: PayloadKinds::Cred.name().to_string(),
            libindy_cred: cred,
            rev_reg_def_json: self.rev_reg_def_json.clone().unwrap_or(String::new()),
            cred_def_id: self.cred_def_id.clone(),
            cred_revoc_id,
            revoc_reg_delta_json,
        })
    }

    fn generate_credential_offer(&self, to_did: &str) -> VcxResult<CredentialOffer> {
        let attr_map = convert_to_map(&self.credential_attributes)?;
        //Todo: make a cred_def_offer error
        let libindy_offer = anoncreds::libindy_issuer_create_credential_offer(&self.cred_def_id)?;

        Ok(CredentialOffer {
            msg_type: PayloadKinds::CredOffer.name().to_string(),
            version: String::from("0.1"),
            to_did: to_did.to_string(),
            from_did: self.issued_did.clone(),
            credential_attrs: attr_map,
            schema_seq_no: self.schema_seq_no.clone(),
            claim_name: String::from(self.credential_name.clone()),
            claim_id: String::from(self.credential_id.clone()),
            msg_ref_id: None,
            cred_def_id: self.cred_def_id.clone(),
            libindy_offer,
            thread_id: None,
        })
    }

    fn revoke_cred(&mut self) -> VcxResult<()> {
        let tails_file = self.tails_file
            .as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidRevocationDetails, "Invalid RevocationInfo: `tails_file` field not found"))?;

        let rev_reg_id = self.rev_reg_id
            .as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidRevocationDetails, "Invalid RevocationInfo: `rev_reg_id` field not found"))?;

        let cred_rev_id = self.cred_rev_id
            .as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidRevocationDetails, "Invalid RevocationInfo: `cred_rev_id` field not found"))?;

        let (payment, _) = anoncreds::revoke_credential(tails_file, rev_reg_id, cred_rev_id)?;

        self.rev_cred_payment_txn = payment;
        Ok(())
    }

    fn generate_payment_info(&mut self) -> VcxResult<Option<PaymentInfo>> {
        if self.price > 0 {
            let address: String = ::utils::libindy::payments::create_address(None)?;
            self.payment_address = Some(address.clone());
            Ok(Some(PaymentInfo {
                payment_required: "one-time".to_string(),
                payment_addr: address,
                price: self.price,
            }))
        } else {
            Ok(None)
        }
    }

    fn verify_payment(&mut self) -> VcxResult<()> {
        if self.price > 0 {
            let invoice_address = self.payment_address.as_ref()
                .ok_or(VcxError::from(VcxErrorKind::InvalidPaymentAddress))?;

            let address = payments::get_address_info(&invoice_address)?;

            if address.balance < self.price { return Err(VcxError::from(VcxErrorKind::InsufficientTokenAmount)); }
        }
        Ok(())
    }

    fn get_payment_txn(&self) -> VcxResult<PaymentTxn> {
        trace!("IssuerCredential::get_payment_txn >>>");

        match self.payment_address {
            Some(ref payment_address) if self.price > 0 => {
                Ok(PaymentTxn {
                    amount: self.price,
                    credit: true,
                    inputs: vec![payment_address.to_string()],
                    outputs: Vec::new(),
                })
            }
            _ => Err(VcxError::from(VcxErrorKind::NoPaymentInformation))
        }
    }

    pub fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize credential"))
    }

    fn from_str(data: &str) -> VcxResult<IssuerCredential> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<IssuerCredential>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize IssuerCredential"))
    }
}

/**
    Input: supporting two formats:
    eg:
    perferred format: json, property/values
    {"address2":"101 Wilson Lane"}
    or
    deprecated format: json, key/array (of one item)
    {"address2":["101 Wilson Lane"]}


    Output: json: dictionary with key, object of raw and encoded values
    eg:
    {
      "address2": {
        "encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
        "raw": "101 Wilson Lane"
      }
    }
*/

pub fn encode_attributes(attributes: &str) -> VcxResult<String> {
    let mut attributes: HashMap<String, serde_json::Value> = serde_json::from_str(attributes)
        .map_err(|err| {
            warn!("Invalid Json for Attribute data");
            VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize credential attributes: {}", err))
        })?;

    let mut dictionary = HashMap::new();

    for (attr, attr_data) in attributes.iter_mut() {
        let first_attr: &str = match attr_data {
            // old style input such as {"address2":["101 Wilson Lane"]}
            serde_json::Value::Array(array_type) => {
                let attrib_value: &str = match array_type.get(0).and_then(serde_json::Value::as_str) {
                    Some(x) => x,
                    None => {
                        warn!("Cannot encode attribute: {}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
                        return Err(VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure, "Attribute value not found"));
                    }
                };

                warn!("Old attribute format detected. See vcx_issuer_create_credential api for additional information.");
                attrib_value
            }

            // new style input such as {"address2":"101 Wilson Lane"}
            serde_json::Value::String(str_type) => str_type,
            // anything else is an error
            _ => {
                warn!("Invalid Json for Attribute data");
                return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "Invalid Json for Attribute data"));
            }
        };

        let encoded = encode(&first_attr)?;
        let attrib_values = json!({
            "raw": first_attr,
            "encoded": encoded
        });

        dictionary.insert(attr, attrib_values);
    }

    serde_json::to_string_pretty(&dictionary)
        .map_err(|err| {
            warn!("Invalid Json for Attribute data");
            VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Invalid Json for Attribute data: {}", err))
        })
}

pub fn get_encoded_attributes(handle: u32) -> VcxResult<String> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        i.create_attributes_encodings()
    })
}

pub fn get_offer_uid(handle: u32) -> VcxResult<String> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        Ok(i.get_offer_uid().to_string())
    })
}

pub fn get_payment_txn(handle: u32) -> VcxResult<PaymentTxn> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        i.get_payment_txn()
    })
}

pub fn issuer_credential_create(cred_def_handle: u32,
                                source_id: String,
                                issuer_did: String,
                                credential_name: String,
                                credential_data: String,
                                price: u64) -> VcxResult<u32> {
    trace!("issuer_credential_create >>> cred_def_handle: {}, source_id: {}, issuer_did: {}, credential_name: {}, credential_data: {}, price: {}",
           cred_def_handle, source_id, issuer_did, credential_name, secret!(&credential_data), price);

    let cred_def_id = ::credential_def::get_cred_def_id(cred_def_handle)?;
    let rev_reg_id = ::credential_def::get_rev_reg_id(cred_def_handle)?;
    let tails_file = ::credential_def::get_tails_file(cred_def_handle)?;
    let rev_reg_def_json = ::credential_def::get_rev_reg_def(cred_def_handle)?;

    let mut new_issuer_credential = IssuerCredential {
        credential_id: source_id.to_string(),
        source_id,
        msg_uid: String::new(),
        credential_attributes: credential_data,
        issuer_did,
        state: VcxStateType::VcxStateNone,
        //Todo: Take out schema
        schema_seq_no: 0,
        credential_request: None,
        credential_offer: None,
        credential_name,
        ref_msg_id: None,
        rev_reg_id,
        rev_reg_def_json,
        cred_rev_id: None,
        rev_cred_payment_txn: None,
        tails_file,
        price,
        payment_address: None,
        issued_did: String::new(),
        issued_vk: String::new(),
        remote_did: String::new(),
        remote_vk: String::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
        cred_def_id,
        cred_def_handle,
        thread: Some(Thread::new()),
    };

    new_issuer_credential.validate_credential_offer()?;

    new_issuer_credential.state = VcxStateType::VcxStateInitialized;

    let handle = ISSUER_CREDENTIAL_MAP.add(new_issuer_credential)?;
    debug!("creating issuer_credential {} with handle {}", get_source_id(handle).unwrap_or_default(), handle);

    Ok(handle)
}

pub fn update_state(handle: u32, message: Option<Message>) -> VcxResult<u32> {
    ISSUER_CREDENTIAL_MAP.get_mut(handle, |i| {
        match i.update_state(message.clone()) {
            Ok(x) => Ok(x),
            Err(x) => Ok(i.get_state()),
        }
    })
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        Ok(i.get_state())
    })
}

pub fn release(handle: u32) -> VcxResult<()> {
    ISSUER_CREDENTIAL_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidIssuerCredentialHandle)))
}

pub fn release_all() {
    ISSUER_CREDENTIAL_MAP.drain().ok();
}

pub fn is_valid_handle(handle: u32) -> bool {
    ISSUER_CREDENTIAL_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        i.to_string()
    })
}

pub fn from_string(credential_data: &str) -> VcxResult<u32> {
    let schema: IssuerCredential = IssuerCredential::from_str(credential_data)?;
    ISSUER_CREDENTIAL_MAP.add(schema)
}

pub fn send_credential_offer(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    ISSUER_CREDENTIAL_MAP.get_mut(handle, |i| {
        i.send_credential_offer(connection_handle)
    })
}

pub fn send_credential(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    ISSUER_CREDENTIAL_MAP.get_mut(handle, |i| {
        i.send_credential(connection_handle)
    })
}

pub fn revoke_credential(handle: u32) -> VcxResult<()> {
    ISSUER_CREDENTIAL_MAP.get_mut(handle, |i| {
        i.revoke_cred()
    })
}

pub fn convert_to_map(s: &str) -> VcxResult<serde_json::Map<String, serde_json::Value>> {
    serde_json::from_str(s)
        .map_err(|err| {
            warn!("{}", error::INVALID_ATTRIBUTES_STRUCTURE.message);
            VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure, error::INVALID_ATTRIBUTES_STRUCTURE.message)
        })
}

pub fn get_credential_attributes(handle: u32) -> VcxResult<String> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        Ok(i.get_credential_attributes().to_string())
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    ISSUER_CREDENTIAL_MAP.get(handle, |i| {
        Ok(i.get_source_id().to_string())
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::Value;
    use settings;
    use connection::tests::build_test_connection;
    use credential_request::CredentialRequest;
    #[allow(unused_imports)]
    use utils::{constants:: *,
                libindy::{ set_libindy_rc,
        anoncreds::{ libindy_create_and_store_credential_def,
        libindy_issuer_create_credential_offer,
        libindy_prover_create_credential_req },
        wallet::get_wallet_handle, wallet},
                get_temp_dir_path,
    };

    static DEFAULT_CREDENTIAL_NAME: &str = "Credential";
    static DEFAULT_CREDENTIAL_ID: &str = "defaultCredentialId";
    static SCHEMA: &str = r#"{{
                            "seqNo":32,
                            "data":{{
                                "name":"gvt",
                                "version":"1.0",
                                "attr_names":["address1","address2","city","state", "zip"]
                            }}
                         }}"#;

    static CREDENTIAL_DATA: &str =
        r#"{"address2":["101 Wilson Lane"],
        "zip":["87121"],
        "state":["UT"],
        "city":["SLC"],
        "address1":["101 Tela Lane"]
        }"#;

    static X_CREDENTIAL_JSON: &str =
        r#"{"claim":{"address1":["101 Tela Lane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101 Wilson Lane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]},"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_seq_no":15,"signature":{"non_revocation_claim":null,"primary_claim":{"a":"","e":"","m2":"","v":""}}}"#;

    pub fn util_put_credential_def_in_issuer_wallet(schema_seq_num: u32, wallet_handle: i32) {
        let stored_xcredential = String::from("");

        let issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let tag = "test_tag";
        let config = "{support_revocation: false}";

        libindy_create_and_store_credential_def(&issuer_did, SCHEMAS_JSON, tag, None, config).unwrap();
    }

    pub fn create_standard_issuer_credential() -> IssuerCredential {
        let credential_req: CredentialRequest = serde_json::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let (credential_offer, _) = ::credential::parse_json_offer(CREDENTIAL_OFFER_JSON).unwrap();
        let issuer_credential = IssuerCredential {
            source_id: "standard_credential".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            credential_attributes: CREDENTIAL_DATA.to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            credential_name: DEFAULT_CREDENTIAL_NAME.to_owned(),
            credential_request: Some(credential_req.to_owned()),
            credential_offer: Some(credential_offer.to_owned()),
            credential_id: String::from(DEFAULT_CREDENTIAL_ID),
            price: 1,
            payment_address: Some(payments::build_test_address("9UFgyjuJxi1i1HD")),
            ref_msg_id: None,
            rev_reg_id: None,
            tails_file: None,
            cred_rev_id: None,
            rev_cred_payment_txn: None,
            rev_reg_def_json: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
            cred_def_id: CRED_DEF_ID.to_string(),
            cred_def_handle: 0,
            thread: Some(Thread::new()),
        };
        issuer_credential
    }

    pub fn create_pending_issuer_credential() -> IssuerCredential {
        let connection_handle = build_test_connection();
        let credential_req: CredentialRequest = serde_json::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let (credential_offer, _) = ::credential::parse_json_offer(CREDENTIAL_OFFER_JSON).unwrap();
        let credential: IssuerCredential = IssuerCredential {
            source_id: "test_has_pending_credential_request".to_owned(),
            schema_seq_no: 32,
            msg_uid: "1234".to_owned(),
            credential_attributes: "nothing".to_owned(),
            issuer_did: "QTrbV4raAcND4DWWzBmdsh".to_owned(),
            issued_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            issued_vk: VERKEY.to_string(),
            state: VcxStateType::VcxStateOfferSent,
            credential_request: Some(credential_req.to_owned()),
            credential_offer: Some(credential_offer.to_owned()),
            credential_name: DEFAULT_CREDENTIAL_NAME.to_owned(),
            credential_id: String::from(DEFAULT_CREDENTIAL_ID),
            cred_def_id: CRED_DEF_ID.to_string(),
            cred_def_handle: 1,
            ref_msg_id: None,
            rev_reg_id: None,
            cred_rev_id: None,
            rev_cred_payment_txn: None,
            rev_reg_def_json: None,
            tails_file: None,
            price: 0,
            payment_address: None,
            remote_did: DID.to_string(),
            remote_vk: VERKEY.to_string(),
            agent_did: DID.to_string(),
            agent_vk: VERKEY.to_string(),
            thread: Some(Thread::new()),
        };
        credential
    }

    fn normalize_credentials(c1: &str, c2: &str) -> (serde_json::Value, serde_json::Value) {
        let mut v1: serde_json::Value = serde_json::from_str(c1.clone()).unwrap();
        let mut v2: serde_json::Value = serde_json::from_str(c2.clone()).unwrap();
        v1["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v1["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["a"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["e"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["v"] = serde_json::to_value("".to_owned()).unwrap();
        v2["signature"]["primary_claim"]["m2"] = serde_json::to_value("".to_owned()).unwrap();
        (v1, v2)
    }

    pub fn create_full_issuer_credential() -> (IssuerCredential, ::credential::Credential) {
        let issuer_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (_, cred_def_handle) = ::credential_def::tests::create_cred_def_real(true);
        let cred_def_id = ::credential_def::get_cred_def_id(cred_def_handle).unwrap();
        let rev_reg_id = ::credential_def::get_rev_reg_id(cred_def_handle).unwrap();
        let tails_file = ::credential_def::get_tails_file(cred_def_handle).unwrap();
        let rev_reg_def_json = ::credential_def::get_rev_reg_def(cred_def_handle).unwrap();
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;

        let mut issuer_credential = IssuerCredential {
            source_id: "source_id".to_string(),
            msg_uid: String::new(),
            credential_attributes: credential_data.to_string(),
            issuer_did,
            state: VcxStateType::VcxStateNone,
            //Todo: Take out schema
            schema_seq_no: 0,
            credential_request: None,
            credential_offer: None,
            credential_name: "cred_name".to_string(),
            credential_id: String::new(),
            ref_msg_id: None,
            rev_reg_id,
            rev_reg_def_json,
            cred_rev_id: None,
            rev_cred_payment_txn: None,
            tails_file,
            price: 1,
            payment_address: None,
            issued_did: String::new(),
            issued_vk: String::new(),
            remote_did: String::new(),
            remote_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            cred_def_id,
            cred_def_handle,
            thread: Some(Thread::new()),
        };

        let payment = issuer_credential.generate_payment_info().unwrap();
        let credential_offer = issuer_credential.generate_credential_offer(&issuer_credential.issued_did).unwrap();
        let cred_json = json!(credential_offer);
        let mut payload = Vec::new();

        if payment.is_some() { payload.push(json!(payment.unwrap())); }
        payload.push(cred_json);
        let payload = serde_json::to_string(&payload).unwrap();

        issuer_credential.credential_offer = Some(issuer_credential.generate_credential_offer(&issuer_credential.issued_did).unwrap());
        let credential = ::credential::tests::create_credential(&payload);
        issuer_credential.credential_request = Some(credential.build_request(&issuer_credential.issuer_did, &issuer_credential.issued_did).unwrap());
        (issuer_credential, credential)
    }

    #[test]
    fn test_issuer_credential_create_succeeds() {
        init!("true");
        match issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                       "1".to_string(),
                                       "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                       "credential_name".to_string(),
                                       "{\"attr\":\"value\"}".to_owned(),
                                       1) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        init!("true");
        let handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                              "1".to_string(),
                                              "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                              "credential_name".to_string(),
                                              "{\"attr\":\"value\"}".to_owned(),
                                              1).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_credential_offer() {
        init!("true");
        let connection_handle = build_test_connection();

        let credential_id = DEFAULT_CREDENTIAL_ID;

        let handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                              "1".to_string(),
                                              "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                              "credential_name".to_string(),
                                              "{\"attr\":\"value\"}".to_owned(),
                                              1).unwrap();

        assert_eq!(send_credential_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_generate_cred_offer() {
        init!("ledger");
        let issuer = create_full_issuer_credential().0.generate_credential_offer(&settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap()).unwrap();
    }

    #[test]
    fn test_retry_send_credential_offer() {
        init!("true");
        let connection_handle = build_test_connection();

        let credential_id = DEFAULT_CREDENTIAL_ID;

        let handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                              "1".to_string(),
                                              "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                              "credential_name".to_string(),
                                              "{\"attr\":\"value\"}".to_owned(),
                                              1).unwrap();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        assert!(send_credential_offer(handle, connection_handle).is_err());
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateInitialized as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "");

        // Can retry after initial failure
        assert_eq!(send_credential_offer(handle, connection_handle).unwrap(), error::SUCCESS.code_num);
        assert_eq!(get_state(handle).unwrap(), VcxStateType::VcxStateOfferSent as u32);
        assert_eq!(get_offer_uid(handle).unwrap(), "ntc2ytb");
    }

    #[test]
    fn test_credential_can_be_resent_after_failure() {
        init!("true");
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, "QTrbV4raAcND4DWWzBmdsh");

        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_test_connection();

        set_libindy_rc(error::TIMEOUT_LIBINDY_ERROR.code_num);
        assert_eq!(credential.send_credential(connection_handle).unwrap_err().kind(), VcxErrorKind::Common(1038));
        assert_eq!(credential.msg_uid, "1234");
        assert_eq!(credential.state, VcxStateType::VcxStateRequestReceived);
        // Retry sending the credential, use the mocked http. Show that you can retry sending the credential
        credential.send_credential(connection_handle).unwrap();
        assert_eq!(credential.msg_uid, "ntc2ytb");
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_from_string_succeeds() {
        init!("true");
        let handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                              "1".to_string(),
                                              "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                              "credential_name".to_string(),
                                              "{\"attr\":\"value\"}".to_owned(),
                                              1, ).unwrap();
        let string = to_string(handle).unwrap();
        let value: serde_json::Value = serde_json::from_str(&string).unwrap();
        assert_eq!(value["version"], "1.0");
        assert!(!string.is_empty());
        assert!(release(handle).is_ok());
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_string, string);
    }

    #[test]
    fn test_update_state_with_pending_credential_request() {
        init!("true");
        let mut credential = create_pending_issuer_credential();

        ::utils::httpclient::set_next_u8_response(CREDENTIAL_REQ_RESPONSE.to_vec());
        ::utils::httpclient::set_next_u8_response(UPDATE_CREDENTIAL_RESPONSE.to_vec());

        credential.update_state(None).unwrap();
        assert_eq!(credential.get_state(), VcxStateType::VcxStateRequestReceived as u32);
    }

    #[test]
    fn test_update_state_with_message() {
        init!("true");
        let mut credential = create_pending_issuer_credential();
        let message: Message = serde_json::from_str(CREDENTIAL_REQ_RESPONSE_STR).unwrap();
        credential.update_state(Some(message)).unwrap();
        assert_eq!(credential.get_state(), VcxStateType::VcxStateRequestReceived as u32);
    }

    #[test]
    fn test_update_state_with_bad_message() {
        init!("true");
        let mut credential = create_pending_issuer_credential();
        let message: Message = serde_json::from_str(INVITE_ACCEPTED_RESPONSE).unwrap();
        let rc = credential.update_state(Some(message));
        assert_eq!(credential.get_state(), VcxStateType::VcxStateOfferSent as u32);
        assert!(rc.is_err());
    }

    #[test]
    fn test_issuer_credential_changes_state_after_being_validated() {
        init!("true");
        let handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                              "1".to_string(),
                                              "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                              "credential_name".to_string(),
                                              "{\"att\":\"value\"}".to_owned(),
                                              1).unwrap();
        let string = to_string(handle).unwrap();
        fn get_state_from_string(s: String) -> u32 {
            let json: serde_json::Value = serde_json::from_str(&s).unwrap();
            if json["data"]["state"].is_number() {
                return json["data"]["state"].as_u64().unwrap() as u32;
            }
            0
        }
        assert_eq!(get_state_from_string(string), 1);
    }

    #[test]
    fn basic_add_attribute_encoding() {
        // FIXME Make this a real test and add additional test for create_attributes_encodings
        let issuer_credential = create_standard_issuer_credential();
        issuer_credential.create_attributes_encodings().unwrap();

        let mut issuer_credential = create_standard_issuer_credential();
        match issuer_credential.credential_attributes.pop() {
            Some(brace) => assert_eq!(brace, '}'),
            None => error!("Malformed credential attributes in the issuer credential test"),
        }
        match issuer_credential.create_attributes_encodings() {
            Ok(_) => {
                error!("basic_add_attribute_encoding test should raise error.");
                assert_ne!(1, 1);
            }
            Err(e) => assert_eq!(e.kind(), VcxErrorKind::InvalidJson)
        }
    }

    #[test]
    fn test_that_test_mode_enabled_bypasses_libindy_create_credential() {
        init!("true");
        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;

        let connection_handle = build_test_connection();

        credential.send_credential(connection_handle).unwrap();
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let h1 = issuer_credential_create(::credential_def::tests::create_cred_def_fake(), "1".to_string(), "8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "credential_name".to_string(), "{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let h2 = issuer_credential_create(::credential_def::tests::create_cred_def_fake(), "1".to_string(), "8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "credential_name".to_string(), "{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let h3 = issuer_credential_create(::credential_def::tests::create_cred_def_fake(), "1".to_string(), "8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "credential_name".to_string(), "{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let h4 = issuer_credential_create(::credential_def::tests::create_cred_def_fake(), "1".to_string(), "8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "credential_name".to_string(), "{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        let h5 = issuer_credential_create(::credential_def::tests::create_cred_def_fake(), "1".to_string(), "8XFh8yBzrpJQmNyZzgoTqB".to_owned(), "credential_name".to_string(), "{\"attr\":\"value\"}".to_owned(), 1).unwrap();
        release_all();
        assert_eq!(release(h1).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
        assert_eq!(release(h2).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
        assert_eq!(release(h3).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
        assert_eq!(release(h4).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
        assert_eq!(release(h5).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
    }

    #[test]
    fn test_errors() {
        init!("false");
        let invalid_handle = 478620;
        assert_eq!(to_string(invalid_handle).unwrap_err().kind(), VcxErrorKind::InvalidHandle);
        assert_eq!(release(invalid_handle).unwrap_err().kind(), VcxErrorKind::InvalidIssuerCredentialHandle);
    }

    #[test]
    fn test_encoding() {
        let issuer_credential_handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                                                "IssuerCredentialName".to_string(),
                                                                "000000000000000000000000Issuer02".to_string(),
                                                                "CredentialNameHere".to_string(),
                                                                r#"["name","gpa"]"#.to_string(),
                                                                1).unwrap();
        assert!(self::get_encoded_attributes(issuer_credential_handle).is_err());
        let issuer_credential_handle = issuer_credential_create(::credential_def::tests::create_cred_def_fake(),
                                                                "IssuerCredentialName".to_string(),
                                                                "000000000000000000000000Issuer02".to_string(),
                                                                "CredentialNameHere".to_string(),
                                                                r#"{"name":["frank"],"gpa":["4.0"]}"#.to_string(),
                                                                1).unwrap();

        let encoded_attributes = self::get_encoded_attributes(issuer_credential_handle).unwrap();
    }

    #[test]
    fn test_payment_information() {
        let payment_info = PaymentInfo {
            payment_addr: "OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j".to_string(),
            payment_required: "one-time".to_string(),
            price: 1000,
        };
        let _ = serde_json::to_string(&payment_info).unwrap();
    }

    #[test]
    fn test_verify_payment() {
        init!("true");
        let mut credential = create_standard_issuer_credential();

        // Success
        credential.price = 3;
        credential.payment_address = Some(payments::build_test_address("9UFgyjuJxi1i1HD"));
        assert!(credential.verify_payment().is_ok());

        // Err - Wrong payment amount
        credential.price = 200;
        assert_eq!(credential.verify_payment().unwrap_err().kind(), VcxErrorKind::InsufficientTokenAmount);

        // Err - address not set
        credential.payment_address = None;
        assert_eq!(credential.verify_payment().unwrap_err().kind(), VcxErrorKind::InvalidPaymentAddress);
    }

    #[test]
    fn test_send_credential_with_payments() {
        init!("true");
        let mut credential = create_standard_issuer_credential();
        credential.state = VcxStateType::VcxStateRequestReceived;
        credential.price = 3;
        credential.payment_address = Some(payments::build_test_address("9UFgyjuJxi1i1HD"));

        let connection_handle = build_test_connection();

        // Success
        credential.send_credential(connection_handle).unwrap();
        assert_eq!(credential.msg_uid, "ntc2ytb");
        assert_eq!(credential.state, VcxStateType::VcxStateAccepted);

        // Amount wrong
        credential.state = VcxStateType::VcxStateRequestReceived;
        credential.price = 200;
        assert!(credential.send_credential(connection_handle).is_err());
        let payment = serde_json::to_string(&credential.get_payment_txn().unwrap()).unwrap();
        assert!(payment.len() > 20);
    }

    #[test]
    fn test_revoke_credential() {
        init!("true");
        let mut credential = create_standard_issuer_credential();

        credential.tails_file = Some(get_temp_dir_path(Some(TEST_TAILS_FILE)).to_str().unwrap().to_string());
        credential.cred_rev_id = None;
        credential.rev_reg_id = None;
        assert_eq!(credential.revoke_cred().unwrap_err().kind(), VcxErrorKind::InvalidRevocationDetails);
        credential.tails_file = None;
        credential.cred_rev_id = Some(CRED_REV_ID.to_string());
        credential.rev_reg_id = None;
        assert_eq!(credential.revoke_cred().unwrap_err().kind(), VcxErrorKind::InvalidRevocationDetails);
        credential.tails_file = None;
        credential.cred_rev_id = None;
        credential.rev_reg_id = Some(REV_REG_ID.to_string());
        assert_eq!(credential.revoke_cred().unwrap_err().kind(), VcxErrorKind::InvalidRevocationDetails);

        credential.tails_file = Some(get_temp_dir_path(Some(TEST_TAILS_FILE)).to_str().unwrap().to_string());
        credential.cred_rev_id = Some(CRED_REV_ID.to_string());
        credential.rev_reg_id = Some(REV_REG_ID.to_string());
        credential.rev_cred_payment_txn = None;

        credential.revoke_cred().unwrap();
        assert!(credential.rev_cred_payment_txn.is_some());
    }


    #[test]
    fn test_encode_with_several_attributes_success() {
        //        for reference....expectation is encode_attributes returns this:

        let expected = json!({
"address2": {
"encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
"raw": "101 Wilson Lane"
},
"zip": {
"encoded": "87121",
"raw": "87121"
},
"city": {
"encoded": "101327353979588246869873249766058188995681113722618593621043638294296500696424",
"raw": "SLC"
},
"address1": {
"encoded": "63690509275174663089934667471948380740244018358024875547775652380902762701972",
"raw": "101 Tela Lane"
},
"state": {
"encoded": "93856629670657830351991220989031130499313559332549427637940645777813964461231",
"raw": "UT"
}
});


        static TEST_CREDENTIAL_DATA: &str =
            r#"{"address2":["101 Wilson Lane"],
            "zip":["87121"],
            "state":["UT"],
            "city":["SLC"],
            "address1":["101 Tela Lane"]
            }"#;

        let results_json = encode_attributes(TEST_CREDENTIAL_DATA).unwrap();

        let results: Value = serde_json::from_str(&results_json).unwrap();

        let address2: &Value = &results["address2"];
        assert_eq!(encode("101 Wilson Lane").unwrap(), address2["encoded"]);
        assert_eq!("101 Wilson Lane", address2["raw"]);

        let state: &Value = &results["state"];
        assert_eq!(encode("UT").unwrap(), state["encoded"]);
        assert_eq!("UT", state["raw"]);

        let zip: &Value = &results["zip"];
        assert_eq!("87121", zip["encoded"]);
        assert_eq!("87121", zip["raw"]);
    }

    #[test]
    fn test_encode_with_one_attribute_success() {
        let expected = json!({
"address2": {
"encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
"raw": "101 Wilson Lane"
}
});

        static TEST_CREDENTIAL_DATA: &str =
            r#"{"address2":["101 Wilson Lane"]}"#;

        let expected_json = serde_json::to_string_pretty(&expected).unwrap();

        let results_json = encode_attributes(TEST_CREDENTIAL_DATA).unwrap();

        assert_eq!(expected_json, results_json, "encode_attributes failed to return expected results");
    }

    #[test]
    fn test_encode_with_new_format_several_attributes_success() {
        //        for reference....expectation is encode_attributes returns this:

        let expected = json!({
"address2": {
"encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
"raw": "101 Wilson Lane"
},
"zip": {
"encoded": "87121",
"raw": "87121"
},
"city": {
"encoded": "101327353979588246869873249766058188995681113722618593621043638294296500696424",
"raw": "SLC"
},
"address1": {
"encoded": "63690509275174663089934667471948380740244018358024875547775652380902762701972",
"raw": "101 Tela Lane"
},
"state": {
"encoded": "93856629670657830351991220989031130499313559332549427637940645777813964461231",
"raw": "UT"
}
});


        static TEST_CREDENTIAL_DATA: &str =
            r#"{"address2":"101 Wilson Lane",
            "zip":"87121",
            "state":"UT",
            "city":"SLC",
            "address1":"101 Tela Lane"
            }"#;

        let results_json = encode_attributes(TEST_CREDENTIAL_DATA).unwrap();

        let results: Value = serde_json::from_str(&results_json).unwrap();

        let address2: &Value = &results["address2"];
        assert_eq!(encode("101 Wilson Lane").unwrap(), address2["encoded"]);
        assert_eq!("101 Wilson Lane", address2["raw"]);

        let state: &Value = &results["state"];
        assert_eq!(encode("UT").unwrap(), state["encoded"]);
        assert_eq!("UT", state["raw"]);

        let zip: &Value = &results["zip"];
        assert_eq!("87121", zip["encoded"]);
        assert_eq!("87121", zip["raw"]);
    }

    #[test]
    fn test_encode_with_new_format_one_attribute_success() {
        let expected = json!({
"address2": {
"encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
"raw": "101 Wilson Lane"
}
});

        static TEST_CREDENTIAL_DATA: &str =
            r#"{"address2": "101 Wilson Lane"}"#;

        let expected_json = serde_json::to_string_pretty(&expected).unwrap();

        let results_json = encode_attributes(TEST_CREDENTIAL_DATA).unwrap();

        assert_eq!(expected_json, results_json, "encode_attributes failed to return expected results");
    }

    #[test]
    fn test_encode_with_mixed_format_several_attributes_success() {
        //        for reference....expectation is encode_attributes returns this:

        let expected = json!({
"address2": {
"encoded": "68086943237164982734333428280784300550565381723532936263016368251445461241953",
"raw": "101 Wilson Lane"
},
"zip": {
"encoded": "87121",
"raw": "87121"
},
"city": {
"encoded": "101327353979588246869873249766058188995681113722618593621043638294296500696424",
"raw": "SLC"
},
"address1": {
"encoded": "63690509275174663089934667471948380740244018358024875547775652380902762701972",
"raw": "101 Tela Lane"
},
"state": {
"encoded": "93856629670657830351991220989031130499313559332549427637940645777813964461231",
"raw": "UT"
}
});


        static TEST_CREDENTIAL_DATA: &str =
            r#"{"address2":["101 Wilson Lane"],
            "zip":"87121",
            "state":"UT",
            "city":["SLC"],
            "address1":"101 Tela Lane"
            }"#;

        let results_json = encode_attributes(TEST_CREDENTIAL_DATA).unwrap();

        let results: Value = serde_json::from_str(&results_json).unwrap();
        let address2: &Value = &results["address2"];

        assert_eq!("68086943237164982734333428280784300550565381723532936263016368251445461241953", address2["encoded"]);
        assert_eq!("101 Wilson Lane", address2["raw"]);

        let state: &Value = &results["state"];
        assert_eq!("93856629670657830351991220989031130499313559332549427637940645777813964461231", state["encoded"]);
        assert_eq!("UT", state["raw"]);

        let zip: &Value = &results["zip"];
        assert_eq!("87121", zip["encoded"]);
        assert_eq!("87121", zip["raw"]);
    }

    #[test]
    fn test_encode_bad_format_returns_error()
    {
        static BAD_TEST_CREDENTIAL_DATA: &str =
            r#"{"format doesnt make sense"}"#;

        assert!(encode_attributes(BAD_TEST_CREDENTIAL_DATA).is_err())
    }

    #[test]
    fn test_encode_old_format_empty_array_error()
    {
        static BAD_TEST_CREDENTIAL_DATA: &str =
            r#"{"address2":[]}"#;

        assert!(encode_attributes(BAD_TEST_CREDENTIAL_DATA).is_err())
    }
}

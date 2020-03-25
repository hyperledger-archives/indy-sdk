use utils::libindy::crypto;

use error::prelude::*;
use v3::messages::a2a::A2AMessage;
use v3::messages::connection::did_doc::DidDoc;
use v3::messages::forward::Forward;

#[derive(Debug)]
pub struct EncryptionEnvelope(pub Vec<u8>);

impl EncryptionEnvelope {
    pub fn create(message: &A2AMessage,
                  pw_verkey: Option<&str>,
                  did_doc: &DidDoc) -> VcxResult<EncryptionEnvelope> {
        trace!("EncryptionEnvelope::create >>> message: {:?}, pw_verkey: {:?}, did_doc: {:?}", message, pw_verkey, did_doc);

        if ::settings::indy_mocks_enabled() { return Ok(EncryptionEnvelope(vec![])); }

        EncryptionEnvelope::encrypt_for_pairwise(message, pw_verkey, did_doc)
            .and_then(|message| EncryptionEnvelope::wrap_into_forward_messages(message, did_doc))
            .map(|message| EncryptionEnvelope(message))
    }

    fn encrypt_for_pairwise(message: &A2AMessage,
                            pw_verkey: Option<&str>,
                            did_doc: &DidDoc) -> VcxResult<Vec<u8>> {
        let message = match message {
            A2AMessage::Generic(message_) => message_.to_string(),
            message => json!(message).to_string()
        };

        let receiver_keys = json!(did_doc.recipient_keys()).to_string();

        crypto::pack_message(pw_verkey, &receiver_keys, message.as_bytes())
    }

    fn wrap_into_forward_messages(mut message: Vec<u8>,
                                  did_doc: &DidDoc) -> VcxResult<Vec<u8>> {
        let (recipient_keys, routing_keys) = did_doc.resolve_keys();

        let mut to = recipient_keys.get(0)
            .map(String::from)
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidConnectionHandle, format!("Recipient Key not found in DIDDoc: {:?}", did_doc)))?;

        for routing_key in routing_keys.iter() {
            message = EncryptionEnvelope::wrap_into_forward(message, &to, &routing_key)?;
            to = routing_key.clone();
        }

        Ok(message)
    }

    fn wrap_into_forward(message: Vec<u8>,
                         to: &str,
                         routing_key: &str) -> VcxResult<Vec<u8>> {
        let message = A2AMessage::Forward(Forward::new(to.to_string(), message)?);

        let message = json!(message).to_string();
        let receiver_keys = json!(vec![routing_key]).to_string();

        crypto::pack_message(None, &receiver_keys, message.as_bytes())
    }

    pub fn open(payload: Vec<u8>) -> VcxResult<A2AMessage> {
        let unpacked_msg = crypto::unpack_message(&payload)?;

        let message: ::serde_json::Value = ::serde_json::from_slice(unpacked_msg.as_slice())
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize message: {}", err)))?;

        let message = message["message"].as_str()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, "Cannot find `message` field"))?.to_string();

        let message: A2AMessage = ::serde_json::from_str(&message)
            .map_err(|err| {
                VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize A2A message: {}", err))
            })?;

        Ok(message)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::ack::tests::_ack;
    use v3::messages::connection::did_doc::tests::*;
    use utils::libindy::tests::test_setup;
    use utils::libindy::crypto::create_key;

    fn _setup(){
        ::settings::set_config_value(::settings::CONFIG_ENABLE_TEST_MODE, "false");
    }

    #[test]
    fn test_encryption_envelope_works_for_no_keys() {
        _setup();
        let setup = test_setup::key();

        let message = A2AMessage::Ack(_ack());

        let res = EncryptionEnvelope::create(&message, Some(&setup.key), &DidDoc::default());
        assert_eq!(res.unwrap_err().kind(), VcxErrorKind::InvalidLibindyParam);
    }

    #[test]
    fn test_encryption_envelope_works_for_recipient_only() {
        _setup();
        let setup = test_setup::key();

        let message = A2AMessage::Ack(_ack());

        let envelope = EncryptionEnvelope::create(&message, Some(&setup.key), &_did_doc_4()).unwrap();
        assert_eq!(message, EncryptionEnvelope::open(envelope.0).unwrap());
    }

    #[test]
    fn test_encryption_envelope_works_for_routing_keys() {
        _setup();
        let setup = test_setup::key();
        let key_1 = create_key(None).unwrap();
        let key_2 = create_key(None).unwrap();

        let mut did_doc = DidDoc::default();
        did_doc.set_service_endpoint(_service_endpoint());
        did_doc.set_keys(_recipient_keys(), vec![key_1.clone(), key_2.clone()]);

        let ack = A2AMessage::Ack(_ack());

        let envelope = EncryptionEnvelope::create(&ack, Some(&setup.key), &did_doc).unwrap();

        let message_1 = EncryptionEnvelope::open(envelope.0).unwrap();

        let message_1 = match message_1 {
            A2AMessage::Forward(forward) => {
                assert_eq!(key_1, forward.to);
                serde_json::to_vec(&forward.msg).unwrap()
            },
            _ => return assert!(false)
        };

        let message_2 = EncryptionEnvelope::open(message_1).unwrap();

        let message_2 = match message_2 {
            A2AMessage::Forward(forward) => {
                assert_eq!(_key_1(), forward.to);
                serde_json::to_vec(&forward.msg).unwrap()
            },
            _ => return assert!(false)
        };

        assert_eq!(ack, EncryptionEnvelope::open(message_2).unwrap());
    }
}
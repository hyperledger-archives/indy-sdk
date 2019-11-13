use utils::libindy::crypto;

use error::prelude::*;
use v3::messages::a2a::A2AMessage;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::forward::Forward;

#[derive(Debug)]
pub struct EncryptionEnvelope(pub Vec<u8>);

impl EncryptionEnvelope {
    pub fn create(message: &A2AMessage,
                  pw_verkey: &str,
                  remote_connection_info: &RemoteConnectionInfo) -> VcxResult<EncryptionEnvelope> {
        EncryptionEnvelope::encrypt_for_pairwise(message, pw_verkey, remote_connection_info)
            .and_then(|message| EncryptionEnvelope::wrap_into_forward_messages(message, remote_connection_info))
            .map(|message| EncryptionEnvelope(message))
    }

    fn encrypt_for_pairwise(message: &A2AMessage,
                            pw_verkey: &str,
                            remote_connection_info: &RemoteConnectionInfo) -> VcxResult<Vec<u8>> {
        let message = match message {
            A2AMessage::Generic(message_) => message_.to_string(),
            message => json!(message).to_string()
        };

        let receiver_keys = json!(remote_connection_info.recipient_keys).to_string();

        crypto::pack_message(Some(&pw_verkey), &receiver_keys, message.as_bytes())
    }

    fn wrap_into_forward_messages(mut message: Vec<u8>,
                                  remote_connection_info: &RemoteConnectionInfo) -> VcxResult<Vec<u8>> {
        let mut to = remote_connection_info.recipient_keys.get(0)
            .map(String::from)
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Recipient Key not found"))?;

        for routing_key in remote_connection_info.routing_keys.iter() {
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

    pub fn open(my_vk: &str, payload: Vec<u8>) -> VcxResult<A2AMessage> {
        let unpacked_msg = crypto::unpack_message(&payload)?;

        let message: ::serde_json::Value = ::serde_json::from_slice(unpacked_msg.as_slice())
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize message: {}", err)))?;

        let message = message["message"].as_str()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, "Cannot find `message` field"))?.to_string();

        let message: A2AMessage = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize message: {}", err)))
            .unwrap_or_else(|_|A2AMessage::Generic(message));

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

    #[test]
    fn test_encryption_envelope_works_for_no_keys() {
        let setup = test_setup::key();

        let info = RemoteConnectionInfo {
            label: _label(),
            recipient_keys: vec![],
            routing_keys: vec![],
            service_endpoint: _service_endpoint(),
        };

        let message = A2AMessage::Ack(_ack());

        let res = EncryptionEnvelope::create(&message, &setup.key, &info);
        assert_eq!(res.unwrap_err().kind(), VcxErrorKind::InvalidLibindyParam);
    }

    #[test]
    fn test_encryption_envelope_works_for_recipient_only() {
        let setup = test_setup::key();

        let info = RemoteConnectionInfo {
            label: _label(),
            recipient_keys: _recipient_keys(),
            routing_keys: vec![],
            service_endpoint: _service_endpoint(),
        };

        let message = A2AMessage::Ack(_ack());

        let envelope = EncryptionEnvelope::create(&message, &setup.key, &info).unwrap();
        assert_eq!(message, EncryptionEnvelope::open(&_key_1(), envelope.0).unwrap());
    }

    #[test]
    fn test_encryption_envelope_works_for_routing_keys() {
        let setup = test_setup::key();
        let key_1 = create_key().unwrap();
        let key_2 = create_key().unwrap();

        let info = RemoteConnectionInfo {
            label: _label(),
            recipient_keys: _recipient_keys(),
            routing_keys: vec![key_1.clone(), key_2.clone()],
            service_endpoint: _service_endpoint(),
        };

        let ack = A2AMessage::Ack(_ack());

        let envelope = EncryptionEnvelope::create(&ack, &setup.key, &info).unwrap();

        let message_1 = EncryptionEnvelope::open(&key_1, envelope.0).unwrap();

        let message_1 = match message_1 {
            A2AMessage::Forward(forward) => {
                assert_eq!(key_1, forward.to);
                serde_json::to_vec(&forward.msg).unwrap()
            },
            _ => return assert!(false)
        };

        let message_2 = EncryptionEnvelope::open(&key_2, message_1).unwrap();

        let message_2 = match message_2 {
            A2AMessage::Forward(forward) => {
                assert_eq!(_key_1(), forward.to);
                serde_json::to_vec(&forward.msg).unwrap()
            },
            _ => return assert!(false)
        };

        assert_eq!(ack, EncryptionEnvelope::open(&_key_1(), message_2).unwrap());
    }
}
use utils::libindy::crypto;
use utils::httpclient;

use error::prelude::*;
use v3::messages::A2AMessage;
use v3::messages::connection::remote_info::RemoteConnectionInfo;
use v3::messages::forward::Forward;

pub struct EncryptionEnvelope(Vec<u8>);

impl EncryptionEnvelope {
    pub fn create(message: &A2AMessage,
                  pw_verkey: &str,
                  remote_connection_info: &RemoteConnectionInfo) -> VcxResult<EncryptionEnvelope> {
        EncryptionEnvelope::encrypt_for_pairwise(message, pw_verkey, remote_connection_info)
            .and_then(|message| EncryptionEnvelope::wrap_into_forward_messages(message, remote_connection_info))
            .map(|message| EncryptionEnvelope(message))
    }

    pub fn send(&self) -> VcxResult<()> {
        httpclient::post_u8(&self.0)?;
        Ok(())
    }

    fn encrypt_for_pairwise(message: &A2AMessage,
                            pw_verkey: &str,
                            remote_connection_info: &RemoteConnectionInfo) -> VcxResult<Vec<u8>> {
        let message = match message {
            A2AMessage::Generic(message_) => message_.to_string(),
            message => json!(message).to_string().to_string()
        };

        let receiver_keys = json!(remote_connection_info.recipient_keys).to_string();

        crypto::pack_message(Some(&pw_verkey), &receiver_keys, message.as_bytes())
    }

    fn wrap_into_forward_messages(message: Vec<u8>,
                                  remote_connection_info: &RemoteConnectionInfo) -> VcxResult<Vec<u8>> {
        let mut routing_keys_iter = remote_connection_info.routing_keys.iter().peekable();

        let mut message: Vec<u8> = Vec::new();

        while let Some(routing_key) = routing_keys_iter.next() {
            let to = routing_keys_iter.peek()
                .map(|key| key.to_string())
                .unwrap_or_else(||
                    remote_connection_info.recipient_keys.get(0).map(String::from)
                        .unwrap_or_default());

            message = EncryptionEnvelope::wrap_into_forward(message, &to, routing_key)?;
        }

        Ok(message)
    }

    fn wrap_into_forward(message: Vec<u8>,
                         to: &str,
                         routing_key: &str) -> VcxResult<Vec<u8>> {
        let message = Forward::new(to.to_string(), message)?;

        let message = json!(message).to_string();
        let receiver_keys = json!(vec![routing_key]).to_string();

        crypto::pack_message(None, &receiver_keys, message.as_bytes())
    }

    pub fn open(my_vk: &str, message: &::serde_json::Value) -> VcxResult<A2AMessage> {
        let payload = ::serde_json::to_vec(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidState, err))?;

        let unpacked_msg = crypto::unpack_message(&payload)?;

        let message: ::serde_json::Value = ::serde_json::from_slice(unpacked_msg.as_slice())
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize message: {}", err)))?;

        let message = message["message"].as_str()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidJson, "Cannot find `message` field"))?.to_string();

        let message: A2AMessage = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize message: {}", err)))?;

        Ok(message)
    }
}


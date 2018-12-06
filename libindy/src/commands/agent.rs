use errors::agent::AgentError;
use serde_json;
use domain::agent::AuthAMES;
use domain::agent::AnonRecipient;
use domain::agent::AnonAMES;
use domain::crypto::key::Key;
use errors::common::CommonError;
use errors::indy::IndyError;
use services::crypto::CryptoService;
use services::agent::AgentService;
use services::wallet::WalletService;
use services::wallet::RecordOptions;
use std::rc::Rc;
use std::result;
use utils::crypto::base64;
use domain::agent::FromAddress;
use domain::agent::ToAddress;

type Result<T> = result::Result<T, IndyError>;

//pub enum AgentCommand {
//    AuthPackMessage(
//        String, // plaintext message
//        String, //list of receiving keys
//        String, //my verkey
//        i32,    //wallet_handle
//        Box<Fn(Result<String /*JWM serialized as string*/>) + Send>,
//    ),
//    AnonPackMessage(
//        String, // plaintext message
//        String, // list of receiving keys
//        Box<Fn(Result<String /*JWM serialized as string*/>) + Send>,
//    ),
//    UnpackMessage(
//        String, // JWE
//        String, // my verkey
//        i32,    // wallet handle
//        Box<Fn(Result<(String, /*plaintext*/ String, /*sender_vk*/)>) + Send>,
//    ),
//}
//
//pub struct AgentCommandExecutor {
//    wallet_service: Rc<WalletService>,
//    crypto_service: Rc<CryptoService>,
//    agent_service: Rc<AgentService>,
//}
//
//impl AgentCommandExecutor {
//    pub fn new(
//        wallet_service: Rc<WalletService>,
//        crypto_service: Rc<CryptoService>,
//        agent_service: Rc<AgentService>,
//    ) -> AgentCommandExecutor {
//        AgentCommandExecutor {
//            wallet_service,
//            crypto_service,
//            agent_service,
//        }
//    }
//
////    pub fn execute(&self, command: AgentCommand) {
////        match command {
////            AgentCommand::AuthPackMessage(message, receiver_keys_json, sender_verkey, wallet_handle, cb) => {
////                info!("PackMessage command received");
////                cb(self.auth_pack_msg(&message, &receiver_keys_json, &sender_verkey, wallet_handle));
////            }
////            AgentCommand::AnonPackMessage(message, receiver_keys_json, cb) => {
////                info!("PackMessage command received");
////                cb(self.anon_pack_msg(&message, &receiver_keys_json));
////            }
////            AgentCommand::UnpackMessage(jwe, sender_verkey, wallet_handle, cb) => {
////                info!("UnpackMessage command received");
////                cb(self.unpack_msg(&jwe, &sender_verkey, wallet_handle));
////            }
////        };
////    }
////
////    //TODO change errors
////    pub fn auth_pack_msg(
////        &self,
////        message: &str,
////        receiver_keys_json: &str,
////        sender_verkey: &str,
////        wallet_handle: i32,
////    ) -> Result<String> {
////        //convert type from json array to Vec<String>
////        let recv_keys: Vec<&str> = serde_json::from_str(receiver_keys_json).map_err(|err| {
////            IndyError::CommonError(CommonError::InvalidParam4(format!("Failed to serialize recv_keys {:?}", err)))
////        })?;
////
////        //encrypt ciphertext
////        let (sym_key, iv, ciphertext) = self.crypto_service.encrypt_ciphertext(message);
////
////        //convert sender_vk to Key
////        let my_key: Key = self.wallet_service.get_indy_object(wallet_handle, sender_verkey, &RecordOptions::id_value())?;
////
////        //encrypt ceks
////        let mut auth_recipients = vec![];
////
////        for their_vk in recv_keys {
////            auth_recipients.push(
////                self.crypto_service
////                    .auth_encrypt_recipient(&my_key, their_vk, &sym_key)
////                    .map_err(|err| {
////                        IndyError::CommonError(CommonError::InvalidStructure(format!("Invalid anon_recipient structure: {}", err)))
////                    })?,
////            );
////        }
////
////        //serialize AuthAMES
////        let jwe_json = AuthAMES {
////            recipients: auth_recipients,
////            ver: "AuthAMES/1.0/".to_string(),
////            enc: "xsalsa20poly1305".to_string(),
////            ciphertext: base64::encode(ciphertext.as_slice()),
////            iv: base64::encode(&iv[..]),
////        };
////        serde_json::to_string(&jwe_json)
////            .map_err(|err| IndyError::CommonError(CommonError::InvalidStructure(format!("Failed to serialize JWE {}", err))))
////    }
////
////    pub fn anon_pack_msg(&self, message: &str, receiver_keys_json: &str) -> Result<String> {
////        //convert type from json array to Vec<&str>
////        let recv_keys: Vec<&str> = serde_json::from_str(receiver_keys_json).map_err(|err| {
////            IndyError::CommonError(CommonError::InvalidParam4(format!("Failed to serialize recv_keys {:?}", err)))
////        })?;
////
////        //encrypt ciphertext
////        let (sym_key, iv, ciphertext) = self.crypto_service.encrypt_ciphertext(message);
////
////        //encrypt ceks
////        let mut anon_recipients: Vec<AnonRecipient> = vec![];
////
////        for their_vk in recv_keys {
////            anon_recipients.push(
////                self.crypto_service
////                    .anon_encrypt_recipient(their_vk, sym_key.clone())
////                    .map_err(|err| {
////                        IndyError::AgentError(AgentError::CommonError(CommonError::InvalidStructure(format!("Invalid anon_recipient structure: {}", err))))
////                    })?,
////            );
////        }
////
////        //serialize AnonAMES
////        let anon_ames_struct = AnonAMES {
////            recipients: anon_recipients,
////            ver: "AnonAMES/1.0/".to_string(),
////            enc: "xsalsa20poly1305".to_string(),
////            ciphertext: base64::encode(ciphertext.as_slice()),
////            iv: base64::encode(&iv[..]),
////        };
////        serde_json::to_string(&anon_ames_struct)
////            .map_err(|err| IndyError::AgentError(AgentError::PackError(format!("Failed to serialize JWE {}", err))))
////    }
////
////    pub fn pack_msg(
////        &self,
////        message: &[u8],
////        receivers: &str,
////        sender: &str
////    ) -> Result<String> {
////        // parse receivers to ToAddress struct
////        //parse senders to FromAddress struct
////
////        let from_address : FromAddress = serde_json::from_str(sender);
////        let to_address : ToAddress = serde_json::from_str(receivers);
////    }
////
////    pub fn unpack_msg(
////        &self,
////        jwe_json: &str,
////        sender: &str,
////        wallet_handle: i32,
////    ) -> Result<(String, String)> {
////
////        if jwe_json.contains("AuthAMES/1.0/") { //handles unpacking auth_crypt JWE
////            //deserialize json string to struct
////            let jwe_json: AuthAMES = serde_json::from_str(jwe_json).map_err(|err| {
////                IndyError::AgentError(AgentError::UnpackError(format!("Failed to deserialize auth ames {}", err)))
////            })?;
////
////            //get recipient struct that matches sender_verkey parameter
////            let recipient_struct =
////                self.agent_service.get_auth_recipient_header(sender, jwe_json.recipients)?;
////
////            //get key to use for decryption
////            let my_key: &Key = &self.wallet_service.get_indy_object(wallet_handle, sender, &RecordOptions::id_value())?;
////
////            //decrypt recipient header
////            let (ephem_sym_key, sender_vk) = self.crypto_service.auth_decrypt_recipient(my_key, recipient_struct)?;
////
////            // decode
////            let message = self.crypto_service.decrypt_ciphertext(
////                &jwe_json.ciphertext,
////                &jwe_json.iv,
////                &ephem_sym_key,
////            )?;
////
////            //TODO convert this to a json_string instead of Tuple
////            Ok((message, sender_vk))
////
////        } else if jwe_json.contains("AnonAMES/1.0/") { //handles unpacking anon_crypt JWE
////           //deserialize json string to struct
////            let jwe_json: AnonAMES = serde_json::from_str(jwe_json).map_err(|err| {
////                IndyError::AgentError(AgentError::UnpackError(format!("Failed to deserialize auth ames {}", err)))
////            })?;
////
////            //get recipient struct that matches sender_verkey parameter
////            let recipient_struct =
////                self.agent_service.get_anon_recipient_header(sender, jwe_json.recipients)?;
////
////            //get key to use for decryption
////            let my_key: &Key = &self.wallet_service
////                .get_indy_object(wallet_handle, sender, &RecordOptions::id_value())
////                .map_err(|err| IndyError::AgentError(AgentError::UnpackError(format!("Can't find my_key: {:?}", err))))?;
////
////            //decrypt recipient header
////            let ephem_sym_key = self.crypto_service.anon_decrypt_recipient(my_key, recipient_struct)?;
////
////            //decrypt message
////            let message = self.crypto_service.decrypt_ciphertext(
////                &jwe_json.ciphertext,
////                &jwe_json.iv,
////                &ephem_sym_key,
////            )?;
////
////            //TODO convert this to a json_string instead of Tuple
////            Ok((message, "".to_string()))
////
////        } else {
////            Err(IndyError::AgentError(AgentError::UnpackError(format!("Failed to unpack - unidentified ver provided"))))
////        }
////    }
//}

#[cfg(test)]
mod tests {
    use domain::agent::{FromAddress, ToAddress};
    use serde_json;
    use serde_json::Error;

    #[test]
    pub fn test_serde_json_works() {
        let sender = r#"{ wallet_key: { public_key: "pubkey12345", wallet_handle: 1 } }"#;
        let receivers = r#"["pubkey_1", "pubkey2", "pubkey3"]"#;

        let from_address : Result<FromAddress, Error> = serde_json::from_str(sender);
        let to_address : Result<ToAddress, Error> = serde_json::from_str(receivers);

        assert!(from_address.is_ok());
        assert!(to_address.is_ok());
    }

    #[test]
    pub fn test_serde_json_to_string_works() {
        let sender = r#"{ wallet_key: { public_key: "pubkey12345", wallet_handle: 1 } }"#;
        let receivers = r#"["pubkey_1", "pubkey2", "pubkey3"]"#;

        let from_address : Result<FromAddress, Error> = serde_json::from_str(sender);
        let to_address : Result<ToAddress, Error> = serde_json::from_str(receivers);

        assert!(from_address.is_ok());
        assert!(to_address.is_ok());
    }
}
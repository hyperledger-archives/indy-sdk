use errors::route::AgentError;
use serde_json;
use services::crypto::CryptoService;
use services::route::RouteService;
use services::wallet::WalletService;
use std::rc::Rc;
use std::result;
use services::agent::AgentService;
use domain::route::AuthAMES;
use domain::route::AnonRecipient;
use domain::route::AnonAMES;
use services::wallet::RecordOptions;
use domain::crypto::key::Key;
use errors::common::CommonError;
use errors::indy::IndyError;

type Result<T> = result::Result<T, IndyError>;

pub enum RouteCommand {
    AuthPackMessage(
        String, // plaintext message
        String, //list of receiving keys
        String, //my verkey
        i32,    //wallet_handle
        Box<Fn(Result<String /*JWM serialized as string*/>) + Send>,
    ),
    AnonPackMessage(
        String, // plaintext message
        String, // list of receiving keys
        Box<Fn(Result<String /*JWM serialized as string*/>) + Send>,
    ),
    UnpackMessage(
        String, // JWE
        String, // my verkey
        i32,    // wallet handle
        Box<Fn(Result<(String, /*plaintext*/ String, /*sender_vk*/)>) + Send>,
    ),
}

pub struct RouteCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    route_service: Rc<AgentService>,
}

impl RouteCommandExecutor {
    pub fn new(
        wallet_service: Rc<WalletService>,
        crypto_service: Rc<CryptoService>,
        route_service: Rc<AgentService>,
    ) -> RouteCommandExecutor {
        RouteCommandExecutor {
            wallet_service,
            crypto_service,
            route_service,
        }
    }

    pub fn execute(&self, command: RouteCommand) {
        match command {
            RouteCommand::AuthPackMessage(message, receiver_keys_json, sender_verkey, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.auth_pack_msg(&message, &receiver_keys_json, sender_verkey, wallet_handle));
            }
            RouteCommand::AnonPackMessage(message, receiver_keys_json, cb) => {
                info!("PackMessage command received");
                cb(self.anon_pack_msg(&message, &receiver_keys_json));
            }
            RouteCommand::UnpackMessage(jwe, sender_verkey, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&ames_json_str, &sender_verkey, wallet_handle));
            }
        };
    }

    //TODO change errors
    pub fn auth_pack_msg(
        &self,
        message: &str,
        receiver_keys_json: &str,
        sender_verkey: String,
        wallet_handle: i32,
    ) -> Result<String> {
        //convert type from json array to Vec<String>
        let recv_keys: Vec<&str> = serde_json::from_str(receiver_keys_json).map_err(|err| {
            CommonError::InvalidParam4(format!("Failed to serialize recv_keys {:?}", err))
        })?;

        //encrypt ciphertext
        let (sym_key, iv, ciphertext) = self.crypto_service.encrypt_ciphertext(message);

        //convert sender_vk to Key
        let my_key = self.wallet_service
            .get_indy_object(wallet_handle, sender_vk, &RecordOptions::id_value())?;

        //encrypt ceks
        let mut auth_recipients = vec![];

        for their_vk in recv_keys {
            auth_recipients.push(
                self.crypto_service
                    .auth_encrypt_recipient(my_key, their_vk, &sym_key)
                    .map_err(|err| {
                        AgentError::CommonError(CommonError::InvalidStructure(format!("Invalid anon_recipient structure: {}", err)))
                    })?,
            );
        }

        //serialize AuthAMES
        let auth_ames_struct = AuthAMES {
            recipients: auth_recipients,
            ver: "AuthAMES/1.0/".to_string(),
            enc: "xsalsa20poly1305".to_string(),
            ciphertext: base64::encode(ciphertext.as_slice()),
            iv: base64::encode(&iv[..]),
        };
        serde_json::to_string(&auth_ames_struct)
            .map_err(|err| IndyError::CommonError(CommonError::InvalidStructure(format!("Failed to serialize JWE {}", err))))
    }

    pub fn anon_pack_msg(&self, message: &str, receiver_keys_json: &str) -> Result<String> {
        //convert type from json array to Vec<&str>
        let recv_keys: Vec<&str> = serde_json::from_str(receiver_keys_json).map_err(|err| {
            CommonError::InvalidParam4(format!("Failed to serialize recv_keys {:?}", err))
        })?;

        //encrypt ciphertext
        let (sym_key, iv, ciphertext) = self.crypto_service.encrypt_ciphertext(message);

        //encrypt ceks
        let mut anon_recipients: Vec<AnonRecipient> = vec![];

        for their_vk in recv_keys {
            anon_recipients.push(
                self.crypto_service
                    .anon_encrypt_recipient(their_vk, sym_key.clone())
                    .map_err(|err| {
                        AgentError::CommonError(CommonError::InvalidStructure(format!("Invalid anon_recipient structure: {}", err)))
                    }?),
            );
        }

        //serialize AnonAMES
        let anon_ames_struct = AnonAMES {
            recipients: anon_recipients,
            ver: "AnonAMES/1.0/".to_string(),
            enc: "xsalsa20poly1305".to_string(),
            ciphertext: base64::encode(ciphertext.as_slice()),
            iv: base64::encode(&iv[..]),
        };
        serde_json::to_string(&anon_ames_struct)
            .map_err(|err| AgentError::PackError(format!("Failed to serialize JWE {}", err)))
    }

    pub fn unpack_msg(
        &self,
        ames_json_str: &str,
        sender_verkey: &str,
        wallet_handle: i32,
    ) -> Result<(String, String)> {

        if ames_json_str.contains("AuthAMES/1.0/") { //handles unpacking auth_crypt JWE
            //deserialize json string to struct
            let auth_ames_struct: AuthAMES = serde_json::from_str(ames_json_str).map_err(|err| {
                AgentError::SerializationError(format!("Failed to deserialize auth ames {}", err))
            })?;

            //get recipient struct that matches sender_verkey parameter
            let recipient_struct =
                self.get_auth_recipient_header(sender_verkey, auth_ames_struct.recipients)?;

            //get key to use for decryption
            let my_key: &Key = &ws
                .get_indy_object(wallet_handle, sender_verkey, &RecordOptions::id_value())
                .map_err(|err| AgentError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

            //decrypt recipient header
            let (ephem_sym_key, sender_vk) =
                self.auth_decrypt_recipient(my_key, recipient_struct, cs)?;

            // decode
            let message = self.decrypt_ciphertext(
                &auth_ames_struct.ciphertext,
                &auth_ames_struct.iv,
                &ephem_sym_key,
            )?;
            Ok((message, sender_vk))

        } else if ames_json_str.contains("AnonAMES/1.0/") { //handles unpacking anon_crypt JWE
           //deserialize json string to struct
            let auth_ames_struct: AnonAMES = serde_json::from_str(ames_json_str).map_err(|err| {
                AgentError::SerializationError(format!("Failed to deserialize auth ames {}", err))
            })?;

            //get recipient struct that matches sender_verkey parameter
            let recipient_struct =
                self.get_anon_recipient_header(sender_verkey, auth_ames_struct.recipients)?;

            //get key to use for decryption
            let my_key: &Key = &ws
                .get_indy_object(wallet_handle, sender_verkey, &RecordOptions::id_value())
                .map_err(|err| AgentError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

            //decrypt recipient header
            let ephem_sym_key = self.anon_decrypt_recipient(my_key, recipient_struct, cs)?;

            //decrypt message
            let message = self.decrypt_ciphertext(
                &auth_ames_struct.ciphertext,
                &auth_ames_struct.iv,
                &ephem_sym_key,
            )?;

            //return message and no key
            Ok((message, "".to_string()))

        } else {
            Err(AgentError::UnpackError(format!(
                "Failed to unpack - unidentified ver provided"
            )))
        }
    }
}
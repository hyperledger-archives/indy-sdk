use errors::route::RouteError;
use serde_json;
use services::crypto::CryptoService;
use services::route::RouteService;
use services::wallet::WalletService;
use std::rc::Rc;
use std::result;

type Result<T> = result::Result<T, RouteError>;

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
        String, // AMES either JSON or Compact Serialization
        String, // my verkey
        i32,    // wallet handle
        Box<Fn(Result<(String, /*plaintext*/ String, /*sender_vk*/)>) + Send>,
    ),
}

pub struct RouteCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    route_service: Rc<RouteService>,
}

impl RouteCommandExecutor {
    pub fn new(
        wallet_service: Rc<WalletService>,
        crypto_service: Rc<CryptoService>,
        route_service: Rc<RouteService>,
    ) -> RouteCommandExecutor {
        RouteCommandExecutor {
            wallet_service,
            crypto_service,
            route_service,
        }
    }

    pub fn execute(&self, command: RouteCommand) {
        match command {
            RouteCommand::AuthPackMessage(message, recv_keys_json, my_vk, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.auth_pack_msg(&message, &recv_keys_json, my_vk, wallet_handle));
            }
            RouteCommand::AnonPackMessage(message, recv_keys_json, cb) => {
                info!("PackMessage command received");
                cb(self.anon_pack_msg(&message, &recv_keys_json));
            }
            RouteCommand::UnpackMessage(ames_json_str, my_vk, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&ames_json_str, &my_vk, wallet_handle));
            }
        };
    }

    pub fn auth_pack_msg(
        &self,
        message: &str,
        recv_keys_json: &str,
        my_vk: String,
        wallet_handle: i32,
    ) -> Result<String> {
        //convert type from json array to Vec<String>
        let recv_keys: Vec<&str> = serde_json::from_str(recv_keys_json).map_err(|err| {
            RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err))
        })?;

        //encrypt ciphertext
        let (cek, iv, ciphertext) = self.crypto_service.encrypt_ciphertext(message);

        //convert sender_vk to Key
        let my_key = &ws
            .get_indy_object(wallet_handle, sender_vk, &RecordOptions::id_value())
            .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

        //encrypt ceks
        let mut auth_recipients = vec![];

        for their_vk in recv_keys {
            auth_recipients.push(
                self.auth_encrypt_recipient(my_key, their_vk, &cek, cs.clone())
                    .map_err(|err| {
                        RouteError::PackError(format!("Failed to push auth recipient {}", err))
                    })?,
            );
        };

        //serialize AuthAMES
        let auth_ames_struct = AuthAMES {
            recipients: auth_recipients,
            ver: "AuthAMES/1.0/".to_string(),
            enc: "xsalsa20poly1305".to_string(),
            ciphertext: base64::encode(ciphertext.as_slice()),
            iv: base64::encode(&iv[..]),
        };
        serde_json::to_string(&auth_ames_struct)
            .map_err(|err| RouteError::PackError(format!("Failed to serialize authAMES {}", err)))
    }

    pub fn anon_pack_msg(&self, message: &str, recv_keys_json: &str) -> Result<String> {
        //convert type from json array to Vec<&str>
        let recv_keys: Vec<&str> = serde_json::from_str(recv_keys_json).map_err(|err| {
            RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err))
        })?;

        //encrypt ciphertext
        let (cek, iv, ciphertext) = self.encrypt_ciphertext(message);

        //encrypt ceks
        let mut anon_recipients: Vec<AnonRecipient> = vec![];
        for their_vk in recv_keys {
            let anon_recipient =
                self.anon_encrypt_recipient(their_vk, cek.clone(), cs.clone())?;
            anon_recipients.push(anon_recipient);
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
            .map_err(|err| RouteError::PackError(format!("Failed to serialize anonAMES {}", err)))
    }

    pub fn unpack_msg(
        &self,
        ames_json_str: &str,
        my_vk: &str,
        wallet_handle: i32,
    ) -> Result<(String, String)> {
        //check if authAMES or anonAMES

        if ames_json_str.contains("AuthAMES/1.0/") {
            //deserialize json string to struct
            let auth_ames_struct: AuthAMES = serde_json::from_str(ames_json_str).map_err(|err| {
                RouteError::SerializationError(format!("Failed to deserialize auth ames {}", err))
            })?;

            //get recipient struct that matches my_vk parameter
            let recipient_struct =
                self.route_service.get_auth_recipient_header(my_vk, auth_ames_struct.recipients)?;

            //get key to use for decryption
            let my_key: &Key = self.wallet_service
                .get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())
                .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

            //decrypt recipient header
            let (ephem_cek, sender_vk) =
                self.crypto_service.auth_decrypt_recipient(my_key, recipient_struct)?;

            // decode
            let message = self.crypto_service.decrypt_ciphertext(
                &auth_ames_struct.ciphertext,
                &auth_ames_struct.iv,
                &ephem_cek,
            )?;

            Ok((message, sender_vk))

        } else if ames_json_str.contains("AnonAMES/1.0/") {

            //deserialize json string to struct
            let auth_ames_struct: AnonAMES = serde_json::from_str(ames_json_str).map_err(|err| {
                RouteError::SerializationError(format!("Failed to deserialize auth ames {}", err))
            })?;

            //get recipient struct that matches my_vk parameter
            let recipient_struct =
                self.route_service.get_anon_recipient_header(my_vk, auth_ames_struct.recipients)?;

            //get key to use for decryption
            let my_key: &Key = self.wallet_service
                .get_indy_object(wallet_handle, my_vk, &RecordOptions::id_value())
                .map_err(|err| RouteError::UnpackError(format!("Can't find my_key: {:?}", err)))?;

            //decrypt recipient header
            let ephem_cek = self.crypto_service.anon_decrypt_recipient(my_key, recipient_struct)?;

            //decrypt message
            let message = self.crypto_service.decrypt_ciphertext(
                &auth_ames_struct.ciphertext,
                &auth_ames_struct.iv,
                &ephem_cek,
            )?;

            //return message and no key
            Ok((message, "".to_string()))

        } else {
            Err(RouteError::UnpackError(format!(
                "Failed to unpack - unidentified ver provided"
            )))
        }

    }
}
use errors::route::RouteError;
use errors::common::CommonError;
use utils::crypto::base64::{decode, encode};
use domain::crypto::combo_box::ComboBox;
use domain::crypto::key::Key;
use services::route::jwm::{JWM, Header, Recipient};
use services::crypto::CryptoService;
use services::wallet::{WalletService, RecordOptions};


pub struct JWMData {
    pub header: Header,
    pub cek: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub iv: Vec<u8>,
    pub tag: Vec<u8>
}

pub struct CEK {
    pub cek : String,
    pub nonce : String,
    pub their_vk : String
}

pub fn get_jwm_data(jwm : JWM, my_vk: &str) -> Result<JWMData, RouteError> {
    match jwm {
        JWM::JWMFull(jwmf) => {
            //finds the recipient index that matches the verkey passed in to the recipient verkey field
            let recipient_index = jwmf.recipients.iter()
                .position(|ref recipient| recipient.header.kid == my_vk);
            match recipient_index {
                Some(v) => {
                    Ok(JWMData {
                        header: jwmf.recipients[v].header.clone(),
                        cek: decode(&jwmf.recipients[v].encrypted_key)?,
                        ciphertext: decode(&jwmf.ciphertext)?,
                        iv: decode(&jwmf.iv)?,
                        tag: decode(&jwmf.tag)?
                    })
                },
                //if no matching index is found return an error
                _ => Err(RouteError::UnpackError("The message doesn't include a header with this verkey".to_string()))
            }
        },

        JWM::JWMCompact(jwmc) => {
            if jwmc.header.kid == my_vk {
                Ok(JWMData {
                    header: jwmc.header,
                    cek: decode(&jwmc.cek)?,
                    ciphertext: decode(&jwmc.ciphertext)?,
                    iv: decode(&jwmc.iv)?,
                    tag: decode(&jwmc.tag)?
                })
            } else {
                Err(RouteError::UnpackError("The message doesn't include a header with this verkey".to_string()))
            }
        }
    }
}

pub fn get_key_from_str(my_vk : &str, wallet_handle: i32, wallet_service: &WalletService) -> Result<Key, RouteError> {
    wallet_service.get_indy_object(wallet_handle,
                                   my_vk,
                                   &RecordOptions::id_value(),
                                   &mut String::new())
    .map_err(|err| RouteError::UnpackError(format!("Can't find key: {:?}", err)))
}

pub fn get_sym_key(key: &Key, cek: &[u8], header: Header, crypto_service: &CryptoService) -> Result<Vec<u8>, RouteError> {
    match header.alg.as_ref() {
        //handles authdecrypting content encryption key
        "x-auth" => {
            let decrypted_header = crypto_service.decrypt_sealed(key, cek)
            .map_err( | err | RouteError::EncryptionError(format ! ("Can't decrypt encrypted_key: {:?}", err)))?;
            let parsed_msg = ComboBox::from_msg_pack(decrypted_header.as_slice())
            .map_err( | err | RouteError::UnpackError(format !("Can't deserialize ComboBox: {:?}", err)))?;
            let cek: Vec < u8 > = decode( &parsed_msg.msg)
            .map_err( | err | RouteError::UnpackError(format ! ("Can't decode encrypted_key msg filed from base64 {}", err)))?;
            let nonce: Vec <u8 > = decode( & parsed_msg.nonce)
            .map_err( | err | RouteError::UnpackError(format ! ("Can't decode encrypted_key nonce from base64 {}", err)))?;

            match &header.jwk {
                Some(jwk) => Ok(crypto_service.decrypt( key, jwk, & cek, &nonce)
                    .map_err( | err | RouteError::EncryptionError(format ! ("{}", err)))?),
                None => Err(RouteError::MissingKeyError("jwk not included to decrypt".to_string()))
            }

        },

        //handles anondecrypting content encryption algorithms
        "x-anon" => Ok(crypto_service.decrypt_sealed(key, cek)
            .map_err( | err | RouteError::EncryptionError(format ! ("{}", err)))?),

        //handles all other unrecognized content encryption algorithms
        _ => Err(RouteError::EncryptionError("Unexpected Content Encryption Algorithm".to_string()))
    }
}

pub fn encrypt_ceks(recv_keys: &Vec<String>, auth: bool, key : Option<Key>, sym_key: &[u8], crypto_service: &CryptoService) -> Result<Vec<String>, RouteError>{
    let mut enc_ceks : Vec<(String)> = vec![];

    if auth {
        // if authcrypting get key to encrypt, if there's not one throw error
        if key.is_some() {
            let my_key = &key.unwrap();
            for their_vk in recv_keys {
                let cek_combo_box = crypto_service.create_combo_box(my_key, &their_vk, sym_key)
                    .map_err(|e| RouteError::EncryptionError(format!("Can't encrypt content encryption key: {:?}", e)))?;
                let msg_pack = cek_combo_box.to_msg_pack()
                    .map_err(|e| CommonError::InvalidState(format!("Can't serialize ComboBox: {:?}", e)))?;
                let cek_as_bytes = crypto_service.encrypt_sealed(&their_vk, &msg_pack)
                    .map_err(|e| RouteError::EncryptionError(format!("Failed to encrypt content encryption key ComboBox: {:?}", e)))?;
                enc_ceks.push(encode(&cek_as_bytes));
            }
        } else {
            return Err(RouteError::MissingKeyError("invalid key parameter, unable to encrypt CEKs".to_string()))
        }
    }else {
        //handles anoncrypt flow of encrypting content keys
        for their_vk in recv_keys {
            let cek_as_bytes = crypto_service.encrypt_sealed(&their_vk, sym_key)
                .map_err(|e| RouteError::EncryptionError(format!("Failed to encrypt cek: {:?}", e)))?;
            enc_ceks.push(encode(&cek_as_bytes));
        }
    }
    Ok(enc_ceks.to_owned())
}


#[cfg(test)]
pub mod tests {
    pub fn test_get_jwm_data_success() {

    }

    pub fn test_get_key_from_str_success() {

    }

    pub fn test_get_sym_key_success() {

    }

    pub fn test_encrypt_ceks_success() {

    }
}
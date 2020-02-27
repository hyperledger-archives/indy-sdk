use indyrs::IndyError;
use futures::{Future, future};
use crate::utils::rand;
use std::str::FromStr;
use failure::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum KeyDerivationFunction {
    #[serde(rename = "ARGON2I_MOD")]
    Argon2iMod,
    #[serde(rename = "ARGON2I_INT")]
    Argon2iInt,
    #[serde(rename = "RAW")]
    Raw
}

impl FromStr for KeyDerivationFunction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAW" => Ok(KeyDerivationFunction::Raw),
            "ARGON2I_MOD" => Ok(KeyDerivationFunction::Argon2iMod),
            "ARGON2I_INT" => Ok(KeyDerivationFunction::Argon2iInt),
            _ => Err(format_err!("Can not convert string to KeyDerivationMethod"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyDerivationDirective {
    pub kdf: KeyDerivationFunction,
    /// Seed for key derivation
    pub key: String,
}

impl KeyDerivationDirective {
    pub fn new(key_derivation_method: KeyDerivationFunction) -> Box<dyn Future<Item=KeyDerivationDirective, Error=IndyError>> {
        let key_future = match key_derivation_method {
            KeyDerivationFunction::Argon2iMod | KeyDerivationFunction::Argon2iInt => Box::new(future::ok(rand::rand_string(10))),
            KeyDerivationFunction::Raw => indyrs::wallet::generate_wallet_key(None)
        };
        Box::new(key_future.map(move |key| {
            KeyDerivationDirective { key, kdf: key_derivation_method.clone() }
        }))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rust_base58::base58::FromBase58;

    #[test]
    fn should_parse_string_as_raw_key_derivation_method() {
        let kdf: KeyDerivationFunction = KeyDerivationFunction::from_str("RAW").unwrap();
        assert_eq!(kdf, KeyDerivationFunction::Raw)
    }

    #[test]
    fn should_parse_string_as_argonint_key_derivation_method() {
        let kdf: KeyDerivationFunction = KeyDerivationFunction::from_str("ARGON2I_INT").unwrap();
        assert_eq!(kdf, KeyDerivationFunction::Argon2iInt)
    }

    #[test]
    fn should_parse_string_as_argonmod_key_derivation_method() {
        let kdf: KeyDerivationFunction = KeyDerivationFunction::from_str("ARGON2I_MOD").unwrap();
        assert_eq!(kdf, KeyDerivationFunction::Argon2iMod)
    }

    #[test]
    fn should_throw_error_if_trying_parse_unknown_kdf_method() {
        let res = KeyDerivationFunction::from_str("FOOBAR");
        assert!(res.is_err())
    }

    #[test]
    fn should_build_argon2iint_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationFunction::Argon2iInt)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.kdf, KeyDerivationFunction::Argon2iInt);
        assert_eq!(derivation.key.len(), 10);
    }

    #[test]
    fn should_build_argon2imod_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationFunction::Argon2iMod)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.kdf, KeyDerivationFunction::Argon2iMod);
        assert_eq!(derivation.key.len(), 10);
    }

    #[test]
    fn should_build_raw_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationFunction::Raw)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.kdf, KeyDerivationFunction::Raw);
        assert_eq!(derivation.key.from_base58().unwrap().len(), 32)
    }

    #[test]
    fn should_serialize_argon2iint_directive() {
        let derivation = KeyDerivationDirective {
            kdf: KeyDerivationFunction::Argon2iInt,
            key: "aaa".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"kdf":"ARGON2I_INT","key":"aaa"}"#, serialized);
    }

    #[test]
    fn should_serialize_argon2imod_directive() {
        let derivation = KeyDerivationDirective {
            kdf: KeyDerivationFunction::Argon2iMod,
            key: "bbb".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"kdf":"ARGON2I_MOD","key":"bbb"}"#, serialized);
    }

    #[test]
    fn should_serialize_raw_directive() {
        let derivation = KeyDerivationDirective {
            kdf: KeyDerivationFunction::Raw,
            key: "BJreaZrbQMbDGrYJjekQfr7YKGAKtPeZ6nubPtwYLAo5".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"kdf":"RAW","key":"BJreaZrbQMbDGrYJjekQfr7YKGAKtPeZ6nubPtwYLAo5"}"#, serialized);
    }
}
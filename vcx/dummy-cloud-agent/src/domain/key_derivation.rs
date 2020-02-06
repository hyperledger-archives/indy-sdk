use indyrs::IndyError;
use futures::{Future, future};
use crate::utils::rand;
use std::str::FromStr;
use failure::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum KeyDerivationMethod {
    #[serde(rename = "ARGON2I_MOD")]
    Argon2iMod,
    #[serde(rename = "ARGON2I_INT")]
    Argon2iInt,
    #[serde(rename = "RAW")]
    Raw
}

impl FromStr for KeyDerivationMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAW" => Ok(KeyDerivationMethod::Raw),
            "ARGON2I_MOD" => Ok(KeyDerivationMethod::Argon2iMod),
            "ARGON2I_INT" => Ok(KeyDerivationMethod::Argon2iInt),
            _ => Err(format_err!("Can not convert string to KeyDerivationMethod"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyDerivationDirective {
    pub key_derivation_method: KeyDerivationMethod,
    pub key: String,
}

impl KeyDerivationDirective {
    pub fn new(key_derivation_method: KeyDerivationMethod) -> Box<dyn Future<Item=KeyDerivationDirective, Error=IndyError>> {
        let key_future = match key_derivation_method {
            KeyDerivationMethod::Argon2iMod | KeyDerivationMethod::Argon2iInt => Box::new(future::ok(()).map(|_| rand::rand_string(10))),
            KeyDerivationMethod::Raw => indyrs::wallet::generate_wallet_key(None)
        };
        Box::new(key_future.map(move |key| {
            KeyDerivationDirective { key, key_derivation_method: key_derivation_method.clone() }
        }))
    }
}



#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::actors::ForwardA2AMsg;
    use crate::utils::tests::*;

    use super::*;

    #[test]
    fn should_parse_string_as_raw_key_derivation_method() {
        let kdf: KeyDerivationMethod = KeyDerivationMethod::from_str("RAW").unwrap();
        assert_eq!(kdf, KeyDerivationMethod::Raw)
    }

    #[test]
    fn should_parse_string_as_argonint_key_derivation_method() {
        let kdf: KeyDerivationMethod = KeyDerivationMethod::from_str("ARGON2I_INT").unwrap();
        assert_eq!(kdf, KeyDerivationMethod::Argon2iInt)
    }

    #[test]
    fn should_parse_string_as_argonmod_key_derivation_method() {
        let kdf: KeyDerivationMethod = KeyDerivationMethod::from_str("ARGON2I_MOD").unwrap();
        assert_eq!(kdf, KeyDerivationMethod::Argon2iMod)
    }

    #[test]
    #[should_panic(
    expected = r#"Can not convert string to KeyDerivationMethod"#
    )]
    fn should_throw_error_if_trying_parse_unknown_kdf_method() {
        KeyDerivationMethod::from_str("FOOBAR").unwrap();
    }

    #[test]
    fn should_build_argon2iint_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationMethod::Argon2iInt)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.key_derivation_method, KeyDerivationMethod::Argon2iInt);
        assert_eq!(derivation.key.len(), 10);
    }

    #[test]
    fn should_build_argon2imod_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationMethod::Argon2iMod)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.key_derivation_method, KeyDerivationMethod::Argon2iMod);
        assert_eq!(derivation.key.len(), 10);
    }

    #[test]
    fn should_build_raw_directive() {
        let derivation = KeyDerivationDirective::new(KeyDerivationMethod::Raw)
            .wait()
            .expect("Failed to build key derivation directive");
        assert_eq!(derivation.key_derivation_method, KeyDerivationMethod::Raw);
        assert_eq!(derivation.key.len(), 44);
    }

    #[test]
    fn should_serialize_argon2iint_directive() {
        let derivation = KeyDerivationDirective {
            key_derivation_method: KeyDerivationMethod::Argon2iInt,
            key: "aaa".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"key":"aaa","key_derivation_method":"ARGON2I_INT"}"#, serialized);
    }

    #[test]
    fn should_serialize_argon2imod_directive() {
        let derivation = KeyDerivationDirective {
            key_derivation_method: KeyDerivationMethod::Argon2iMod,
            key: "bbb".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"key":"bbb","key_derivation_method":"ARGON2I_MOD"}"#, serialized);
    }

    #[test]
    fn should_serialize_raw_directive() {
        let derivation = KeyDerivationDirective {
            key_derivation_method: KeyDerivationMethod::Raw,
            key: "BJreaZrbQMbDGrYJjekQfr7YKGAKtPeZ6nubPtwYLAo5".into()
        };
        let derivation_value = json!(derivation);
        let serialized = serde_json::to_string(&derivation_value).expect("Failed to serialize into string");
        assert_eq!(r#"{"key":"BJreaZrbQMbDGrYJjekQfr7YKGAKtPeZ6nubPtwYLAo5","key_derivation_method":"RAW"}"#, serialized);
    }
}
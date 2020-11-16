use serde_json::{Map, Value};

use crate::domain::key_derivation::{KeyDerivationDirective, KeyDerivationFunction};

fn serialize_kdf_as_string(kdf: KeyDerivationFunction) -> String {
    match kdf {
        KeyDerivationFunction::Argon2iMod => "ARGON2I_MOD".into(),
        KeyDerivationFunction::Argon2iInt => "ARGON2I_INT".into(),
        KeyDerivationFunction::Raw => "RAW".into()
    }
}

pub fn build_wallet_credentials(kdf_directive: &KeyDerivationDirective, storage_credentials: Option<Value>) -> Value {
    match storage_credentials {
        None => json!(kdf_directive),
        Some(storage_credentials) => {
            let mut map = Map::new();
            map.insert(String::from("key"), Value::String(kdf_directive.key.clone()));
            map.insert(String::from("key_derivation_method"), Value::String(serialize_kdf_as_string(kdf_directive.kdf.clone())));
            map.insert(String::from("storage_credentials"), storage_credentials);
            Value::Object(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::key_derivation::KeyDerivationFunction;

    use super::*;
    use futures::Future;

    #[test]
    fn should_build_wallet_credentials() {
        let storage_credentials = Some(json!({
          "account": "postgres",
          "password": "mysecretpassword",
          "admin_account": "postgres",
          "admin_password": "mysecretpassword"
        }));
        let kdf_directive = KeyDerivationDirective::new(KeyDerivationFunction::Raw).wait().unwrap();
        let o = build_wallet_credentials(&kdf_directive, storage_credentials);
        let result_key = o["key"].as_str().unwrap();
        let result_kdf = o["key_derivation_method"].as_str().unwrap();
        let storage_credentials = o["storage_credentials"].as_object().unwrap();
        assert!(result_key.len() >= 43);
        assert_eq!(result_kdf, "RAW");
        assert_eq!(storage_credentials["account"].as_str().unwrap(), "postgres");
        assert_eq!(storage_credentials["password"].as_str().unwrap(), "mysecretpassword");
        assert_eq!(storage_credentials["admin_account"].as_str().unwrap(), "postgres");
        assert_eq!(storage_credentials["admin_password"].as_str().unwrap(), "mysecretpassword");
    }
}
use serde_json::{Map, Value};

use crate::domain::key_derivation::{KeyDerivationDirective, KeyDerivationMethod};

fn seriaize_kdf_as_string(kdf: KeyDerivationMethod) -> String {
    match kdf {
        KeyDerivationMethod::Argon2iMod => "ARGON2I_MOD".into(),
        KeyDerivationMethod::Argon2iInt => "ARGON2I_INT".into(),
        KeyDerivationMethod::Raw => "RAW".into()
    }
}

pub fn build_wallet_credentials(kdf_directive: &KeyDerivationDirective, storage_credentials: Option<Value>) -> Value {
    match storage_credentials {
        None => json!(kdf_directive),
        Some(storage_credentials) => {
            let mut map = Map::new();
            map.insert(String::from("key"), Value::String(kdf_directive.key.clone()));
            map.insert(String::from("key_derivation_method"), Value::String(seriaize_kdf_as_string(kdf_directive.key_derivation_method.clone())));
            map.insert(String::from("storage_credentials"), storage_credentials);
            Value::Object(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::actors::ForwardA2AMsg;
    use crate::domain::key_derivation::KeyDerivationMethod;
    use crate::utils::tests::*;

    use super::*;

    #[test]
    fn should_build_wallet_credentials() {
        let storage_credentials = Some(json!({
          "account": "postgres",
          "password": "mysecretpassword",
          "admin_account": "postgres",
          "admin_password": "mysecretpassword"
        }));
        let kdf_directive = KeyDerivationDirective::new(KeyDerivationMethod::Raw).wait().unwrap();
        let o = build_wallet_credentials(&kdf_directive, storage_credentials);
        let result_key = o["key"].as_str().unwrap();
        let result_kdf = o["key_derivation_method"].as_str().unwrap();
        let storage_credentials = o["storage_credentials"].as_object().unwrap();
        assert_eq!(result_key.len(), 44);
        assert_eq!(result_kdf, "RAW");
        assert_eq!(storage_credentials["account"].as_str().unwrap(), "postgres");
        assert_eq!(storage_credentials["password"].as_str().unwrap(), "mysecretpassword");
        assert_eq!(storage_credentials["admin_account"].as_str().unwrap(), "postgres");
        assert_eq!(storage_credentials["admin_password"].as_str().unwrap(), "mysecretpassword");
    }
}
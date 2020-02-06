use envconfig::Envconfig;
use crate::domain::key_derivation::KeyDerivationMethod;

lazy_static! {
    static ref APP_ENV_CONFIG: AppEnvConfig = AppEnvConfig::init().unwrap();
}

pub fn get_app_env_config() -> &'static AppEnvConfig {
    return &APP_ENV_CONFIG
}

#[derive(Envconfig, Debug)]
pub struct AppEnvConfig {
    #[envconfig(from = "NEW_AGENT_KDF", default = "RAW")]
    pub new_agent_kdf: KeyDerivationMethod,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn should_construct_app_env_config_with_correct_kdf() {
        env::remove_var("NEW_AGENT_KDF");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, KeyDerivationMethod::Raw, "Default new_agent_kdf should be Raw");

        env::set_var("NEW_AGENT_KDF", "RAW");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, KeyDerivationMethod::Raw, "Expected new_agent_kdf to be Raw.");

        env::set_var("NEW_AGENT_KDF", "ARGON2I_INT");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, KeyDerivationMethod::Argon2iInt, "Expected new_agent_kdf to be Argon2iInt.");

        env::set_var("NEW_AGENT_KDF", "ARGON2I_MOD");
        let app_config = AppEnvConfig::init().unwrap();
        assert_eq!(app_config.new_agent_kdf, KeyDerivationMethod::Argon2iMod, "Expected new_agent_kdf to be Argon2iMod.");

        env::set_var("NEW_AGENT_KDF", "FOOBAR");
        assert!(AppEnvConfig::init().is_err())
    }
}
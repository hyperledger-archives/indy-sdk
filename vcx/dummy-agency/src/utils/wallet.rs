use errors::*;
use indy::errors::{Error as IndyError, ErrorKind as IndyErrorKind};
use futures::*;
use indy::wallet;
use serde_json;

pub fn ensure_created(id: &str,
                      passphrase: &str,
                      storage_type: Option<&str>,
                      storage_config: Option<&str>,
                      storage_credentials: Option<&str>) -> BoxedFuture<()> {
    let wallet_config = {
        let res = _wallet_config(id, storage_type, storage_config)
            .chain_err(|| "Invalid wallet config");

        ftry!(res)
    };

    let wallet_credentials = {
        let res = _wallet_credentials(passphrase, storage_credentials)
            .chain_err(|| "Invalid wallet credentials");

        ftry!(res)
    };

    wallet::create_wallet(wallet_config.as_ref(),
                          wallet_credentials.as_ref())
        .then(|res| {
            match res {
                Err(IndyError(IndyErrorKind::WalletAlreadyExistsError, _)) => Ok(()),
                r => r,
            }
        })
        .chain_err(|| "Can't create a wallet")
}

pub fn open(id: &str,
            passphrase: &str,
            storage_type: Option<&str>,
            storage_config: Option<&str>,
            storage_credentials: Option<&str>) -> BoxedFuture<i32> {
    let wallet_config = {
        let res = _wallet_config(id, storage_type, storage_config)
            .chain_err(|| "Invalid wallet config");
        ftry!(res)
    };

    let wallet_credentials = {
        let res = _wallet_credentials(passphrase, storage_credentials)
            .chain_err(|| "Invalid wallet credentials");
        ftry!(res)
    };

    wallet::open_wallet(wallet_config.as_ref(),
                        wallet_credentials.as_ref())
        .chain_err(|| "Can't open wallet")
}

fn _wallet_config(id: &str,
                  storage_type: Option<&str>,
                  storage_config: Option<&str>) -> Result<String> {
    let storage_config = storage_config
        .map(|v| serde_json::from_str::<serde_json::Value>(v))
        .map_or(Ok(None), |v| v.map(Some))
        .chain_err(|| "Invalid storage config")?;

    let wallet_config = json!({
        "id": id,
        "storage_type": storage_type,
        "storage_config": storage_config,
    });

    Ok(wallet_config.to_string())
}

fn _wallet_credentials(passphrase: &str,
                       storage_credentials: Option<&str>) -> Result<String> {
    let storage_credentials = storage_credentials
        .map(|v| serde_json::from_str::<serde_json::Value>(v))
        .map_or(Ok(None), |v| v.map(Some))
        .chain_err(|| "Invalid storage credentials")?;

    let wallet_credentials = json!({
        "key": passphrase,
        "storage_credentials": storage_credentials,
    });

    Ok(wallet_credentials.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _wallet_config_works() {
        let config = _wallet_config("id", Some("default"), Some(r#"{"path": "/wallets"}"#)).unwrap();
        assert_eq!(config, r#"{"id":"id","storage_config":{"path":"/wallets"},"storage_type":"default"}"#);

        let config = _wallet_config("id", None, Some(r#"{"path": "/wallets"}"#)).unwrap();
        assert_eq!(config, r#"{"id":"id","storage_config":{"path":"/wallets"},"storage_type":null}"#);

        let config = _wallet_config("id", None, None).unwrap();
        assert_eq!(config, r#"{"id":"id","storage_config":null,"storage_type":null}"#);
    }

    #[test]
    fn _wallet_credentials_works() {
        let config = _wallet_credentials("passphrase", Some(r#"{"key": "passphrase2"}"#)).unwrap();
        assert_eq!(config, r#"{"key":"passphrase","storage_credentials":{"key":"passphrase2"}}"#);

        let config = _wallet_credentials("passphrase", None).unwrap();
        assert_eq!(config, r#"{"key":"passphrase","storage_credentials":null}"#);
    }
}
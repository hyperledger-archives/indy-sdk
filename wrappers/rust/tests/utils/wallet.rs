extern crate futures;

use self::futures::Future;
use super::indy;
use indy::IndyError;
use utils::rand::random_string;
use indy::{WalletHandle, INVALID_WALLET_HANDLE};

static USEFUL_CREDENTIALS : &'static str =  r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;

/**
A test wallet that deletees itself when it leaves scope.

Use by calling `let wallet = Wallet::new();` and pass the `wallet.handle`.

```
use utils::wallet;
// The wallet is opened and created.
let wallet_1 = Wallet::new();
{
    let wallet_2 = Wallet::new();
    // we have the wallet and wallet handle.
    assert!(wallet.handle > 0);
}
// Now wallet_2 is out of scope, it closes and deletes itself.
assert!(wallet.handle > 0);
```

*/
pub struct Wallet {
    name: String,
    pub handle: WalletHandle,
}

impl Wallet {
    /* constructors */
    pub fn new() -> Wallet {
        let wallet_name : String = random_string(20);
        let mut wallet = Wallet { name : wallet_name , handle: INVALID_WALLET_HANDLE };
        wallet.create().unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    pub fn from_name(name: &str) -> Wallet {
        let mut wallet = Wallet { name: name.to_string(), handle: INVALID_WALLET_HANDLE };
        wallet.create().unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    /* private static method to help create config that is passed to wallet functions */
    fn create_wallet_config(wallet_name: &str) -> String {
        let config = json!({ "id" : wallet_name.to_string() }).to_string();
        return config.to_string();
    }

    /* private instance methods for open/create/etc...*/

    fn open(&mut self) -> Result<WalletHandle, IndyError> {
        let config : String = Wallet::create_wallet_config(&self.name);
        let handle = indy::wallet::open_wallet(&config, USEFUL_CREDENTIALS).wait()?;
        self.handle = handle;
        return Ok(handle);
    }

    fn create(&self) -> Result<(), IndyError> {
        let config = Wallet::create_wallet_config(&self.name);
        return indy::wallet::create_wallet(&config, USEFUL_CREDENTIALS).wait()
    }

    fn close(&self) -> Result<(), IndyError> {
        indy::wallet::close_wallet(self.handle).wait()
    }

    fn delete(&self) -> Result<(), IndyError> {
        let config : String = Wallet::create_wallet_config(&self.name);
        return indy::wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait()
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.close().unwrap();
        self.delete().unwrap();
    }
}

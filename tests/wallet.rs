extern crate rust_indy_sdk as indy;
use indy::wallet::Wallet;

use indy::ErrorCode;

mod tests {
    use super::*;

    #[test]
    fn create_delete_wallet_works() {
        let wallet_name = "create_delete_wallet_works";
        match Wallet::create("pool1", wallet_name, None, None, None) {
            Ok(..) => assert!(Wallet::delete(wallet_name, None).is_ok()),
            Err(e) => match e {
                ErrorCode::WalletAlreadyExistsError => {
                    //This is ok, just delete
                    assert!(Wallet::delete(wallet_name, None).is_ok())
                }
                _ => {
                    panic!("{:#?}", e)
                }
            }
        }
    }

    #[test]
    fn open_close_wallet_works() {
        let wallet_name = "open_wallet_works";
        let open_closure = || {
            match Wallet::open(wallet_name, None, None) {
                Ok(handle) => {
                    Wallet::close(handle).unwrap();
                    Wallet::delete(wallet_name, None).unwrap();
                },
                Err(e) => {
                    Wallet::delete(wallet_name, None).unwrap();
                    panic!("{:#?}", e);
                }
            }
        };

        match Wallet::create("pool1", wallet_name, None, None, None) {
            Err(e) => match e {
                ErrorCode::WalletAlreadyExistsError => {
                    open_closure()
                }
                _ => panic!("{:#?}", e)
            }
            _ => open_closure()
        };
    }
}

use indy::did;
use indy::IndyError;
use indy::future::Future;
use indy::WalletHandle;

pub struct Did {}

impl Did {
    pub fn new(wallet_handle: WalletHandle, my_did_json: &str) -> Result<(String, String), IndyError> {
        did::create_and_store_my_did(wallet_handle, my_did_json).wait()
    }

    pub fn replace_keys_start(wallet_handle: WalletHandle, did: &str, identity_json: &str) -> Result<String, IndyError> {
        did::replace_keys_start(wallet_handle, did, identity_json).wait()
    }

    pub fn replace_keys_apply(wallet_handle: WalletHandle, did: &str) -> Result<(), IndyError> {
        did::replace_keys_apply(wallet_handle, did).wait()
    }

    pub fn set_metadata(wallet_handle: WalletHandle, did: &str, metadata: &str) -> Result<(), IndyError> {
        did::set_did_metadata(wallet_handle, did, metadata).wait()
    }

    pub fn get_did_with_meta(wallet_handle: WalletHandle, did: &str) -> Result<String, IndyError> {
        did::get_my_did_with_metadata(wallet_handle, did).wait()
    }

    pub fn list_dids_with_meta(wallet_handle: WalletHandle) -> Result<String, IndyError> {
        did::list_my_dids_with_metadata(wallet_handle).wait()
    }

    pub fn abbreviate_verkey(did: &str, verkey: &str) -> Result<String, IndyError> {
        did::abbreviate_verkey(did, verkey).wait()
    }

    pub fn qualify_did(wallet_handle: WalletHandle, did: &str, method: &str) -> Result<String, IndyError> {
        did::qualify_did(wallet_handle, did, method).wait()
    }
}

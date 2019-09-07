use indy::did;
use indy::IndyError;
use indy::future::Future;
use indy::WalletHandle;

pub fn create_and_store_my_did(wallet_handle: WalletHandle, seed: Option<&str>) -> Result<(String, String), IndyError> {
    let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));
    did::create_and_store_my_did(wallet_handle, &my_did_json).wait()
}
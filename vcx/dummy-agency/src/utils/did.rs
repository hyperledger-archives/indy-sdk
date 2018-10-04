use errors::*;
use futures::*;
use indy::did;
use indy::errors::{Error as IndyError, ErrorKind as IndyErrorKind};

pub fn ensure_created(wallet_handle: i32, did: &str, seed: Option<&str>) -> BoxedFuture<()> {
    let did_info = {
        let res = _did_info(did, seed)
            .chain_err(|| "Invalid did info");
        ftry!(res)
    };

    did::create_and_store_my_did(wallet_handle, did_info.as_ref())
        .then(|res| match res {
            Ok(_) => Ok(()),
            Err(IndyError(IndyErrorKind::DidAlreadyExistsError, _)) => Ok(()), // Already exists
            Err(err) => Err(err),
        })
        .chain_err(|| "Can't create did")
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> BoxedFuture<String> {
    did::key_for_local_did(wallet_handle, did)
        .chain_err(|| "Can't get key for local did")
}

fn _did_info(did: &str,
             seed: Option<&str>) -> Result<String> {
    let did_info = json!({
        "did": did,
        "seed": seed,
    });

    Ok(did_info.to_string())
}
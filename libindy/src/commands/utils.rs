use std::rc::Rc;

use errors::indy::IndyError;
use errors::wallet::WalletError;
use services::pool::PoolService;
use services::wallet::WalletService;

#[macro_export]
macro_rules! check_wallet_and_pool_handles_consistency {
    ($ws:expr, $ps:expr, $wh:expr, $ph:expr, $cb:ident) => (
        if let Err(err) = check_wallet_and_pool_handles_consistency($ws.clone(), $ps.clone(), $wh, $ph) {
            $cb(Err(err));
            return;
        }
    );
}

pub fn check_wallet_and_pool_handles_consistency(ref ws: Rc<WalletService>, ref ps: Rc<PoolService>,
                                                 wallet_handle: i32, pool_handle: i32)
                                                 -> Result<(), IndyError> {
    if ws.get_pool_name(wallet_handle)?.eq(&ps.get_pool_name(pool_handle)?) {
        return Ok(());
    } else {
        return Err(IndyError::WalletError(
            WalletError::IncorrectPool(
                format!("Pool handle {} invalid for wallet handle {}",
                        pool_handle, wallet_handle))));
    }
}

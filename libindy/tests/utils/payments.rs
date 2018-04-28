use indy::api::ErrorCode;
use utils::callback::CallbackUtils;

pub struct PaymentsUtils {}

impl PaymentsUtils {
    pub fn create_payment_address(wallet_handle: i32) -> Result<> {
        let (receiver, cmd_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();
        let pname = CString::new("null_payment_plugin").unwrap();
        let cfg = CString::new("{}").unwrap();
        let errc = indy_create_payment_address(
            cmd_handle,
            wallet.unwrap(),
            pname.as_ptr(),
            cfg.as_ptr(),
            cb
        );
        let res = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    }
}
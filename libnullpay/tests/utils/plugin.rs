use nullpay;

use std::sync::{Once, ONCE_INIT};

lazy_static! {
        static ref CREATE_PAYMENT_METHOD_INIT: Once = ONCE_INIT;
}

pub fn init_plugin() {
    CREATE_PAYMENT_METHOD_INIT.call_once(|| {
        ::std::env::set_var("ENABLE_FEES", "1");
        nullpay::nullpay_init();
    });
}
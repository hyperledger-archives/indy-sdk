use std::env;

use crate::{domain::IndyConfig, services::PoolService};

pub(crate) struct ConfigController {}

impl ConfigController {
    pub(crate) fn new() -> ConfigController {
        ConfigController {}
    }

    pub(crate) fn set_runtime_config(&self, config: IndyConfig) {
        trace!("set_runtime_config > {:?}", config);

        // FIXME: Deprecate this param.
        if let Some(_crypto_thread_pool_size) = config.crypto_thread_pool_size {
            warn!("indy_set_runtime_config ! unsupported param used");
        }

        match config.collect_backtrace {
            Some(true) => env::set_var("RUST_BACKTRACE", "1"),
            Some(false) => env::set_var("RUST_BACKTRACE", "0"),
            _ => {}
        }

        if let Some(threshold) = config.freshness_threshold {
            PoolService::set_freshness_threshold(threshold);
        }

        trace!("set_runtime_config <");
    }
}

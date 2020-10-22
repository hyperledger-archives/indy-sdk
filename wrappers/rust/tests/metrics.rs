extern crate indyrs as indy;
#[macro_use]
use indy::metrics;

use indy::ErrorCode;

mod utils;
#[allow(unused_imports)]
use futures::Future;

#[cfg(test)]
mod collect {
    use super::*;

    #[test]
    fn collect_metrics() {
        let result_metrics = metrics::collect_metrics().wait().unwrap();
        assert_eq!("{}", &result_metrics);
    }
}

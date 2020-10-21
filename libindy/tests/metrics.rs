#[macro_use]
mod utils;

inject_indy_dependencies!();

extern crate indyrs as indy;
extern crate indyrs as api;


use crate::utils::metrics;

mod collect {
    use super::*;

    #[test]
    fn collect_metrics_works() {
        let result_metrics = metrics::collect_metrics().unwrap();
        println!("result_metrics");
        println!("{}", &result_metrics);
        assert_eq!("{}", &result_metrics);
    }
}

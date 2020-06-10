extern crate indyrs as indy;
extern crate indyrs as api;

use std::sync::Mutex;

use indy::future::Future;
use indy_api_types::PoolHandle;
use log::{LevelFilter, Log, Metadata, Record};

#[macro_use]
mod utils;

inject_indy_dependencies!();

struct LogCounter {}

static LOG_COUNTER: LogCounter = LogCounter {};

lazy_static! {
    static ref LOG_STAT: Mutex<[usize; 6]> = Mutex::new([0usize; 6]);
    static ref LOG_IGNORE_IN_STAT: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
}


impl Log for LogCounter {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !LOG_IGNORE_IN_STAT.lock().unwrap().contains(&record.target()) {
            LOG_STAT.lock().unwrap()[record.metadata().level() as usize] += 1;
        }
    }

    fn flush(&self) {}
}

#[test]
fn indy_set_log_max_lvl_works() {
    indy::logger::set_logger(&LOG_COUNTER).unwrap();
    unsafe { indy_sys::logger::indy_set_log_max_lvl(LevelFilter::Trace as usize as u32); }
    LOG_IGNORE_IN_STAT.lock().unwrap().push("indy::api::logger");

    indy::pool::close_pool_ledger(1 as PoolHandle).wait().unwrap_err();
    let log_stat_default = LOG_STAT.lock().unwrap().clone();

    indy::pool::close_pool_ledger(1 as PoolHandle).wait().unwrap_err();
    let log_stat_default_2 = LOG_STAT.lock().unwrap().clone();

    unsafe { indy_sys::logger::indy_set_log_max_lvl(LevelFilter::Off as usize as u32); }
    indy::pool::close_pool_ledger(1 as PoolHandle).wait().unwrap_err();
    let log_stat_off = LOG_STAT.lock().unwrap().clone();

    unsafe { indy_sys::logger::indy_set_log_max_lvl(LevelFilter::max() as usize as u32); }
    indy::pool::close_pool_ledger(1 as PoolHandle).wait().unwrap_err();
    let log_stat_all = LOG_STAT.lock().unwrap().clone();

    unsafe { indy_sys::logger::indy_set_log_max_lvl(LevelFilter::Debug as usize as u32); }
    indy::pool::close_pool_ledger(1 as PoolHandle).wait().unwrap_err();
    let log_stat_no_trace = LOG_STAT.lock().unwrap().clone();

    // make sure pool close operation triggers some logging
    assert_ne!(log_stat_default[LevelFilter::Debug as usize], log_stat_default_2[LevelFilter::Debug as usize]);
    assert_ne!(log_stat_default[LevelFilter::Trace as usize], log_stat_default_2[LevelFilter::Trace as usize]);

    // check logging off results
    assert_eq!(log_stat_default_2[LevelFilter::Debug as usize], log_stat_off[LevelFilter::Debug as usize]);
    assert_eq!(log_stat_default_2[LevelFilter::Trace as usize], log_stat_off[LevelFilter::Trace as usize]);

    // check re-enabled logs
    assert_ne!(log_stat_off[LevelFilter::Debug as usize], log_stat_all[LevelFilter::Debug as usize]);
    assert_ne!(log_stat_off[LevelFilter::Trace as usize], log_stat_all[LevelFilter::Trace as usize]);

    // check disabled trace
    assert_ne!(log_stat_all[LevelFilter::Debug as usize], log_stat_no_trace[LevelFilter::Debug as usize]);
    assert_eq!(log_stat_all[LevelFilter::Trace as usize], log_stat_no_trace[LevelFilter::Trace as usize]);
}

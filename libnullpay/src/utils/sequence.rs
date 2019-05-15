use std::sync::atomic::{AtomicUsize, Ordering};

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = AtomicUsize::new(1);
}

pub fn get_next_id() -> i32 {
        (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
    }

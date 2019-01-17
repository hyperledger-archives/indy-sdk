use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT; //TODO use AtomicI32
}

pub fn get_next_id() -> i32 {
        (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
    }

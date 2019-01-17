use std::time::Duration;

pub fn short_timeout() -> Duration {
    Duration::from_secs(10)
}

pub fn medium_timeout() -> Duration {
    Duration::from_secs(300)
}

pub fn long_timeout() -> Duration {
    Duration::from_secs(1000)
}

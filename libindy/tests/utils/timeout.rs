use std::time::Duration;

pub fn short_timeout() -> Duration {
    Duration::from_secs(5)
}

pub fn medium_timeout() -> Duration {
    Duration::from_secs(20)
}

pub fn long_timeout() -> Duration {
    Duration::from_secs(200)
}

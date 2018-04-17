use std::time::Duration;

pub struct TimeoutUtils {}

impl TimeoutUtils {
    pub fn short_timeout() -> Duration {
        Duration::from_secs(5)
    }

    pub fn medium_timeout() -> Duration {
        Duration::from_secs(10)
    }

    pub fn long_timeout() -> Duration {
        Duration::from_secs(100)
    }
}
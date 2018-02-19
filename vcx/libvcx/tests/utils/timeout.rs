use std::time::Duration;
pub struct TimeoutUtils {}
impl TimeoutUtils {
    #[allow(dead_code)]
    pub fn short_timeout() -> Duration {
        Duration::from_secs(5)
    }

    #[allow(dead_code)]
    pub fn medium_timeout() -> Duration {
        Duration::from_secs(10)
    }

    #[allow(dead_code)]
    pub fn long_timeout() -> Duration {
        Duration::from_secs(100)
    }
}
use std::time::Duration;

pub struct TimeoutUtils {}

impl TimeoutUtils {
    pub fn short_timeout() -> Duration {
        Duration::from_secs(5)
    }

    pub fn medium_timeout() -> Duration {
        Duration::from_secs(15)
    }

    pub fn long_timeout() -> Duration {
        Duration::from_secs(50)
    }

    pub fn some_long() -> Option<Duration> { Some(TimeoutUtils::long_timeout())}

    pub fn some_medium() -> Option<Duration> { Some(TimeoutUtils::medium_timeout())}

    pub fn some_short() -> Option<Duration> { Some(TimeoutUtils::short_timeout())}
}
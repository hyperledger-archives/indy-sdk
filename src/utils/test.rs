use utils::environment::EnvironmentUtils;

use std::fs;

pub struct TestUtils {}

impl TestUtils {
    pub fn cleanup_sovrin_home() {
        let path = EnvironmentUtils::sovrin_home_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }
}

macro_rules! assert_match {
    ($var:expr, $pattern:pat) => (
        assert!(match $var {
            $pattern => true,
            _ => false
        })
    );
}
use utils::environment::EnvironmentUtils;

use std::fs;

pub struct TestUtils {}

impl TestUtils {
    pub fn cleanup_indy_home() {
        let path = EnvironmentUtils::indy_home_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    pub fn cleanup_temp() {
        let path = EnvironmentUtils::tmp_path();
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    pub fn cleanup_storage() {
        TestUtils::cleanup_indy_home();
        TestUtils::cleanup_temp();
    }
}

#[cfg(test)]
macro_rules! assert_match {
    ($pattern:pat, $var:expr) => (
        assert!(match $var {
            $pattern => true,
            _ => false
        })
    );
    ($pattern:pat, $var:expr, $val_in_pattern:ident, $exp_value:expr) => (
        assert!(match $var {
            $pattern => $val_in_pattern == $exp_value,
            _ => false
        })
    );
    ($pattern:pat, $var:expr, $val_in_pattern1:ident, $exp_value1:expr, $val_in_pattern2:ident, $exp_value2:expr) => (
        assert!(match $var {
            $pattern => $val_in_pattern1 == $exp_value1 && $val_in_pattern2 == $exp_value2,
            _ => false
        })
    );
}
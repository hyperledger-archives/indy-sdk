use std::env;
use std::path::PathBuf;

pub struct EnvironmentUtils {}

impl EnvironmentUtils {
    pub fn indy_home_path() -> PathBuf {
        // TODO: FIXME: Provide better handling for the unknown home path case!!!
        let mut path = env::home_dir().unwrap_or(PathBuf::from("/home/indy"));
        path.push(if cfg!(target_os = "ios") { "Documents/.indy" } else { ".indy" });
        path
    }

    pub fn wallet_home_path() -> PathBuf {
        let mut path = EnvironmentUtils::indy_home_path();
        path.push("wallet");
        path
    }

    pub fn wallet_path(wallet_name: &str) -> PathBuf {
        let mut path = EnvironmentUtils::wallet_home_path();
        path.push(wallet_name);
        path
    }

    pub fn pool_home_path() -> PathBuf {
        let mut path = EnvironmentUtils::indy_home_path();
        path.push("pool");
        path
    }

    pub fn pool_path(pool_name: &str) -> PathBuf {
        let mut path = EnvironmentUtils::pool_home_path();
        path.push(pool_name);
        path
    }

    pub fn tmp_path() -> PathBuf {
        let mut path = env::temp_dir();
        path.push("indy");
        path
    }

    pub fn tmp_file_path(file_name: &str) -> PathBuf {
        let mut path = EnvironmentUtils::tmp_path();
        path.push(file_name);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indy_home_path_works() {
        let path = EnvironmentUtils::indy_home_path();

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains(".indy"));
    }

    #[test]
    fn wallet_home_path_works() {
        let path = EnvironmentUtils::wallet_home_path();

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains(".indy"));
        assert!(path.to_string_lossy().contains("wallet"));
    }

    #[test]
    fn wallet_path_works() {
        let path = EnvironmentUtils::wallet_path("wallet1");

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains(".indy"));
        assert!(path.to_string_lossy().contains("wallet1"));
    }

    #[test]
    fn pool_home_path_works() {
        let path = EnvironmentUtils::pool_home_path();

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains(".indy"));
        assert!(path.to_string_lossy().contains("pool"));
    }

    #[test]
    fn pool_path_works() {
        let path = EnvironmentUtils::pool_path("pool1");

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains(".indy"));
        assert!(path.to_string_lossy().contains("pool1"));
    }

    #[test]
    fn tmp_path_works() {
        let path = EnvironmentUtils::tmp_path();

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains("indy"));
    }

    #[test]
    fn tmp_file_path_works() {
        let path = EnvironmentUtils::tmp_file_path("test.txt");

        assert!(path.is_absolute());
        assert!(path.has_root());
        assert!(path.to_string_lossy().contains("indy"));
        assert!(path.to_string_lossy().contains("test.txt"));
    }
}
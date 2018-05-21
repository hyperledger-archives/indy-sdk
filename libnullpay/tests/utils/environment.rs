use std::path::PathBuf;
use std::env;

pub fn tmp_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("indy_client");
    path
}

pub fn indy_home_path() -> PathBuf {
    // TODO: FIXME: Provide better handling for the unknown home path case!!!
    let mut path = env::home_dir().unwrap_or(PathBuf::from("/home/indy"));
    path.push(if cfg!(target_os = "ios") { "Documents/.indy_client" } else { ".indy_client" });
    path
}
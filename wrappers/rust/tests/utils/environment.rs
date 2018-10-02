use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
pub fn test_pool_ip() -> String {
    env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string())
}

pub fn tmp_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("indy_client");
    path
}

pub fn tmp_file_path(file_name: &str) -> PathBuf {
    let mut path = tmp_path();
    path.push(file_name);
    path
}

pub fn pool_home_path() -> PathBuf {
    let mut path = indy_home_path();
    path.push("pool");
    path
}

pub fn pool_path(pool_name: &str) -> PathBuf {
    let mut path = pool_home_path();
    path.push(pool_name);
    path
}

pub fn indy_home_path() -> PathBuf {
    // TODO: FIXME: Provide better handling for the unknown home path case!!!
    let mut path = env::home_dir().unwrap_or(PathBuf::from("/home/indy"));
    let mut indy_client_dir = ".indy_client";
    if cfg!(target_os = "ios"){
        indy_client_dir = "Documents/.indy_client";
    }
    path.push(indy_client_dir);

    if cfg!(target_os = "android"){
        android_create_indy_client_dir();
        path = android_indy_client_dir_path();
    }
    path
}
pub fn android_indy_client_dir_path() -> PathBuf{
    let external_storage= env::var("EXTERNAL_STORAGE");
    let android_dir :String;
    match external_storage {
        Ok (val) => android_dir = val + "/.indy_client",
        Err(err) => {
            panic!("Failed to find external storage path {:?}", err)
        }
    }

    PathBuf::from(android_dir)
}

pub fn android_create_indy_client_dir(){
    //Creates directory only if it is not present.
    fs::create_dir_all(android_indy_client_dir_path().as_path()).unwrap();
}

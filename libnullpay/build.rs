use std::env;

fn main() {
    println!("cargo:rustc-link-lib=dylib=indy.dll");

    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    if target.find("-windows-").is_some() {
        let profile = env::var("PROFILE").unwrap();
        println!("profile={}", profile);

        let indy_dir = env::var("INDY_DIR").unwrap_or(format!("..\\libindy\\target\\{}", profile));
        println!("indy_dir={}", indy_dir);

        println!("cargo:rustc-flags=-L {}", indy_dir);
        return;
    }
}

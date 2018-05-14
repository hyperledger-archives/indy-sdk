use std::env;

fn main() {
    println!("cargo:rustc-link-lib=dylib=indy");

    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    if target.find("-windows-").is_some() {
        println!("cargo:rustc-link-lib=dylib=ssleay32");
        println!("cargo:rustc-link-lib=dylib=zmq");
        println!("cargo:rustc-link-lib=dylib=sodium");
        let prebuilt_dir = env::var("INDY_PREBUILT_DEPS_DIR").unwrap();
        println!("cargo:rustc-flags=-L {}\\lib", prebuilt_dir);
        return;
    }
}

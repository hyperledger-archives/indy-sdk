use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=indy");

    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    if target.find("-windows-").is_some() {
        println!("cargo:rustc-link-lib=ssleay32");
        println!("cargo:rustc-link-lib=zmq");
        println!("cargo:rustc-link-lib=sodium");

        // TODO: FIXME: Provide more reliable dependencies resolving

        let output_dir = env::var("OUT_DIR").unwrap();
        let prebuilt_dir = env::var("INDY_DIR").unwrap();

        let dst = Path::new(&output_dir[..]).join("..\\..\\..");
        let prebuilt_lib = Path::new(&prebuilt_dir[..]).join("lib");

        println!("cargo:rustc-link-search=native={}", prebuilt_dir);
        println!("cargo:rustc-flags=-L {}\\lib", prebuilt_dir);
        println!("cargo:include={}\\include", prebuilt_dir);

        let files = vec!["indy.dll", "libeay32md.dll", "libsodium.dll", "libzmq.dll", "ssleay32md.dll"];
        for f in files.iter() {
            if let Ok(_) = fs::copy(&prebuilt_lib.join(f), &dst.join(f)) {
                println!("copy {} -> {}", &prebuilt_lib.join(f).display(), &dst.join(f).display());
            }
        }
        return;
    } 
}

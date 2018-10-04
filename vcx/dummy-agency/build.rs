use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    if target.find("-windows-").is_some() {
        println!("cargo:rustc-link-lib=indy.dll");

        let profile = env::var("PROFILE").unwrap();
        println!("profile={}", profile);

        let output_dir = env::var("OUT_DIR").unwrap();
        println!("output_dir={}", output_dir);
        let output_dir = Path::new(output_dir.as_str());

        let indy_dir = env::var("INDY_DIR").unwrap_or(format!("..\\..\\libindy\\target\\{}", profile));
        println!("indy_dir={}", indy_dir);
        let indy_dir = Path::new(indy_dir.as_str());

        let dst = output_dir.join("..\\..\\..");
        println!("cargo:rustc-flags=-L {}", indy_dir.as_os_str().to_str().unwrap());

        let files = vec!["indy.dll", "libeay32md.dll", "libsodium.dll", "libzmq.dll", "ssleay32md.dll"];
        for f in files.iter() {
            if let Ok(_) = fs::copy(&indy_dir.join(f), &dst.join(f)) {
                println!("copy {} -> {}", &indy_dir.join(f).display(), &dst.join(f).display());
            }
        }
    } else {
        println!("cargo:rustc-link-lib=indy");
    }
}

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);
    match target.find("-linux-") {
        Some(..) => {
            println!("cargo:rustc-link-lib=indy");
            println!("cargo:rustc-link-search=native=/usr/lib");
            return;
        }
        None => {}
    }
}

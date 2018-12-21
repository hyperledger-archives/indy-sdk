#[cfg(not(target_env = "msvc"))]
extern crate pkg_config;
extern crate regex;
#[cfg(target_env = "msvc")]
extern crate vcpkg;

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBINDY_DIR");
    println!("cargo:rerun-if-env-changed=LIBINDY_STATIC");
    println!("cargo:rerun-if-env-changed=LIBINDY_PKG");

    if cfg!(target_env = "msvc") {
        // vcpkg requires to set env VCPKGRS_DYNAMIC
        println!("cargo:rerun-if-env-changed=VCPKGRS_DYNAMIC");
    }

    let use_static = cfg!(any(target_os="android", target_os="ios")) || env::var("LIBINDY_STATIC").is_ok();
    let use_dir = env::var("LIBINDY_DIR").is_ok();
    let use_pkg = env::var("LIBINDY_USE_PKG_CONFIG").is_ok();

    if use_dir && use_pkg {
        panic!("LIBINDY_DIR is incompatible with LIBINDY_USE_PKG_CONFIG. Set the only one env variable");
    }

    if use_pkg {
        find_pkg(use_static)
    } else {
        find_dir(env::var("LIBINDY_DIR").ok(), use_static)
    }
}

fn find_dir(dir_name: Option<String>, use_static: bool) {
    match (use_static, cfg!(windows)) {
        (true, true) => println!("cargo:rustc-link-lib=indy"),
        (true, false) => println!("cargo:rustc-link-lib=static=indy"),
        (false, true) => println!("cargo:rustc-link-lib=indy.dll"),
        (false, false) => println!("cargo:rustc-link-lib=dylib=indy"),
    };

    if let Some(dir_name) = dir_name {
        println!("cargo:rustc-link-search=native={}", dir_name);
    }
}

#[cfg(not(target_env = "msvc"))]
fn find_pkg(use_static: bool) {
    match pkg_config::Config::new().statik(use_static).probe("libindy") {
        Ok(_) => println!("cargo:warning=Libindy found in pkgcfg tree."),
        Err(e) => panic!(format!("Error: {:?}", e)),
    }
}

#[cfg(target_env = "msvc")]
fn find_pkg(_use_static: bool) {
    match vcpkg::Config::new().emit_includes(true).lib_name("libindy").probe("indy") {
        Ok(_) => println!("cargo:warning=Libindy found in vcpkg tree."),
        Err(e) => panic!(format!("Error: {:?}", e)),
    }
}
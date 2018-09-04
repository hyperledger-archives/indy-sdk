extern crate pkg_config;
#[cfg(target_env = "msvc")]
extern crate vcpkg;
extern crate regex;

use regex::Regex;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let re = Regex::new(r"(?:android|ios|aarch64|arm)").unwrap();

    println!("cargo:rerun-if-env-changed=LIBINDY_DIR");
    match env::var("LIBINDY_DIR") {
        Ok(libindy_dir) => {
            let libindy_path = PathBuf::from(&libindy_dir);

            if !Path::new(&libindy_path).exists() {
                panic!("Libindy library does not exist: {}", libindy_path.to_string_lossy());
            }
            println!("cargo:rustc-link-search=native={}",libindy_dir);

            if re.is_match(&target) || env::var("LIBINDY_STATIC").is_ok(){
                println!("cargo:rustc-link-lib=static=indy");
            } else {
                println!("cargo:rustc-link-lib=dylib=indy");
            }
        },
        Err(..) => {
            try_pkg_config();
            try_vcpkg();

            //Couldn't find it through pkg_config, so try looking in default locations
            println!("cargo:rustc-link-lib=indy");

            if target.contains("darwin") {
                println!("cargo:rustc-link-search=native=/usr/local/lib");
            } else if target.contains("-linux-") {
                println!("cargo:rustc-link-search=native=/usr/lib");
            } else if target.contains("-windows-") {
                println!("cargo:rustc-link-lib=dylib=ssleay32");
                println!("cargo:rustc-link-lib=dylib=zmq");
                println!("cargo:rustc-link-lib=dylib=sodium");
                let prebuilt_dir = env::var("INDY_PREBUILT_DEPS_DIR").unwrap();
                println!("cargo:rustc-link-search=native={}\\lib", prebuilt_dir);
            }
        }
    };
}

fn try_pkg_config() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    // If we're going to windows-gnu we can use pkg-config, but only so long as
    // we're coming from a windows host.
    //
    // Otherwise if we're going to windows we probably can't use pkg-config.
    if target.contains("windows-gnu") && host.contains("windows") {
        env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    } else if target.contains("windows") {
        return;
    }

    let lib = match pkg_config::Config::new().print_system_libs(false).find("indy") {
        Ok(lib) => lib,
        Err(..) => {
            match pkg_config::find_library("libindy") {
                Ok(lib) => lib,
                Err(e) => {
                    println!("Couldn't find libindy from pkgconfig ({:?})", e);
                    return
                }
            }
        }
    };

    for include in lib.include_paths.iter() {
        println!("cargo:include={}", include.display());
    }

    std::process::exit(0);
}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg() {}

#[cfg(target_env = "msvc")]
fn try_vcpkg() {
    // vcpkg will not emit any metadata if it can not find libraries
    // appropriate for the target triple with the desired linkage.

    let mut lib = vcpkg::Config::new()
        .emit_includes(true)
        .lib_name("libindy")
        .probe("indy");

    if let Err(e) = lib {
        println!("note: vcpkg did not find ffi-sdk as libindy : {:?}", e);
        return;
    }

    let lib = lib.unwrap();

    println!("cargo:rustc-link-lib=indy");

    std::process::exit(0);
}

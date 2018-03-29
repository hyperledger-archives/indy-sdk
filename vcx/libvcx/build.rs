use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

extern crate toml;

#[macro_use]
extern crate serde_derive;

// error in rust compiler.  Bugfix requested in Sept. 2017
// these are used, but the compiler is not seeing it for
// some reason
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
// error in rust compiler.  Bugfix has been submitted in Sept. 2017
#[allow(unused_imports)]
#[macro_use]
extern crate serde;

// used in formatting the Cargo.toml file
#[derive(Deserialize, Debug)]
struct Tomlfile {
    contents: Contents,
}

// used in formatting the Cargo.toml file
#[derive(Deserialize, Debug)]
struct Metadata {
    deb: Deb,
}

// used in formatting the Cargo.toml file
#[derive(Deserialize, Debug)]
struct Deb {
    revision: Option<String>,
}


// used in formatting the Cargo.toml file
#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: Option<String>,
    metadata: Metadata,
}

// used in formatting the Cargo.toml file
#[derive(Deserialize, Debug)]
struct Contents {
    package: Package,
    dependencies: Option<toml::Value>,
}

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);
    if target.contains("darwin"){
        //OSX specific logic
        println!("cargo:rustc-link-lib=indy");
        //OSX does not allow 3rd party libs to be installed in /usr/lib. Instead install it in /usr/local/lib
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }
    if target.contains("linux"){
        //Linux specific logic
        println!("cargo:rustc-link-lib=indy");
        println!("cargo:rustc-link-search=native=/usr/lib");
    }

    match env::var("CARGO_FEATURE_CI") {
        Ok(_) => {
            println!("injecting version information");
            // Leaving as unwrap, this is in the build script.
            let revision = get_revision().unwrap();
            write_variables(&revision);
        },
        Err(_) => {println!("NOT injecting version information"); },
    };
}


// Writes to the file 'src/utils/version_constants.rs' for use
// in outputing the version dynamically.
fn write_variables(revision:&str) {
    let out_dir = "src/utils/";
    let dest_path = Path::new(&out_dir).join("version_constants.rs");
    let mut f = File::create(&dest_path).unwrap();
    let s = format!("pub const VERSION: &'static str = env!(\"CARGO_PKG_VERSION\");\npub const REVISION: &'static str = \"{}\";\n", revision);
    if let Err(e) = f.write_all(s.as_bytes()) {
       panic!("Error creating version_constants.rs: {}", e);
    };

}

// Gets the revision number from the Cargo.toml file.
pub fn get_revision() -> Option<String> {
    let dir = match  env::var("CARGO_MANIFEST_DIR"){
        Ok(d) => d,
        Err(_) => panic!("Couldn't Manifest Directory"),
    };
    let filename = "Cargo.toml";
    let p = format!("{}/{}",dir,filename);
    let mut input = String::new();
    File::open(p).and_then(|mut f| {
        f.read_to_string(&mut input)}).unwrap();
    let tomlfile:Contents = toml::from_str(&input).unwrap();
    let revision:String = match tomlfile.package.metadata.deb.revision {
        Some(v) => v,
        None => String::from(""),
    };
    Some(format!("+{}", revision))
}



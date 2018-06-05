fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    //println!("cargo:rustc-link-search=/Projects/stack-overflow/using-c-static/");
    println!("cargo:rustc-link-search=native={}","/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/libvcx/target/" + target + "/release");
    println!("cargo:rustc-link-lib=static=vcx");
}

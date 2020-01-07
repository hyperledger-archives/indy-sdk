#[cfg(all(unix, test))]
pub fn load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::os::unix::Library::open(Some(library), ::libc::RTLD_NOW | ::libc::RTLD_NODELETE)
        .map(libloading::Library::from)
}

#[cfg(any(not(unix), not(test)))]
pub fn load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::Library::new(library)
}
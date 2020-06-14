use settings;
use indy::ErrorCode;

static INIT_PLUGIN: std::sync::Once = std::sync::Once::new();

pub fn init_plugin(library: &str, initializer: &str) {
    settings::set_config_value(settings::CONFIG_PAYMENT_METHOD, settings::DEFAULT_PAYMENT_METHOD);

    INIT_PLUGIN.call_once(|| {
        if let Ok(lib) = _load_lib(library) {
            unsafe {
                if let Ok(init_func) = lib.get(initializer.as_bytes()) {
                    let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = init_func;

                    match init_func() {
                        ErrorCode::Success => {
                            debug!("Plugin has been loaded: {:?}", library);
                        }
                        _ => {
                            error!("Plugin has not been loaded: {:?}", library);
                            std::process::exit(123);
                        }
                    }
                } else {
                    error!("Init function not found: {:?}", initializer);
                    std::process::exit(123);
                }
            }
        } else {
            error!("Plugin not found: {:?}", library);
            std::process::exit(123);
        }
    });
}

#[cfg(all(unix, test, not(target_os = "android")))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::os::unix::Library::open(Some(library), libc::RTLD_NOW | libc::RTLD_NODELETE)
        .map(libloading::Library::from)
}

#[cfg(any(not(unix), not(test), target_os = "android"))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::Library::new(library)
}
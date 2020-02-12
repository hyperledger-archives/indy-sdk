//#[link(name="libvcx", kind = "static")]
extern "C" {
    fn vcx_version() -> *const c_char;
    fn vcx_agent_provision_async(command_handle : u32,
            json: *const c_char,
            cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, config: *const c_char)>) -> u32;
}

//extern crate libloading as lib;
extern crate libc;

use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::{mem, thread};
use self::libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<u8>>,
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(0)),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn get_version() -> &'static str {
    unsafe {
        //let vcx_version: lib::Symbol<unsafe extern fn() -> *const c_char> = libvcxall.get(b"vcx_version")?;
        let version = vcx_version();
        let c_str: &CStr = { CStr::from_ptr(version) };
        let str_slice: &str = c_str.to_str().unwrap();
        str_slice
    }
}

extern "C" fn generic_cb(command_handle: CommandHandle, err: u32, config: *const c_char) {
    if err != 0 {panic!("generic_cb failed")}
    //check_useful_c_str!(config, ());
    println!("successfully called generic_cb: {:?}", config);
    println!("Unpark the thread");
    let s = singleton();
    let mut data = s.inner.lock().unwrap();
    *data = 9 as u8;
    //waiting_thread.thread().unpark();
    //waiting_thread.join().unwrap();
    //thread::unpark();
}

fn do_provision() -> u32 {
    unsafe {
        //let vcx_agent_provision_async: lib::Symbol<
        //    unsafe extern fn(command_handle : u32,
        //        json: *const c_char,
        //        cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, config: *const c_char)>) -> u32>
        //    = libvcxall.get(b"vcx_agent_provision_async")?;

        let json_string = r#"{"agency_url": "https://cagency.pdev.evernym.com", "agency_did": "dTLdJqRZLwMuWSogcKfBT","wallet_name":"wallet2","wallet_key":"wallet-key","agent_seed":null,"enterprise_seed":null, "agency_verkey": "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH"}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_agent_provision_async(0, c_json, Some(generic_cb));
        result
    }
}

fn main() {
    //let libcrypto = lib::Library::new("/data/vcxlib/libcrypto.so").unwrap();
    //let libz = lib::Library::new("/data/vcxlib/libz.so").unwrap();
    //let libzmq = lib::Library::new("/data/vcxlib/libzmq.so").unwrap();
    //let libssl = lib::Library::new("/data/vcxlib/libssl.so").unwrap();
    //let libindy = lib::Library::new("/data/vcxlib/libindy.so").unwrap();
    //let libvcx = lib::Library::new("/data/vcxlib/libvcx.so").unwrap();
    //let libvcxall = lib::Library::new("/data/vcxlib/libvcxall.so").unwrap();
    //let libsodium = lib::Library::new("/data/vcxlib/libsodium.so").unwrap();
    //let lib = lib::Library::new("libvcxall.so")?;
    //panic!("hi bob");

    let version = get_version();
    println!("Version: {:?}", version);

    let result = do_provision();
    println!("Provision result: {:?}", result);
    
    // Let's use the singleton in a few threads
    // let threads: Vec<_> = (0..10)
    //     .map(|i| {
    //         thread::spawn(move || {
    //             thread::sleep(Duration::from_millis(i * 100));
    //             let s = singleton();
    //             let mut data = s.inner.lock().unwrap();
    //             *data = i as u8;
    //             println!("Data is: {}", *data);
    //         })
    //     })
    //     .collect();

    // And let's check the singleton every so often
    //for _ in 0u8..20 {
    loop {
        thread::sleep(Duration::from_millis(100));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("Singleton data is: {}", *data);
        if *data >= 8 {
            break;
        }
    }

    // for thread in threads.into_iter() {
    //     thread.join().unwrap();
    // }
}

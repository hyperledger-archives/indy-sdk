macro_rules! c_str {
    ($x:ident) => {
        ::std::ffi::CString::new($x).unwrap()
    };
    ($x:expr) => {
        ::std::ffi::CString::new($x).unwrap()
    }
}

macro_rules! rust_str {
    ($x:ident) => {
        unsafe { ::std::ffi::CStr::from_ptr($x).to_str().unwrap().to_string() }
    }
}

pub mod did;
pub mod errors;
pub mod wallet;




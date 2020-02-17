extern crate  libc;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::Mutex;
use std::slice;
use std::hash::Hash;

pub const POISON_MSG: &str = "FAILED TO LOCK CALLBACK MAP!";

pub fn build_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null(){
        return None;
    }

    let cstr: &CStr = unsafe {
        CStr::from_ptr(ptr)
    };

    match cstr.to_str() {
        Ok(s) => Some(s.to_string()),
        Err(e) => {
            warn!("String from libindy with malformed utf8: {}",e);
            None
        }
    }
}

pub fn build_buf(ptr: *const u8, len: u32) -> Vec<u8>{
    let data = unsafe {
        slice::from_raw_parts(ptr, len as usize)
    };

    data.to_vec()
}

pub fn get_cb<H: Eq + Hash,T>(command_handle: H, map: &Mutex<HashMap<H, T>>) -> Option<T> {
    //TODO Error case, what should we do if the static map can't be locked? Some what
    //TODO general question for all of our Mutexes.
    let mut locked_map = map.lock().expect(POISON_MSG);
    match locked_map.remove(&command_handle){
        Some(t) => Some(t),
        None => {
            warn!("Unable to find callback in map for libindy call");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use utils::devsetup::SetupDefaults;

    fn cstring(str_val: &String) -> CString {
        CString::new(str_val.clone()).unwrap()
    }

    #[test]
    fn test_build_string() {
        let _setup = SetupDefaults::init();

        let test_str = "Journey before destination".to_string();

        let test = build_string(cstring(&test_str).as_ptr());
        assert_eq!(test_str, test.unwrap());
    }

    #[test]
    fn test_get_cb(){
        let _setup = SetupDefaults::init();

        let mutex_map: Mutex<HashMap<i32, Box<dyn FnMut(i32) + Send>>> = Default::default();
        assert!(get_cb(2123, &mutex_map).is_none());

        let closure: Box<dyn FnMut(i32) + Send> = Box::new(move |_ | {

        });

        mutex_map.lock().unwrap().insert(2123,closure);
        let cb = get_cb(2123, &mutex_map);
        assert!(cb.is_some());
    }
}
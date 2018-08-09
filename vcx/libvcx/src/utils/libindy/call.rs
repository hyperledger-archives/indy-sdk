// This was an idea that is not ready. Not sure it would be a good idea. A different function would
// need to be written for each combination of inputs and outputs from libindy. That could be a lot.
// We could consider just creating ones for the must common.

//extern crate libc;
//
//use std::ffi::CString;
//use utils::libindy::types::*;
//use utils::libindy::error_codes::map_indy_error_code;
//use self::libc::c_char;
//use utils::libindy::{map_string_error, indy_function_eval};
//
//
//
//type fn_i32_r_i32 = unsafe extern "C" fn(i32, i32, Option<extern "C" fn(i32, i32)>) -> i32;
//type fn_str_r_i32 = unsafe extern "C" fn(i32, *const c_char, Option<extern "C" fn(i32, i32)>) -> i32;
//type fn_str_i32_r_i32_str = unsafe extern "C" fn(i32, *const c_char, i32, Option<extern "C" fn(i32, i32, *const c_char)>) -> i32;
//
//pub fn call_i32_r_i32(arg1: i32, func: fn_i32_r_i32) -> Result<(), u32> {
//    let rtn_obj = Return_I32::new()?;
//    unsafe {
//        indy_function_eval(func(rtn_obj.command_handle,
//                                arg1,
//                                Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }
//
//    rtn_obj.receive()
//}
//
//pub fn call_str_r_i32(arg1: String, func: fn_str_r_i32) -> Result<(), u32> {
//    let arg1_cstr = CString::new(arg1).map_err(map_string_error)?;
//    let rtn_obj = Return_I32::new()?;
//
//    unsafe {
//        indy_function_eval(
//            func(rtn_obj.command_handle,
//                 arg1_cstr.as_ptr(),
//                 Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }
//
//    rtn_obj.receive()
//}
//
//pub fn call_str_i32_r_i32_str(arg1: String, arg2: i32, func: fn_str_i32_r_i32_str) -> Result<Option<String>, u32> {
//    let arg1_cstr = CString::new(arg1).map_err(map_string_error)?;
//    let rtn_obj = Return_I32_STR::new()?;
//
//    unsafe {
//        indy_function_eval(
//            func(rtn_obj.command_handle,
//                 arg1_cstr.as_ptr(),
//                 arg2,
//                 Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }
//
//    rtn_obj.receive()
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::time::Duration;
//    use std::thread;
//
//    extern "C" fn test_indy_func(command_handle: i32, arg1: i32, cb: Option<extern "C" fn(i32, i32)>) -> i32 {
//        thread::spawn(move ||{
//            thread::sleep(Duration::from_millis(10));
//            cb.unwrap()(command_handle,0);
//        });
//        0
//    }
//
//
//    #[test]
//    fn test_call_i32_r_i32() {
//        call_i32_r_i32(44, test_indy_func).unwrap();
//    }
//}
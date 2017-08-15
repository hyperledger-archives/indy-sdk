macro_rules! get_byte_array {
    ($x:ident, $l:expr, $e:expr) => {
        if $x.is_null() {
            return $e
        }

        let $x =  unsafe { slice::from_raw_parts($x, $l as usize) };
        let $x = $x.to_vec();
    }
}

//Returnable pointer is valid only before first vector modification
pub fn vec_to_pointer(v: &Vec<u8>) -> (*const u8, u32) {
    let len = v.len() as u32;
    (v.as_ptr() as *const u8, len)
}
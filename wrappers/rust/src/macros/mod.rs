macro_rules! c_str {
    ($x:ident) => {
        CString::new($x).unwrap()
    };
    ($x:expr) => {
        CString::new($x).unwrap()
    }
}

macro_rules! opt_c_str {
    ($x:ident) => {
        $x.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap())
    }
}

macro_rules! opt_c_str_json {
    ($x:ident) => {
        $x.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("{}").unwrap())
    }
}

macro_rules! opt_c_ptr {
    ($x:ident, $y:ident) => {
        if $x.is_some() { $y.as_ptr() } else { null() }
    }
}

macro_rules! rust_str {
    ($x:ident) => {
        unsafe { CStr::from_ptr($x).to_str().unwrap().to_string() }
    }
}

macro_rules! opt_rust_str {
    ($x:ident) => {
        if $x.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr($x).to_str().unwrap().to_string() })
        }
    };
}

macro_rules! rust_slice {
    ($x:ident, $y:ident) => {
        unsafe { ::std::slice::from_raw_parts($x, $y as usize) }
    }
}

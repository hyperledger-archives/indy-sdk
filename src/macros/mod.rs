macro_rules! c_str {
    ($x:ident) => {
        CString::new($x).unwrap()
    }
}

macro_rules! opt_c_str {
    ($x:ident) => {
        $x.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap())
    }
}

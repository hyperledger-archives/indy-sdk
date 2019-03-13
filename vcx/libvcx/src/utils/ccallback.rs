macro_rules! check_useful_c_callback {
    ($x:ident, $e:expr) => {
        let $x = match $x {
            Some($x) => $x,
            None => return VcxError::from_msg($e, "Invalid callback has been passed").into()
        };
    }
}
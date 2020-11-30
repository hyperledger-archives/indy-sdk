macro_rules! check_useful_c_callback {
    ($x:ident, $e:expr) => {
        let $x = match $x {
            Some($x) => $x,
            None => return VcxError::from_msg($e, "Invalid callback has been passed").into()
        };
    }
}

#[macro_export]
macro_rules! check_u32_less_or_eq {
    ($x:ident, $lim:expr, $e:expr) => {
        let $x: u32 = if $x <= $lim {
            $x
        } else {
            return err_msg($e.into(), format!("Invalid integer has been passed (should be non-negative and less or equal to {}", $lim)).into()
        };
    }
}
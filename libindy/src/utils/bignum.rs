extern crate indy_crypto;

use self::indy_crypto::bn::{BigNumber};

/*
macro_rules! check_useful_bignum_decimal_str {
    ($x:ident, $e:expr) => {
        let $x = match CStringUtils::c_str_to_string($x) {
            Ok(Some(val)) => {
                match BigNumber::from_dec(&val) {
                    Ok(n) => n,
                    _ => return $e,
                }
            },
            _ => return $e,
        };

        if $x.is_empty() {
            return $e
        }
    }
}*/

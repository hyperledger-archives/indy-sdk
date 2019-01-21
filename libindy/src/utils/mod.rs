macro_rules! memzeroize {
    ($thing:ident, $member1:ident) => {
        impl Zeroize for $thing {
            fn zeroize(&mut self) {
                self.$member1.zeroize();
            }
        }

        impl Drop for $thing {
            fn drop(&mut self) {
                self.$member1.zeroize();
            }
        }
    };
    ($thing:ident, $member1:tt) => {
        impl Zeroize for $thing {
            fn zeroize(&mut self) {
                self.$member1.zeroize();
            }
        }

        impl Drop for $thing {
            fn drop(&mut self) {
                self.$member1.zeroize();
            }
        }
    };
}

pub mod environment;

#[macro_use]
pub mod ctypes;

#[macro_use]
pub mod ccallback;

pub mod crypto;
#[macro_use]
pub mod logger;

#[cfg(test)]
pub mod inmem_wallet;

#[allow(unused_macros)]
#[macro_use]
pub mod result;

pub mod sequence;

#[cfg(test)]
#[macro_use]
#[allow(unused_macros)]
pub mod test;

#[macro_use]
pub mod try;

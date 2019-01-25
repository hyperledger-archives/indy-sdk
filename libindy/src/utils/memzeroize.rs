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

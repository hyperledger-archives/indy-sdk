pub struct BigNumber {

}

impl BigNumber {
    pub fn new() -> BigNumber {
        BigNumber {}
    }

    pub fn safe_prime() -> BigNumber {
        BigNumber {}
    }

    pub fn from_hex(hex: &str) -> BigNumber {
        unimplemented!();
    }

    pub fn from_bytes(bytes: &[u8]) -> BigNumber {
        unimplemented!();
    }

    pub fn to_hex(&self) -> String {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        unimplemented!();
    }

    pub fn add(&self, bn: &BigNumber) -> BigNumber {
        unimplemented!();
    }

    pub fn mul(&self, bn: &BigNumber) -> BigNumber {
        unimplemented!();
    }

    pub fn exp(&self, bn: &BigNumber) -> BigNumber {
        unimplemented!();
    }

    pub fn sub(&self, bn: &BigNumber) -> BigNumber {
        unimplemented!();
    }

    fn modulus(&self, bn: &BigNumber) -> BigNumber {
        unimplemented!();
    }


}
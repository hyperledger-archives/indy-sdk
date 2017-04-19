use errors::crypto::CryptoError;

pub struct PointG1 {}

pub struct PointG2 {}

pub struct GroupOrderElement {}

pub struct Pair {}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        unimplemented!();
    }

    pub fn mul(&mut self, gr: &mut GroupOrderElement) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn add(&mut self, q: &mut PointG1) -> Result<&mut PointG1, CryptoError> {
        //unimplemented!();
        Ok(self)
    }

    pub fn to_string(&mut self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl GroupOrderElement {
    pub fn new() -> Result<GroupOrderElement, CryptoError> {
        // returns random element in 0, ..., GroupOrder-1
        unimplemented!();
    }

    pub fn pow(&mut self, e: &mut GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        // need to use powmod where n - group_order
        unimplemented!();
    }

    pub fn add(&mut self, r: &GroupOrderElement) -> Result<&mut GroupOrderElement, CryptoError> {
        //need to use rmod after add
        //unimplemented!();
        Ok(self)
    }

    pub fn inverse(&mut self) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&mut self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&mut self, b: &mut [u8]) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }
}
extern crate bs58;

pub trait FromBase58 {
    fn from_base58(&self) -> Result<Vec<u8>, bs58::decode::Error>;
    fn from_base58_check(&self) -> Result<Vec<u8>, bs58::decode::Error>;
}

impl<I: AsRef<[u8]>> FromBase58 for I {
    fn from_base58(&self) -> Result<Vec<u8>, bs58::decode::Error> {
        bs58::decode(self).into_vec()
    }

    fn from_base58_check(&self) -> Result<Vec<u8>, bs58::decode::Error> {
        bs58::decode(self).with_check(None).into_vec()
    }
}


pub trait IntoBase58 {
    fn into_base58(&self) -> String;
    fn into_base58_check(&self) -> String;
}


impl<I: AsRef<[u8]>> IntoBase58 for I {
    fn into_base58(&self) -> String {
        bs58::encode(self).into_string()
    }

    fn into_base58_check(&self) -> String {
        bs58::encode(self).with_check().into_string()
    }
}
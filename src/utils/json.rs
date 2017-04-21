use rustc_serialize::{Decodable, Encodable, json};
use std::string::String;

pub trait JsonEncodable: Encodable + Sized {
    fn encode(&self) -> Result<String, json::EncoderError> {
        json::encode(self)
    }
}

pub trait JsonDecodable: Decodable {
    fn decode(encoded: &str) -> Result<Self, json::DecoderError> {
        json::decode(encoded)
    }
}
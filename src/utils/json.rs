extern crate serde;
extern crate serde_json;

use self::serde::{Serialize, Deserialize};
use self::serde_json::Error;
use std::string::String;


pub trait JsonEncodable: Serialize + Sized {
    fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self)
    }
}

pub trait JsonDecodable<'a>: Deserialize<'a> {
    fn from_str(to_stringd: &'a str) -> Result<Self, Error> {
        serde_json::from_str(to_stringd)
    }
}

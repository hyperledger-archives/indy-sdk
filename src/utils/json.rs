extern crate serde;
extern crate serde_json;

use self::serde::{Serialize, Deserialize};
use self::serde_json::Error;
use std::string::String;
use errors::anoncreds::AnoncredsError;
use errors::common::CommonError;


pub trait JsonEncodable: Serialize + Sized {
    fn encode(&self) -> Result<String, Error> {
        serde_json::to_string(self)
    }
}

pub trait JsonDecodable<'a>: Deserialize<'a> {
    fn decode(encoded: &'a str) -> Result<Self, Error> {
        serde_json::from_str(encoded)
    }
}

impl From<Error> for AnoncredsError {
    fn from(err: Error) -> AnoncredsError {
        AnoncredsError::CommonError(CommonError::InvalidStructure(err.to_string()))
    }
}
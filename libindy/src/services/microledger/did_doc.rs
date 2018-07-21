use services::microledger::view::View;
use std::collections::HashMap;
use errors::common::CommonError;

pub struct DidDoc {
    pub did: String,
}


impl View for DidDoc where Self: Sized {
    // initialize
    fn new(name: &str, options: HashMap<String, String>) -> Result<Self, CommonError> {
        Ok(DidDoc {did: "a".to_string()})
    }
}

use std::collections::HashMap;

use errors::common::CommonError;


pub trait View where Self: Sized {
    // initialize
    fn new(name: &str, options: HashMap<String, String>) -> Result<Self, CommonError>;
    // apply txns
    fn apply_txn(&mut self, txn: &str) -> Result<(), CommonError>;
}

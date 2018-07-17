use std::collections::HashMap;

use errors::common::CommonError;
//use services::microledger::view::View;


pub trait Microledger where Self: Sized {
    // initialize
    fn new(name: &str, options: HashMap<String, String>) -> Result<Self, CommonError>;
    // Gets sha256 root of merkle tree
    fn get_root_hash(&self) -> String;
    // Gets no of txns in ledger
    fn get_size(&self) -> usize;
    // Add a txn and return seq no
    fn add(&self, txn: &str) -> Result<usize, CommonError>;
    // get txns in seq_no range [from, to]
    //fn get(&self, from: u64, to: Option<u64>) -> Vec<String>;
    /*// registers a view
    fn register_view(&self, view: View);
    // deregisters a view
    fn deregister_view(&self, view_id: &str);*/
}

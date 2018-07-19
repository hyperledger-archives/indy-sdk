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
    fn add(&mut self, txn: &str) -> Result<usize, CommonError>;
    // Add multiple txns and return start and end seq no
    fn add_multiple(&mut self, txns: Vec<&str>) -> Result<(usize, usize), CommonError>;
    // get txns in seq_no range [from, to]
    fn get(&self, from: u64, to: Option<u64>) -> Result<Vec<String>, CommonError>;
    // get txns in seq_no range [from, to], including the seq no
    fn get_with_seq_no(&self, from: u64, to: Option<u64>) -> Result<Vec<(u64, String)>, CommonError>;
    /*// registers a view
    fn register_view(&self, view: View);
    // deregisters a view
    fn deregister_view(&self, view_id: &str);*/
}

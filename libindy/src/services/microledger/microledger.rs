//use services::microledger::view::View;


pub trait Microledger {
    // initialize
    fn new(name: &str) -> Self;
    // Add a txn and return seq no
    //fn add(&self, txn: &str) -> u64;
    // Gets sha256 root of merkle tree
    //fn get_root_hash(&self) -> String;
    // Gets no of txns in ledger
    //fn get_size(&self) -> u64;
    // get txns in seq_no range [from, to]
    //fn get(&self, from: u64, to: Option<u64>) -> Vec<String>;
    /*// registers a view
    fn register_view(&self, view: View);
    // deregisters a view
    fn deregister_view(&self, view_id: &str);*/
}

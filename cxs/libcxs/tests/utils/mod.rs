
#[macro_use]
pub mod cstring;
pub mod timeout;
pub mod demo;
pub mod claim_def_wallet;
//use std::fs;
//use std::path::Path;

//pub fn create_genesis_txn_file(){
//    fs::File::create("/tmp/genesis.txn").unwrap();
//}

//pub fn deletes_default_genesis_txn_file(){
//    let filename = "/tmp/genesis.txn";
//    if Path::new(filename).exists() {
//        println!("{}", format!("Removing file for testing: {}.", &filename));
//        fs::remove_file(filename).unwrap();
//    }
//}




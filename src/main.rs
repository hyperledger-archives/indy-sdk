extern crate sovrin_client;

use sovrin_client::SovrinClient;

fn cb(arg: String) {
    println!("callback {}", arg);
}

pub fn main() {
    let mut sovrin_client: SovrinClient = SovrinClient::new();
    sovrin_client.init(Box::new(cb));
    sovrin_client.do_call(&["qwe"]);
    sovrin_client.deinit();
}
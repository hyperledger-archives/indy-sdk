/*
Example demonstrating Proof Verification.

First Issuer creates Claim Definition for existing Schema.
After that, it issues a Claim to Prover (as in issue_credential.py example)

Once Prover has successfully stored its Claim, it uses Proof Request that he
received, to get Claims which satisfy the Proof Request from his wallet.
Prover uses the output to create Proof, using its Master Secret.
After that, Proof is verified against the Proof Request
*/

// ------------------------------------------
// crates.io
// ------------------------------------------
#[macro_use]
extern crate serde_json;


// ------------------------------------------
// hyperledger crates
// ------------------------------------------
extern crate indy;                      // rust wrapper project

use std::io::Write;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use indy::did::Did;
use indy::wallet::Wallet;
use indy::crypto::Crypto;

static USEFUL_CREDENTIALS: &'static str = r#"{"key": "12345678901234567890123456789012"}"#;
static FILE: &'static str = "message.txt";

fn main() {
    let (wallet_handle, verkey, other_verkey) = init();

    loop {
        println!("ToDo");

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_string();

        if cmd == "prep" {
            prep(wallet_handle, &verkey, &other_verkey);
        } else if cmd == "read" {
            read(wallet_handle, &verkey);
        } else if cmd == "quit" {
            break;
        } else {
            println!("wrong")
        }
    }
}

fn init() -> (i32, String, String) {
    let mut cmd = String::new();

    println!("Who are you? ");
    io::stdin().read_line(&mut cmd).unwrap();

    let config = json!({ "id" : format!("{}-wallet", cmd) }).to_string();
    Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();
    let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

    let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
    println!("My DID and Verkey: {} {}", did, verkey);

    println!("Other party's DID and Verkey? ");
    let mut other = String::new();
    io::stdin().read_line(&mut other).unwrap();
    let other_verkey = other.trim().split(" ").collect::<Vec<&str>>()[1].trim().to_string();

    (wallet_handle, verkey, other_verkey)
}

fn prep(wallet_handle: i32, sender_vk: &str, receipt_vk: &str) {
    let mut file = File::create(FILE).unwrap();

    println!("Enter message");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    let encrypted_msg = Crypto::auth_crypt(wallet_handle, &sender_vk, &receipt_vk, message.trim().as_bytes()).unwrap();
    file.write_all(&encrypted_msg).unwrap();
}

fn read(wallet_handle: i32, receipt_vk: &str) {
    let mut file = File::open(FILE).unwrap();

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let (sender, decrypted_msg) = Crypto::auth_decrypt(wallet_handle, &receipt_vk, &contents).unwrap();
    println!("Sender Verkey: {:?}", sender);
    println!("Decrypted message: {:?}", str::from_utf8(&decrypted_msg).unwrap());
}
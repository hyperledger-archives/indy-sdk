extern crate ansi_term;
extern crate rust_base58;
use std::env;
use zmq;
use base64;
use indy_utils::crypto::ed25519_sign;
use self::rust_base58::FromBase58;
use failure::Context;
extern crate indy_api_types;
use indy_api_types::errors::prelude::*;
use serde::{de, Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use zmq::Error;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LedgerStatus {
    pub txnSeqNo: usize,
    pub merkleRoot: String,
    pub ledgerId: u8,
    pub ppSeqNo: Option<u32>,
    pub viewNo: Option<u32>,
    pub protocolVersion: Option<usize>,
}


fn main() {

    let mut args = env::args();
    args.next();

    let mut dest: String = "".to_string();
    let mut zmq_ip: String = "".to_string();
    let mut zmq_port: String = "".to_string();
    let mut timeout: i64 = 15;
    let mut tries_count: i32 = 3;

    if args.len() == 0 {
        _print_help();
        return;
    }
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return _print_help(),
            "--dest" => {
                dest = args.next().unwrap_or_default();
            }
            "--zmq_ip" => {
                zmq_ip = args.next().unwrap_or_default();
            }
            "--zmq_port" => {
                zmq_port = args.next().unwrap_or_default();
            }
            "--timeout" => {
                timeout = args.next().unwrap_or_default().parse::<i64>().unwrap();
            }
            "--tries-count" => {
                tries_count = args.next().unwrap_or_default().parse::<i32>().unwrap();
            }
            _ => {
                println!("Unknown option {}", arg);
                return _print_help();
            }
        }
    }
    match _connect_to_validator(&dest, &zmq_ip, &zmq_port) {
        Err(why) => panic!("{:?}", why),
        Ok(sock) => match _send_test_msg(&sock, timeout, tries_count) {
            Ok(reply) => {
                println!("ZMQ CONNECTION IS POSSIBLE!!!");
            },
            Err(err) => {
                println!("Error: {}", err);
                _print_recom(dest, zmq_ip, zmq_port);
            },
        }
    }
}

fn _send_test_msg(sock: &zmq::Socket, timeout: i64, tries_count: i32) -> Result<String, String> {
    let msg = "{ \"op\": \"LEDGER_STATUS\", \"txnSeqNo\": 0, \"merkleRoot\": null, \"ledgerId\": 0, \"ppSeqNo\": null, \"viewNo\": null, \"protocolVersion\": 2}";
    match sock.send(&msg, 0) {
        Ok(()) => {
            println!("Successfully sent message: {}", msg.to_string());
            for i in 0..tries_count {
                match _wait_for_response(&sock, timeout) {
                    Ok(reply) => {
                        println!("Got reply from validator");
                        return Ok(reply)
                    },
                    Err(_) => {
                        println!("Make another try for checking")
                    },
                }
            };
            return Err("Cannot connect to remote server".to_string());
        },

        Err(err) => {
            println!("Error {} was occurred", err);
            return Err(std::format!("{:?}", err))
        },
    }

    Ok("".to_string())
}

fn _wait_for_response(sock: &zmq::Socket, timeout: i64) -> Result<String, String> {
    let mut pool_items = [sock.as_poll_item(zmq::POLLIN)];
    println!("Waiting for {} seconds for getting reply from server", timeout);
    zmq::poll(&mut pool_items, timeout * 1000).unwrap();
    if pool_items[0].is_readable() {
        match sock.recv_string(0) {
            Ok(in_result) => match in_result {
                Ok(rep) => {
                    return Ok(rep);
                },
                Err(err) => {
                    println!("Error {:?} was occurred", err);
                    return Err(std::format!("{:?}", err))
                }
            }
            Err(err) => return Err(std::format!("{:?}", err)),
        };
    }
    Err("".to_string())
}

fn _connect_to_validator(dest: &String, address: &String, port: &String) -> Result<zmq::Socket, String> {
    let zmq_context = zmq::Context::new();
    let mut zmq_sock = zmq_context.socket(zmq::SocketType::DEALER).unwrap();
    let key_pair = zmq::CurveKeyPair::new().expect("FIXME");
    let zaddr = std::format!("tcp://{}:{}", address, port);
    let node_verkey = dest
        .as_str()
        .from_base58()
        .map_err(Context::new)
        .to_indy(IndyErrorKind::InvalidStructure, "Error while transform to base58").unwrap();

    let node_verkey = ed25519_sign::PublicKey::from_slice(&node_verkey)
        .and_then(|vk| ed25519_sign::vk_to_curve25519(&vk))
        .to_indy(IndyErrorKind::InvalidStructure, "Error while transform to curve25519").unwrap();
    let public_key = node_verkey[..].to_vec();
    zmq_sock.set_identity(base64::encode(&key_pair.public_key)
        .as_bytes())
        .unwrap();
    zmq_sock.set_curve_secretkey(&key_pair.secret_key)
        .unwrap();
    zmq_sock.set_curve_publickey(&key_pair.public_key)
        .unwrap();
    zmq_sock.set_curve_serverkey(zmq::z85_encode(public_key.as_slice())
        .to_indy(IndyErrorKind::InvalidStructure, "Can't encode server key as z85")
        .unwrap()
        .as_bytes())
        .unwrap();
    zmq_sock.set_linger(0);
    println!("Trying to connect to {}", zaddr);
    match zmq_sock.connect(&zaddr) {
        Ok(()) => {
            println!("Connection should be created");
            return Ok(zmq_sock)
        },
        Err(err) => {
            println!("{}", err);
            return Err(std::format!("{:?}", err))
        },
    }
}


fn _print_args(dest: String, zmq_ip: String, zmq_port: String ) {
    println!("Dest is: {}", dest);
    println!("ZMQ address {}", zmq_ip);
    println!("ZMQ port {}", zmq_port);
}

fn _print_help() {
    println!("Check zmq connection");
    println!();
    println!("Parameters:");
    println!();
    println!("--dest <target_nym>");
    println!("--zmq_ip <ip address>");
    println!("--zmq_port <port for zmq connections>");
    println!("Optional parameters:");
    println!("--timeout <timeout in seconds>  Timeout for each response waiting from server (default: 15 seconds)");
    println!("--tries-count <count>   Count of tries for waiting response from server (default: 3)");
    println!();
}


fn _print_recom(dest: String, zmq_ip: String, zmq_port: String){
    println!("Looks like ZMQ connection to {}:{} IS NOT POSSIBLE!!!", zmq_ip, zmq_port);
    println!("Don't panic.");
    println!("Please check that address and port you provide are corrected.");
    println!("Secondly, please check, that validator has exactly given dest: {}", dest);
    println!("Then, maybe you should to check firewall rules on validator's node, that can drop/reject incoming traffic");
}

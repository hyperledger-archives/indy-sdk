extern crate rust_base58;
extern crate ursa;
extern crate clap;

use zmq;
use base64;
use rust_base58::FromBase58;
use clap::{Arg, App};

use ursa::{
    keys::PublicKey,
    signatures::ed25519::Ed25519Sha512,
};

const EXPECTED_PK_LENGTH : usize = 32;

fn main() {

    let timeout: i64;
    let tries_count: i32;

    let args = App::new("Tool For checking ZeroMQ connection to validator node")
        .version("1.0.0")
        .arg(Arg::with_name("dest")
            .required(true)
            .long("dest")
            .empty_values(false)
            .value_name("Target NYM")
            .help("Target NYM")
            .index(1))
        .arg(Arg::with_name("zmq_ip")
            .required(true)
            .long("zmq_ip")
            .empty_values(false)
            .value_name("zmq ip")
            .help("IP address of validator node which used for client's connections")
            .index(2))
        .arg(Arg::with_name("zmq_port")
            .required(true)
            .long("zmq_port")
            .empty_values(false)
            .value_name("zmq port")
            .help("Port number of validator node which used for client's connections")
            .index(3))
        .arg(Arg::with_name("timeout")
            .long("timeout")
            .short("t")
            .empty_values(false)
            .value_name("seconds")
            .default_value("15")
            .help("Timeout in seconds for waiting reply from server"))
        .arg(Arg::with_name("tries-count")
            .long("tries-count")
            .short("c")
            .empty_values(false)
            .value_name("int")
            .default_value("3")
            .help("Timeout in seconds for waiting reply from server"))
        .get_matches();
    let dest = args.value_of("dest").unwrap().to_string();
    let zmq_ip = args.value_of("zmq_ip").unwrap().to_string();
    let zmq_port = args.value_of("zmq_port").unwrap().to_string();
    match args.value_of("timeout").unwrap().to_string().parse::<i64>() {
        Ok(_t) => {
            timeout = _t
        }
        Err(_err) => {
            println!("Error was caused while parsing input value to integer. \
            Please check input value for '--timeout' parameter");
            return;
        }
    }
    match args.value_of("tries-count").unwrap().to_string().parse::<i32>(){
        Ok(_tc) => {
            tries_count = _tc
        }
        Err(_err) => {
            println!("Error was caused while parsing input value to integer. \
            Please check input value for '--tries-count' parameter");
            return;
        }
    }
    match _connect_to_validator(&dest, &zmq_ip, &zmq_port) {
        Err(why) => panic!("{:?}", why),
        Ok(sock) => match _send_test_msg(&sock, timeout, tries_count) {
            Ok(_reply) => {
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
    let msg = r#"{ "op": "LEDGER_STATUS", "txnSeqNo": 0, "merkleRoot": null, "ledgerId": 0, "ppSeqNo": null, "viewNo": null, "protocolVersion": 2}"#;
    match sock.send(&msg, 0) {
        Ok(()) => {
            println!("Successfully sent message: {}", msg.to_string());
            for _i in 0..tries_count {
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
}

fn _wait_for_response(sock: &zmq::Socket, timeout: i64) -> Result<String, ()> {
    let mut pool_items = [sock.as_poll_item(zmq::POLLIN)];
    println!("Waiting for {} seconds for getting reply from server", timeout);
    zmq::poll(&mut pool_items, timeout * 1000).expect("Error while polling ZMQ socket [Internal Error]");
    if pool_items[0].is_readable() {
        match sock.recv_string(0) {
            Ok(Ok(rep)) => return Ok(rep),
            _ => return Err(())
        };
    }
    Err(())
}

fn _connect_to_validator(dest: &String, address: &String, port: &String) -> Result<zmq::Socket, String> {
    let zmq_context = zmq::Context::new();
    let zmq_sock = zmq_context.socket(zmq::SocketType::DEALER)
        .map_err(|_| format!("Error while creating a socket instance [Internal Error]"))
        .unwrap();
    let key_pair = zmq::CurveKeyPair::new().expect("FIXME");
    let zaddr = format!("tcp://{}:{}", address, port);
    let node_verkey = dest
        .as_str()
        .from_base58()
        .map_err(|err| format!("Error while transform to base58: {:?} [Internal Error]", err))
        .unwrap();

    if node_verkey.len() != EXPECTED_PK_LENGTH {
        return Err(format!("Public key which is got from dest {} \
        has wrong length (expected length of Public Key is 32)", dest))
    }
    let node_verkey = Ed25519Sha512::ver_key_to_key_exchange(&PublicKey(node_verkey))
        .map_err(|err| format!("Cannot convert key to curve25519 key: {:?} [Internal Error]", err))
        .unwrap();

    let public_key = node_verkey[..].to_vec();
    zmq_sock.set_identity(base64::encode(&key_pair.public_key)
        .as_bytes())
        .expect("Error while setting identity for ZMQ socket [Internal Error]");
    zmq_sock.set_curve_secretkey(&key_pair.secret_key)
        .expect("Error while setting secret key for ZMQ socket [Internal Error]");
    zmq_sock.set_curve_publickey(&key_pair.public_key)
        .expect("Error while setting public key for ZMQ socket [Internal Error]");
    zmq_sock.set_curve_serverkey(zmq::z85_encode(public_key.as_slice())
        .map_err(|err| format!("Can't encode server key as z85: {:?}", err))?
        .as_bytes())
        .unwrap();
    zmq_sock.set_linger(0)
        .expect("Error while setting LINGER option for ZMQ socket [Internal Error]");
    println!("Trying to connect to {}", zaddr);
    match zmq_sock.connect(&zaddr) {
        Ok(()) => {
            println!("Connection should be created");
            return Ok(zmq_sock)
        },
        Err(err) => {
            println!("{}", err);
            return Err(format!("{:?}", err))
        },
    }
}

fn _print_recom(dest: String, zmq_ip: String, zmq_port: String){
    println!("Looks like ZMQ connection to {}:{} IS NOT POSSIBLE!!!", zmq_ip, zmq_port);
    println!("Don't panic.");
    println!("Please check that address and port you provide are corrected.");
    println!("Secondly, please check, that validator has exactly given dest: {}", dest);
    println!("Then, maybe you should to check firewall rules on validator's node, that can drop/reject incoming traffic");
}

extern crate indy_crypto;
extern crate rmp_serde;

use self::indy_crypto::bn::BigNumber;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Policy {
    pub address: BigNumber,
    pub agents: HashMap<String, PolicyAgent> // key is the verkey from the PolicyAgent
}

impl Policy {
    pub fn new(address: BigNumber, agents: HashMap<String, PolicyAgent>) -> Policy {
        Policy {
            address,
            agents
        }
    }
}

impl JsonEncodable for Policy {}

impl<'a> JsonDecodable<'a> for Policy {}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PolicyAgent {
    pub verkey: String,
    pub secret: Option<BigNumber>,
    pub double_commitment: Option<BigNumber>, // can be generated from secret, blinding factor and policy address
    pub blinding_factor: Option<BigNumber>,
    pub blinding_factor_1: Option<BigNumber>,
    pub witness: Option<BigNumber>,
}

impl PolicyAgent {
    pub fn new(verkey: String, secret: Option<BigNumber>, double_commitment: Option<BigNumber>,
               blinding_factor: Option<BigNumber>, blinding_factor_1: Option<BigNumber>,
               witness: Option<BigNumber>) -> PolicyAgent {
        PolicyAgent {
            verkey,
            secret,
            double_commitment,
            blinding_factor,
            blinding_factor_1,
            witness
        }
    }
}

impl JsonEncodable for PolicyAgent {}

impl<'a> JsonDecodable<'a> for PolicyAgent {}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PolicyAgentInfo {
    pub seed: Option<String>,
    pub crypto_type: Option<String>,
    pub secret: Option<BigNumber>,
}

impl PolicyAgentInfo {
    pub fn new(seed: Option<String>, crypto_type: Option<String>, secret: Option<BigNumber>) -> PolicyAgentInfo {
        PolicyAgentInfo {
            seed,
            crypto_type,
            secret
        }
    }
}

impl JsonEncodable for PolicyAgentInfo {}

impl<'a> JsonDecodable<'a> for PolicyAgentInfo {}
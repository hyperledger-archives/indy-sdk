extern crate indy_crypto;

use self::indy_crypto::bn::BigNumber;
use self::indy_crypto::authz::helpers::{generate_policy_address, gen_double_commitment_to_secret};
use self::indy_crypto::authz::constants::*;
use errors::authz::AuthzError;
use self::types::*;

use utils::crypto::base58::Base58;

use services::signus::SignusService;
use services::signus::types::Key;
use services::signus::types::KeyInfo;

pub mod types;
pub mod constants;

use std::rc::Rc;
use std::collections::HashMap;


pub struct AuthzService {
    crypto_service: Rc<SignusService>
}

// Should it be singleton
impl AuthzService {
    pub fn new(crypto_service: Rc<SignusService>) -> AuthzService { AuthzService { crypto_service } }

    pub fn get_double_commitment_to_secret(secret: &BigNumber, policy_address: &BigNumber) -> Result<(BigNumber, BigNumber), AuthzError> {
        let g_1 = BigNumber::from_dec(G_1_1).unwrap();
        let g_2 = BigNumber::from_dec(G_2_1).unwrap();
        let h_1 = BigNumber::from_dec(G_1_2).unwrap();
        let h_2 = BigNumber::from_dec(G_2_2).unwrap();
        let mod_1 = BigNumber::from_dec(P_1).unwrap();
        let mod_2 = BigNumber::from_dec(P_2).unwrap();
        let mut ctx = BigNumber::new_context()?;
        Ok(gen_double_commitment_to_secret(&g_1, &h_1, &secret, &g_2, &h_2,
                                           &policy_address, &mod_1,
                                           &mod_2, &mut ctx)?)
    }

    pub fn generate_new_policy(&self) -> Result<Policy, AuthzError> {
        let address = generate_policy_address()?;
        let agents:HashMap<String, PolicyAgent> = HashMap::new();
        let policy = Policy::new(address, agents);
        Ok(policy)
    }

    pub fn generate_new_agent(&self, policy_address: &BigNumber, agent_info: Option<&PolicyAgentInfo>) -> Result<(PolicyAgent, Key), AuthzError> {
        let (vk, sk, secret, comm, blinding_factor) = match agent_info {
            Some(ref info) => {
                let (vk, sk) = match self.crypto_service.get_crypto_name_and_keypair(&info.crypto_type, &info.seed) {
                    Ok((_, vk, sk)) => (vk, sk),
                    Err(err) => return Err(AuthzError::SignusError(err)),
                };

                let (comm, blinding_factor) = match info.secret {
                    Some(ref s) => {
                        let (c, b) = AuthzService::get_double_commitment_to_secret(&s, policy_address)?;
                        (Some(c), Some(b))
                    },
                    None => (None, None)
                };
                (vk, sk, info.secret.as_ref().map(|bn| bn.clone().unwrap()), comm, blinding_factor)
            },
            None => {
                let (vk, sk) = match self.crypto_service.get_crypto_name_and_keypair(&None, &None) {
                    Ok((_, vk, sk)) => (vk, sk),
                    Err(err) => return Err(AuthzError::SignusError(err)),
                };
                (vk, sk, None, None, None)
            }
        };

        let vk = Base58::encode(&vk);
        let sk = Base58::encode(&sk);

        Ok((PolicyAgent::new(vk.clone(), secret, comm, blinding_factor, None), Key::new(vk, sk)))
    }

    pub fn add_new_agent_to_policy(&self, policy: &mut Policy,
                                   agent_info: Option<&PolicyAgentInfo>) -> Result<Key, AuthzError> {
        let (agent, key) = self.generate_new_agent(&policy.address,
                                                   agent_info)?;
        policy.agents.insert(agent.verkey.clone(), agent);
        Ok(key)
    }

    pub fn add_new_agent_to_policy_with_verkey(&self, policy: &mut Policy, verkey: String,
                                               secret: Option<BigNumber>) -> Result<String, AuthzError> {
        let (comm, blinding_factor) = match secret {
            Some(ref s) => {
                let (c, b) = AuthzService::get_double_commitment_to_secret(&s, &policy.address)?;
                (Some(c), Some(b))
            },
            None => (None, None)
        };
        let agent = PolicyAgent::new(verkey.clone(), secret, comm, blinding_factor, None);
        policy.agents.insert(agent.verkey.clone(), agent);
        Ok(verkey)
    }

    pub fn update_agent_witness(&self, policy: &mut Policy, agent_key:String, witness: &BigNumber)  -> Result<(), AuthzError> {
        match policy.agents.get_mut(&agent_key) {
            Some(agent) => {
                if agent.secret.is_some() {
                    agent.witness = Some(witness.clone()?);
                    Ok(())
                } else {
                    Err(
                        AuthzError::AgentHasNoSecretError(
                            format!("Agent with key {} has no secret", agent_key)))
                }

            }
            None => Err(
                AuthzError::AgentDoesNotExistError(
                    format!("Policy has no agent with key: {}", agent_key)))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_new_authz_service() -> AuthzService {
        let crypto_service: Rc<SignusService> = Rc::new(SignusService::new());
        AuthzService::new(crypto_service.clone())
    }

    fn check_new_agent(agent: &PolicyAgent, verkey: String, no_secret: bool) {
        assert_eq!(agent.verkey, verkey);
        if no_secret {
            assert!(agent.secret.is_none());
            assert!(agent.double_commitment.is_none());
            assert!(agent.blinding_factor.is_none());
        } else {
            assert!(agent.secret.is_some());
            assert!(agent.double_commitment.is_some());
            assert!(agent.blinding_factor.is_some());
        }
        assert!(agent.witness.is_none());
    }

    #[test]
    fn test_new_policy() {
        let authz_service = get_new_authz_service();
        let new_policy = authz_service.generate_new_policy().unwrap();
        println!("{:?}", new_policy);
        assert!(&new_policy.agents.is_empty())
    }

    #[test]
    fn test_new_agent_without_info() {
        let authz_service = get_new_authz_service();
        let new_policy = authz_service.generate_new_policy().unwrap();
        let (agent, key) = authz_service.generate_new_agent(&new_policy.address, None).unwrap();
        println!("{:?}", agent);
        check_new_agent(&agent, key.verkey, true);
    }

    #[test]
    fn test_new_agent_without_secret() {
        let authz_service = get_new_authz_service();
        let new_policy = authz_service.generate_new_policy().unwrap();
        let info = PolicyAgentInfo::new(None, None, None);
        let (agent, key) = authz_service.generate_new_agent(&new_policy.address, Some(&info)).unwrap();
        println!("{:?}", agent);
        check_new_agent(&agent, key.verkey, true);
    }

    #[test]
    fn test_new_agent_with_secret() {
        let authz_service = get_new_authz_service();
        let new_policy = authz_service.generate_new_policy().unwrap();
        let secret = BigNumber::rand(SECRET_SIZE).unwrap();
        let info = PolicyAgentInfo::new(None, None, Some(secret));
        let (agent, key) = authz_service.generate_new_agent(&new_policy.address, Some(&info)).unwrap();
        println!("{:?}", agent);
        check_new_agent(&agent, key.verkey, false);
    }

    #[test]
    fn test_update_policy_with_agents() {
        let authz_service = get_new_authz_service();
        let mut new_policy = authz_service.generate_new_policy().unwrap();
        assert_eq!(new_policy.agents.len(), 0);

        let secret1 = BigNumber::rand(SECRET_SIZE).unwrap();
        let info1 = PolicyAgentInfo::new(None, None, Some(secret1));
        let key1 = authz_service.add_new_agent_to_policy(&mut new_policy, Some(&info1)).unwrap();
        assert_eq!(new_policy.agents.len(), 1);
        {
            let agent1 = new_policy.agents.get(&key1.verkey).unwrap();
            check_new_agent(agent1, key1.verkey, false);
        }


        let secret2 = BigNumber::rand(SECRET_SIZE).unwrap();
        let info2 = PolicyAgentInfo::new(None, None, Some(secret2));
        let key2 = authz_service.add_new_agent_to_policy(&mut new_policy, Some(&info2)).unwrap();
        assert_eq!(new_policy.agents.len(), 2);
        {
            let agent2 = new_policy.agents.get(&key2.verkey).unwrap();
            check_new_agent(agent2, key2.verkey, false);
        }

        let info3 = PolicyAgentInfo::new(None, None, None);
        let key3 = authz_service.add_new_agent_to_policy(&mut new_policy, Some(&info3)).unwrap();
        assert_eq!(new_policy.agents.len(), 3);
        {
            let agent3 = new_policy.agents.get(&key3.verkey).unwrap();
            check_new_agent(agent3, key3.verkey, true);
        }

        let k0 = authz_service.crypto_service.create_key(&KeyInfo::new(None, None)).unwrap();
        let (vk, sk) = (k0.verkey, k0.signkey);
        let sk_raw = Base58::decode(&sk).unwrap();
        let sk_num = BigNumber::from_bytes(sk_raw.as_slice()).unwrap();
        let key4 = authz_service.add_new_agent_to_policy_with_verkey(&mut new_policy, vk.clone(), Some(sk_num)).unwrap();
        assert_eq!(vk, key4);
        assert_eq!(new_policy.agents.len(), 4);
        {
            let agent4 = new_policy.agents.get(&vk).unwrap();
            check_new_agent(agent4, vk, false);
        }

        let k1 = authz_service.crypto_service.create_key(&KeyInfo::new(None, None)).unwrap();
        let vk1 = k1.verkey;
        let key5 = authz_service.add_new_agent_to_policy_with_verkey(&mut new_policy, vk1.clone(), None).unwrap();
        assert_eq!(vk1, key5);
        assert_eq!(new_policy.agents.len(), 5);
        {
            let agent5 = new_policy.agents.get(&vk1).unwrap();
            check_new_agent(agent5, vk1, true);
        }
    }

    #[test]
    fn test_update_policy_agent_with_witness() {
        let authz_service = get_new_authz_service();
        let mut new_policy = authz_service.generate_new_policy().unwrap();
        let secret = BigNumber::rand(SECRET_SIZE).unwrap();
        let info1 = PolicyAgentInfo::new(None, None, Some(secret));
        let key1 = authz_service.add_new_agent_to_policy(&mut new_policy, Some(&info1)).unwrap();
        {
            let agent = new_policy.agents.get_mut(&key1.verkey).unwrap();
            check_new_agent(agent, key1.verkey.clone(), false);
        }
        let witness1 = BigNumber::rand(1024).unwrap();
        authz_service.update_agent_witness(&mut new_policy, key1.verkey.clone(), &witness1).unwrap();
        {
            let agent = new_policy.agents.get(&key1.verkey).unwrap();
//            assert_eq!(agent.witness.unwrap().to_bytes(), witness1.to_bytes());
            assert!(agent.witness.is_some());
        }

        let witness2 = BigNumber::rand(1024).unwrap();
        let (_, _, vk) = authz_service.crypto_service.get_crypto_name_and_keypair(&None, &None).unwrap();
        assert!(authz_service.update_agent_witness(&mut new_policy, Base58::encode(&vk), &witness2).is_err());

        let info2 = PolicyAgentInfo::new(None, None, None);
        let key2 = authz_service.add_new_agent_to_policy(&mut new_policy, Some(&info2)).unwrap();
        let witness3 = BigNumber::rand(1024).unwrap();
        assert!(authz_service.update_agent_witness(&mut new_policy, key2.verkey.clone(), &witness3).is_err());
    }
}
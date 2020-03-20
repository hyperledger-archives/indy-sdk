use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error};
use futures::Future;
use futures::future::ok;

use crate::actors::HandleAdminMessage;
use crate::actors::agent::Agent;
use crate::actors::agent_connection::AgentConnection;
use crate::actors::forward_agent::ForwardAgent;
use crate::actors::forward_agent_connection::ForwardAgentConnection;
use crate::domain::admin_message::{AdminQuery, GetDetailAgentConnParams, GetDetailAgentParams, ResAdminQuery, ResQueryAdmin};
use crate::utils::futures::FutureExt;

/// Admin actor is much like Router aware of existing instances such as Forward Agent, Forward Agent Connections,
/// Agents and Agent Connections. Agent can receive requests to retrieve various information about
/// these instances. As the name indicates, Admin interface is not supposed to be public as it's
/// capable revealing various metadata about agency and instances in it.
pub struct Admin {
    forward_agent: Option<Addr<ForwardAgent>>,
    forward_agent_connections: HashMap<String, Addr<ForwardAgentConnection>>,
    agents: HashMap<String, Addr<Agent>>,
    agent_connections: HashMap<String, Addr<AgentConnection>>,
}

impl Admin {
    pub fn create() -> Arc<RwLock<Admin>> {
        trace!("Admin::create >>");
        let admin = Admin {
            forward_agent: None,
            forward_agent_connections: HashMap::new(),
            agents: HashMap::new(),
            agent_connections: HashMap::new(),
        };
        Arc::new(RwLock::new(admin))
    }

    pub fn register_forward_agent(&mut self, fwa: Addr<ForwardAgent>) {
        trace!("Admin::register_forward_agent >>");
        self.forward_agent = Some(fwa);
    }

    pub fn register_forward_agent_connection(&mut self, did: String, fwac: Addr<ForwardAgentConnection>) {
        trace!("Admin::register_forward_agent_connection >>");
        self.forward_agent_connections.insert(did, fwac);
    }

    pub fn register_agent(&mut self, did: String, agent: Addr<Agent>){
        trace!("Admin::register_agent >>");
        self.agents.insert(did, agent);
    }

    pub fn register_agent_connection(&mut self, did: String, aconn: Addr<AgentConnection>){
        trace!("Admin::register_agent_connection>>");
        self.agent_connections.insert(did, aconn);
    }

    pub fn get_actor_overview(&self) -> Box<dyn Future<Item=ResAdminQuery, Error=Error>> {
        let forward_agent_connections = self.forward_agent_connections.iter().map(|(did, _address)| did.clone()).collect::<Vec<_>>().clone();
        let agents = self.agents.iter().map(|(did, _address)| did.clone()).collect::<Vec<_>>().clone();
        let agent_connections = self.agent_connections.iter().map(|(did, _address)| did.clone()).collect::<Vec<_>>().clone();
        ok(ResAdminQuery::Admin(
            ResQueryAdmin {
                forward_agent_connections,
                agents,
                agent_connections,
            })
        ).into_box()
    }

    pub fn get_detail_forward_agents(&self) -> Box<dyn Future<Item=ResAdminQuery, Error=Error>> {
        if let Some(addr) = self.forward_agent.as_ref() {
            addr
                .send(HandleAdminMessage(AdminQuery::GetDetailForwardAgents))
                .from_err()
                .and_then(|res| res)
                .into_box()
        } else {
            err!(err_msg("Forward agent is not registered in Admin."))
        }
    }

    pub fn get_detail_agent(&self, agent_did: String) -> Box<dyn Future<Item=ResAdminQuery, Error=Error>> {
        let agent = self.agents.get(&agent_did);
        let admin_query = AdminQuery::GetDetailAgent(GetDetailAgentParams { agent_did });
        match agent {
            Some(agent) => {
                agent
                    .send(HandleAdminMessage(admin_query))
                    .from_err()
                    .and_then(|res| res)
                    .into_box()
            }
            None => err!(err_msg("Agent not found."))
        }
    }

    pub fn get_detail_agent_connection(&self, agent_pairwise_did: String) -> Box<dyn Future<Item=ResAdminQuery, Error=Error>> {
        let agent_connection = self.agent_connections.get(&agent_pairwise_did);
        let admin_query = AdminQuery::GetDetailAgentConnection(GetDetailAgentConnParams { agent_pairwise_did });
        match agent_connection {
            Some(agent_connection) => {
                agent_connection
                    .send(HandleAdminMessage(admin_query))
                    .from_err()
                    .and_then(|res| res)
                    .into_box()
            }
            None => err!(err_msg("Agent connection not found."))
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::domain::admin_message::{GetDetailAgentConnParams, GetDetailAgentParams};
    use crate::utils::tests::*;

    use super::*;

    #[test]
    fn get_actor_overview_returns_info() {
        let legacy_or_qualified_did_regex = Regex::new("^[a-z0-9]+:([a-z0-9]+):(.*)$|[a-zA-Z0-9]{21,}").unwrap();

        run_admin_test(|(e_wallet_handle, _, _, _, _, _, admin)| {
            admin.read().unwrap()
                .get_actor_overview()
                .from_err()
                .map(move |res| {
                    if let ResAdminQuery::Admin(details) = res {
                        assert_eq!(details.forward_agent_connections.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.forward_agent_connections[0]));
                        assert_eq!(details.agents.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.agents[0]));
                        assert_eq!(details.agent_connections.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.agent_connections[0]));
                    }
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn get_agent_detail_returns_info() {
        let legacy_or_qualified_did_regex = Regex::new("^[a-z0-9]+:([a-z0-9]+):(.*)$|[a-zA-Z0-9]{21,}").unwrap();
        let verkey_regex = Regex::new("[a-zA-Z0-9]{42,46}").unwrap();

        run_admin_test(|(e_wallet_handle, _, _, _, _, _, admin)| {
            let admin1 = admin.clone();
            admin.read().unwrap()
                .get_actor_overview()
                .from_err()
                .map(move |res| {
                    if let ResAdminQuery::Admin(details) = res {
                        details.agents[0].to_owned()
                    } else {
                        panic!("Response was expected to be ResAdminQuery::Admin variant.");
                    }
                })
                .and_then(move |agent_did| {
                    admin1.read().unwrap()
                        .get_detail_agent(agent_did.clone())
                        .from_err()
                        .map(move |res| {
                            if let ResAdminQuery::Agent(res_query_agent) = res {
                                assert!(legacy_or_qualified_did_regex.is_match(&res_query_agent.owner_did));
                                assert!(legacy_or_qualified_did_regex.is_match(&res_query_agent.did));
                                assert!(verkey_regex.is_match(&res_query_agent.owner_verkey));
                                assert!(verkey_regex.is_match(&res_query_agent.verkey));
                                assert_eq!(res_query_agent.configs.len(), 0);
                                assert_eq!(res_query_agent.did, agent_did);
                            } else {
                                panic!("Response was expected to be AdminQuery::GetDetailAgent variant, but got {:?}", res);
                            }
                        })
                })
                .map(move |_| {
                    e_wallet_handle
                })
        })
    }


    #[test]
    fn get_agent_connection_detail_returns_info() {
        let legacy_or_qualified_did_regex = Regex::new("^[a-z0-9]+:([a-z0-9]+):(.*)$|[a-zA-Z0-9]{21,}").unwrap();
        let verkey_regex = Regex::new("[a-zA-Z0-9]{42,46}").unwrap();

        run_admin_test(|(e_wallet_handle, _, _, _, _, _, admin)| {
            let admin1 = admin.clone();
            admin.read().unwrap()
                .get_actor_overview()
                .from_err()
                .map(move |res| {
                    if let ResAdminQuery::Admin(details) = res {
                        details.agent_connections[0].to_owned()
                    } else {
                        panic!("Response was expected to be ResAdminQuery::Admin variant.");
                    }
                })
                .and_then(move |agent_did| {
                    admin1.read().unwrap()
                        .get_detail_agent_connection(agent_did)
                        .from_err()
                        .map(move |res| {
                            if let ResAdminQuery::AgentConn(res_query_agent_conn) = res {
                                assert!(legacy_or_qualified_did_regex.is_match(&res_query_agent_conn.owner_did));
                                assert!(legacy_or_qualified_did_regex.is_match(&res_query_agent_conn.user_pairwise_did));
                                assert!(legacy_or_qualified_did_regex.is_match(&res_query_agent_conn.agent_pairwise_did));
                                assert!(verkey_regex.is_match(&res_query_agent_conn.owner_verkey));
                                assert!(verkey_regex.is_match(&res_query_agent_conn.user_pairwise_verkey));
                                assert!(verkey_regex.is_match(&res_query_agent_conn.agent_pairwise_verkey));
                                assert_eq!(res_query_agent_conn.name, "unknown");
                                assert_eq!(res_query_agent_conn.logo, "unknown");
                                assert_eq!(res_query_agent_conn.agent_configs.len(), 0);
                                assert_eq!(res_query_agent_conn.remote_agent_detail_verkey, "unknown");
                                assert_eq!(res_query_agent_conn.remote_agent_detail_did, "unknown");
                                assert_eq!(res_query_agent_conn.remote_forward_agent_detail_verkey, "unknown");
                                assert_eq!(res_query_agent_conn.remote_forward_agent_detail_did, "unknown");
                                assert_eq!(res_query_agent_conn.remote_forward_agent_detail_endpoint, "unknown");
                            } else {
                                panic!("Response was expected to be AdminQuery::GetDetailAgentConnection variant, but got {:?}", res);
                            }
                        })
                })
                .map(move |_| {
                    e_wallet_handle
                })
        })
    }
}
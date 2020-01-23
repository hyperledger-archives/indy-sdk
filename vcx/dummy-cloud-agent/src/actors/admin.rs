use actix::prelude::*;
use actors::{HandleAdminMessage, AdminRegisterForwardAgent, AdminRegisterRouter, AdminRegisterForwardAgentConnection, AdminRegisterAgent, AdminRegisterAgentConnection};
use failure::{Error, err_msg};
use std::collections::HashMap;
use domain::admin_message::{AdminQuery, ResAdminQuery, ResQueryAdmin};
use utils::futures::FutureExt;
use futures::Future;
use futures::future::ok;

pub struct Admin {
    forward_agent: Option<Recipient<HandleAdminMessage>>,
    forward_agent_connections: HashMap<String, Recipient<HandleAdminMessage>>,
    agents: HashMap<String, Recipient<HandleAdminMessage>>,
    agent_connections: HashMap<String, Recipient<HandleAdminMessage>>,
}

impl Admin {
    pub fn create() -> Addr<Admin> {
        trace!("Admin::create >>");
        let admin = Admin {
            forward_agent: None,
            forward_agent_connections: HashMap::new(),
            agents: HashMap::new(),
            agent_connections: HashMap::new(),
        };
        admin.start()
    }

    pub fn handle_admin_message(&self, admin_msg: &AdminQuery)
                                -> Box<Future<Item=ResAdminQuery, Error=Error>> {
        match admin_msg {
            AdminQuery::GetDetailForwardAgents => {
                if let Some(addr) = self.forward_agent.as_ref() {
                    addr
                        .send(HandleAdminMessage(admin_msg.clone()))
                        .from_err()
                        .and_then(|res| res)
                        .into_box()
                } else {
                    err!(err_msg("Forward agent is not registered in Admin."))
                }
            }
            AdminQuery::GetDetailAgent(query) => {
                let agent = self.agents.get(&query.agent_did);
                match agent {
                    Some(agent) => {
                        agent
                            .send(HandleAdminMessage(admin_msg.clone()))
                            .from_err()
                            .and_then(|res| res)
                            .into_box()
                    }
                    None => err!(err_msg("Agent not found."))
                }
            }
            AdminQuery::GetDetailAgentConnection(query) => {
                let agent_connection = self.agent_connections.get(&query.agent_pairwise_did);
                trace!("resolveding agent connectioon {:?}", query.agent_pairwise_did);
//                err!(err_msg("resolveding agent connectioon"))
                match agent_connection {
                    Some(agent_connection) => {
                        agent_connection
                            .send(HandleAdminMessage(admin_msg.clone()))
                            .from_err()
                            .and_then(|res| res)
                            .into_box()
                    }
                    None => err!(err_msg("Agent connection not found."))
                }
            }
            AdminQuery::GetDetailForwardAgentConnection => {
                err!(err_msg("GetDetailForwardAgentConnection not implemented"))
            }
            AdminQuery::GetDetailRouter => {
                err!(err_msg("GetDetailRouter not implemented"))
            }
            AdminQuery::GetActorOverview => {
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
        }
    }
}

impl Actor for Admin {
    type Context = Context<Self>;
}

impl Handler<HandleAdminMessage> for Admin {
    type Result = Box<Future<Item=ResAdminQuery, Error=Error>>;

    fn handle(&mut self, msg: HandleAdminMessage, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<HandleAdminMessage>::handle");
        self.handle_admin_message(&msg.0)
    }
}

impl Handler<AdminRegisterForwardAgentConnection> for Admin {
    type Result = Result<(), Error>;

    fn handle(&mut self, _msg: AdminRegisterForwardAgentConnection, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<AdminRegisterForwardAgentConnection>::handle >>");
        self.forward_agent_connections.insert(_msg.0, _msg.1);
        Ok(())
    }
}

impl Handler<AdminRegisterAgent> for Admin {
    type Result = Result<(), Error>;

    fn handle(&mut self, _msg: AdminRegisterAgent, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<AdminRegisterAgent>::handle >>");
        self.agents.insert(_msg.0, _msg.1);
        Ok(())
    }
}

impl Handler<AdminRegisterAgentConnection> for Admin {
    type Result = Result<(), Error>;

    fn handle(&mut self, _msg: AdminRegisterAgentConnection, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<AdminRegisterAgentConnection>::handle >>");
        self.agent_connections.insert(_msg.0, _msg.1);
        Ok(())
    }
}

impl Handler<AdminRegisterForwardAgent> for Admin {
    type Result = Result<(), Error>;

    fn handle(&mut self, _msg: AdminRegisterForwardAgent, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<AdminRegisterForwardAgent>::handle >>", );
        self.forward_agent = Some(_msg.0);
        Ok(())
    }
}

impl Handler<AdminRegisterRouter> for Admin {
    type Result = Result<(), Error>;

    fn handle(&mut self, _msg: AdminRegisterRouter, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Admin Handler<AdminRegisterRouter>::handle >>", );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::tests::*;
    use regex::Regex;
    use domain::admin_message::{GetDetailAgentParams, GetDetailAgentConnParams};

    #[test]
    fn get_actor_overview_returns_info() {
        let legacy_or_qualified_did_regex = Regex::new("^[a-z0-9]+:([a-z0-9]+):(.*)$|[a-zA-Z0-9]{21,}").unwrap();

        run_admin_test(|(e_wallet_handle, _, _, _, _, _, admin)| {
            admin
                .send(HandleAdminMessage(AdminQuery::GetActorOverview))
                .from_err()
                .map(move |res| {
                    if let Ok(ResAdminQuery::Admin(details)) = res {
                        assert_eq!(details.forward_agent_connections.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.forward_agent_connections[0]));
                        assert_eq!(details.agents.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.agents[0]));
                        assert_eq!(details.agent_connections.len(), 1);
                        assert!(legacy_or_qualified_did_regex.is_match(&details.agent_connections[0]));
                    } else {
                        panic!("Response was expected to be AdminQuery::GetActorOverview variant, but got {:?}", res);
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
            admin.clone()
                .send(HandleAdminMessage(AdminQuery::GetActorOverview))
                .from_err()
                .map(move |res| {
                    if let Ok(ResAdminQuery::Admin(details)) = res {
                        details.agents[0].to_owned()
                    } else {
                        panic!("Response was expected to be ResAdminQuery::Admin variant.");
                    }
                })
                .and_then(move |agent_did| {
                    admin
                        .send(HandleAdminMessage(AdminQuery::GetDetailAgent(GetDetailAgentParams { agent_did: agent_did.clone() })))
                        .from_err()
                        .map(move |res| {
                            if let Ok(ResAdminQuery::Agent(res_query_agent)) = res {
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
            admin.clone()
                .send(HandleAdminMessage(AdminQuery::GetActorOverview))
                .from_err()
                .map(move |res| {
                    if let Ok(ResAdminQuery::Admin(details)) = res {
                        details.agent_connections[0].to_owned()
                    } else {
                        panic!("Response was expected to be ResAdminQuery::Admin variant.");
                    }
                })
                .and_then(move |agent_did| {
                    admin
                        .send(HandleAdminMessage(AdminQuery::GetDetailAgentConnection(GetDetailAgentConnParams { agent_pairwise_did: agent_did.clone() })))
                        .from_err()
                        .map(move |res| {
                            if let Ok(ResAdminQuery::AgentConn(res_query_agent_conn)) = res {
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
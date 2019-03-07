use serde_json::Value;

use super::constants::{AUTH_RULE, GET_AUTH_RULE};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub enum AuthAction {
    ADD,
    EDIT
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "constraint_id")]
pub enum Constraint {
    #[serde(rename = "OR")]
    OrConstraint(CombinationConstraint),
    #[serde(rename = "AND")]
    AndConstraint(CombinationConstraint),
    #[serde(rename = "ROLE")]
    RoleConstraint(RoleConstraint),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RoleConstraint {
    pub sig_count: u32,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_be_owner: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CombinationConstraint {
    pub auth_constraints: Vec<Constraint>
}

#[derive(Serialize, PartialEq, Debug)]
pub struct AuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub auth_type: String,
    pub field: String,
    pub auth_action: AuthAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    pub new_value: String,
    pub constraint: Constraint,
}

impl AuthRuleOperation {
    pub fn new(auth_type: String, field: String, auth_action: AuthAction,
               old_value: Option<String>, new_value: String, constraint: Constraint) -> AuthRuleOperation {
        AuthRuleOperation {
            _type: AUTH_RULE.to_string(),
            auth_type,
            field,
            auth_action,
            old_value,
            new_value,
            constraint,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<AuthAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>
}

impl GetAuthRuleOperation {
    pub fn new(auth_type: Option<String>, field: Option<String>, auth_action: Option<AuthAction>,
               old_value: Option<String>, new_value: Option<String>) -> GetAuthRuleOperation {
        GetAuthRuleOperation {
            _type: GET_AUTH_RULE.to_string(),
            auth_type,
            field,
            auth_action,
            old_value,
            new_value,
        }
    }
}
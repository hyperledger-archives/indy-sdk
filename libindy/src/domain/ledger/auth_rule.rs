use serde_json::Value;

use super::constants::{AUTH_RULE, AUTH_RULES, GET_AUTH_RULE};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub enum AuthAction {
    ADD,
    EDIT
}

/**
   Enum of the constraint type within the GAT_AUTH_RULE result data
    # parameters
   Role - The final constraint
   And - Combine multiple constraints all of them must be met
   Or - Combine multiple constraints any of them must be met
   Forbidden - action is forbidden
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "constraint_id")]
pub enum Constraint {
    #[serde(rename = "OR")]
    OrConstraint(CombinationConstraint),
    #[serde(rename = "AND")]
    AndConstraint(CombinationConstraint),
    #[serde(rename = "ROLE")]
    RoleConstraint(RoleConstraint),
    #[serde(rename = "FORBIDDEN")]
    ForbiddenConstraint(ForbiddenConstraint),
}

/**
   The final constraint
    # parameters
   sig_count - The number of signatures required to execution action
   role - The role which the user must have to execute the action.
   metadata -  An additional parameters of the constraint (contains transaction FEE cost).
   need_to_be_owner - The flag specifying if a user must be an owner of the transaction.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RoleConstraint {
    pub sig_count: Option<u32>,
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_be_owner: Option<bool>,
}

/**
   Combine multiple constraints
    # parameters
   auth_constraints - The type of the combination
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CombinationConstraint {
    pub auth_constraints: Vec<Constraint>
}

/**
   The forbidden constraint means that action is forbidden
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ForbiddenConstraint {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum AuthRuleOperation {
    Add(AddAuthRuleOperation),
    Edit(EditAuthRuleOperation),
}

#[derive(Serialize, PartialEq, Debug)]
pub struct AddAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub auth_type: String,
    pub field: String,
    pub auth_action: AuthAction,
    pub new_value: Option<String>,
    pub constraint: Constraint,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct EditAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub auth_type: String,
    pub field: String,
    pub auth_action: AuthAction,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub constraint: Constraint,
}

impl AuthRuleOperation {
    pub fn new(auth_type: String, field: String, auth_action: AuthAction,
               old_value: Option<String>, new_value: Option<String>, constraint: Constraint) -> AuthRuleOperation {
        match auth_action {
            AuthAction::ADD =>
                AuthRuleOperation::Add(AddAuthRuleOperation {
                    _type: AUTH_RULE.to_string(),
                    auth_type,
                    field,
                    auth_action,
                    new_value,
                    constraint,
                }),
            AuthAction::EDIT =>
                AuthRuleOperation::Edit(EditAuthRuleOperation {
                    _type: AUTH_RULE.to_string(),
                    auth_type,
                    field,
                    auth_action,
                    old_value,
                    new_value,
                    constraint,
                })
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum GetAuthRuleOperation {
    All(GetAllAuthRuleOperation),
    Add(GetAddAuthRuleOperation),
    Edit(GetEditAuthRuleOperation),
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetAllAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetAddAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub auth_type: String,
    pub field: String,
    pub auth_action: AuthAction,
    pub new_value: Option<String>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetEditAuthRuleOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub auth_type: String,
    pub field: String,
    pub auth_action: AuthAction,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl GetAuthRuleOperation {
    pub fn get_all() -> GetAuthRuleOperation {
        GetAuthRuleOperation::All(GetAllAuthRuleOperation {
            _type: GET_AUTH_RULE.to_string(),
        })
    }

    pub fn get_one(auth_type: String, field: String, auth_action: AuthAction,
                   old_value: Option<String>, new_value: Option<String>) -> GetAuthRuleOperation {
        match auth_action {
            AuthAction::ADD =>
                GetAuthRuleOperation::Add(GetAddAuthRuleOperation {
                    _type: GET_AUTH_RULE.to_string(),
                    auth_type,
                    field,
                    auth_action,
                    new_value,
                }),
            AuthAction::EDIT =>
                GetAuthRuleOperation::Edit(GetEditAuthRuleOperation {
                    _type: GET_AUTH_RULE.to_string(),
                    auth_type,
                    field,
                    auth_action,
                    old_value,
                    new_value,
                })
        }
    }
}

pub type AuthRules = Vec<AuthRuleData>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "auth_action")]
pub enum AuthRuleData {
    #[serde(rename = "ADD")]
    Add(AddAuthRuleData),
    #[serde(rename = "EDIT")]
    Edit(EditAuthRuleData),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AddAuthRuleData {
    pub auth_type: String,
    pub field: String,
    pub new_value: Option<String>,
    pub constraint: Constraint,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EditAuthRuleData {
    pub auth_type: String,
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub constraint: Constraint,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct AuthRulesOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub rules: AuthRules
}

impl AuthRulesOperation {
    pub fn new(rules: AuthRules) -> AuthRulesOperation {
        AuthRulesOperation { _type: AUTH_RULES.to_string(), rules }
    }
}
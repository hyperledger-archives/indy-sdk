extern crate indy_crypto;

use self::indy_crypto::utils::json::JsonDecodable;


#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Filter {
    pub schema_id: Option<String>,
    pub schema_issuer_did: Option<String>,
    pub schema_name: Option<String>,
    pub schema_version: Option<String>,
    pub issuer_did: Option<String>,
    pub cred_def_id: Option<String>
}

impl<'a> JsonDecodable<'a> for Filter {}

pub trait Filtering {
    fn schema_id(&self) -> String;
    fn schema_issuer_did(&self) -> String;
    fn schema_name(&self) -> String;
    fn schema_version(&self) -> String;
    fn issuer_did(&self) -> String;
    fn cred_def_id(&self) -> String;
}
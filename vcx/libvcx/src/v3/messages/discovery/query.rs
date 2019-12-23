use v3::messages::a2a::{MessageId, A2AMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Query {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>
}

impl Query {
    pub fn create() -> Query {
        Query::default()
    }

    pub fn set_query(mut self, query: Option<String>) -> Self {
        self.query = query;
        self
    }

    pub fn set_comment(mut self, comment: Option<String>) -> Self {
        self.comment = comment;
        self
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::Query(self.clone()) // TODO: THINK how to avoid clone
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _query_string() -> String {
        String::from("did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/")
    }


    fn _comment() -> String {
        String::from("I'm wondering if we can...")
    }

    pub fn _query() -> Query {
        Query {
            id: MessageId::id(),
            query: Some(_query_string()),
            comment: Some(_comment()),
        }
    }

    #[test]
    fn test_query_build_works() {
        let query: Query = Query::default()
            .set_query(Some(_query_string()))
            .set_comment(Some(_comment()));

        assert_eq!(_query(), query);
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum MessageStatusCode {
    #[serde(rename = "MS-101")]
    Created,
    #[serde(rename = "MS-102")]
    Sent,
    #[serde(rename = "MS-103")]
    Received,
    #[serde(rename = "MS-104")]
    Accepted,
    #[serde(rename = "MS-105")]
    Rejected,
    #[serde(rename = "MS-106")]
    Reviewed,
}

impl MessageStatusCode{
    pub fn message(&self) -> &'static str{
        match self {
            MessageStatusCode::Created => "message created",
            MessageStatusCode::Sent => "message sent",
            MessageStatusCode::Received => "message received",
            MessageStatusCode::Accepted => "message accepted",
            MessageStatusCode::Rejected => "message rejected",
            MessageStatusCode::Reviewed => "message reviewed",
        }
    }

    pub fn valid_status_codes() -> Vec<MessageStatusCode>{
        vec![MessageStatusCode::Accepted, MessageStatusCode::Rejected]
    }

    //validNewMsgStatusesAllowedToBeUpdatedTo
    pub fn valid_new_message_status_codes_allowed_update_to() -> Vec<MessageStatusCode>{
        vec![MessageStatusCode::Accepted, MessageStatusCode::Rejected, MessageStatusCode::Reviewed]
    }

    //validExistingMsgStatusesAllowedToBeUpdated
    pub fn valid_existing_message_statuses_to_update() -> Vec<MessageStatusCode>{
        vec![MessageStatusCode::Received]
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum ConnectionStatus {
    #[serde(rename = "CS-101")]
    AlreadyConnected,
    #[serde(rename = "CS-102")]
    NotConnected,
    #[serde(rename = "CS-103")]
    Deleted,
}

impl Default for ConnectionStatus{
    fn default() -> Self {
        ConnectionStatus::NotConnected
    }
}
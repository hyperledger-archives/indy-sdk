use actix::{Arbiter, ResponseFuture};
use failure::{err_msg, Error, Fail};
use futures::*;
use uuid::Uuid;

use crate::actors::{RemoteMsg, requester};
use crate::actors::agent::agent::Agent;
use crate::actors::agent_connection::agent_connection::AgentConnection;
use crate::domain::status::MessageStatusCode;
use crate::utils::futures::*;
use crate::domain::a2a::RemoteMessageType;
use crate::domain::internal_message::InternalMessage;
use futures::future::Either;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MessageNotification {
    msg_uid: String,
    msg_type: RemoteMessageType,
    their_pw_did: String,
    msg_status_code: MessageStatusCode,
    notification_id: String,
    pw_did: String,
}

impl AgentConnection {
    /// Dispatches metadata about receive message in form of HTTP POST on specified url.
    /// Any HTTP error codes returned from the target URL are ignored. The HTTP request dispatch is
    /// non blocking.
    ///
    /// * `webhook_url` - URL address where the data shall be sent
    /// * `msg_notification` - metadata about received message
    pub(super) fn send_webhook_notification(webhook_url: &str, msg_notification: MessageNotification) -> ResponseFuture<(), ()>{
        let ser_msg_notification = serde_json::to_string(&msg_notification).unwrap();
        let notification_id_1 = msg_notification.notification_id.clone();
        let notification_id_2 = msg_notification.notification_id.clone();
        debug!("Sending webhook notification {} to {} data", &notification_id_1, webhook_url);
        requester::REQWEST_CLIENT
            .post(webhook_url)
            .header("Accepts", "application/json")
            .header("Content-type", "application/json")
            .body(ser_msg_notification)
            .send()
            .map_err(move |error| {
                error!("Problem sending webhook notification. NotificationId {} {:?}",
                       &notification_id_1, error);
            })
            .map(move |res| {
                if let Err(res_err) = res.error_for_status_ref() {
                    error!("Error code returned from webhook url. NotificationId {} {:?}",
                           &notification_id_2, res_err);
                };
            })
            .into_box()

    }

    pub(super) fn try_send_notification(&self, msg: InternalMessage) -> ResponseFuture<(), ()>{
        let user_pairwise_did = self.user_pairwise_did.clone();
        self.load_webhook_url()
            .map_err(|err| {
                error!("Failed attempt to load webhook_url");
            })
            .and_then(move |webhook_url| {
                match webhook_url {
                    Some(webhook_url) => {
                        let msg_notification = MessageNotification {
                            msg_uid: msg.uid.clone(),
                            msg_type: msg._type.clone(),
                            their_pw_did: msg.sender_did.clone(),
                            msg_status_code: msg.status_code.clone(),
                            pw_did: user_pairwise_did,
                            notification_id: Uuid::new_v4().to_string(),
                        };
                        Either::A(Self::send_webhook_notification(&webhook_url, msg_notification))
                    }
                    None => Either::B(future::ok(()))
                }
            })
            .into_box()
    }

    fn load_webhook_url(&self) -> ResponseFuture<Option<String>, Error> {
        Agent::load_config(self.wallet_handle.clone(), self.agent_did.clone(), "notificationWebhookUrl".into())
            .into_box()
    }
}

pub mod agent;
pub mod states;
pub mod connection;

use self::connection::*;

use messages::get_message::Message;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;

use v3::handlers::connection::states::*;
use v3::messages::a2a::{A2AMessage, MessageId};
use v3::messages::connection::invite::Invitation;

use std::collections::HashMap;

lazy_static! {
    pub static ref CONNECTION_MAP: ObjectCache<Connection> = Default::default();
}

pub fn create_connection(source_id: &str) -> VcxResult<u32> {
    let connection = Connection::create(source_id, Actor::Inviter);

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn create_connection_with_invite(source_id: &str, invitation: Invitation) -> VcxResult<u32> {
    let mut connection: Connection = Connection::create(source_id, Actor::Invitee);

    connection = connection.process_invite(invitation)?;

    CONNECTION_MAP.add(connection)
        .or(Err(VcxError::from(VcxErrorKind::CreateConnection)))
}

pub fn connect(handle: u32, _options: Option<String>) -> VcxResult<u32> {
    // Do we need it now????
    // let options_obj: ConnectionOptions = ConnectionOptions::from_opt_str(options)?;
    CONNECTION_MAP.map(handle, |connection| {
        connection.connect()
    }).map(|_| error::SUCCESS.code_num)
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    CONNECTION_MAP.map(handle, |connection| {
        connection.update_state(message.as_ref().map(String::as_str))
    }).map(|_| error::SUCCESS.code_num)
}

pub fn get_state(handle: u32) -> u32 {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.state())
    }).unwrap_or(0)
}

pub fn get_messages(handle: u32) -> VcxResult<HashMap<String, A2AMessage>> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_messages()
    })
}

pub fn update_message_status(handle: u32, uid: String) -> VcxResult<()> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.update_message_status(uid.clone())
    })
}

pub fn get_message_by_id(handle: u32, msg_id: String) -> VcxResult<A2AMessage> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_message_by_id(&msg_id)
    })
}

pub fn decode_message(handle: u32, message: Message) -> VcxResult<A2AMessage> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.decode_message(&message)
    })
}

pub fn send_message(handle: u32, message: A2AMessage) -> VcxResult<()> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.send_message(&message)
    })
}

pub fn get_invite_details(handle: u32, abbreviated: bool) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_invite_details()
    })
}

pub fn send_generic_message(handle: u32, msg: &str, msg_options: &str) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.send_generic_message(msg, msg_options)
    })
}

pub fn get_pw_did(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_vk.to_string())
    })
}

pub fn get_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Ok(connection.agent_info().pw_vk.to_string())
    })
}

pub fn get_their_pw_verkey(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.remote_vk()
    })
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.get_source_id()
    })
}

pub fn release(handle: u32) -> VcxResult<()> {
    CONNECTION_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidConnectionHandle)))
}

pub fn delete_connection(handle: u32) -> VcxResult<u32> {
    CONNECTION_MAP.get(handle, |connection| {
        connection.delete()
    })
        .or(Err(VcxError::from(VcxErrorKind::DeleteConnection)))
        .and(release(handle))
        .and_then(|_| Ok(error::SUCCESS.code_num))
}

// Actually it handles any message
pub fn process_acceptance_message(handle: u32, message: Message) -> VcxResult<u32> {
    CONNECTION_MAP.map(handle, |connection| {
        connection.update_state(Some(&json!(message).to_string()))
    }).map(|_| error::SUCCESS.code_num)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    CONNECTION_MAP.get(handle, |connection| {
        Connection::to_string(&connection)
    })
}

pub fn from_string(connection_data: &str) -> VcxResult<u32> {
    let connection: Connection = Connection::from_str(connection_data)?;
    CONNECTION_MAP.add(connection)
}

pub fn add_pending_messages(handle: u32, messages: HashMap<MessageId, String>) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.add_pending_messages(messages.clone())
    })
}

pub fn remove_pending_message(handle: u32, id: &MessageId) -> VcxResult<()> {
    CONNECTION_MAP.get_mut(handle, |connection| {
        connection.remove_pending_message(id.clone())
    })
}

#[cfg(feature = "aries")]
#[cfg(test)]
mod test {
    use super::*;
    use v3::test::{Faber, Alice};
    use v3::messages::connection::invite::tests::_invitation;
    use v3::messages::ack::tests::_ack;

    fn _source_id() -> &'static str {
        "test connection"
    }

    fn actor(handle: u32) -> VcxResult<Actor> {
        CONNECTION_MAP.get(handle, |connection| {
            Ok(connection.actor())
        })
    }

    #[test]
    fn test_create_connection_works() {
        let connection_handle = create_connection(_source_id()).unwrap();
        assert!(CONNECTION_MAP.has_handle(connection_handle));
        assert_eq!(Actor::Inviter, actor(connection_handle).unwrap());
        assert_eq!(1, get_state(connection_handle));
    }

    #[test]
    fn test_create_connection_with_invite_works() {
        let connection_handle = create_connection_with_invite(_source_id(), _invitation()).unwrap();
        assert!(CONNECTION_MAP.has_handle(connection_handle));
        assert_eq!(Actor::Invitee, actor(connection_handle).unwrap());
        assert_eq!(2, get_state(connection_handle));
    }

    #[test]
    fn test_get_connection_state_works() {
        let connection_handle = create_connection(_source_id()).unwrap();
        assert_eq!(1, get_state(connection_handle));
    }

    #[test]
    fn test_connection_send_works() {
        let mut faber = Faber::setup();
        let mut alice = Alice::setup();

        let invite = faber.create_invite();
        alice.accept_invite(&invite);

        faber.update_state(5);
        alice.update_state(4);
        faber.update_state(4);

        let mut uid: String;
        let message = _ack();

        // Send Message works
        {
            faber.activate();
            send_message(faber.connection_handle, message.to_a2a_message()).unwrap();
        }

        {
            // Get Messages works
            alice.activate();

            let messages = get_messages(alice.connection_handle).unwrap();
            assert_eq!(1, messages.len());

            uid = messages.keys().next().unwrap().clone();
            let received_message = messages.values().next().unwrap().clone();

            match received_message {
                A2AMessage::Ack(received_message) => assert_eq!(message, received_message.clone()),
                _ => assert!(false)
            }
        }

        // Get Message by id works
        {
            alice.activate();

            let message = get_message_by_id(alice.connection_handle, uid.clone()).unwrap();

            match message {
                A2AMessage::Ack(ack) => assert_eq!(_ack(), ack),
                _ => assert!(false)
            }
        }

        // Update Message Status works
        {
            alice.activate();

            update_message_status(alice.connection_handle, uid).unwrap();
            let messages = get_messages(alice.connection_handle).unwrap();
            assert_eq!(0, messages.len());
        }

        // Send Generic Message works
        {
            faber.activate();

            let generic_message = "some message";
            send_generic_message(faber.connection_handle, generic_message, "").unwrap();

            alice.activate();

            let messages = get_messages(alice.connection_handle).unwrap();
            assert_eq!(1, messages.len());

            let uid = messages.keys().next().unwrap().clone();
            let message = messages.values().next().unwrap().clone();

            match message {
                A2AMessage::Generic(message) => assert_eq!(generic_message, message),
                _ => assert!(false)
            }
            update_message_status(alice.connection_handle, uid).unwrap();
        }

        // Pending Message
        {
            faber.activate();

            let message = _ack();
            send_message(faber.connection_handle, message.to_a2a_message()).unwrap();

            alice.activate();

            let messages = get_messages(alice.connection_handle).unwrap();
            assert_eq!(1, messages.len());
            let uid = messages.keys().next().unwrap().clone();

            add_pending_messages(alice.connection_handle, map!( message.id.clone() => uid )).unwrap();

            remove_pending_message(alice.connection_handle, &message.id).unwrap();

            let messages = get_messages(alice.connection_handle).unwrap();
            assert_eq!(0, messages.len());
        }

        // Helpers
        {
            faber.activate();

            get_pw_did(faber.connection_handle).unwrap();
            get_pw_verkey(faber.connection_handle).unwrap();
            get_their_pw_verkey(faber.connection_handle).unwrap();
            get_source_id(faber.connection_handle).unwrap();
        }
    }

    #[test]
    fn test_connection_delete(){
        let connection_handle = create_connection(_source_id()).unwrap();
        assert!(CONNECTION_MAP.has_handle(connection_handle));

        release(connection_handle).unwrap();
        assert!(!CONNECTION_MAP.has_handle(connection_handle));
    }

    #[test]
    fn test_connection_serialization_works(){
        let connection_handle = create_connection(_source_id()).unwrap();
        assert!(CONNECTION_MAP.has_handle(connection_handle));

        let connection_json = to_string(connection_handle).unwrap();
        println!("{}", connection_json);

        let connection_handle_2 = from_string(&connection_json).unwrap();
        assert!(CONNECTION_MAP.has_handle(connection_handle_2));
    }
}
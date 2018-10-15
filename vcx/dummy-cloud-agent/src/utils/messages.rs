use domain::messages::{Bundle, Message};
use failure::*;
use futures::*;
use indy::crypto;
use rmp_serde;
use utils::futures::*;

pub fn bundle(msg: &Message) -> Result<Vec<u8>, Error> {
    rmp_serde::to_vec_named(msg)
        .map(|msg| Bundle { bundled: vec![msg] })
        .and_then(|msg| rmp_serde::to_vec_named(&msg))
        .map_err(|err| err.into())
}

pub fn bundle_authcrypted(wallet_handle: i32,
                          sender_vk: &str,
                          recipient_vk: &str,
                          msg: &Message) -> BoxedFuture<Vec<u8>, Error> {
    let sender_vk = sender_vk.to_owned();
    let recipient_vk = recipient_vk.to_owned();

    done(bundle(msg))
        .and_then(move |msg| {
            crypto::auth_crypt(wallet_handle, &sender_vk, &recipient_vk, &msg)
                .from_err()
        })
        .into_box()
}

pub fn unbundle(msg: &[u8]) -> Result<Message, Error> {
    rmp_serde::from_slice::<Bundle>(msg)
        .map_err(|err| err.into())
        .and_then(|mut msg| msg.bundled.pop().ok_or(err_msg("Invalid bundle")))
        .and_then(|msg| rmp_serde::from_slice::<Message>(&msg).map_err(|err| err.into()))
        .map_err(|err| err.into())
}

pub fn unbundle_anoncrypted(wallet_handle: i32,
                            recipient_vk: &str,
                            msg: &[u8]) -> BoxedFuture<Message, Error> {
    crypto::anon_decrypt(wallet_handle, recipient_vk, &msg)
        .from_err()
        .and_then(|msg| {
            unbundle(&msg)
        })
        .into_box()
}

pub fn unbundle_authcrypted(wallet_handle: i32,
                            recipient_vk: &str,
                            msg: &[u8]) -> BoxedFuture<(String, Message), Error> {
    crypto::auth_decrypt(wallet_handle, recipient_vk, &msg)
        .from_err()
        .and_then(|(sender_vk, msg)| {
            unbundle(&msg).map(|msg| (sender_vk, msg))
        })
        .into_box()
}

#[cfg(test)]
mod tests {
    use domain::messages::*;
    use super::*;

    #[test]
    fn bundle_unbundle_works_for_forward() {
        let msg = Message::Forward(
            Forward::V1(
                ForwardV1 {
                    fwd: "fwd".into(),
                    msg: vec![0, 1, 2, 3],
                }));

        use ::std::fs::File;
        use std::io::Write;

        {
            let mut file = File::create("c://workspace/1.txt").unwrap();
            file.write_all(&bundle(&msg).unwrap()).unwrap();
        }

        let msg = unbundle(&bundle(&msg).unwrap()).unwrap();

        if let Message::Forward(Forward::V1(msg)) = msg {
            assert_eq!(msg.fwd, "fwd");
            assert_eq!(msg.msg, vec![0, 1, 2, 3]);
        } else {
            panic!("Unexpected message type")
        }
    }

    #[test]
    fn bundle_unbundle_works_for_connect() {
        let msg = Message::Connect(
            Connect::V1(
                ConnectV1 {
                    from_did: "from_did".into(),
                    from_did_verkey: "from_did_verkey".into(),
                }));

        let msg = unbundle(&bundle(&msg).unwrap()).unwrap();

        if let Message::Connect(Connect::V1(msg)) = msg {
            assert_eq!(msg.from_did, "from_did");
            assert_eq!(msg.from_did_verkey, "from_did_verkey");
        } else {
            panic!("Unexpected message type")
        }
    }

    #[test]
    fn unbundle_works_for_unexpected() {
        let res = unbundle(&[0, 1, 2, 3]);
        assert!(res.is_err());
    }
}
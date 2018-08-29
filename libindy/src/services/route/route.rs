use services::route::jwm::create_receipients;
use services::route::route_table;
use services::microledger::did_doc::DidDoc;
use domain::crypto::key::Key;
use std::collections::HashMap;

// send_msg(jwm : JWM, endpoint : &str) -> <sends message to endpoint>:
// //message is the base64 encoded JWM
// //auth is a boolean variable to indicate if you want to use AuthCrypt(true) or AnonCrypt(false)
// //my_vk MUST be included if auth is true, otherwise it's not needed

pub fn unpack_msg(jwm : String, my_vk : &str) -> String {
// takes in a JWM and my_vk and decrypts the JWM. If the underlying forward message 
// matches my_vk it will decrypt that message as well and output the application layer message. 
// Else it will output the forward message so it can be forwarded on again.


}


// This API call is made to encrypt both Application layer messages and Transport layer
// messages. The purpose of it is to take a message and wrap it up so that it can be fed into
// send_msg and on the other end open_msg can be called on it.
pub fn pack_msg(plaintext: String, auth: bool, recv_did: &str,  my_vk: Option<&str>, wallet_handle: i32) -> String {

    // let recipients = create_receipients(encrypted_keys, recipient_vks, sender_vk, auth);
}

pub fn get_next_hop(did_with_key_frag: &str) -> (&str, &str) {
//DID#key is a reference identifier to the next hop
//their_vk is used to encrypt the message
//endpoint is the endpoint which the message is being sent to.
//called by send_msg()
//returns (endpoint, their_vk)


}
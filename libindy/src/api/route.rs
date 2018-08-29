send_msg(jwm : JWM, endpoint : &str) -> <sends message to endpoint>:
//message is the base64 encoded JWM
//auth is a boolean variable to indicate if you want to use AuthCrypt(true) or AnonCrypt(false)
//my_vk MUST be included if auth is true, otherwise it's not needed

open_msg(jwm : JWM, my_vk : &str) -> String {
// takes in a JWM and my_vk and decrypts the JWM. If the underlying forward message 
// matches my_vk it will decrypt that message as well and output the application layer message. 
// Else it will output the forward message so it can be forwarded on again.
}

package_msg(plaintext: String, auth: bool, did: &str,  my_vk: Option<&str>) -> JWM {
// This API call is made to encrypt both Application layer messages and Transport layer
// messages. The purpose of it is to take a message and wrap it up so that it can be fed into
// send_msg and on the other end open_msg can be called on it.
}


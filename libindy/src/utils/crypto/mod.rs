#[cfg(feature = "ed25519_sign_sodium")]
#[path = "ed25519_sign/sodium.rs"]
pub mod ed25519_sign;

#[cfg(feature = "ed25519_box_sodium")]
#[path = "ed25519_box/sodium.rs"]
// TODO: The name is misleading as the operations do not happen over ed25519 curve
pub mod ed25519_box;

pub use indy_utils::crypto::base64;

#[allow(dead_code)] /* FIXME Do we really need this module? */
#[cfg(feature = "xsalsa20_sodium")]
#[path = "xsalsa20/sodium.rs"]
pub mod xsalsa20;

pub use indy_utils::crypto::chacha20poly1305_ietf;

pub mod signature_serializer;

pub mod verkey_builder;

#[cfg(feature = "sealedbox_sodium")]
#[path = "sealedbox/sodium.rs"]
pub mod sealedbox;

#[cfg(feature = "randombytes_sodium")]
#[path = "randombytes/sodium.rs"]
pub mod randombytes;

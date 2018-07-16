extern crate openssl;

use errors::common::CommonError;

use self::openssl::hash::{hash2, MessageDigest, Hasher, DigestBytes};
use self::openssl::error::ErrorStack;

use std::error::Error;

pub const HASH_OUTPUT_LEN: usize = 32;

pub struct Digest {
    data: DigestBytes
}

impl Digest {
    fn new(data: DigestBytes) -> Digest {
        Digest {
            data: data
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub struct Hash {}

impl Hash {
    pub fn new_context() -> Result<Hasher, CommonError> {
        Ok(Hasher::new(MessageDigest::sha256())?)
    }

    pub fn hash_empty() -> Result<Digest, CommonError> {
        Ok(Digest::new(hash2(MessageDigest::sha256(), &[])?))

    }

    pub fn hash_leaf<T>(leaf: &T) -> Result<Digest, CommonError> where T: Hashable {
        let mut ctx = Hash::new_context()?;
        ctx.update(&[0x00])?;
        leaf.update_context(&mut ctx)?;
        Ok(Digest::new(ctx.finish2()?))
    }

    pub fn hash_nodes<T>(left: &T, right: &T) -> Result<Digest, CommonError> where T: Hashable {
        let mut ctx = Hash::new_context()?;
        ctx.update(&[0x01])?;
        left.update_context(&mut ctx)?;
        right.update_context(&mut ctx)?;
        Ok(Digest::new(ctx.finish2()?))
    }

}

/// The type of values stored in a `MerkleTree` must implement
/// this trait, in order for them to be able to be fed
/// to a Ring `Context` when computing the hash of a leaf.
///
/// A default instance for types that already implements
/// `AsRef<[u8]>` is provided.
///
/// ## Example
///
/// Here is an example of how to implement `Hashable` for a type
/// that does not (or cannot) implement `AsRef<[u8]>`:
///
/// ```ignore
/// impl Hashable for PublicKey {
///     fn update_context(&self, context: &mut Hasher) -> Result<(), CommonError> {
///         let bytes: Vec<u8> = self.to_bytes();
///         Ok(context.update(&bytes)?)
///     }
/// }
/// ```
pub trait Hashable {

    /// Update the given `context` with `self`.
    ///
    /// See `openssl::hash::Hasher::update` for more information.
    fn update_context(&self, context: &mut Hasher) -> Result<(), CommonError>;

}

impl <T: AsRef<[u8]>> Hashable for T {

    fn update_context(&self, context: &mut Hasher) -> Result<(), CommonError> {
        Ok(context.update(self.as_ref())?)
    }
}

impl From<ErrorStack> for CommonError {
    fn from(err: ErrorStack) -> CommonError {
        // TODO: FIXME: Analyze ErrorStack and split invalid structure errors from other errors
        CommonError::InvalidStructure(err.description().to_string())
    }
}
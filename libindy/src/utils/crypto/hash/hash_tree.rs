//extern crate openssl;

use errors::common::CommonError;

//use self::openssl::hash::{hash2, MessageDigest, Hasher, DigestBytes};
//use self::openssl::error::ErrorStack;

use sha2::{Sha256, Digest as Sha256Digest};
use generic_array::GenericArray;

use generic_array::typenum::U32;
pub type HashBytes = GenericArray<u8, U32>;

pub const HASHBYTES: usize = 32;

pub fn hash(input: &[u8]) -> Result<Vec<u8>, CommonError> {
    let mut hasher = Sha256::default();
    hasher.input(input);
    let output : HashBytes = hasher.result();
    Ok(output.to_vec())
}

pub struct Hash {}

impl Hash {
    pub fn new_context() -> Result<Sha256, CommonError> {
        Ok(Sha256::default())
    }

    pub fn hash_empty() -> Result<HashBytes, CommonError> {
        let mut hasher = Sha256::default();
        hasher.input(&[]);
        let output : HashBytes = hasher.result();
        Ok(output)
    }

    pub fn hash_leaf<T>(leaf: &T) -> Result<HashBytes, CommonError> where T: Hashable {
        let mut hasher = Hash::new_context()?;
        hasher.input(&[0x00]);
        leaf.update_context(&mut hasher)?;
        let output : HashBytes = hasher.result();
        Ok(output)
    }

    pub fn hash_nodes<T>(left: &T, right: &T) -> Result<HashBytes, CommonError> where T: Hashable {
        let mut hasher = Hash::new_context()?;
        hasher.input(&[0x01]);
        left.update_context(&mut hasher)?;
        right.update_context(&mut hasher)?;
        let output : HashBytes = hasher.result();
        Ok(output)
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
    fn update_context(&self, context: &mut Sha256) -> Result<(), CommonError>;

}

impl <T: AsRef<[u8]>> Hashable for T {

    fn update_context(&self, context: &mut Sha256) -> Result<(), CommonError> {
        Ok(context.input(self.as_ref()))
    }
}

/*
impl From<ErrorStack> for CommonError {
    fn from(err: ErrorStack) -> CommonError {
        // TODO: FIXME: Analyze ErrorStack and split invalid structure errors from other errors
        CommonError::InvalidStructure(err.description().to_string())
    }
}
*/
extern crate sodiumoxide;
extern crate libc;

use self::libc::size_t;
use errors::common::CommonError;

pub const SEEDBYTES: usize = 32; // randombytes_seedbytes

pub struct Seed([u8; SEEDBYTES]);

impl Seed {
    pub fn from_slice(bytes: &[u8]) -> Result<Seed, CommonError> {
        if bytes.len() != SEEDBYTES {
            return Err(CommonError::InvalidStructure(format!("Invalid Seed bytes")));
        }
        let mut seed = Seed([0; SEEDBYTES]);
        for (ni, &bsi) in seed.0.iter_mut().zip(bytes.iter()) {
            *ni = bsi
        }
        Ok(seed)
    }
}

pub fn randombytes(size: usize) -> Vec<u8> {
    self::sodiumoxide::randombytes::randombytes(size)
}

pub fn randombytes_deterministic(size: usize, seed: &Seed) -> Vec<u8> {
    let mut out = vec![0u8; size];
    unsafe {
        randombytes_buf_deterministic(out.as_mut_ptr(),
                                      size,
                                      &seed.0)
    };
    out
}


extern {
    fn randombytes_buf_deterministic(out: *mut u8,
                                     size: size_t,
                                     seed: *const [u8; SEEDBYTES]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randombytes_deterministic_works() {
        let seed = Seed::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]).unwrap();
        let res = randombytes_deterministic(32, &seed);
        let expected_bytes = vec![203, 243, 240, 238, 23, 2, 1, 74, 141, 80, 55, 246, 124, 240, 253, 28, 40, 65, 244, 88, 126, 56, 211, 233, 241, 217, 224, 244, 179, 12, 168, 8];
        assert_eq!(expected_bytes, res);
    }
}
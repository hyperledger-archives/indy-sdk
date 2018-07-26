extern crate sodiumoxide;

pub fn randombytes(size: usize) -> Vec<u8> {
    self::sodiumoxide::randombytes::randombytes(size)
}
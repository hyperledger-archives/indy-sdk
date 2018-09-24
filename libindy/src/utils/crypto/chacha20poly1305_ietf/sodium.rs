extern crate sodiumoxide;

use self::sodiumoxide::crypto::aead::chacha20poly1305_ietf;
use self::sodiumoxide::utils;

use std::cmp;
use std::io;
use std::io::{Read, Write};

use utils::crypto::pwhash_argon2i13;
use errors::common::CommonError;
use domain::wallet::KeyDerivationMethod;

pub const KEYBYTES: usize = chacha20poly1305_ietf::KEYBYTES;
pub const NONCEBYTES: usize = chacha20poly1305_ietf::NONCEBYTES;
pub const TAGBYTES: usize = chacha20poly1305_ietf::TAGBYTES;

sodium_type!(Key, chacha20poly1305_ietf::Key, KEYBYTES);
sodium_type!(Nonce, chacha20poly1305_ietf::Nonce, NONCEBYTES);

impl Nonce {
    pub fn increment(&mut self) {
        utils::increment_le(&mut (self.0).0);
    }
}

pub fn gen_key() -> Key {
    Key(chacha20poly1305_ietf::gen_key())
}

pub fn derive_key(passphrase: &str, salt: &pwhash_argon2i13::Salt, key_derivation_method: &KeyDerivationMethod) -> Result<Key, CommonError> {
    let mut key_bytes = [0u8; chacha20poly1305_ietf::KEYBYTES];
    pwhash_argon2i13::pwhash(&mut key_bytes, passphrase.as_bytes(), salt, key_derivation_method)
        .map_err(|err| CommonError::InvalidStructure(format!("Can't derive key: {}", err)))?;
    Ok(Key::new(key_bytes))
}

pub fn gen_nonce() -> Nonce {
    Nonce(chacha20poly1305_ietf::gen_nonce())
}

pub fn gen_nonce_and_encrypt(data: &[u8], key: &Key) -> (Vec<u8>, Nonce) {
    let nonce = gen_nonce();

    let encrypted_data = chacha20poly1305_ietf::seal(
        data,
        None,
        &nonce.0,
        &key.0
    );

    (encrypted_data, nonce)
}

pub fn encrypt(data: &[u8], key: &Key, nonce: &Nonce) -> Vec<u8> {
    chacha20poly1305_ietf::seal(
        data,
        None,
        &nonce.0,
        &key.0,
    )
}

pub fn decrypt(data: &[u8], key: &Key, nonce: &Nonce) -> Result<Vec<u8>, CommonError> {
    chacha20poly1305_ietf::open(
        &data,
        None,
        &nonce.0,
        &key.0,
    )
        .map_err(|err| CommonError::InvalidStructure(format!("Unable to decrypt data: {:?}", err)))
}

pub struct Writer<W: Write> {
    buffer: Vec<u8>,
    chunk_size: usize,
    key: Key,
    nonce: Nonce,
    inner: W,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W, key: Key, nonce: Nonce, chunk_size: usize) -> Self {
        Writer {
            buffer: Vec::new(),
            chunk_size,
            key,
            nonce,
            inner,
        }
    }

    #[allow(unused)]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write_all(buf)?; // TODO: Small optimizations are possible

        let mut chunk_start = 0;

        while self.buffer.len() >= chunk_start + self.chunk_size {
            let chunk = &self.buffer[chunk_start..chunk_start + self.chunk_size];
            self.inner.write_all(&encrypt(chunk, &self.key, &self.nonce))?;
            self.nonce.increment();
            chunk_start += self.chunk_size;
        }

        if chunk_start > 0 {
            self.buffer.drain(..chunk_start);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.buffer.len() > 0 {
            self.inner.write_all(&encrypt(&self.buffer, &self.key, &self.nonce))?;
            self.nonce.increment();
        }

        self.buffer.flush()
    }
}

pub struct Reader<R: Read> {
    rest_buffer: Vec<u8>,
    chunk_buffer: Vec<u8>,
    key: Key,
    nonce: Nonce,
    inner: R,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R, key: Key, nonce: Nonce, chunk_size: usize) -> Self {
        Reader {
            rest_buffer: Vec::new(),
            chunk_buffer: vec![0; chunk_size + TAGBYTES],
            key,
            nonce,
            inner,
        }
    }

    #[allow(unused)]
    pub fn into_inner(self) -> R {
        self.inner
    }

    fn _read_chunk(&mut self) -> io::Result<usize> {
        let mut read = 0;

        while read < self.chunk_buffer.len() {
            match self.inner.read(&mut self.chunk_buffer[read..]) {
                Ok(0) => break,
                Ok(n) => read += n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e)?
            }
        }

        if read == 0 {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more crypto chucks to consume"))
        } else {
            Ok(read)
        }
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut pos = 0;

        // Consume from rest buffer
        if self.rest_buffer.len() > 0 {
            let to_copy = cmp::min(self.rest_buffer.len(), buf.len() - pos);
            buf[pos..pos + to_copy].copy_from_slice(&self.rest_buffer[..to_copy]);
            pos = pos + to_copy;
            self.rest_buffer.drain(..to_copy);
        }

        // Consume from chunks
        while pos < buf.len() {
            let chunk_size = self._read_chunk()?;

            let chunk = decrypt(&self.chunk_buffer[..chunk_size], &self.key, &self.nonce)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid data in crypto chunk"))?;

            self.nonce.increment();

            let to_copy = cmp::min(chunk.len(), buf.len() - pos);
            buf[pos..pos + to_copy].copy_from_slice(&chunk[..to_copy]);
            pos = pos + to_copy;

            // Save rest in rest buffer
            if pos == buf.len() && to_copy < chunk.len() {
                self.rest_buffer.extend(&chunk[to_copy..]);
            }
        }

        Ok(buf.len())
    }
}


#[cfg(test)]
mod tests {
    extern crate rmp_serde;

    use super::*;
    use utils::crypto::randombytes::randombytes;

    #[test]
    fn encrypt_decrypt_works() {
        let data = randombytes(100);
        let key = gen_key();

        let (c, nonce) = gen_nonce_and_encrypt(&data, &key);
        let u = decrypt(&c, &key, &nonce).unwrap();

        assert_eq!(data, u);
    }

    #[test]
    fn encrypt_decrypt_works_for_nonce() {
        let data = randombytes(16);

        let key = gen_key();
        let nonce = gen_nonce();
        let c = encrypt(&data, &key, &nonce);
        let u = decrypt(&c, &key, &nonce).unwrap();

        assert_eq!(data, u)
    }

    #[test]
    fn nonce_serialize_deserialize_works() {
        let nonce = gen_nonce();
        let serialized = rmp_serde::to_vec(&nonce).unwrap();
        let deserialized: Nonce = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(serialized.len(), NONCEBYTES + 2);
        assert_eq!(nonce, deserialized)
    }

    #[test]
    fn key_serialize_deserialize_works() {
        let key = gen_key();
        let serialized = rmp_serde::to_vec(&key).unwrap();
        let deserialized: Key = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(serialized.len(), KEYBYTES + 2);
        assert_eq!(key, deserialized)
    }

    #[test]
    fn writer_reader_works_for_less_than_one_chunk() {
        let plain = randombytes(7);
        let key = gen_key();
        let nonce = gen_nonce();

        let mut writer = Writer::new(Vec::<u8>::new(), key.clone(), nonce.clone(), 10);
        writer.write_all(&plain).unwrap();
        writer.flush().unwrap();

        let encrypted = writer.into_inner();
        assert_eq!(encrypted.len(), 7 + TAGBYTES);

        let mut decrypted = vec![0u8; 7];
        let mut reader = Reader::new(&encrypted[..], key, nonce, 10);
        reader.read_exact(&mut decrypted).unwrap();

        assert_eq!(plain, decrypted);
    }

    #[test]
    fn writer_reader_works_for_exact_one_chunk() {
        let plain = randombytes(10);
        let key = gen_key();
        let nonce = gen_nonce();

        let mut writer = Writer::new(Vec::<u8>::new(), key.clone(), nonce.clone(), 10);
        writer.write_all(&plain).unwrap();
        writer.flush().unwrap();

        let encrypted = writer.into_inner();
        assert_eq!(encrypted.len(), 10 + TAGBYTES);

        let mut decrypted = vec![0u8; 10];
        let mut reader = Reader::new(&encrypted[..], key, nonce, 10);
        reader.read_exact(&mut decrypted).unwrap();

        assert_eq!(plain, decrypted);
    }

    #[test]
    fn writer_reader_works_for_one_to_two_chunks() {
        let plain = randombytes(13);
        let key = gen_key();
        let nonce = gen_nonce();

        let mut writer = Writer::new(Vec::<u8>::new(), key.clone(), nonce.clone(), 10);
        writer.write_all(&plain).unwrap();
        writer.flush().unwrap();

        let encrypted = writer.into_inner();
        assert_eq!(encrypted.len(), 13 + 2 * TAGBYTES);

        let mut decrypted = vec![0u8; 13];
        let mut reader = Reader::new(&encrypted[..], key, nonce, 10);
        reader.read_exact(&mut decrypted).unwrap();

        assert_eq!(plain, decrypted);
    }

    #[test]
    fn writer_reader_works_for_exact_two_chunks() {
        let plain = randombytes(20);
        let key = gen_key();
        let nonce = gen_nonce();

        let mut writer = Writer::new(Vec::<u8>::new(), key.clone(), nonce.clone(), 10);
        writer.write_all(&plain).unwrap();
        writer.flush().unwrap();

        let encrypted = writer.into_inner();
        assert_eq!(encrypted.len(), 20 + 2 * TAGBYTES);

        let mut decrypted = vec![0u8; 20];
        let mut reader = Reader::new(&encrypted[..], key, nonce, 10);
        reader.read_exact(&mut decrypted).unwrap();

        assert_eq!(plain, decrypted);
    }
}

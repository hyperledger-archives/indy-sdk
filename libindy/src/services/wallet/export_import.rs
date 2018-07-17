use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io;
use std::io::{Write, Read, BufWriter, BufReader};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rmp_serde;

use domain::wallet::export_import::{Header, EncryptionMethod, Record};
use utils::crypto::hash::{hash, HASHBYTES};
use utils::crypto::chacha20poly1305_ietf;

use errors::common::CommonError;

use super::{WalletError, Wallet};

pub(super) fn export(wallet: &Wallet, writer: &mut Write, passphrase: &str, version: u32) -> Result<(), WalletError> {
    let encryption_method = EncryptionMethod::chacha20poly1305_ietf();

    let header = Header {
        encryption_method,
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        version,
    };

    // Writes plain structs
    let mut writer = StructWriter::new(BufWriter::new(writer));
    writer.write_next(&header)?;

    // Writes encrypted structs
    let mut writer = {
        let mut writer = match header.encryption_method {
            EncryptionMethod::ChaCha20Poly1305IETF { salt, nonce, chunk_size } => {
                chacha20poly1305_ietf::Writer::new(writer.into_inner(),
                                                   chacha20poly1305_ietf::derive_key(passphrase, &salt)?,
                                                   nonce,
                                                   chunk_size)
            }
        };

        StructWriter::new(writer)
    };

    let mut records = wallet.get_all()?;

    while let Some(record) = records.next()? {
        let record = Record {
            type_: record
                .get_type()
                .map(|s| s.to_string())
                .ok_or(CommonError::InvalidState("No type fetched for exported record".to_string()))?,
            id: record
                .get_id()
                .to_string(),
            value: record
                .get_value()
                .map(|s| s.to_string())
                .ok_or(CommonError::InvalidState("No value fetched for exported record".to_string()))?,
            tags: record
                .get_tags()
                .map(HashMap::clone)
                .ok_or(CommonError::InvalidState("No tags fetched for exported record".to_string()))?,
        };

        writer.write_next(&record)?;
    }

    writer.finalize()?;

    Ok(())
}

pub(super) fn import(wallet: &Wallet, reader: &mut Read, passphrase: &str) -> Result<(), WalletError> {
    // Reads plain structs
    let mut reader = StructReader::new(BufReader::new(reader));

    let header: Header = reader.read_next()?
        .ok_or(CommonError::InvalidStructure("No header found ".to_string()))?;

    // Reads encrypted structs
    let mut reader = {
        let mut reader = match header.encryption_method {
            EncryptionMethod::ChaCha20Poly1305IETF { salt, nonce, chunk_size } => {
                chacha20poly1305_ietf::Reader::new(reader.into_inner(),
                                                   chacha20poly1305_ietf::derive_key(passphrase, &salt)?,
                                                   nonce,
                                                   chunk_size)
            }
        };

        StructReader::new(reader)
    };

    while let Some(record) = reader.read_next::<Record>()? {
        wallet.add(&record.type_, &record.id, &record.value, &record.tags)?;
    }

    Ok(())
}

struct StructWriter<W: Write> {
    inner: W,
}

impl<W: Write> StructWriter<W> {
    pub fn new(inner: W) -> StructWriter<W> {
        StructWriter {
            inner,
        }
    }

    pub fn write_next<T>(&mut self, obj: &T) -> Result<(), CommonError> where T: Serialize {
        let obj = rmp_serde::to_vec(obj)
            .map_err(|err| CommonError::InvalidState(format!("Cannot serialize exported struct: {:?}", err)))?;

        self.inner.write_u32::<LittleEndian>(obj.len() as u32)?;
        self.inner.write_all(&obj)?;
        self.inner.write_all(&hash(&obj)?)?;
        Ok(())
    }

    pub fn finalize(mut self) -> Result<(), CommonError> {
        self.inner.write_u32::<LittleEndian>(0)?; // END message
        self.inner.flush()?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

struct StructReader<R: Read> {
    inner: R,
}

impl<R: Read> StructReader<R> {
    pub fn new(inner: R) -> StructReader<R> {
        StructReader {
            inner,
        }
    }

    pub fn read_next<T>(&mut self) -> Result<Option<T>, CommonError> where T: DeserializeOwned {
        let len = self.inner.read_u32::<LittleEndian>().map_err(_map_io_err)? as usize;

        if len > 0 {
            let mut buf = vec![0u8; len + HASHBYTES];

            self.inner.read_exact(&mut buf).map_err(_map_io_err)?;

            let obj = &buf[..len];
            let hashbytes = &buf[len..];

            if hash(obj)? == hashbytes {
                let obj: T = rmp_serde::from_slice(obj)
                    .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize struct: {}", err)))?;
                Ok(Some(obj))
            } else {
                Err(CommonError::InvalidStructure("Invalid hashbytes for serialized structure".to_string()))
            }
        } else {
            Ok(None)
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

fn _map_io_err(e: io::Error) -> CommonError {
    match e {
        ref e if e.kind() == io::ErrorKind::UnexpectedEof
            || e.kind() == io::ErrorKind::InvalidData => CommonError::InvalidStructure("Invalid export file format".to_string()),
        e => CommonError::IOError(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use std::rc::Rc;
    use std::collections::HashMap;

    use domain::wallet::Metadata;
    use utils::crypto::pwhash_argon2i13;
    use utils::test::TestUtils;
    use services::wallet::encryption;
    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::wallet::{Keys, Wallet};

    #[test]
    fn export_import_works_for_empty_wallet() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_wallet1(), &mut output, _passphrase(), _version1()).unwrap();

        let wallet = _wallet2();
        _assert_is_empty(&wallet);

        import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
        _assert_is_empty(&wallet);
    }

    #[test]
    fn export_import_works_for_2_items() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_add_2_records(_wallet1()), &mut output, _passphrase(), _version1()).unwrap();

        let wallet = _wallet2();
        _assert_is_empty(&wallet);

        import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
        _assert_has_2_records(&wallet);
    }

    #[test]
    fn export_import_works_for_multiple_items() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_add_300_records(_wallet1()), &mut output, _passphrase(), _version1()).unwrap();

        let wallet = _wallet2();
        _assert_is_empty(&wallet);

        import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
        _assert_has_300_records(&wallet);
    }

    #[test]
    fn import_works_for_empty() {
        _cleanup();

        let res = import(&_wallet1(), &mut "".as_bytes(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_cut_header_length() {
        _cleanup();

        let res = import(&_wallet1(), &mut "\x00".as_bytes(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_cut_header_body() {
        _cleanup();

        let res = import(&_wallet1(), &mut "\x00\x20small".as_bytes(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_invalid_header_body() {
        _cleanup();

        let output = {
            let invalid_header = "invalid_header".as_bytes();
            let mut output: Vec<u8> = Vec::new();
            output.write_u32::<LittleEndian>(invalid_header.len() as u32).unwrap();
            output.write_all(invalid_header).unwrap();
            output.write_all(&hash(invalid_header).unwrap()).unwrap();
            output
        };

        let res = import(&_wallet1(), &mut output.as_slice(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_invalid_header_hash() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_wallet1(), &mut output, _passphrase(), _version1()).unwrap();

        // Modifying one of the bytes in the header hash
        let pos = (&mut output.as_slice()).read_u32::<LittleEndian>().unwrap() as usize + 2;
        _change_byte(&mut output, pos);

        let res = import(&mut _wallet2(), &mut output.as_slice(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn export_import_works_for_changed_record() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_add_300_records(_wallet1()), &mut output, _passphrase(), _version1()).unwrap();

        // Modifying one byte in the middle of encrypted part
        let pos = output.len() / 2;
        _change_byte(&mut output, pos);

        let res = import(&mut _wallet2(), &mut output.as_slice(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_data_cut() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_add_2_records(_wallet1()), &mut output, _passphrase(), _version1()).unwrap();

        output.pop().unwrap();

        let res = import(&mut _wallet2(), &mut output.as_slice(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    #[test]
    fn import_works_for_data_extended() {
        _cleanup();

        let mut output: Vec<u8> = Vec::new();
        export(&_add_2_records(_wallet1()), &mut output, _passphrase(), _version1()).unwrap();

        output.push(10);

        let res = import(&mut _wallet2(), &mut output.as_slice(), _passphrase());
        assert_match!(Err(WalletError::CommonError(CommonError::InvalidStructure(_))), res);
    }

    fn _cleanup() {
        TestUtils::cleanup_storage()
    }

    fn _wallet1_id() -> &'static str {
        "w1"
    }

    fn _wallet2_id() -> &'static str {
        "w2"
    }

    fn _wallet(id: &str) -> Wallet {
        let storage_type = SQLiteStorageType::new();
        let master_key = _master_key();
        let keys = Keys::new();

        let metadata = {
            let master_key_salt = encryption::gen_master_key_salt().unwrap();

            let metadata = Metadata {
                master_key_salt: master_key_salt[..].to_vec(),
                keys: keys.serialize_encrypted(&master_key).unwrap(),
            };

            serde_json::to_vec(&metadata)
                .map_err(|err| CommonError::InvalidState(format!("Cannot serialize wallet metadata: {:?}", err))).unwrap()
        };

        storage_type.create_storage(id,
                                    None,
                                    None,
                                    &metadata).unwrap();

        let storage = storage_type.open_storage(id, None, None).unwrap();

        Wallet::new(id.to_string(), storage, Rc::new(keys))
    }

    fn _wallet1() -> Wallet {
        _wallet(_wallet1_id())
    }

    fn _wallet2() -> Wallet {
        _wallet(_wallet2_id())
    }

    fn _assert_is_empty(wallet: &Wallet) {
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());
    }

    fn _add_2_records(wallet: Wallet) -> Wallet {
        wallet.add(&_type1(), &_id1(), &_value1(), &_tags1()).unwrap();
        wallet.add(&_type2(), &_id2(), &_value2(), &_tags2()).unwrap();
        wallet
    }

    fn _assert_has_2_records(wallet: &Wallet) {
        let record = wallet.get(&_type1(), &_id1(), _options()).unwrap();
        assert_eq!(record.type_.unwrap(), _type1());
        assert_eq!(record.id, _id1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(record.tags.unwrap(), _tags1());

        let record = wallet.get(&_type2(), &_id2(), _options()).unwrap();
        assert_eq!(record.type_.unwrap(), _type2());
        assert_eq!(record.id, _id2());
        assert_eq!(record.value.unwrap(), _value2());
        assert_eq!(record.tags.unwrap(), _tags2());
    }

    fn _add_300_records(wallet: Wallet) -> Wallet {
        for i in 0..300 {
            wallet.add(&_type(i % 3), &_id(i), &_value(i), &_tags(i)).unwrap();
        }

        wallet
    }

    fn _assert_has_300_records(wallet: &Wallet) {
        for i in 0..300 {
            let record = wallet.get(&_type(i % 3), &_id(i), _options()).unwrap();
            assert_eq!(record.type_.unwrap(), _type(i % 3));
            assert_eq!(record.id, _id(i));
            assert_eq!(record.value.unwrap(), _value(i));
            assert_eq!(record.tags.unwrap(), _tags(i));
        }
    }

    fn _master_key() -> chacha20poly1305_ietf::Key {
        chacha20poly1305_ietf::gen_key()
    }

    fn _nonce() -> chacha20poly1305_ietf::Nonce {
        chacha20poly1305_ietf::gen_nonce()
    }

    fn _change_byte(data: &mut[u8], pos: usize) {
        let value = data[pos];
        data[pos] = if value < 255 { value + 1 } else { 0 };
    }

    fn _options() -> &'static str {
        r##"{"retrieveType": true, "retrieveValue": true, "retrieveTags": true}"##
    }

    fn _salt() -> pwhash_argon2i13::Salt {
        pwhash_argon2i13::Salt::new([
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
        ])
    }

    fn _version1() -> u32 {
        1
    }

    fn _id(suffix: usize) -> String {
        format!("id_{}", suffix)
    }

    fn _id1() -> String {
        _id(1)
    }

    fn _id2() -> String {
        _id(2)
    }

    fn _value(suffix: usize) -> String {
        format!("id_{}", suffix)
    }

    fn _value1() -> String {
        _value(1)
    }

    fn _value2() -> String {
        _value(2)
    }

    fn _type(suffix: usize) -> String {
        format!("type_{}", suffix)
    }

    fn _type1() -> String {
        _type(1)
    }

    fn _type2() -> String {
        _type(2)
    }

    fn _tags(suffix: usize) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert(format!("tag_id_{}_1", suffix), format!("tag_value_{}_1", suffix));
        tags.insert(format!("tag_id_{}_2", suffix), format!("tag_value_{}_2", suffix));
        tags.insert(format!("~tag_id_{}_3", suffix), format!("tag_value_{}_3", suffix));
        tags
    }

    fn _tags1() -> HashMap<String, String> {
        _tags(1)
    }

    fn _tags2() -> HashMap<String, String> {
        _tags(2)
    }

    fn _passphrase() -> &'static str {
        "key"
    }
}

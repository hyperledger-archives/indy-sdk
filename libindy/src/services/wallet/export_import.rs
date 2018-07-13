use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
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


//    let mut reader = BufReader::new(reader);
//
//    let mut header_length_bytes: [u8; 2] = [0; 2];
//    let read_count = reader.read(&mut header_length_bytes)?;
//    if read_count < 2 {
//        return Err(WalletError::CommonError(CommonError::InvalidStructure("Failed to read import header bytes".to_string())));
//    }
//    let header_length = bytes_to_u16(&header_length_bytes) as usize;
//    if header_length < 48 {
//        return Err(WalletError::CommonError(CommonError::InvalidStructure("Wallet import header not of sufficient minimal length".to_string())));
//    }
//
//    let mut header_data: Vec<u8> = vec![0; header_length + 2];
//    header_data[0] = header_length_bytes[0];
//    header_data[1] = header_length_bytes[1];
//
//    let mut header_read_count = 0;
//    while header_read_count < header_length {
//        let read_count = reader.read(&mut header_data[2 + header_read_count..])?;
//        if read_count == 0 {
//            return Err(WalletError::CommonError(CommonError::InvalidStructure("Header body length less than specified".to_string())));
//        } else {
//            header_read_count += read_count;
//        }
//    }
//
//    let header = Header::deserialise(&header_data)?;
//    let key = derive_master_key(passphrase, &header.salt)?;
//    let mut nonce = header.nonce;
//
//    let mut encrypted_chunk: [u8; 1040] = [0; 1040];
//    let mut decrypted_buffer = Vec::new();
//
//    let mut has_more = true;
//    while has_more {
//        let mut chunk_read_count = 0;
//        while chunk_read_count < 1040 {
//            let read_count = reader.read(&mut encrypted_chunk[chunk_read_count..])?;
//            if read_count == 0 {
//                has_more = false;
//                break;
//            }
//            chunk_read_count += read_count;
//        }
//
//        if chunk_read_count == 0 {
//            continue;
//        }
//
//        decrypted_buffer.extend(&decrypt(&encrypted_chunk[0..chunk_read_count], &key, &nonce)?);
//        nonce.increment();
//
//        add_records_from_buffer(wallet, &mut decrypted_buffer)?;
//    }
//
//    add_records_from_buffer(wallet, &mut decrypted_buffer)?;
//    if decrypted_buffer.len() != 0 {
//        return Err(WalletError::CommonError(CommonError::InvalidStructure("Failed to import all content".to_string())));
//    }

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
        let len = self.inner.read_u32::<LittleEndian>()? as usize;

        if len > 0 {
            let mut buf = vec![0u8; len + HASHBYTES];
            self.inner.read_exact(&mut buf)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use std;
    use std::io::{Read, BufReader};
    use std::rc::Rc;
    use std::collections::HashMap;
    use std::path::PathBuf;

    use domain::wallet::Metadata;
    use utils::crypto::pwhash_argon2i13;
    use utils::environment::EnvironmentUtils;
    use utils::test::TestUtils;
    use services::wallet::encryption;
    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::wallet::{Keys, Wallet};

    use super::super::WalletRecord;

    #[test]
    fn export_works_for_empty_wallet() {
        _cleanup();
        export(&_wallet1(), &mut _export_file(), _passphrase(), _version1()).unwrap();
    }

    #[test]
    fn export_works_for_2_records() {
        _cleanup();

        let wallet = _wallet1();
        wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
        wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

        export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
    }

    #[test]
    fn export_works_for_multiple_records() {
        _cleanup();

        let wallet = _wallet1();

        for i in 0..300 {
            let type_ = format!("type_{}", i);
            let id = format!("id_{}", i);
            let value = format!("value_{}", i);
            let tags = _tags(i);
            wallet.add(&type_, &id, &value, &tags).unwrap();
        }

        export(&wallet, &mut _export_file(), _passphrase(), 0).unwrap();
    }

    #[test]
    fn import_fails_if_header_length_too_small() {
        _cleanup();

        let mut wallet = _wallet1();
        let mut reader = BufReader::new("\x00\x20some_hash00000000000000000000000".as_bytes());

        let res = import(&mut wallet, &mut reader, "import_key");
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn import_fails_if_header_body_too_small() {
        _cleanup();

        let mut wallet = _wallet1();
        let mut reader = BufReader::new("\x00\x30this_hash_is_too_short".as_bytes());

        let res = import(&mut wallet, &mut reader, "import_key");
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_empty_wallet() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut wallet = _wallet2();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());

        import(&mut wallet, &mut _import_file(), _passphrase()).unwrap();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());
    }

    #[test]
    fn export_import_2_items() {
        _cleanup();

        {
            let mut wallet = _wallet("w1");

            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        import(&mut wallet, &mut _import_file(), _passphrase()).unwrap();
        let first_record = wallet.get("type1", "id1", _options()).unwrap();
        let second_record = wallet.get("type2", "id2", _options()).unwrap();

        assert_eq!(first_record.type_.unwrap(), "type1");
        assert_eq!(first_record.id, "id1");
        assert_eq!(first_record.value.unwrap(), "value1");
        assert_eq!(first_record.tags.unwrap(), _tags1());

        assert_eq!(second_record.type_.unwrap(), "type2");
        assert_eq!(second_record.id, "id2");
        assert_eq!(second_record.value.unwrap(), "value2");
        assert_eq!(second_record.tags.unwrap(), _tags2());
    }

    #[test]
    fn export_import_multiple_items() {
        _cleanup();

        let items_count = 300usize;

        {
            let mut wallet = _wallet1();

            for i in 0..items_count {
                let id = format!("id_{}", i);
                let value = format!("value_{}", i);
                let mut tags = _tags(i);
                wallet.add("type", &id, &value, &tags).unwrap();
            }

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut wallet = _wallet2();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());

        import(&mut wallet, &mut _import_file(), _passphrase()).unwrap();

        for i in 0..items_count {
            let id = format!("id_{}", i);
            let value = format!("value_{}", i);
            let tags = _tags(i);

            let record = wallet.get("type", &id, _options()).unwrap();
            assert_eq!(record.value.unwrap(), value);
            assert_eq!(record.tags.unwrap(), tags);
        }
    }

    #[test]
    fn export_import_returns_error_if_header_hash_broken() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        // Modifying one of the bytes in the header hash
        let mut content = _export_file_content();
        let index = 60;
        let byte_value = content[index];
        content[index] = if byte_value < 255 { byte_value + 1 } else { 0 };
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        let res = import(&mut wallet, &mut _export_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_data_does_not_match_header_hash() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        // Modifying one of the bytes in the header (version)
        let mut content = _export_file_content();
        let index = 4;
        let byte_value = content[index];
        content[index] = if byte_value < 255 { byte_value + 1 } else { 0 };
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_modified() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        let index = content.len() - 20;
        let byte_value = content[index];
        content[index] = if byte_value < 255 { byte_value + 1 } else { 0 };
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_cut_short() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        content.pop().unwrap();
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);

        let res = wallet.get("type1", "id1", _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_extended() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "id1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "id2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        content.push(10);
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "id1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);

        let res = wallet.get("type1", "id1", _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
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

    fn _master_key() -> chacha20poly1305_ietf::Key {
        chacha20poly1305_ietf::gen_key()
    }

    fn _nonce() -> chacha20poly1305_ietf::Nonce {
        chacha20poly1305_ietf::gen_nonce()
    }

    fn _export_file_path() -> PathBuf {
        let mut path = EnvironmentUtils::tmp_file_path("export_directory");
        path.push("export_file");
        path
    }

    fn _export_file() -> std::fs::File {
        let path = _export_file_path();
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::File::create(path).unwrap()
    }

    fn _export_file_content() -> Vec<u8> {
        let mut file = std::fs::File::open(_export_file_path()).unwrap();
        let mut v = Vec::new();
        let content = file.read_to_end(&mut v).expect("Failed to read exported file");
        v
    }

    fn _import_file() -> std::fs::File {
        std::fs::File::open(_export_file_path()).unwrap()
    }

    fn _replace_export_file(data: Vec<u8>) {
        let path = _export_file_path();
        std::fs::remove_file(path.as_path()).unwrap();
        let mut new_file = std::fs::File::create(path).unwrap();
        new_file.write_all(&data).unwrap();
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

    fn _wallet_record() -> WalletRecord {
        WalletRecord::new("name".to_string(),
                          Some("type".to_string()),
                          Some("value".to_string()),
                          Some(_tags1()))
    }

    fn _passphrase() -> &'static str {
        "key"
    }
}

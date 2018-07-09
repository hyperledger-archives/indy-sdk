use std::mem;
use std::io::{Write, Read, BufWriter, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use serde_json;

use utils::crypto::hash::Hash;
use utils::crypto::{chacha20poly1305_ietf, pwhash_argon2i13};
use services::wallet::encryption::{decrypt, derive_master_key};

use errors::common::CommonError;

use super::{WalletRecord, WalletError, Wallet};

#[derive(Debug)]
struct Header {
    version: u32,
    time: u64,
    encryption_method: String,
    nonce: chacha20poly1305_ietf::Nonce,
    salt: pwhash_argon2i13::Salt,
}

impl Header {
    fn new(version: u32, encryption_method: String, nonce: chacha20poly1305_ietf::Nonce, salt: pwhash_argon2i13::Salt) -> Header {
        Header {
            version,
            encryption_method,
            nonce,
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            salt,
        }
    }

    fn deserialise(serialised: &[u8]) -> Result<Header, WalletError> {
        let mut data_length: u16 = (serialised.len() - 52) as u16;
        let version = bytes_to_u32(&serialised[2..6]);
        let time = bytes_to_u64(&serialised[6..14]);

        let method_length = bytes_to_u16(&serialised[14..16]);
        if data_length < method_length {
            return Err(WalletError::CommonError(CommonError::InvalidStructure("Wallet header of insufficient size".to_string())));
        }
        let method_start_index: usize = 16;
        let method_end_index: usize = method_start_index + method_length as usize;
        let method_data = &serialised[method_start_index..method_end_index];
        let method = String::from_utf8(method_data.to_vec())?;
        data_length -= method_length;

        let nonce_length = bytes_to_u16(&serialised[method_end_index..method_end_index + 2]);
        if nonce_length > data_length {
            return Err(WalletError::CommonError(CommonError::InvalidStructure("Specified nonce length too long".to_string())));
        }
        let nonce_start_index = method_end_index + 2;
        let nonce_end_index = nonce_start_index + nonce_length as usize;
        let nonce_slice = &serialised[nonce_start_index..nonce_end_index];
        let nonce = chacha20poly1305_ietf::Nonce::from_slice(nonce_slice).unwrap(); // FIXME:
        data_length -= nonce_length;

        let salt_length = bytes_to_u16(&serialised[nonce_end_index..nonce_end_index + 2]);
        if salt_length != data_length {
            return Err(WalletError::CommonError(CommonError::InvalidStructure("Wallet header lengths mismatch".to_string())));
        }
        let salt_start_index = nonce_end_index + 2;
        let salt_end_index = salt_start_index + salt_length as usize;
        let salt_slice = &serialised[salt_start_index..salt_end_index];
        let salt = pwhash_argon2i13::Salt::from_slice(salt_slice).unwrap(); // FIXME:

        let actual_hash = sha256_hash(&serialised[..salt_end_index])?;
        if actual_hash != &serialised[salt_end_index..salt_end_index + 32] {
            return Err(WalletError::CommonError(CommonError::InvalidStructure("Wallet header hash mismatch".to_string())));
        }

        Ok(Header {
            version,
            time,
            encryption_method: method,
            nonce,
            salt,
        })
    }

    // Must return Result, since underlying hash library returns Result for some reason
    fn serialise(&self) -> Result<Vec<u8>, WalletError> {
        let mut v = Vec::new();
        let header_length = (18 + self.encryption_method.len() + chacha20poly1305_ietf::NONCEBYTES + self.salt[..].len() + 32) as u16;
        v.extend(&u16_to_bytes(header_length));
        let version_bytes = u32_to_bytes(self.version);
        v.extend(&version_bytes);
        let time_bytes = u64_to_bytes(self.time);
        v.extend(&time_bytes);
        let method_length_bytes = u16_to_bytes(self.encryption_method.len() as u16);
        v.extend(&method_length_bytes);
        v.extend(self.encryption_method.as_bytes());
        let nonce_length_bytes = u16_to_bytes(chacha20poly1305_ietf::NONCEBYTES as u16);
        v.extend(&nonce_length_bytes);
        v.extend(&self.nonce[..]);
        let salt_length_bytes = u16_to_bytes(self.salt[..].len() as u16);
        v.extend(&salt_length_bytes);
        v.extend(&self.salt[..]);
        let header_hash = sha256_hash(&v)?;
        v.extend(&header_hash);
        Ok(v)
    }
}

pub(super) fn export(wallet: &Wallet, writer: &mut Write, passphrase: &str, version: u32) -> Result<(), WalletError> {
    let salt = pwhash_argon2i13::gen_salt();
    let key = derive_master_key(passphrase, &salt)?;
    let mut writer = BufWriter::new(writer);
    let mut wallet_iterator = wallet.get_all()?;
    let mut nonce = chacha20poly1305_ietf::gen_nonce();
    let mut buffer = Vec::new();

    let header = Header::new(version, "ChaCha20Poly1305IETF".to_string(), nonce.clone(), salt);
    let serialised_header = header.serialise()?;
    writer.write_all(&serialised_header)?;

    while let Some(wallet_record) = wallet_iterator.next()? {
        serialize_record(wallet_record, &mut buffer)?;
        if buffer.len() < 1024 {
            continue;
        }

        let mut decrypt_index = 0;
        while decrypt_index + 1024 <= buffer.len() {
            let chunk = &buffer[decrypt_index..decrypt_index + 1024];
            let encrypted_chunk = chacha20poly1305_ietf::encrypt(chunk, &key, &nonce);
            nonce.increment();
            writer.write_all(&encrypted_chunk)?;
            decrypt_index += 1024;
        }

        let remaining = buffer.len() % 1024;
        if remaining > 0 {
            for i in 0..remaining {
                buffer[i] = buffer[decrypt_index + i];
            }
        }
        buffer.resize(remaining, 0);
    }

    if buffer.len() > 0 {
        let last_encrypted_chunk = chacha20poly1305_ietf::encrypt(&buffer, &key, &nonce);
        writer.write_all(&last_encrypted_chunk)?;
    }

    writer.flush()?;

    Ok(())
}

pub(super) fn import(wallet: &Wallet, reader: &mut Read, passphrase: &str) -> Result<(), WalletError> {
    let mut reader = BufReader::new(reader);

    let mut header_length_bytes: [u8; 2] = [0; 2];
    let read_count = reader.read(&mut header_length_bytes)?;
    if read_count < 2 {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Failed to read import header bytes".to_string())));
    }
    let header_length = bytes_to_u16(&header_length_bytes) as usize;
    if header_length < 48 {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Wallet import header not of sufficient minimal length".to_string())));
    }

    let mut header_data: Vec<u8> = vec![0; header_length + 2];
    header_data[0] = header_length_bytes[0];
    header_data[1] = header_length_bytes[1];

    let mut header_read_count = 0;
    while header_read_count < header_length {
        let read_count = reader.read(&mut header_data[2 + header_read_count..])?;
        if read_count == 0 {
            return Err(WalletError::CommonError(CommonError::InvalidStructure("Header body length less than specified".to_string())));
        } else {
            header_read_count += read_count;
        }
    }

    let header = Header::deserialise(&header_data)?;
    let key = derive_master_key(passphrase, &header.salt)?;
    let mut nonce = header.nonce;

    let mut encrypted_chunk: [u8; 1040] = [0; 1040];
    let mut decrypted_buffer = Vec::new();

    let mut has_more = true;
    while has_more {
        let mut chunk_read_count = 0;
        while chunk_read_count < 1040 {
            let read_count = reader.read(&mut encrypted_chunk[chunk_read_count..])?;
            if read_count == 0 {
                has_more = false;
                break;
            }
            chunk_read_count += read_count;
        }

        if chunk_read_count == 0 {
            continue;
        }

        decrypted_buffer.extend(&decrypt(&encrypted_chunk[0..chunk_read_count], &key, &nonce)?);
        nonce.increment();

        add_records_from_buffer(wallet, &mut decrypted_buffer)?;
    }

    add_records_from_buffer(wallet, &mut decrypted_buffer)?;
    if decrypted_buffer.len() != 0 {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Failed to import all content".to_string())));
    }

    Ok(())
}

fn add_records_from_buffer(wallet: &Wallet, buff: &mut Vec<u8>) -> Result<(), WalletError> {
    let mut index = 0;
    while index + 4 < buff.len() {
        let item_length = bytes_to_u32(&buff[index..index + 4]);
        let end_index = index + 4 + item_length as usize;
        if end_index > buff.len() {
            break;
        }

        let record = deserialize_record(&buff[index + 4..end_index])?;
        wallet.add(&record.type_.unwrap(), &record.name, &record.value.unwrap(), &record.tags.unwrap())?;
        index = end_index;
    }

    let remaining = buff.len() - index;
    for i in 0..remaining {
        buff[i] = buff[index + i];
    }
    buff.resize(remaining, 0);

    Ok(())
}

#[allow(deprecated)]
fn sha256_hash(input: &[u8]) -> Result<Vec<u8>, CommonError> {
    let mut hasher = Hash::new_context()?;
    hasher.update(input)?;
    Ok(hasher.finish()?) // TODO: use of deprecated item 'openssl::hash::Hasher::finish': use finish2 instead
}

fn serialize_record(record: WalletRecord, buffer: &mut Vec<u8>) -> Result<(), WalletError> {
    let record_type = record.type_.unwrap();
    let record_name = record.name;
    let record_value = record.value.unwrap();
    let record_tags = record.tags.unwrap();
    let record_tags_json = serde_json::to_string(&record_tags)?;
    let record_length = record_type.len() + record_name.len() + record_value.len() + record_tags_json.len() + 16;

    buffer.extend(&u32_to_bytes(record_length as u32));
    buffer.extend(&u32_to_bytes(record_type.len() as u32));
    buffer.extend(record_type.as_bytes());
    buffer.extend(&u32_to_bytes(record_name.len() as u32));
    buffer.extend(record_name.as_bytes());
    buffer.extend(&u32_to_bytes(record_value.len() as u32));
    buffer.extend(record_value.as_bytes());
    buffer.extend(&u32_to_bytes(record_tags_json.len() as u32));
    buffer.extend(record_tags_json.as_bytes());

    Ok(())
}

fn deserialize_record(mut buffer: &[u8]) -> Result<WalletRecord, WalletError> {
    let expected_total_length = buffer.len();
    let type_length = bytes_to_u32(&buffer[..4]) as usize;
    if type_length + 16 > buffer.len() {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Insufficient serialised data length".to_string())));
    }
    let type_ = String::from_utf8(buffer[4..4 + type_length].to_owned())?;
    buffer = &buffer[4 + type_length..];

    let name_length = bytes_to_u32(&buffer[..4]) as usize;
    if name_length + 12 > buffer.len() {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Insufficient serialised data length".to_string())));
    }
    let name = String::from_utf8(buffer[4..4 + name_length].to_owned())?;
    buffer = &buffer[4 + name_length..];

    let value_length = bytes_to_u32(&buffer[..4]) as usize;
    if value_length + 8 > buffer.len() {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Insufficient serialised data length".to_string())));
    }
    let value = String::from_utf8(buffer[4..4 + value_length].to_owned())?;
    buffer = &buffer[4 + value_length..];

    let tags_json_length = bytes_to_u32(&buffer[..4]) as usize;
    if tags_json_length > buffer.len() {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Insufficient serialised data length".to_string())));
    }

    let total_length = type_length + name_length + value_length + tags_json_length + 16;
    if total_length != expected_total_length {
        return Err(WalletError::CommonError(CommonError::InvalidStructure("Lengths mismatch during record deserialisation".to_string())));
    }

    let tags_json = String::from_utf8(buffer[4..4 + tags_json_length].to_owned())?;
    let tags: HashMap<String, String> = serde_json::from_str(&tags_json)?;

    let wallet_record = WalletRecord::new(name, Some(type_), Some(value), Some(tags));
    Ok(wallet_record)
}


fn u64_to_bytes(u: u64) -> [u8; 8] {
    let u = u.to_be();
    let res: [u8; 8] = unsafe { mem::transmute(u) };
    res
}

fn u32_to_bytes(u: u32) -> [u8; 4] {
    let u = u.to_be();
    let res: [u8; 4] = unsafe { mem::transmute(u) };
    res
}

fn u16_to_bytes(u: u16) -> [u8; 2] {
    let u = u.to_be();
    let res: [u8; 2] = unsafe { mem::transmute(u) };
    res
}

fn bytes_to_u64(b: &[u8]) -> u64 {
    let mut byte_array: [u8; 8] = [0; 8];
    byte_array.clone_from_slice(&b[0..8]);
    let res: u64 = unsafe { mem::transmute(byte_array) };
    u64::from_be(res)
}

fn bytes_to_u32(b: &[u8]) -> u32 {
    let mut byte_array: [u8; 4] = [0; 4];
    byte_array.clone_from_slice(&b[0..4]);
    let res: u32 = unsafe { mem::transmute(byte_array) };
    u32::from_be(res)
}

fn bytes_to_u16(b: &[u8]) -> u16 {
    let mut byte_array: [u8; 2] = [0; 2];
    byte_array.clone_from_slice(&b[0..2]);
    let res: u16 = unsafe { mem::transmute(byte_array) };
    u16::from_be(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    use std;
    use std::io::{Read, BufReader};
    use std::rc::Rc;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use serde_json;

    use domain::wallet::Metadata;
    use utils::environment::EnvironmentUtils;
    use utils::test::TestUtils;
    use services::wallet::encryption;
    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::wallet::{Keys, Wallet};

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

    fn _record_length_serialized(type_: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> usize {
        let tags_length = serde_json::to_string(tags).unwrap().len();
        type_.len() + name.len() + value.len() + tags_length + 16
    }

    fn _encryption_method() -> &'static str {
        "ChaCha20Poly1305IETF"
    }

    fn _expected_header_length() -> usize {
        2 + 12 + 2 + _encryption_method().len() + 2 + chacha20poly1305_ietf::NONCEBYTES + 2 + 32 + 32
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

    fn _header() -> Header {
        Header::new(_version1(),
                    _encryption_method().to_string(),
                    _nonce(),
                    _salt())
    }

    fn _tags(suffix: usize) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert(format!("tag_name_{}_1", suffix), format!("tag_value_{}_1", suffix));
        tags.insert(format!("tag_name_{}_2", suffix), format!("tag_value_{}_2", suffix));
        tags.insert(format!("~tag_name_{}_3", suffix), format!("tag_value_{}_3", suffix));
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

    fn _serialized_wallet_record() -> Vec<u8> {
        let mut buf = Vec::new();
        serialize_record(_wallet_record(), &mut buf).unwrap();
        buf
    }

    fn _passphrase() -> &'static str {
        "key"
    }

    /**
        Header tests
    */
    #[test]
    fn test_header_serialised_length() {
        let serialised_header = _header().serialise().unwrap();
        assert_eq!(serialised_header.len(), _expected_header_length());
    }

    #[test]
    fn test_header_equal_after_deserialization() {
        let header = _header();

        let serialized_header = header.serialise().unwrap();
        let deserialized_header = Header::deserialise(&serialized_header).unwrap();

        assert_eq!(header.version, deserialized_header.version);
        assert_eq!(header.time, deserialized_header.time);
        assert_eq!(header.encryption_method, deserialized_header.encryption_method);
        assert_eq!(header.nonce, deserialized_header.nonce);
        assert_eq!(header.salt, deserialized_header.salt);
    }

    #[test]
    fn test_header_deserialization_raises_error_if_data_changed() {
        let mut serialized_header = _header().serialise().unwrap();
        serialized_header[3] = 1;

        let res = Header::deserialise(&serialized_header);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_header_deserialization_raises_error_if_hash_changed() {
        let mut serialized_header = _header().serialise().unwrap();

        let index = serialized_header.len() - 5;
        let byte_value = serialized_header[index];
        serialized_header[index] = if byte_value < 255 { byte_value + 1 } else { 0 };

        let res = Header::deserialise(&serialized_header);

        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    /**
        Record serialisation deserialization tests
    */
    #[test]
    fn test_wallet_record_serialization_and_deserialization() {
        let serialized_record = _serialized_wallet_record();
        let deserialized_record = deserialize_record(&serialized_record[4..]).unwrap();

        assert_eq!(bytes_to_u32(&serialized_record[0..4]) as usize, serialized_record.len() - 4);
        assert_eq!(&deserialized_record.name, "name");
        assert_eq!(&deserialized_record.type_.unwrap(), "type");
        assert_eq!(&deserialized_record.value.unwrap(), "value");
        assert_eq!(&deserialized_record.tags.unwrap(), &_tags1());
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_type_length_changed() {
        let mut serialized_record = _serialized_wallet_record();

        let length_changed = u32_to_bytes(1000);
        serialized_record.splice(4..7, length_changed.iter().cloned());

        let res = deserialize_record(&serialized_record[4..]);
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_name_length_changed() {
        let mut serialized_record = _serialized_wallet_record();

        let length_changed = u32_to_bytes(1000);
        let length_index = 4 + 4 + "type".len();
        serialized_record.splice(length_index..length_index + 3, length_changed.iter().cloned());

        let res = deserialize_record(&serialized_record[4..]);
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_value_length_changed() {
        let mut serialized_record = _serialized_wallet_record();

        let length_changed = u32_to_bytes(1000);
        let length_index = 4 + 4 + "type".len() + 4 + "name".len();
        serialized_record.splice(length_index..length_index + 3, length_changed.iter().cloned());

        let res = deserialize_record(&serialized_record[4..]);
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_tags_length_changed() {
        let mut serialized_record = _serialized_wallet_record();

        let length_changed = u32_to_bytes(1000);
        let length_index = 4 + 4 + "type".len() + 4 + "name".len() + 4 + "value".len();
        serialized_record.splice(length_index..length_index + 3, length_changed.iter().cloned());

        let res = deserialize_record(&serialized_record[4..]);
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    /**
        Export/Import tests
    */
    #[test]
    fn export_empty_wallet() {
        _cleanup();
        
        let wallet = _wallet1();

        let mut export_file = _export_file();
        export(&wallet, &mut export_file, _passphrase(), _version1()).unwrap();

        assert_eq!(_export_file_content().len(), _expected_header_length());
    }

    #[test]
    fn export_2_items() {
        _cleanup();

        let wallet = _wallet1();

        wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
        let record_1_length = _record_length_serialized("type1", "name1", "value1", &_tags1());

        wallet.add("type2", "name2", "value2", &_tags2()).unwrap();
        let record_2_length = _record_length_serialized("type2", "name2", "value2", &_tags2());

        let mut export_file = _export_file();
        export(&wallet, &mut export_file, _passphrase(), _version1()).unwrap();

        assert_eq!(_export_file_content().len(), _expected_header_length() + record_1_length + record_2_length + 2 * 4 + 16);
    }

    #[test]
    fn export_multiple_items() {
        _cleanup();

        let wallet = _wallet1();
        let mut export_file = _export_file();

        let items_count = 300usize;
        let mut items_length = 0usize;

        for i in 0..items_count {
            let name = format!("name_{}", i);
            let value = format!("value_{}", i);
            let tags = _tags(i);
            items_length += 4 + name.len() + value.len() + serde_json::to_string(&tags).unwrap().len();
            wallet.add("type", &name, &value, &tags).unwrap();
        }

        export(&wallet, &mut export_file, _passphrase(), 0).unwrap();

        let total_unencrypted_length = items_length + items_count * 20;
        let chunk_count = f64::ceil(total_unencrypted_length as f64 / 1024.0) as usize;
        let expected_length = _expected_header_length() + total_unencrypted_length + (chunk_count * 16);

        assert_eq!(_export_file_content().len(), expected_length);
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

        let (record_1_length, record_2_length) = {
            let mut wallet = _wallet("w1");

            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            let record_1_length = _record_length_serialized("type1", "name1", "value1", &_tags1());

            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();
            let record_2_length = _record_length_serialized("type2", "name2", "value2", &_tags2());

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();

            (record_1_length, record_2_length)
        };

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        import(&mut wallet, &mut _import_file(), _passphrase()).unwrap();
        let first_record = wallet.get("type1", "name1", _options()).unwrap();
        let second_record = wallet.get("type2", "name2", _options()).unwrap();

        assert_eq!(first_record.type_.unwrap(), "type1");
        assert_eq!(first_record.name, "name1");
        assert_eq!(first_record.value.unwrap(), "value1");
        assert_eq!(first_record.tags.unwrap(), _tags1());

        assert_eq!(second_record.type_.unwrap(), "type2");
        assert_eq!(second_record.name, "name2");
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
                let name = format!("name_{}", i);
                let value = format!("value_{}", i);
                let mut tags = _tags(i);
                wallet.add("type", &name, &value, &tags).unwrap();
            }

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut wallet = _wallet2();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());

        import(&mut wallet, &mut _import_file(), _passphrase()).unwrap();

        for i in 0..items_count {
            let name = format!("name_{}", i);
            let value = format!("value_{}", i);
            let tags = _tags(i);

            let record = wallet.get("type", &name, _options()).unwrap();
            assert_eq!(record.value.unwrap(), value);
            assert_eq!(record.tags.unwrap(), tags);
        }
    }

    #[test]
    fn export_import_returns_error_if_header_hash_broken() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();

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
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        let res = import(&mut wallet, &mut _export_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_data_does_not_match_header_hash() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();

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
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_modified() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        let index = content.len() - 20;
        let byte_value = content[index];
        content[index] = if byte_value < 255 { byte_value + 1 } else { 0 };
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_cut_short() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        content.pop().unwrap();
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);

        let res = wallet.get("type1", "name1", _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_extended() {
        _cleanup();

        {
            let mut wallet = _wallet1();
            wallet.add("type1", "name1", "value1", &_tags1()).unwrap();
            wallet.add("type2", "name2", "value2", &_tags2()).unwrap();

            export(&wallet, &mut _export_file(), _passphrase(), _version1()).unwrap();
            wallet.close().unwrap();
        }

        let mut content = _export_file_content();
        content.push(10);
        _replace_export_file(content);

        let mut wallet = _wallet2();
        assert_match!(Err(WalletError::ItemNotFound), wallet.get("type1", "name1", "{}"));

        let res = import(&mut wallet, &mut _import_file(), _passphrase());
        assert_match!(Err(WalletError::CommonError(_)), res);

        let res = wallet.get("type1", "name1", _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }
}

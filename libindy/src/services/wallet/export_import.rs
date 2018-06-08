use std::mem;
use std::io::{Write, Read, BufWriter, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use serde_json;

use utils::crypto::hash::Hash;
use utils::crypto::chacha20poly1305_ietf::ChaCha20Poly1305IETF;
use utils::crypto::pwhash_argon2i13::PwhashArgon2i13;
use utils::byte_array::_clone_into_array;

use errors::common::CommonError;

use super::{WalletRecord, WalletError, Wallet};


#[derive(Debug)]
struct Header {
    version: u32,
    time: u64,
    encryption_method: String,
    nonce: Vec<u8>,
    salt: [u8; PwhashArgon2i13::SALTBYTES],
}


impl Header {
    fn new(version: u32, encryption_method: &str, nonce: &[u8], salt: [u8; PwhashArgon2i13::SALTBYTES]) -> Header {
        let current_time = SystemTime::now();
        let unix_time = current_time.duration_since(UNIX_EPOCH).unwrap();

        Header {
            version: version,
            encryption_method: encryption_method.to_owned(),
            nonce: nonce.to_owned(),
            time: unix_time.as_secs(),
            salt: salt,
        }
    }

    fn deserialise(serialised: &[u8]) -> Result<Header, WalletError> {
        let mut data_length: u16 = (serialised.len() - 52) as u16;
        let version = bytes_to_u32(&serialised[2..6]);
        let time = bytes_to_u64(&serialised[6..14]);

        let method_length = bytes_to_u16(&serialised[14..16]);
        if data_length < method_length {
            return Err(WalletError::StructureError("Wallet header of insufficient size".to_string()));
        }
        let method_start_index: usize = 16;
        let method_end_index: usize = method_start_index + method_length as usize;
        let method_data = &serialised[method_start_index..method_end_index];
        let method = String::from_utf8(method_data.to_vec())?;
        data_length -= method_length;

        let nonce_length = bytes_to_u16(&serialised[method_end_index..method_end_index + 2]);
        if nonce_length > data_length {
            return Err(WalletError::StructureError("Specified nonce length too long".to_string()));
        }
        let nonce_start_index = method_end_index + 2;
        let nonce_end_index = nonce_start_index + nonce_length as usize;
        let nonce_slice = &serialised[nonce_start_index..nonce_end_index];
        let nonce = nonce_slice.to_vec();
        data_length -= nonce_length;

        let salt_length = bytes_to_u16(&serialised[nonce_end_index..nonce_end_index+2]);
        if salt_length != data_length {
            return Err(WalletError::StructureError("Wallet header lengths mismatch".to_string()));
        }
        let salt_start_index = nonce_end_index + 2;
        let salt_end_index = salt_start_index + salt_length as usize;
        let salt_slice = &serialised[salt_start_index..salt_end_index];
        let salt: [u8; PwhashArgon2i13::SALTBYTES] = _clone_into_array(salt_slice);

        let actual_hash = sha256_hash(&serialised[..salt_end_index])?;
        if actual_hash != &serialised[salt_end_index..salt_end_index+32] {
            return Err(WalletError::StructureError("Wallet header hash mismatch".to_string()));
        }

        Ok(Header {
            version: version,
            time: time,
            encryption_method: method,
            nonce: nonce,
            salt: salt,
        })
    }

    // Must return Result, since underlying hash library returns Result for some reason
    fn serialise(&self) -> Result<Vec<u8>, WalletError> {
        let mut v = Vec::new();
        let header_length = (18 + self.encryption_method.len() + self.nonce.len() + self.salt.len() + 32) as u16;
        v.extend(&u16_to_bytes(header_length));
        let version_bytes = u32_to_bytes(self.version);
        v.extend(&version_bytes);
        let time_bytes = u64_to_bytes(self.time);
        v.extend(&time_bytes);
        let method_length_bytes = u16_to_bytes(self.encryption_method.len() as u16);
        v.extend(&method_length_bytes);
        v.extend(self.encryption_method.as_bytes());
        let nonce_length_bytes = u16_to_bytes(self.nonce.len() as u16);
        v.extend(&nonce_length_bytes);
        v.extend(&self.nonce);
        let salt_length_bytes = u16_to_bytes(self.salt.len() as u16);
        v.extend(&salt_length_bytes);
        v.extend(&self.salt);
        let header_hash = sha256_hash(&v)?;
        v.extend(&header_hash);
        Ok(v)
    }
}


pub (super) fn export(wallet: &Wallet, writer: Box<Write>, passphrase: &str, version: u32) -> Result<(), WalletError> {
    let mut key: [u8; ChaCha20Poly1305IETF::KEYBYTES] = [0; ChaCha20Poly1305IETF::KEYBYTES];
    let salt = PwhashArgon2i13::gen_salt();
    PwhashArgon2i13::derive_key(&mut key, passphrase.as_bytes(), &salt)?;
    let mut writer = BufWriter::new(writer);
    let mut wallet_iterator = wallet.get_all()?;
    let mut nonce = ChaCha20Poly1305IETF::gen_nonce();
    let mut buffer = Vec::new();

    let header = Header::new(version, "ChaCha20Poly1305IETF", &nonce, salt);
    let serialised_header = header.serialise()?;
    writer.write_all(&serialised_header)?;

    while let Some(wallet_record) = wallet_iterator.next()? {
        serialise_record(wallet_record, &mut buffer)?;
        if buffer.len() < 1024 {
            continue;
        }

        let mut decrypt_index = 0;
        while decrypt_index + 1024 <= buffer.len() {
            let chunk = &buffer[decrypt_index .. decrypt_index+1024];
            let encrypted_chunk = ChaCha20Poly1305IETF::encrypt(chunk, &key, &nonce);
            ChaCha20Poly1305IETF::increment_nonce(&mut nonce);
            writer.write_all(&encrypted_chunk)?;
            decrypt_index += 1024;
        }

        let remaining = buffer.len() % 1024;
        if remaining > 0 {
            for i in 0 .. remaining {
                buffer[i] = buffer[decrypt_index + i];
            }
        }
        buffer.resize(remaining, 0);
    }

    if buffer.len() > 0 {
        let last_encrypted_chunk = ChaCha20Poly1305IETF::encrypt(&buffer, &key, &nonce);
        writer.write_all(&last_encrypted_chunk)?;
    }

    writer.flush()?;

    Ok(())
}


pub (super) fn import(wallet: &Wallet, reader: Box<Read>, passphrase: &str) -> Result<(), WalletError> {
    let mut reader = BufReader::new(reader);

    let mut header_length_bytes: [u8; 2] = [0; 2];
    let read_count = reader.read(&mut header_length_bytes)?;
    if read_count < 2 {
        return Err(WalletError::StructureError("Failed to read import header bytes".to_string()));
    }
    let header_length = bytes_to_u16(&header_length_bytes) as usize;
    if header_length < 48 {
        return Err(WalletError::StructureError("Wallet import header not of sufficient minimal length".to_string()));
    }

    let mut header_data: Vec<u8> = vec![0; header_length + 2];
    header_data[0] = header_length_bytes[0];
    header_data[1] = header_length_bytes[1];

    let mut header_read_count = 0;
    while header_read_count < header_length {
        let read_count = reader.read(&mut header_data[2 + header_read_count..])?;
        if read_count == 0 {
            return Err(WalletError::StructureError("Header body length less than specified".to_string()));
        } else {
            header_read_count += read_count;
        }
    }

    let header = Header::deserialise(&header_data)?;
    let mut key: [u8; ChaCha20Poly1305IETF::KEYBYTES] = [0; ChaCha20Poly1305IETF::KEYBYTES];
    PwhashArgon2i13::derive_key(&mut key, passphrase.as_bytes(), &header.salt)?;
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

        decrypted_buffer.extend(&ChaCha20Poly1305IETF::decrypt(&encrypted_chunk[0..chunk_read_count], &key, &nonce)?);
        ChaCha20Poly1305IETF::increment_nonce(&mut nonce);

        add_records_from_buffer(wallet, &mut decrypted_buffer)?;
    }

    add_records_from_buffer(wallet, &mut decrypted_buffer)?;
    if decrypted_buffer.len() != 0 {
        return Err(WalletError::StructureError("Failed to import all content".to_string()));
    }

    Ok(())
}



fn add_records_from_buffer(wallet: &Wallet, buff: &mut Vec<u8>) -> Result<(), WalletError> {
    let mut index = 0;
    while index + 4 < buff.len() {
        let item_length = bytes_to_u32(&buff[index..index+4]);
        let end_index = index + 4 + item_length as usize;
        if end_index > buff.len() {
            break;
        }

        let record = deserialise_record(&buff[index+4..end_index])?;
        wallet.add(&record.type_.unwrap(), &record.name, &record.value.unwrap(), &record.tags.unwrap())?;
        index = end_index;
    }

    let remaining = buff.len() - index;
    for i in 0 .. remaining {
        buff[i] = buff[index + i];
    }
    buff.resize(remaining, 0);

    Ok(())
}


fn sha256_hash(input: &[u8]) -> Result<Vec<u8>, CommonError> {
    let mut hasher = Hash::new_context()?;
    hasher.update(input)?;
    Ok(hasher.finish()?)
}


fn serialise_record(record: WalletRecord, buffer: &mut Vec<u8>) -> Result<(), WalletError> {
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


fn deserialise_record(mut buffer: &[u8]) -> Result<WalletRecord, WalletError> {
    let expected_total_length = buffer.len();
    let type_length = bytes_to_u32(&buffer[..4]) as usize;
    if type_length + 16 > buffer.len() {
        return Err(WalletError::StructureError("Insufficient serialised data length".to_string()));
    }
    let type_ = String::from_utf8(buffer[4..4+type_length].to_owned())?;
    buffer = &buffer[4+type_length..];

    let name_length = bytes_to_u32(&buffer[..4]) as usize;
    if name_length + 12 > buffer.len() {
        return Err(WalletError::StructureError("Insufficient serialised data length".to_string()));
    }
    let name = String::from_utf8(buffer[4..4+name_length].to_owned())?;
    buffer = &buffer[4+name_length..];

    let value_length = bytes_to_u32(&buffer[..4]) as usize;
    if value_length + 8 > buffer.len() {
        return Err(WalletError::StructureError("Insufficient serialised data length".to_string()));
    }
    let value = String::from_utf8(buffer[4..4+value_length].to_owned())?;
    buffer = &buffer[4+value_length..];

    let tags_json_length = bytes_to_u32(&buffer[..4]) as usize;
    if tags_json_length > buffer.len() {
        return Err(WalletError::StructureError("Insufficient serialised data length".to_string()));
    }

    let total_length = type_length + name_length + value_length + tags_json_length + 16;
    if total_length != expected_total_length {
        return Err(WalletError::StructureError("Lengths mismatch during record deserialisation".to_string()));
    }

    let tags_json = String::from_utf8(buffer[4..4+tags_json_length].to_owned())?;
    let tags: HashMap<String, String> = serde_json::from_str(&tags_json)?;

    let wallet_record = WalletRecord::new(name,Some(type_), Some(value),Some(tags));
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
    use std;
    use std::io;
    use std::io::{Read, BufReader};
    use std::env;
    use std::collections::HashMap;
    use std::time;
    extern crate rand;
    use self::rand::*;
    use serde_json;

    use services::wallet::storage::WalletStorageType;
    use services::wallet::storage::default::SQLiteStorageType;
    use services::wallet::wallet::{Keys, Wallet};
    use super::*;


    fn _wallet_path() -> std::path::PathBuf {
        let mut path = env::home_dir().unwrap();
        path.push(".indy_client");
        path.push("wallet");
        path.push("test_wallet");
        path
    }

    fn _cleanup() {
        if _wallet_path().exists() {
            std::fs::remove_dir_all(_wallet_path()).unwrap();
        }
        std::fs::create_dir_all(_wallet_path()).unwrap();
    }

    fn _credentials() -> String {
        r##"{"master_key": "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=\n", "storage_credentials": {}}}"##.to_string()
    }

    fn _create_wallet() -> Wallet {
        let name = "test_wallet";
        let pool_name = "test_pool";
        let storage_type = SQLiteStorageType::new();
        let master_key = _get_test_master_key();
        storage_type.create_storage("test_wallet", None, "", &Keys::gen_keys(master_key)).unwrap();
        let credentials = _credentials();
        let storage = storage_type.open_storage("test_wallet", None, &credentials[..]).unwrap();

        let keys = Keys::new(
            ChaCha20Poly1305IETF::decrypt_merged(
                &storage.get_storage_metadata().unwrap(),
                &master_key
            ).unwrap()
        );

        Wallet::new(name, pool_name, storage, keys)
    }

    fn _get_test_master_key() -> [u8; 32] {
        return [
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }

    fn _create_export_file() -> Box<io::Write> {
        let path_str = "/tmp/wallet/export_test";
        let path = std::path::Path::new(path_str);
        if path.exists() {
            std::fs::remove_dir_all(path).unwrap();
        }
        std::fs::create_dir_all(path).unwrap();

        let export_file_path = "/tmp/wallet/export_test/export";
        let file = std::fs::File::create(export_file_path).unwrap();
        Box::new(file)
    }

    fn _get_export_file_content() -> Vec<u8> {
        let export_file_path = "/tmp/wallet/export_test/export";
        let mut file = std::fs::File::open(export_file_path).unwrap();
        let mut v = Vec::new();
        let content = file.read_to_end(&mut v).expect("Failed to read exported file");
        v
    }

    fn _replace_export_file(data: Vec<u8>) {
        let export_file_path = std::path::Path::new("/tmp/wallet/export_test/export");
        std::fs::remove_file(export_file_path).unwrap();
        let mut new_file = std::fs::File::create(export_file_path).unwrap();
        new_file.write_all(&data).unwrap();
    }

    fn _get_export_file_reader() -> Box<io::Read> {
        let export_file_path = "/tmp/wallet/export_test/export";
        Box::new(std::fs::File::open(export_file_path).unwrap())
    }

    fn _record_length_serialized(type_: &str, name: &str, value: &str, tags: &HashMap<String, String>) -> usize {
        let tags_length = serde_json::to_string(tags).unwrap().len();
        type_.len() + name.len() + value.len() + tags_length + 16
    }

    fn _expected_header_length() -> usize {
        2 + 12 + 2 + "ChaCha20Poly1305IETF".len() + 2 + 12 + 2 + 32 + 32
    }

    fn _options() -> &'static str {
        r##"{"retrieveType": true, "retrieveValue": true, "retrieveTags": true}"##
    }

    fn _get_test_salt() -> [u8; PwhashArgon2i13::SALTBYTES] {
        [
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,

        ]
    }

    /**
        Header tests
    */
    #[test]
    fn test_header_serialised_length() {
        let nonce = vec![1,2,3,4,5];
        let salt = _get_test_salt();
        let header = Header::new(1, "TEST_ENCRYPTION_METHOD", &nonce, salt);
        let serialised_header = header.serialise().unwrap();

        assert_eq!(serialised_header.len(), 2 + 12 + 2 + "TEST_ENCRYPTION_METHOD".len() + 2 + 5 + 2 + 32 + 32);
    }

    #[test]
    fn test_header_equal_after_deserialisation() {
        let nonce = vec![1,2,3,4,5];
        let salt = _get_test_salt();
        let header = Header::new(1, "TEST_ENCRYPTION_METHOD", &nonce, salt);

        let serialised_header = header.serialise().unwrap();
        let deserialised_header = Header::deserialise(&serialised_header).unwrap();

        assert_eq!(header.version, deserialised_header.version);
        assert_eq!(header.time, deserialised_header.time);
        assert_eq!(header.encryption_method, deserialised_header.encryption_method);
        assert_eq!(header.nonce, deserialised_header.nonce);
        assert_eq!(header.salt, deserialised_header.salt);
    }

    #[test]
    fn test_header_deserialisation_raises_error_if_data_changed() {
        let nonce = vec![1,2,3,4,5];
        let salt = _get_test_salt();
        let header = Header::new(1, "TEST_ENCRYPTION_METHOD", &nonce, salt);

        let mut serialised_header = header.serialise().unwrap();
        serialised_header[3] = 1;
        let res = Header::deserialise(&serialised_header);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn test_header_deserialisation_raises_error_if_hash_changed() {
        let nonce = vec![1,2,3,4,5];
        let salt = _get_test_salt();
        let header = Header::new(1, "TEST_ENCRYPTION_METHOD", &nonce, salt);

        let mut serialised_header = header.serialise().unwrap();
        let index = serialised_header.len() - 5;
        let mut byte_value = serialised_header[index];
        if byte_value == 255 {
            byte_value = 0;
        } else {
            byte_value += 1;
        }
        serialised_header[index] = byte_value;

        let res = Header::deserialise(&serialised_header);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    /**
        Record serialisation deserialisation tests
    */
    #[test]
    fn test_wallet_record_serialization_and_deserialization() {
        let name = String::from("name");
        let type_ = String::from("type");
        let value = String::from("value");
        let mut tags = HashMap::new();
        tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        tags.insert(String::from("~tag_name_2"), String::from("tag_value_2"));
        tags.insert(String::from("tag_name_3"), String::from("tag_value_3"));

        let record = WalletRecord::new(name.clone(), Some(type_.clone()), Some(value.clone()), Some(tags.clone()));
        let mut buff = Vec::new();
        serialise_record(record, &mut buff).unwrap();

        let serialised_length = bytes_to_u32(&buff[0..4]) as usize;
        let deserialised_record = deserialise_record(&buff[4..]).unwrap();
        assert_eq!(serialised_length, buff.len() - 4);
        assert_eq!(&deserialised_record.name, &name);
        assert_eq!(&deserialised_record.type_.unwrap(), &type_);
        assert_eq!(&deserialised_record.value.unwrap(), &value);
        assert_eq!(&deserialised_record.tags.unwrap(), &tags);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_type_length_changed() {
        let name = String::from("name");
        let type_ = String::from("type");
        let value = String::from("value");
        let mut tags = HashMap::new();
        tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        tags.insert(String::from("~tag_name_2"), String::from("tag_value_2"));
        tags.insert(String::from("tag_name_3"), String::from("tag_value_3"));

        let record = WalletRecord::new(name.clone(), Some(type_.clone()), Some(value.clone()), Some(tags.clone()));
        let mut buff = Vec::new();
        serialise_record(record, &mut buff).unwrap();
        let new_type_length = 1000;
        let new_type_length_bytes = u32_to_bytes(new_type_length);
        buff[4] = new_type_length_bytes[0];
        buff[5] = new_type_length_bytes[1];
        buff[6] = new_type_length_bytes[2];
        buff[7] = new_type_length_bytes[3];

        let serialised_length = bytes_to_u32(&buff[0..4]) as usize;
        let res = deserialise_record(&buff[4..]);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_name_length_changed() {
        let name = String::from("name");
        let type_ = String::from("type");
        let value = String::from("value");
        let mut tags = HashMap::new();
        tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        tags.insert(String::from("~tag_name_2"), String::from("tag_value_2"));
        tags.insert(String::from("tag_name_3"), String::from("tag_value_3"));

        let record = WalletRecord::new(name.clone(), Some(type_.clone()), Some(value.clone()), Some(tags.clone()));
        let mut buff = Vec::new();
        serialise_record(record, &mut buff).unwrap();
        let new_name_length = 1000;
        let new_name_length_bytes = u32_to_bytes(new_name_length);
        let name_length_index = 4 + 4 + "type".len();
        buff[name_length_index] = new_name_length_bytes[0];
        buff[name_length_index + 1] = new_name_length_bytes[1];
        buff[name_length_index + 2] = new_name_length_bytes[2];
        buff[name_length_index + 3] = new_name_length_bytes[3];

        let res = deserialise_record(&buff[4..]);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_value_length_changed() {
        let name = String::from("name");
        let type_ = String::from("type");
        let value = String::from("value");
        let mut tags = HashMap::new();
        tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        tags.insert(String::from("~tag_name_2"), String::from("tag_value_2"));
        tags.insert(String::from("tag_name_3"), String::from("tag_value_3"));

        let record = WalletRecord::new(name.clone(), Some(type_.clone()), Some(value.clone()), Some(tags.clone()));
        let mut buff = Vec::new();
        serialise_record(record, &mut buff).unwrap();
        let new_value_length = 1000;
        let new_value_length_bytes = u32_to_bytes(new_value_length);
        let value_length_index = 4 + 4 + "type".len() + 4 + "name".len();
        buff[value_length_index] = new_value_length_bytes[0];
        buff[value_length_index + 1] = new_value_length_bytes[1];
        buff[value_length_index + 2] = new_value_length_bytes[2];
        buff[value_length_index + 3] = new_value_length_bytes[3];

        let res = deserialise_record(&buff[4..]);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn test_wallet_record_serialization_and_deserialization_if_tags_length_changed() {
        let name = String::from("name");
        let type_ = String::from("type");
        let value = String::from("value");
        let mut tags = HashMap::new();
        tags.insert(String::from("~tag_name_1"), String::from("tag_value_1"));
        tags.insert(String::from("~tag_name_2"), String::from("tag_value_2"));
        tags.insert(String::from("tag_name_3"), String::from("tag_value_3"));

        let record = WalletRecord::new(name.clone(), Some(type_.clone()), Some(value.clone()), Some(tags.clone()));
        let mut buff = Vec::new();
        serialise_record(record, &mut buff).unwrap();
        let new_tags_length = 1000;
        let new_tags_length_bytes = u32_to_bytes(new_tags_length);
        let tags_length_index = 4 + 4 + "type".len() + 4 + "name".len() + 4 + "value".len();
        buff[tags_length_index] = new_tags_length_bytes[0];
        buff[tags_length_index + 1] = new_tags_length_bytes[1];
        buff[tags_length_index + 2] = new_tags_length_bytes[2];
        buff[tags_length_index + 3] = new_tags_length_bytes[3];

        let res = deserialise_record(&buff[4..]);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }


    /**
        Export/Import tests
    */


    #[test]
    fn export_empty_wallet() {
        _cleanup();
        let mut wallet = _create_wallet();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 0).unwrap();

        let exported_content = _get_export_file_content();
        assert_eq!(exported_content.len(), _expected_header_length());
    }

    #[test]
    fn export_2_items() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 0).unwrap();

        let exported_content = _get_export_file_content();
        assert_eq!(exported_content.len(), _expected_header_length() + record_1_length + record_2_length + 2 * 4 + 16);
    }

    #[test]
    fn export_multiple_items() {
        _cleanup();
        let mut wallet = _create_wallet();

        let mut total_item_length = 0;
        let item_count = 300;
        for i in 0 .. item_count {
            let name = format!("name_{}", i);
            let value = format!("value_{}", i);
            let mut tags = HashMap::new();
            tags.insert(format!("tag_name_{}_1", i), format!("tag_value_{}_1", i));
            tags.insert(format!("tag_name_{}_2", i), format!("tag_value_{}_2", i));
            tags.insert(format!("~tag_name_{}_3", i), format!("tag_value_{}_3", i));
            let tags_len = serde_json::to_string(&tags).unwrap().len();
            total_item_length += (4 + name.len() + value.len() + tags_len);
            wallet.add("type", &name, &value, &tags).unwrap();
        }
        let total_unencrypted_length = total_item_length + item_count * 20;

        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 0).unwrap();

        let exported_content = _get_export_file_content();
        let chunk_count = f64::ceil(total_unencrypted_length as f64 / 1024.0) as usize;
        let expected_length = _expected_header_length() + total_unencrypted_length + (chunk_count * 16);
        assert_eq!(exported_content.len(), expected_length);
    }

    #[test]
    fn import_fails_if_header_length_too_small() {
        _cleanup();
        let mut wallet = _create_wallet();
        let reader = Box::new(BufReader::new("\x00\x20some_hash00000000000000000000000".as_bytes()));
        let key = "import_key";

        let res = import(&mut wallet, reader, key);

        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn import_fails_if_header_body_too_small() {
        _cleanup();
        let mut wallet = _create_wallet();
        let reader = Box::new(BufReader::new("\x00\x30this_hash_is_too_short".as_bytes()));
        let key = "import_key";

        let res = import(&mut wallet, reader, key);

        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn export_import_empty_wallet() {
        _cleanup();
        let mut wallet = _create_wallet();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());

        import(&mut wallet, reader, key).expect("Failed to import wallet");
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());
    }

    #[test]
    fn export_import_2_items() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let first_res = wallet.get(type1, name1, "{}");
        assert_match!(Err(WalletError::ItemNotFound), first_res);

        import(&mut wallet, reader, key).expect("Failed to import wallet");
        let first_record = wallet.get(type1, name1, _options()).expect("Failed to retrieve first item");
        let second_record = wallet.get(type2, name2, _options()).expect("Failed to retrieve second item");

        assert_eq!(first_record.type_.unwrap(), type1);
        assert_eq!(first_record.name, name1);
        assert_eq!(first_record.value.unwrap(), value1);
        assert_eq!(first_record.tags.unwrap(), tags1);

        assert_eq!(second_record.type_.unwrap(), type2);
        assert_eq!(second_record.name, name2);
        assert_eq!(second_record.value.unwrap(), value2);
        assert_eq!(second_record.tags.unwrap(), tags2);
    }

    #[test]
    fn export_import_multiple_items() {
        _cleanup();
        let mut wallet = _create_wallet();

        let mut total_item_length = 0;
        let item_count = rand::thread_rng().gen_range(40, 80);
        for i in 0 .. item_count {
            let name = format!("name_{}", i);
            let value = format!("value_{}", i);
            let mut tags = HashMap::new();
            tags.insert(format!("tag_name_{}_1", i), format!("tag_value_{}_1", i));
            tags.insert(format!("tag_name_{}_2", i), format!("tag_value_{}_2", i));
            tags.insert(format!("~tag_name_{}_3", i), format!("tag_value_{}_3", i));
            let tags_len = serde_json::to_string(&tags).unwrap().len();
            total_item_length += (4 + name.len() + value.len() + tags_len);
            wallet.add("type", &name, &value, &tags).unwrap();
        }
        let total_unencrypted_length = total_item_length + item_count * 20;

        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 0).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        assert!(wallet.get_all().unwrap().next().unwrap().is_none());
        import(&mut wallet, reader, key).expect("Failed to import wallet");

        for i in 0 .. item_count {
            let name = format!("name_{}", i);
            let value = format!("value_{}", i);
            let mut tags = HashMap::new();
            tags.insert(format!("tag_name_{}_1", i), format!("tag_value_{}_1", i));
            tags.insert(format!("tag_name_{}_2", i), format!("tag_value_{}_2", i));
            tags.insert(format!("~tag_name_{}_3", i), format!("tag_value_{}_3", i));
            let retrieved_record = wallet.get("type", &name, _options()).unwrap();
            assert_eq!(retrieved_record.value.unwrap(), value);
            assert_eq!(retrieved_record.tags.unwrap(), tags);
        }
    }

    #[test]
    fn export_import_returns_error_if_header_hash_broken() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        // Modifying one of the bytes in the header hash
        let mut content = _get_export_file_content();
        let index = 60;
        let byte_value = content[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        content[index] = new_byte_value;
        _replace_export_file(content);

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let first_res = wallet.get(type1, name1, "{}");
        assert_match!(Err(WalletError::ItemNotFound), first_res);

        let res = import(&mut wallet, reader, key);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_data_does_not_match_header_hash() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        // Modifying one of the bytes in the header (version)
        let mut content = _get_export_file_content();
        let index = 4;
        let byte_value = content[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        content[index] = new_byte_value;
        _replace_export_file(content);

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let first_res = wallet.get(type1, name1, "{}");
        assert_match!(Err(WalletError::ItemNotFound), first_res);

        let res = import(&mut wallet, reader, key);
        assert_match!(Err(WalletError::StructureError(_)), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_modified() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let mut content = _get_export_file_content();
        let index = content.len() - 20;
        let byte_value = content[index];
        let new_byte_value = if byte_value == 255 { 0 } else { byte_value + 1 };
        content[index] = new_byte_value;
        _replace_export_file(content);

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let res = import(&mut wallet, reader, key);
        assert_match!(Err(_), res);
        let res = wallet.get(type1, name1, _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_cut_short() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let mut content = _get_export_file_content();
        content.pop().unwrap();
        _replace_export_file(content);

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let res = import(&mut wallet, reader, key);
        assert_match!(Err(_), res);
        let res = wallet.get(type1, name1, _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }

    #[test]
    fn export_import_returns_error_if_encrypted_data_extended() {
        _cleanup();
        let type1 = "type1";
        let name1 = "name1";
        let value1 = "value1";
        let mut tags1 = HashMap::new();
        tags1.insert("tag_name_1".to_string(), "tag_value_1".to_string());
        tags1.insert("tag_name_2".to_string(), "tag_value_2".to_string());
        tags1.insert("~tag_name_3".to_string(), "tag_value_3".to_string());
        let record_1_length = _record_length_serialized(type1, name1, value1, &tags1);
        let type2 = "type2";
        let name2 = "name2";
        let value2 = "value2";
        let mut tags2 = HashMap::new();
        tags2.insert("tag_name_21".to_string(), "tag_value_21".to_string());
        tags2.insert("tag_name_22".to_string(), "tag_value_22".to_string());
        tags2.insert("~tag_name_23".to_string(), "tag_value_23".to_string());
        let record_2_length = _record_length_serialized(type2, name2, value2, &tags2);
        let mut wallet = _create_wallet();
        wallet.add(type1, name1, value1, &tags1).unwrap();
        wallet.add(type2, name2, value2, &tags2).unwrap();
        let export_writer = _create_export_file();
        let key = "key";

        export(&wallet, export_writer, key, 1).unwrap();
        wallet.close().unwrap();
        _cleanup();

        let mut content = _get_export_file_content();
        content.push(10);
        _replace_export_file(content);

        let reader = _get_export_file_reader();
        let mut wallet = _create_wallet();
        let res = import(&mut wallet, reader, key);
        assert_match!(Err(_), res);
        let res = wallet.get(type1, name1, _options());
        assert_match!(Err(WalletError::ItemNotFound), res);
    }
}
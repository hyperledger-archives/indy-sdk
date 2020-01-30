use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rmp_serde;

use indy_api_types::domain::wallet::Record;
use indy_api_types::domain::wallet::KeyDerivationMethod;
use indy_api_types::errors::prelude::*;
use crate::encryption::KeyDerivationData;
use indy_utils::crypto::{chacha20poly1305_ietf, pwhash_argon2i13};
use indy_utils::crypto::hash::{hash, HASHBYTES};

use super::{Wallet, WalletRecord};

const CHUNK_SIZE: usize = 1024;

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionMethod {
    // **ChaCha20-Poly1305-IETF** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETF {
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        salt: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
    // **ChaCha20-Poly1305-IETF interactive key derivation** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETFInteractive {
        // pwhash_argon2i13::Salt as bytes. Random salt used for deriving of key from passphrase
        salt: Vec<u8>,
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
    // **ChaCha20-Poly1305-IETF raw key** cypher in blocks per chunk_size bytes
    ChaCha20Poly1305IETFRaw {
        // chacha20poly1305_ietf::Nonce as bytes. Random start nonce. We increment nonce for each chunk to be sure in export file consistency
        nonce: Vec<u8>,
        // size of encrypted chunk
        chunk_size: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    // Method of encryption for encrypted stream
    pub encryption_method: EncryptionMethod,
    // Export time in seconds from UNIX Epoch
    pub time: u64,
    // Version of header
    pub version: u32
}

// Note that we use externally tagged enum serialization and header will be represented as:
//
// {
//   "encryption_method": {
//     "ChaCha20Poly1305IETF": {
//       "salt": ..,
//       "nonce": ..,
//       "chunk_size": ..,
//     },
//   },
//   "time": ..,
//   "version": ..,
// }

pub(super) fn export_continue(wallet: &Wallet, writer: &mut dyn Write, version: u32, key: chacha20poly1305_ietf::Key, key_data: &KeyDerivationData) -> IndyResult<()> {
    let nonce = chacha20poly1305_ietf::gen_nonce();
    let chunk_size = CHUNK_SIZE;

    let encryption_method = match key_data {
        KeyDerivationData::Argon2iMod(_, salt) => EncryptionMethod::ChaCha20Poly1305IETF {
            salt: salt[..].to_vec(),
            nonce: nonce[..].to_vec(),
            chunk_size,
        },
        KeyDerivationData::Argon2iInt(_, salt) => EncryptionMethod::ChaCha20Poly1305IETFInteractive {
            salt: salt[..].to_vec(),
            nonce: nonce[..].to_vec(),
            chunk_size,
        },
        KeyDerivationData::Raw(_) => EncryptionMethod::ChaCha20Poly1305IETFRaw {
            nonce: nonce[..].to_vec(),
            chunk_size,
        }
    };

    let header = Header {
        encryption_method,
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        version
    };

    let header = rmp_serde::to_vec(&header)
        .to_indy(IndyErrorKind::InvalidState, "Can't serialize wallet export file header")?;

    // Write plain
    let mut writer = BufWriter::new(writer);
    writer.write_u32::<LittleEndian>(header.len() as u32)?;
    writer.write_all(&header)?;

    // Write ecnrypted
    let mut writer = chacha20poly1305_ietf::Writer::new(writer,
                                                        key,
                                                        nonce,
                                                        chunk_size);

    writer.write_all(&hash(&header)?)?;

    let mut records = wallet.get_all()?;

    while let Some(WalletRecord { type_, id, value, tags }) = records.next()? {
        let record = Record {
            type_: type_.ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No type fetched for exported record"))?,
            id,
            value: value.ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No value fetched for exported record"))?,
            tags: tags.ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No tags fetched for exported record"))?,
        };

        let record = rmp_serde::to_vec(&record)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize record")?;

        writer.write_u32::<LittleEndian>(record.len() as u32)?;
        writer.write_all(&record)?;
    }

    writer.write_u32::<LittleEndian>(0)?; // END message
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
fn import<T>(wallet: &Wallet, reader: T, passphrase: &str) -> IndyResult<()> where T: Read {
    let (reader, import_key_derivation_data, nonce, chunk_size, header_bytes) = preparse_file_to_import(reader, passphrase)?;
    let import_key = import_key_derivation_data.calc_master_key()?;
    finish_import(wallet, reader, import_key, nonce, chunk_size, header_bytes)
}

pub(super) fn preparse_file_to_import<T>(reader: T, passphrase: &str) -> IndyResult<(BufReader<T>, KeyDerivationData, chacha20poly1305_ietf::Nonce, usize, Vec<u8>)> where T: Read {
    // Reads plain
    let mut reader = BufReader::new(reader);

    let header_len = reader.read_u32::<LittleEndian>().map_err(_map_io_err)? as usize;

    if header_len == 0 {
        return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid header length"));
    }

    let mut header_bytes = vec![0u8; header_len];
    reader.read_exact(&mut header_bytes).map_err(_map_io_err)?;

    let header: Header = rmp_serde::from_slice(&header_bytes)
        .to_indy(IndyErrorKind::InvalidStructure, "Header is malformed json")?;

    if header.version != 0 {
        return Err(err_msg(IndyErrorKind::InvalidStructure, "Unsupported version"));
    }

    let key_derivation_method = match header.encryption_method {
        EncryptionMethod::ChaCha20Poly1305IETF { .. } => KeyDerivationMethod::ARGON2I_MOD,
        EncryptionMethod::ChaCha20Poly1305IETFInteractive { .. } => KeyDerivationMethod::ARGON2I_INT,
        EncryptionMethod::ChaCha20Poly1305IETFRaw { .. } => KeyDerivationMethod::RAW,
    };

    let (import_key_derivation_data, nonce, chunk_size) = match header.encryption_method {
        EncryptionMethod::ChaCha20Poly1305IETF { salt, nonce, chunk_size } | EncryptionMethod::ChaCha20Poly1305IETFInteractive { salt, nonce, chunk_size } => {
            let salt = pwhash_argon2i13::Salt::from_slice(&salt)
                .to_indy(IndyErrorKind::InvalidStructure, "Invalid salt")?;

            let nonce = chacha20poly1305_ietf::Nonce::from_slice(&nonce)
                .to_indy(IndyErrorKind::InvalidStructure, "Invalid nonce")?;

            let passphrase = passphrase.to_owned();

            let key_data = match key_derivation_method {
                KeyDerivationMethod::ARGON2I_INT =>
                    KeyDerivationData::Argon2iInt(passphrase, salt),
                KeyDerivationMethod::ARGON2I_MOD =>
                    KeyDerivationData::Argon2iMod(passphrase, salt),
                _ => unimplemented!("FIXME") //FIXME
            };

            (key_data, nonce, chunk_size)
        }
        EncryptionMethod::ChaCha20Poly1305IETFRaw { nonce, chunk_size } => {
            let nonce = chacha20poly1305_ietf::Nonce::from_slice(&nonce)
                .to_indy(IndyErrorKind::InvalidStructure, "Invalid nonce")?;

            let key_data = KeyDerivationData::Raw(passphrase.to_owned());

            (key_data, nonce, chunk_size)
        }
    };

    Ok((reader, import_key_derivation_data, nonce, chunk_size, header_bytes))
}

pub(super) fn finish_import<T>(wallet: &Wallet, reader: BufReader<T>, key: chacha20poly1305_ietf::Key, nonce: chacha20poly1305_ietf::Nonce, chunk_size: usize, header_bytes: Vec<u8>) -> IndyResult<()> where T: Read {
    // Reads encrypted
    let mut reader = chacha20poly1305_ietf::Reader::new(reader, key, nonce, chunk_size);

    let mut header_hash = vec![0u8; HASHBYTES];
    reader.read_exact(&mut header_hash).map_err(_map_io_err)?;

    if hash(&header_bytes)? != header_hash {
        return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid header hash"));
    }

    loop {
        let record_len = reader.read_u32::<LittleEndian>().map_err(_map_io_err)? as usize;

        if record_len == 0 {
            break;
        }

        let mut record = vec![0u8; record_len];
        reader.read_exact(&mut record).map_err(_map_io_err)?;

        let record: Record = rmp_serde::from_slice(&record)
            .to_indy(IndyErrorKind::InvalidStructure, "Record is malformed msgpack")?;

        wallet.add(&record.type_, &record.id, &record.value, &record.tags)?;
    }

    Ok(())
}

fn _map_io_err(e: io::Error) -> IndyError {
    match e {
        ref e if e.kind() == io::ErrorKind::UnexpectedEof
            || e.kind() == io::ErrorKind::InvalidData => err_msg(IndyErrorKind::InvalidStructure, "Invalid export file format"),
        e => e.to_indy(IndyErrorKind::IOError, "Can't read export file"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;

    use serde_json;

    use crate::{Metadata, MetadataArgon};
    use crate::encryption;
    use crate::storage::default::SQLiteStorageType;
    use crate::storage::WalletStorageType;
    use crate::wallet::{Keys, Wallet};
    use indy_utils::crypto::pwhash_argon2i13;
    use indy_utils::test;

    use super::*;

    fn export(wallet: &Wallet, writer: &mut dyn Write, passphrase: &str, version: u32, key_derivation_method: &KeyDerivationMethod) -> IndyResult<()> {
        if version != 0 {
            Err(err_msg(IndyErrorKind::InvalidState, "Unsupported version"))?;
        }

        let key_data = KeyDerivationData::from_passphrase_with_new_salt(passphrase, key_derivation_method);
        let key = key_data.calc_master_key()?;

        export_continue(wallet, writer, version, key, &key_data)
    }

    #[test]
    fn export_import_works_for_empty_wallet() {
        _cleanup("export_import_works_for_empty_wallet1");
        _cleanup("export_import_works_for_empty_wallet2");
        {
            let mut output: Vec<u8> = Vec::new();
            {
                let mut wallet1 = _wallet("export_import_works_for_empty_wallet1");
                export(&wallet1, &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

                wallet1.close().unwrap();
            }
            test::cleanup_wallet("export_import_works_for_empty_wallet1");

            let mut wallet = _wallet("export_import_works_for_empty_wallet2");
            _assert_is_empty(&wallet);

            import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
            _assert_is_empty(&wallet);

            wallet.close().unwrap();
        }
        test::cleanup_wallet("export_import_works_for_empty_wallet2");
    }

    #[test]
    fn export_import_works_for_2_items() {
        _cleanup("export_import_works_for_2_items1");
        _cleanup("export_import_works_for_2_items2");
        {
            let mut output: Vec<u8> = Vec::new();
            export(&_add_2_records(_wallet("export_import_works_for_2_items1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

            let wallet = _wallet("export_import_works_for_2_items2");
            _assert_is_empty(&wallet);

            import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
            _assert_has_2_records(&wallet);
        }
        _cleanup("export_import_works_for_2_items1");
        _cleanup("export_import_works_for_2_items2");
    }

    #[test]
    fn export_import_works_for_2_items_and_interactive_method() {
        _cleanup("export_import_works_for_2_items_and_interactive_method1");
        _cleanup("export_import_works_for_2_items_and_interactive_method2");
        {
            let mut output: Vec<u8> = Vec::new();
            {
                export(&_add_2_records(_wallet("export_import_works_for_2_items_and_interactive_method1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_INT).unwrap();
            }
            _cleanup("export_import_works_for_2_items_and_interactive_method1");

            let wallet = _wallet("export_import_works_for_2_items_and_interactive_method2");
            _assert_is_empty(&wallet);

            import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
            _assert_has_2_records(&wallet);
        }
        _cleanup("export_import_works_for_2_items_and_interactive_method2");
    }

    #[test]
    fn export_import_works_for_multiple_items() {
        _cleanup("export_import_works_for_multiple_items1");
        _cleanup("export_import_works_for_multiple_items2");
        {
            let mut output: Vec<u8> = Vec::new();
            export(&_add_300_records(_wallet("export_import_works_for_multiple_items1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

            let wallet = _wallet("export_import_works_for_multiple_items2");
            _assert_is_empty(&wallet);

            import(&wallet, &mut output.as_slice(), _passphrase()).unwrap();
            _assert_has_300_records(&wallet);
        }
        _cleanup("export_import_works_for_multiple_items1");
        _cleanup("export_import_works_for_multiple_items2");
    }

    #[test]
    fn import_works_for_empty() {
        _cleanup("import_works_for_empty");

        let res = import(&_wallet("import_works_for_empty"), &mut "".as_bytes(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_empty");
    }

    #[test]
    fn import_works_for_cut_header_length() {
        _cleanup("import_works_for_cut_header_length");

        let res = import(&_wallet("import_works_for_cut_header_length"), &mut "\x00".as_bytes(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_cut_header_length");
    }

    #[test]
    fn import_works_for_cut_header_body() {
        _cleanup("import_works_for_cut_header_body");

        let res = import(&_wallet("import_works_for_cut_header_body"), &mut "\x00\x20small".as_bytes(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_cut_header_body");
    }

    #[test]
    fn import_works_for_invalid_header_body() {
        _cleanup("import_works_for_invalid_header_body");

        let output = {
            let invalid_header = "invalid_header".as_bytes();
            let mut output: Vec<u8> = Vec::new();
            output.write_u32::<LittleEndian>(invalid_header.len() as u32).unwrap();
            output.write_all(invalid_header).unwrap();
            output.write_all(&hash(invalid_header).unwrap()).unwrap();
            output
        };

        let res = import(&_wallet("import_works_for_invalid_header_body"), &mut output.as_slice(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_invalid_header_body");
    }

    #[test]
    fn import_works_for_invalid_header_hash() {
        _cleanup("import_works_for_invalid_header_hash1");
        _cleanup("import_works_for_invalid_header_hash2");

        let mut output: Vec<u8> = Vec::new();
        export(&_wallet("import_works_for_invalid_header_hash1"), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

        // Modifying one of the bytes in the header hash
        let pos = (&mut output.as_slice()).read_u32::<LittleEndian>().unwrap() as usize + 2;
        _change_byte(&mut output, pos);

        let res = import(&mut _wallet("import_works_for_invalid_header_hash2"), &mut output.as_slice(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_invalid_header_hash1");
        _cleanup("import_works_for_invalid_header_hash2");
    }

    #[test]
    fn export_import_works_for_changed_record() {
        _cleanup("export_import_works_for_changed_record1");
        _cleanup("export_import_works_for_changed_record2");

        let mut output: Vec<u8> = Vec::new();
        export(&_add_300_records(_wallet("export_import_works_for_changed_record1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

        // Modifying one byte in the middle of encrypted part
        let pos = output.len() / 2;
        _change_byte(&mut output, pos);

        let res = import(&mut _wallet("export_import_works_for_changed_record2"), &mut output.as_slice(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("export_import_works_for_changed_record1");
        _cleanup("export_import_works_for_changed_record2");
    }

    #[test]
    fn import_works_for_data_cut() {
        _cleanup("import_works_for_data_cut1");
        _cleanup("import_works_for_data_cut2");

        let mut output: Vec<u8> = Vec::new();
        export(&_add_2_records(_wallet("import_works_for_data_cut1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

        output.pop().unwrap();

        let res = import(&mut _wallet("import_works_for_data_cut2"), &mut output.as_slice(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_data_cut1");
        _cleanup("import_works_for_data_cut2");
    }

    #[test]
    fn import_works_for_data_extended() {
        _cleanup("import_works_for_data_extended1");
        _cleanup("import_works_for_data_extended2");

        let mut output: Vec<u8> = Vec::new();
        export(&_add_2_records(_wallet("import_works_for_data_extended1")), &mut output, _passphrase(), _version1(), &KeyDerivationMethod::ARGON2I_MOD).unwrap();

        output.push(10);

        let res = import(&mut _wallet("import_works_for_data_extended2"), &mut output.as_slice(), _passphrase());
        assert_eq!(IndyErrorKind::InvalidStructure, res.unwrap_err().kind());
        _cleanup("import_works_for_data_extended1");
        _cleanup("import_works_for_data_extended2");
    }

    fn _cleanup(name: &str) {
        test::cleanup_storage(name)
    }

    fn _cleanup_wallet(wallet: &mut Wallet, name: &str) {
        wallet.close().unwrap();
        test::cleanup_wallet(name);
    }

    fn _wallet(id: &str) -> Wallet {
        let storage_type = SQLiteStorageType::new();
        let master_key = _master_key();
        let keys = Keys::new();

        let metadata = {
            let master_key_salt = encryption::gen_master_key_salt().unwrap();

            let metadata = Metadata::MetadataArgon(MetadataArgon {
                master_key_salt: master_key_salt[..].to_vec(),
                keys: keys.serialize_encrypted(&master_key).unwrap(),
            });

            serde_json::to_vec(&metadata)
                .to_indy(IndyErrorKind::InvalidState, "Cannot serialize wallet metadata").unwrap()
        };

        storage_type.create_storage(id,
                                    None,
                                    None,
                                    &metadata).unwrap();

        let storage = storage_type.open_storage(id, None, None).unwrap();

        Wallet::new(id.to_string(), storage, Rc::new(keys))
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

    fn _change_byte(data: &mut [u8], pos: usize) {
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
        0
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

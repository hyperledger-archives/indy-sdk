from .libindy import do_call, create_cb

from ctypes import *
from typing import Optional

import logging

def convert_to_pointer(fn):
    return fn

def get_fn_pointer(lib, fn_name: str):
    return getattr(lib, fn_name)

async def register_wallet_storage_library(storage_type: str, c_library: str, fn_pfx: str):
    """
    Register a wallet storage provider

    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("register_wallet_storage_library: >>> type: %r",
                 storage_type)

    stg_lib = CDLL(c_library)
    await register_wallet_storage(storage_type,
                                  get_fn_pointer(stg_lib, fn_pfx + "create"),
                                  get_fn_pointer(stg_lib, fn_pfx + "open"),
                                  get_fn_pointer(stg_lib, fn_pfx + "close"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete"),
                                  get_fn_pointer(stg_lib, fn_pfx + "add_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "update_record_value"),
                                  get_fn_pointer(stg_lib, fn_pfx + "update_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "add_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_id"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_type"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_value"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "set_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "search_records"),
                                  get_fn_pointer(stg_lib, fn_pfx + "search_all_records"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_search_total_count"),
                                  get_fn_pointer(stg_lib, fn_pfx + "fetch_search_next_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_search"))

    logger.debug("register_wallet_storage_library: <<<")


async def register_wallet_storage(storage_type: str,
                                  fn_create,
                                  fn_open,
                                  fn_close,
                                  fn_delete,
                                  fn_add_record,
                                  fn_update_record_value,
                                  fn_update_record_tags,
                                  fn_add_record_tags,
                                  fn_delete_record_tags,
                                  fn_delete_record,
                                  fn_get_record,
                                  fn_get_record_id,
                                  fn_get_record_type,
                                  fn_get_record_value,
                                  fn_get_record_tags,
                                  fn_free_record,
                                  fn_get_storage_metadata,
                                  fn_set_storage_metadata,
                                  fn_free_storage_metadata,
                                  fn_search_records,
                                  fn_search_all_records,
                                  fn_get_search_total_count,
                                  fn_fetch_search_next_record,
                                  fn_free_search) -> None:
    """
    Register a wallet storage provider

    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("register_wallet_storage: >>> type: %r",
                 storage_type)

    if not hasattr(register_wallet_storage, "cb"):
        logger.debug("register_wallet_storage: Creating callback")
        register_wallet_storage.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_storage_type = c_char_p(storage_type.encode('utf-8'))
    c_fn_create = convert_to_pointer(fn_create)
    c_fn_open = convert_to_pointer(fn_open)
    c_fn_close = convert_to_pointer(fn_close)
    c_fn_delete = convert_to_pointer(fn_delete)
    c_fn_add_record = convert_to_pointer(fn_add_record)
    c_fn_update_record_value = convert_to_pointer(fn_update_record_value)
    c_fn_update_record_tags = convert_to_pointer(fn_update_record_tags)
    c_fn_add_record_tags = convert_to_pointer(fn_add_record_tags)
    c_fn_delete_record_tags = convert_to_pointer(fn_delete_record_tags)
    c_fn_delete_record = convert_to_pointer(fn_delete_record)
    c_fn_get_record = convert_to_pointer(fn_get_record)
    c_fn_get_record_id = convert_to_pointer(fn_get_record_id)
    c_fn_get_record_type = convert_to_pointer(fn_get_record_type)
    c_fn_get_record_value = convert_to_pointer(fn_get_record_value)
    c_fn_get_record_tags = convert_to_pointer(fn_get_record_tags)
    c_fn_free_record = convert_to_pointer(fn_free_record)
    c_fn_get_storage_metadata = convert_to_pointer(fn_get_storage_metadata)
    c_fn_set_storage_metadata = convert_to_pointer(fn_set_storage_metadata)
    c_fn_free_storage_metadata = convert_to_pointer(fn_free_storage_metadata)
    c_fn_search_records = convert_to_pointer(fn_search_records)
    c_fn_search_all_records = convert_to_pointer(fn_search_all_records)
    c_fn_get_search_total_count = convert_to_pointer(fn_get_search_total_count)
    c_fn_fetch_search_next_record = convert_to_pointer(fn_fetch_search_next_record)
    c_fn_free_search = convert_to_pointer(fn_free_search)

    await do_call('indy_register_wallet_storage',
                  c_storage_type,
                  c_fn_create,
                  c_fn_open,
                  c_fn_close,
                  c_fn_delete,
                  c_fn_add_record,
                  c_fn_update_record_value,
                  c_fn_update_record_tags,
                  c_fn_add_record_tags,
                  c_fn_delete_record_tags,
                  c_fn_delete_record,
                  c_fn_get_record,
                  c_fn_get_record_id,
                  c_fn_get_record_type,
                  c_fn_get_record_value,
                  c_fn_get_record_tags,
                  c_fn_free_record,
                  c_fn_get_storage_metadata,
                  c_fn_set_storage_metadata,
                  c_fn_free_storage_metadata,
                  c_fn_search_records,
                  c_fn_search_all_records,
                  c_fn_get_search_total_count,
                  c_fn_fetch_search_next_record,
                  c_fn_free_search,
                  register_wallet_storage.cb)

    logger.debug("register_wallet_storage: <<<")


def convert_to_pointer(fn):
    return fn

def get_fn_pointer(lib, fn_name: str):
    return getattr(lib, fn_name)

async def register_wallet_storage_library(storage_type: str, c_library: str, fn_pfx: str):
    """
    Register a wallet storage provider

    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("register_wallet_storage_library: >>> type: %r",
                 storage_type)

    stg_lib = CDLL(c_library)
    await register_wallet_storage(storage_type,
                                  get_fn_pointer(stg_lib, fn_pfx + "create"),
                                  get_fn_pointer(stg_lib, fn_pfx + "open"),
                                  get_fn_pointer(stg_lib, fn_pfx + "close"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete"),
                                  get_fn_pointer(stg_lib, fn_pfx + "add_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "update_record_value"),
                                  get_fn_pointer(stg_lib, fn_pfx + "update_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "add_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "delete_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_id"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_type"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_value"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_record_tags"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "set_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_storage_metadata"),
                                  get_fn_pointer(stg_lib, fn_pfx + "search_records"),
                                  get_fn_pointer(stg_lib, fn_pfx + "search_all_records"),
                                  get_fn_pointer(stg_lib, fn_pfx + "get_search_total_count"),
                                  get_fn_pointer(stg_lib, fn_pfx + "fetch_search_next_record"),
                                  get_fn_pointer(stg_lib, fn_pfx + "free_search"))

    logger.debug("register_wallet_storage_library: <<<")


async def register_wallet_storage(storage_type: str,
                                  fn_create,
                                  fn_open,
                                  fn_close,
                                  fn_delete,
                                  fn_add_record,
                                  fn_update_record_value,
                                  fn_update_record_tags,
                                  fn_add_record_tags,
                                  fn_delete_record_tags,
                                  fn_delete_record,
                                  fn_get_record,
                                  fn_get_record_id,
                                  fn_get_record_type,
                                  fn_get_record_value,
                                  fn_get_record_tags,
                                  fn_free_record,
                                  fn_get_storage_metadata,
                                  fn_set_storage_metadata,
                                  fn_free_storage_metadata,
                                  fn_search_records,
                                  fn_search_all_records,
                                  fn_get_search_total_count,
                                  fn_fetch_search_next_record,
                                  fn_free_search) -> None:
    """
    Register a wallet storage provider

    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("register_wallet_storage: >>> type: %r",
                 storage_type)

    if not hasattr(register_wallet_storage, "cb"):
        logger.debug("register_wallet_storage: Creating callback")
        register_wallet_storage.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_storage_type = c_char_p(storage_type.encode('utf-8'))
    c_fn_create = convert_to_pointer(fn_create)
    c_fn_open = convert_to_pointer(fn_open)
    c_fn_close = convert_to_pointer(fn_close)
    c_fn_delete = convert_to_pointer(fn_delete)
    c_fn_add_record = convert_to_pointer(fn_add_record)
    c_fn_update_record_value = convert_to_pointer(fn_update_record_value)
    c_fn_update_record_tags = convert_to_pointer(fn_update_record_tags)
    c_fn_add_record_tags = convert_to_pointer(fn_add_record_tags)
    c_fn_delete_record_tags = convert_to_pointer(fn_delete_record_tags)
    c_fn_delete_record = convert_to_pointer(fn_delete_record)
    c_fn_get_record = convert_to_pointer(fn_get_record)
    c_fn_get_record_id = convert_to_pointer(fn_get_record_id)
    c_fn_get_record_type = convert_to_pointer(fn_get_record_type)
    c_fn_get_record_value = convert_to_pointer(fn_get_record_value)
    c_fn_get_record_tags = convert_to_pointer(fn_get_record_tags)
    c_fn_free_record = convert_to_pointer(fn_free_record)
    c_fn_get_storage_metadata = convert_to_pointer(fn_get_storage_metadata)
    c_fn_set_storage_metadata = convert_to_pointer(fn_set_storage_metadata)
    c_fn_free_storage_metadata = convert_to_pointer(fn_free_storage_metadata)
    c_fn_search_records = convert_to_pointer(fn_search_records)
    c_fn_search_all_records = convert_to_pointer(fn_search_all_records)
    c_fn_get_search_total_count = convert_to_pointer(fn_get_search_total_count)
    c_fn_fetch_search_next_record = convert_to_pointer(fn_fetch_search_next_record)
    c_fn_free_search = convert_to_pointer(fn_free_search)

    await do_call('indy_register_wallet_storage',
                  c_storage_type,
                  c_fn_create,
                  c_fn_open,
                  c_fn_close,
                  c_fn_delete,
                  c_fn_add_record,
                  c_fn_update_record_value,
                  c_fn_update_record_tags,
                  c_fn_add_record_tags,
                  c_fn_delete_record_tags,
                  c_fn_delete_record,
                  c_fn_get_record,
                  c_fn_get_record_id,
                  c_fn_get_record_type,
                  c_fn_get_record_value,
                  c_fn_get_record_tags,
                  c_fn_free_record,
                  c_fn_get_storage_metadata,
                  c_fn_set_storage_metadata,
                  c_fn_free_storage_metadata,
                  c_fn_search_records,
                  c_fn_search_all_records,
                  c_fn_get_search_total_count,
                  c_fn_fetch_search_next_record,
                  c_fn_free_search,
                  register_wallet_storage.cb)

    logger.debug("register_wallet_storage: <<<")


async def create_wallet(config: str,
                        credentials: str) -> None:
    """
    Creates a new secure wallet with the given unique name.

    :param config: Wallet configuration json.
     {
       "id": string, Identifier of the wallet.
             Configured storage uses this identifier to lookup exact wallet data placement.
       "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                      'Default' storage type allows to store wallet data in the local file.
                      Custom storage types can be registered with indy_register_wallet_storage call.
       "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                         Can be optional if storage supports default configuration.
                          For 'default' storage type configuration is:
       {
         "path": optional<string>, Path to the directory with wallet files.
                 Defaults to $HOME/.indy_client/wallet.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Key or passphrase used for wallet key derivation.
                      Look to key_derivation_method param for information about supported key derivation methods.
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                               For 'default' storage type should be empty.
       "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                                ARGON2I_MOD - derive secured wallet master key (used by default)
                                ARGON2I_INT - derive secured wallet master key (less secured but faster)
                                RAW - raw wallet key master provided (skip derivation).
                                      RAW keys can be generated with generate_wallet_key call
     }
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_wallet: >>> config: %r, credentials: %r",
                 config,
                 credentials)

    if not hasattr(create_wallet, "cb"):
        logger.debug("create_wallet: Creating callback")
        create_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config = c_char_p(config.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8'))

    await do_call('indy_create_wallet',
                  c_config,
                  c_credentials,
                  create_wallet.cb)

    logger.debug("create_wallet: <<<")


async def open_wallet(config: str,
                      credentials: str) -> int:
    """
    Opens the wallet with specific name.
    Wallet with corresponded name must be previously created with indy_create_wallet method.
    It is impossible to open wallet with the same name more than once.

    :param name: Name of the wallet.
    :param config: Wallet configuration json.
    {
       "id": string, Identifier of the wallet.
             Configured storage uses this identifier to lookup exact wallet data placement.
       "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                       'Default' storage type allows to store wallet data in the local file.
                       Custom storage types can be registered with indy_register_wallet_storage call.
       "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                         Can be optional if storage supports default configuration.
                          For 'default' storage type configuration is:
           {
              "path": optional<string>, Path to the directory with wallet files.
                      Defaults to $HOME/.indy_client/wallet.
                      Wallet will be stored in the file {path}/{id}/sqlite.db
           }

    }
    :param credentials: Wallet credentials json
    {
       "key": string, Key or passphrase used for wallet key derivation.
                      Look to key_derivation_method param for information about supported key derivation methods.
       "rekey": optional<string>, If present, then wallet master key will be rotated to a new one.
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.
       "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                               ARGON2I_MOD - derive secured wallet master key (used by default)
                               ARGON2I_INT - derive secured wallet master key (less secured but faster)
                               RAW - raw wallet master key provided (skip derivation)
       "rekey_derivation_method": optional<string> algorithm to use for master rekey derivation:
                               ARGON2I_MOD - derive secured wallet master rekey (used by default)
                               ARGON2I_INT - derive secured wallet master rekey (less secured but faster)
                               RAW - raw wallet rekey master provided (skip derivation).
                                     RAW keys can be generated with generate_wallet_key call
    }
    :return: Handle to opened wallet to use in methods that require wallet access.
    """

    logger = logging.getLogger(__name__)
    logger.debug("open_wallet: >>> config: %r, credentials: %r",
                 config,
                 credentials)

    if not hasattr(open_wallet, "cb"):
        logger.debug("open_wallet: Creating callback")
        open_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_config = c_char_p(config.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8'))

    res = await do_call('indy_open_wallet',
                        c_config,
                        c_credentials,
                        open_wallet.cb)

    logger.debug("open_wallet: <<< res: %r", res)
    return res


async def close_wallet(handle: int) -> None:
    """
    Closes opened wallet and frees allocated resources.

    :param handle: wallet handle returned by indy_open_wallet.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("close_wallet: >>> handle: %i", handle)

    if not hasattr(close_wallet, "cb"):
        logger.debug("close_wallet: Creating callback")
        close_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    await do_call('indy_close_wallet',
                  c_handle,
                  close_wallet.cb)

    logger.debug("close_wallet: <<<")


async def delete_wallet(config: str,
                        credentials: str) -> None:
    """
    Deletes created wallet.

    :param config: Wallet configuration json.
     {
       "id": string, Identifier of the wallet.
             Configured storage uses this identifier to lookup exact wallet data placement.
       "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                      'Default' storage type allows to store wallet data in the local file.
                      Custom storage types can be registered with indy_register_wallet_storage call.
       "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                         Can be optional if storage supports default configuration.
                          For 'default' storage type configuration is:
       {
         "path": optional<string>, Path to the directory with wallet files.
                 Defaults to $HOME/.indy_client/wallet.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Key or passphrase used for wallet key derivation.
                      Look to key_derivation_method param for information about supported key derivation methods.
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.
       "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                                ARGON2I_MOD - derive secured wallet master key (used by default)
                                ARGON2I_INT - derive secured wallet master key (less secured but faster)
                                RAW - raw wallet key master provided (skip derivation).
                                      RAW keys can be generated with generate_wallet_key call
     }
    :return:
    """

    logger = logging.getLogger(__name__)
    logger.debug("delete_wallet: >>> config: %r, credentials: %r",
                 config,
                 credentials)

    if not hasattr(delete_wallet, "cb"):
        logger.debug("delete_wallet: Creating callback")
        delete_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config = c_char_p(config.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8'))

    await do_call('indy_delete_wallet',
                  c_config,
                  c_credentials,
                  delete_wallet.cb)

    logger.debug("delete_wallet: <<<")


async def export_wallet(handle: int,
                        export_config_json: str) -> None:
    """
    Exports opened wallet to the file.

    :param handle: wallet handle returned by indy_open_wallet.
    :param export_config_json: JSON containing settings for input operation.
       {
          "path": path of the file that contains exported wallet content
          "key": string, Key or passphrase used for wallet export key derivation.
                         Look to key_derivation_method param for information about supported key derivation methods.
          "key_derivation_method": optional<string> algorithm to use for export key derivation:
                                ARGON2I_MOD - derive secured wallet export key (used by default)
                                ARGON2I_INT - derive secured wallet export key (less secured but faster)
                                RAW - raw wallet export key provided (skip derivation).
                                      RAW keys can be generated with generate_wallet_key call
       }
    :return:
    """

    logger = logging.getLogger(__name__)
    logger.debug("export_wallet: >>> handle: %r, export_config_json: %r",
                 handle,
                 export_config_json)

    if not hasattr(export_wallet, "cb"):
        logger.debug("export_wallet: Creating callback")
        export_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_export_config_json = c_char_p(export_config_json.encode('utf-8'))

    await do_call('indy_export_wallet',
                  handle,
                  c_export_config_json,
                  export_wallet.cb)

    logger.debug("export_wallet: <<<")


async def import_wallet(config: str,
                        credentials: str,
                        import_config_json: str) -> None:
    """
    Creates a new secure wallet with the given unique name and then imports its content
    according to fields provided in import_config
    This can be seen as an indy_create_wallet call with additional content import

    :param config: Wallet configuration json.
     {
       "id": string, Identifier of the wallet.
             Configured storage uses this identifier to lookup exact wallet data placement.
       "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                      'Default' storage type allows to store wallet data in the local file.
                      Custom storage types can be registered with indy_register_wallet_storage call.
       "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                         Can be optional if storage supports default configuration.
                         For 'default' storage type configuration is:
       {
         "path": optional<string>, Path to the directory with wallet files.
                 Defaults to $HOME/.indy_client/wallet.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Key or passphrase used for wallet key derivation.
                      Look to key_derivation_method param for information about supported key derivation methods.
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.
       "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                                 ARGON2I_MOD - derive secured wallet master key (used by default)
                                 ARGON2I_INT - derive secured wallet master key (less secured but faster)
                                 RAW - raw wallet key master provided (skip derivation).
                                       RAW keys can be generated with generate_wallet_key call
     }
    :param import_config_json: JSON containing settings for input operationЖ {
     "path": path of the file that contains exported wallet content
     "key": key used for export of the wallet
   }
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("import_wallet: >>> config: %r, credentials: %r, import_config_json: %r",
                 config,
                 credentials,
                 import_config_json)

    if not hasattr(import_wallet, "cb"):
        logger.debug("import_wallet: Creating callback")
        import_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config = c_char_p(config.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8'))
    c_import_config_json = c_char_p(import_config_json.encode('utf-8'))

    await do_call('indy_import_wallet',
                  c_config,
                  c_credentials,
                  c_import_config_json,
                  import_wallet.cb)

    logger.debug("import_wallet: <<<")


async def generate_wallet_key(config: Optional[str]) -> str:
    """
    Generate wallet master key.
    Returned key is compatible with "RAW" key derivation method.
    It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.

    :param config: (optional) key configuration json.
     {
        "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
                                   Can be UTF-8, base64 or hex string.
     }
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("generate_wallet_key: >>> config: %r",
                 config)

    if not hasattr(generate_wallet_key, "cb"):
        logger.debug("generate_wallet_key: Creating callback")
        generate_wallet_key.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_config = c_char_p(config.encode('utf-8')) if config is not None else None

    key = await do_call('indy_generate_wallet_key',
                        c_config,
                        generate_wallet_key.cb)

    res = key.decode()

    logger.debug("generate_wallet_key: <<< res: %r", res)
    return res

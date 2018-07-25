from .libindy import do_call, create_cb

from ctypes import *

import logging


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
                 Defaults to $HOME/.indy_client/wallets.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Passphrase used to derive wallet master key
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                               For 'default' storage type should be empty.

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
                      Defaults to $HOME/.indy_client/wallets.
                      Wallet will be stored in the file {path}/{id}/sqlite.db
           }

    }
    :param credentials: Wallet credentials json
    {
       "key": string, Passphrase used to derive current wallet master key
       "rekey": optional<string>, If present than wallet master key will be rotated to a new one
                                  derived from this passphrase.
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.

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
                 Defaults to $HOME/.indy_client/wallets.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Passphrase used to derive wallet master key
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.

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
          "key": passphrase used to export key
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
                 Defaults to $HOME/.indy_client/wallets.
                 Wallet will be stored in the file {path}/{id}/sqlite.db
       }
     }
    :param credentials: Wallet credentials json
     {
       "key": string, Passphrase used to derive wallet master key
       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                              Can be optional if storage supports default configuration.
                              For 'default' storage type should be empty.

     }
    :param import_config_json: JSON containing settings for input operationЖ {
     "path": path of the file that contains exported wallet content
     "key": passphrase used to export key
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

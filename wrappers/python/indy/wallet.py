from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def create_wallet(pool_name: str,
                        name: str,
                        xtype: Optional[str],
                        config: Optional[str],
                        credentials: str) -> None:
    """
    Creates a new secure wallet with the given unique name.

    :param pool_name: Name of the pool that corresponds to this wallet.
    :param name: Name of the wallet.
    :param xtype: (optional) Type of the wallet. Defaults to 'default'.
     Custom types can be registered with indy_register_wallet_type call.
    :param config: (optional) Wallet configuration json. List of supported keys are defined by wallet type.
     if NULL, then default config will be used.
    :param credentials: Wallet credentials json: {
        "key": <wallet_key>
    }
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_wallet: >>> pool_name: %r, name: %r, xtype: %r, config: %r, credentials: %r",
                 pool_name,
                 name,
                 xtype,
                 config,
                 credentials)

    if not hasattr(create_wallet, "cb"):
        logger.debug("create_wallet: Creating callback")
        create_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_pool_name = c_char_p(pool_name.encode('utf-8'))
    c_name = c_char_p(name.encode('utf-8'))
    c_xtype = c_char_p(xtype.encode('utf-8')) if xtype is not None else None
    c_config = c_char_p(config.encode('utf-8')) if config is not None else None
    c_credentials = c_char_p(credentials.encode('utf-8'))

    await do_call('indy_create_wallet',
                  c_pool_name,
                  c_name,
                  c_xtype,
                  c_config,
                  c_credentials,
                  create_wallet.cb)

    logger.debug("create_wallet: <<<")


async def open_wallet(name: str,
                      runtime_config: Optional[str],
                      credentials: str) -> int:
    """
    Opens the wallet with specific name.
    Wallet with corresponded name must be previously created with indy_create_wallet method.
    It is impossible to open wallet with the same name more than once.

    :param name: Name of the wallet.
    :param runtime_config: (optional) Runtime wallet configuration json.
     if NULL, then default runtime_config will be used. Example:
        {
            "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
            ... List of additional supported keys are defined by wallet type.
        }
    :param credentials: Wallet credentials json: {
        "key": <wallet_key>
    }
    :return: Handle to opened wallet to use in methods that require wallet access.
    """

    logger = logging.getLogger(__name__)
    logger.debug("open_wallet: >>> name: %r, runtime_config: %r, credentials: %r",
                 name,
                 runtime_config,
                 credentials)

    if not hasattr(open_wallet, "cb"):
        logger.debug("open_wallet: Creating callback")
        open_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_name = c_char_p(name.encode('utf-8'))
    c_runtime_config = c_char_p(runtime_config.encode('utf-8')) if runtime_config is not None else None
    c_credentials = c_char_p(credentials.encode('utf-8'))

    res = await do_call('indy_open_wallet',
                        c_name,
                        c_runtime_config,
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


async def delete_wallet(name: str,
                        credentials: str) -> None:
    """
    Deletes created wallet.

    :param name: Name of the wallet to delete.
    :param credentials: Wallet credentials json: {
        "key": <wallet_key>
    }
    :return:
    """

    logger = logging.getLogger(__name__)
    logger.debug("delete_wallet: >>> name: %r, credentials: %r",
                 name,
                 credentials)

    if not hasattr(delete_wallet, "cb"):
        logger.debug("delete_wallet: Creating callback")
        delete_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_name = c_char_p(name.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8'))

    await do_call('indy_delete_wallet',
                  c_name,
                  c_credentials,
                  delete_wallet.cb)

    logger.debug("delete_wallet: <<<")

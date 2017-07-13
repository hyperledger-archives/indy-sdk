from .libindy import LibIndy
from .error import ErrorCode, IndyError

from ctypes import *
from asyncio import get_event_loop

import logging


class Wallet(object):
    _XCOMMAND_HANDLE = 0

    _CREATE_WALLET_CB_TYPE = CFUNCTYPE(None, c_int32, c_int32)

    @staticmethod
    async def create_wallet(pool_name: str,
                            name: str,
                            xtype: str,
                            config: str,
                            credentials: str) -> None:
        logger = logging.getLogger(__name__)

        c_comman_handle = c_int32(Wallet._XCOMMAND_HANDLE)
        c_pool_name = c_char_p(pool_name.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_xtype = c_char_p(xtype.encode('utf-8')) if xtype is not None else None
        c_config = c_char_p(config.encode('utf-8')) if config is not None else None
        c_credentials = c_char_p(credentials.encode('utf-8')) if credentials is not None else None

        event_loop = get_event_loop()
        future = event_loop.create_future()

        def create_wallet_cb(xcommand_handle: int, err: int):
            logger.debug("create_wallet_cb called, xcommand_handle: %i, err: %i", xcommand_handle, err)
            event_loop.call_soon_threadsafe(create_wallet_loop_cb, err)

        def create_wallet_loop_cb(err):
            logger.debug("create_wallet_loop_cb called, err: %i", err)
            future.set_result(err)

        c_cb = Wallet._CREATE_WALLET_CB_TYPE(create_wallet_cb)

        res = LibIndy.cdll().indy_create_wallet(c_comman_handle,
                                              c_pool_name,
                                              c_name,
                                              c_xtype,
                                              c_config,
                                              c_credentials,
                                              c_cb)

        logger.debug("cdll.indy_create_wallet, res: %i", res)
        if res != ErrorCode.Success:
            raise IndyError(res)

        res = await future

        logger.debug("cdll.indy_create_wallet future, res: %i", res)
        if ErrorCode(res) != ErrorCode.Success:
            raise IndyError(res)

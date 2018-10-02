import asyncio
import json
import logging
import sys
from os import environ
from pathlib import Path
from shutil import rmtree
from tempfile import gettempdir

from ctypes import *

import pytest

from indy import wallet
from indy.error import IndyError, ErrorCode

logging.basicConfig(level=logging.DEBUG)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_register_custom_storage_works():
    logger = logging.getLogger(__name__)

    wallet_config = '{"id":"wallet1c", "storage_type":"custom_inmem"}'
    wallet_credentials = '{"key":"key"}'

    # load dynamic library
    logger.debug("register_wallet: Load library with custom storage")
    if sys.platform == 'darwin':
        f_ext = 'dylib'
    else:
        f_ext = 'so'
    inmemlib = CDLL("../../samples/storage/storage-inmem/target/debug/libindystrginmem." + f_ext)

    await wallet.register_wallet_storage("custom_inmem",
                                         inmemlib.inmemwallet_fn_create,
                                         inmemlib.inmemwallet_fn_open,
                                         inmemlib.inmemwallet_fn_close,
                                         inmemlib.inmemwallet_fn_delete,
                                         inmemlib.inmemwallet_fn_add_record,
                                         inmemlib.inmemwallet_fn_update_record_value,
                                         inmemlib.inmemwallet_fn_update_record_tags,
                                         inmemlib.inmemwallet_fn_add_record_tags,
                                         inmemlib.inmemwallet_fn_delete_record_tags,
                                         inmemlib.inmemwallet_fn_delete_record,
                                         inmemlib.inmemwallet_fn_get_record,
                                         inmemlib.inmemwallet_fn_get_record_id,
                                         inmemlib.inmemwallet_fn_get_record_type,
                                         inmemlib.inmemwallet_fn_get_record_value,
                                         inmemlib.inmemwallet_fn_get_record_tags,
                                         inmemlib.inmemwallet_fn_free_record,
                                         inmemlib.inmemwallet_fn_get_storage_metadata,
                                         inmemlib.inmemwallet_fn_set_storage_metadata,
                                         inmemlib.inmemwallet_fn_free_storage_metadata,
                                         inmemlib.inmemwallet_fn_search_records,
                                         inmemlib.inmemwallet_fn_search_all_records,
                                         inmemlib.inmemwallet_fn_get_search_total_count,
                                         inmemlib.inmemwallet_fn_fetch_search_next_record,
                                         inmemlib.inmemwallet_fn_free_search)

    # create/open/close/delete wallet
    logger.debug("register_wallet: Creating wallet")
    await wallet.create_wallet(wallet_config, wallet_credentials)

    logger.debug("register_wallet: Opening wallet")
    wallet_handle = await wallet.open_wallet(wallet_config, wallet_credentials)
    assert type(wallet_handle) is int

    logger.debug("register_wallet: Closing wallet")
    await wallet.close_wallet(wallet_handle)

    logger.debug("register_wallet: Deleting wallet")
    await wallet.delete_wallet(wallet_config, wallet_credentials)

    logger.debug("register_wallet: Done!")

    pass


@pytest.mark.asyncio
async def test_register_custom_storage_library_works():
    logger = logging.getLogger(__name__)

    wallet_config = '{"id":"wallet1c", "storage_type":"custom_inmem2"}'
    wallet_credentials = '{"key":"key"}'

    # register custom wallet storage
    logger.debug("register_wallet: Register custom wallet storage")
    if sys.platform == 'darwin':
        f_ext = 'dylib'
    else:
        f_ext = 'so'
    await wallet.register_wallet_storage_library("custom_inmem2",
                                "../../samples/storage/storage-inmem/target/debug/libindystrginmem." + f_ext,
                                "inmemwallet_fn_")

    # create/open/close/delete wallet
    logger.debug("register_wallet: Creating wallet")
    await wallet.create_wallet(wallet_config, wallet_credentials)

    logger.debug("register_wallet: Opening wallet")
    wallet_handle = await wallet.open_wallet(wallet_config, wallet_credentials)
    assert type(wallet_handle) is int

    logger.debug("register_wallet: Closing wallet")
    await wallet.close_wallet(wallet_handle)

    logger.debug("register_wallet: Deleting wallet")
    await wallet.delete_wallet(wallet_config, wallet_credentials)

    logger.debug("register_wallet: Done!")

    pass



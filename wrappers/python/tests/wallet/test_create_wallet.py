import indy

from ..utils.storage import StorageUtils

import logging
import pytest

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_indy_create_wallet_works():
    logging.error("test!")

    StorageUtils.cleanup()

    pool_name = 'indy_create_wallet_works'
    wallet_name = 'indy_create_wallet_works'
    xtype = 'default'

    await indy.Wallet.create_wallet(pool_name, wallet_name, xtype, None, None)

    StorageUtils.cleanup()
